use crate::{cellars, error::Error, prelude::*, somm_send};
use abscissa_core::Application;
use deep_space::{private_key::PrivateKey, Coin, Contact};
use ethers::types::{Address as EthAddress, H160};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use somm_proto::{somm, somm::query_client::QueryClient as AllocationQueryClient};
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tonic::transport::Channel;

pub struct Connections {
    pub grpc: AllocationQueryClient<Channel>,
    pub contact: Contact,
}

pub fn format_eth_address(address: EthAddress) -> String {
    format!("0x{}", bytes_to_hex_str(address.as_bytes()))
}

pub fn bytes_to_hex_str(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:0>2x?}", b))
        .fold(String::new(), |acc, x| acc + &x)
}

pub async fn allocation_precommit(
    cosmos_key: &PrivateKey,
    allocation: &somm::Allocation,
    cellar_address: String,
) -> Result<somm::AllocationPrecommit, Error> {
    let config = APP.config();
    let delegate_cosmos_address = cosmos_key.to_address(&config.cosmos.prefix)?.to_string();
    let hasher = somm_send::data_hash(allocation, delegate_cosmos_address).await?;
    Ok(somm::AllocationPrecommit {
        hash: hasher.hash,
        cellar_id: cellar_address,
    })
}

pub async fn get_connections() -> Result<Connections, Error> {
    let config = APP.config();
    let timeout = Duration::from_secs(10);
    let try_base = AllocationQueryClient::connect(config.cosmos.grpc.clone()).await;
    let (grpc, contact) = match try_base {
        Ok(val) => (
            val,
            Contact::new(&config.cosmos.grpc, timeout, &config.cosmos.prefix)?,
        ),
        Err(e) => {
            error!(
                "Failed to access Cosmos gRPC with {:?} and create connections",
                e
            );
            return Err(Error::from(e));
        }
    };
    Ok(Connections { grpc, contact })
}

pub async fn decide_rebalance(
    tick_range: Vec<somm::TickRange>,
    cellar_address: H160,
) -> Result<(), Error> {
    if std::env::var("CELLAR_DRY_RUN").expect("Expect CELLAR_DRY_RUN var") == "TRUE" {
        return Ok(());
    } else {
        let mut connections = get_connections().await?;
        let (contact, grpc_client) = (connections.contact, &mut connections.grpc);

        let eth_gas_price = match cellars::get_gas_price().await {
            Ok(gp) => gp,
            Err(err) => {
                error!("Failed to get cellar gas price: {:?}", err);
                // TO-DO handle this better
                0.into()
            }
        };
        let config = APP.config();
        let allocation = to_allocation(
            tick_range,
            format_eth_address(cellar_address),
            eth_gas_price.as_u64(),
        );

        let name = &config.keys.rebalancer_key;
        let cosmos_key = config.load_deep_space_key(name.clone());
        let delegate_cosmos_address = cosmos_key.to_address(&contact.get_prefix())?;

        let cosmos_gas_price = config.cosmos.gas_price.as_tuple();
        let fee = Coin {
            denom: cosmos_gas_price.1,
            amount: 0u32.into(),
        };

        match timeout(Duration::from_secs(100), async {
            loop {
                // Waiting for new vote period
                let res = somm_send::query_commit_period(grpc_client).await;
                if let Ok(val) = res {
                    if val.vote_period_start == val.current_height {
                        break val;
                    }
                }
                sleep(Duration::from_secs(1)).await;
            }
        })
        .await
        {
            Ok(_) => {
                // Sending Pre-commits
                somm_send::send_precommit(
                    &contact,
                    delegate_cosmos_address,
                    cosmos_key,
                    fee.clone(),
                    vec![
                        allocation_precommit(
                            &cosmos_key,
                            &allocation,
                            format_eth_address(cellar_address),
                        )
                        .await?,
                    ],
                )
                .await?;
            }
            Err(e) => {
                error!("Couldn't Send Precommit {:?}", e);
            }
        }

        match timeout(Duration::from_secs(100), async {
            loop {
                // Checking Pre-commits for validators
                let res = somm_send::query_allocation_precommits(grpc_client).await;
                if let Ok(val) = res {
                    break val;
                }
                sleep(Duration::from_secs(1)).await;
            }
        })
        .await
        {
            Ok(_) => {
                // Sending Commits
                somm_send::send_allocation(
                    &contact,
                    delegate_cosmos_address,
                    cosmos_key,
                    fee,
                    vec![allocation],
                )
                .await?;
            }
            Err(e) => {
                error!("Couldn't Send Commit {:?}", e);
            }
        }
    }

    Ok(())
}

pub fn to_allocation(
    tick_ranges: Vec<somm::TickRange>,
    cellar_address: String,
    eth_gas_price: u64,
) -> somm::Allocation {
    somm::Allocation {
        vote: Some(somm::RebalanceVote {
            cellar: Some(somm::Cellar {
                id: cellar_address,
                tick_ranges: tick_ranges,
            }),
            current_price: eth_gas_price,
        }),
        salt: OsRng
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect(),
    }
}
