use crate::{cellars, error::Error, prelude::*, somm_send, utils};
use abscissa_core::{Application, tracing::field::debug};
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

pub async fn allocation_precommit(
    delegate_address: String,
    allocation: &somm::Allocation,
    cellar_address: String,
) -> Result<somm::AllocationPrecommit, Error> {
    let hasher = somm_send::data_hash(allocation, delegate_address).await?;

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
    cellar_address: String,
) -> Result<(), Error> {
    if std::env::var("CELLAR_DRY_RUN").expect("Expect CELLAR_DRY_RUN var") == "TRUE" {
        return Ok(());
    } else {
        debug!("deciding rebalance for cellar address {}", cellar_address);
        let mut connections = get_connections().await?;
        let (contact, grpc_client) = (connections.contact, &mut connections.grpc);

        debug!("getting eth gas price");
        let eth_gas_price = match cellars::get_gas_price().await {
            Ok(gp) => {
                debug!("eth gas price is {}", gp);
                gp
            },
            Err(err) => {
                error!("Failed to get cellar gas price: {:?}", err);
                // TO-DO handle this better
                0.into()
            }
        };
        let config = APP.config();

        debug!("building allocation message from tick range {:?}", tick_range);
        let allocation = to_allocation(
            tick_range,
            cellar_address.clone(),
            eth_gas_price.as_u64(),
        );

        let name = &config.keys.rebalancer_key;
        let cosmos_key = config.load_deep_space_key(name.clone());
        let delegate_cosmos_address = cosmos_key.to_address(&config.cosmos.prefix)?;
        debug!("precommit will be signed by {}", delegate_cosmos_address);

        let cosmos_gas_price = config.cosmos.gas_price.as_tuple();
        let fee = Coin {
            amount: (cosmos_gas_price.0 as u64).into(),
            denom: cosmos_gas_price.1,
        };
        debug!("cosmos fee: {:?}", fee);

        let allocation_precommit = &allocation_precommit(delegate_cosmos_address.to_string(), &allocation, cellar_address).await?;

        match timeout(Duration::from_secs(100), async {
            debug!("waiting for new vote period to start...");
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
                debug!("sending allocation precommit");
                // Sending Pre-commits
                let response = somm_send::send_precommit(
                    &contact,
                    delegate_cosmos_address,
                    cosmos_key,
                    fee.clone(),
                    vec![allocation_precommit.to_owned()],
                )
                .await?;
                debug!("precommit response: {:?}", response);
            }
            Err(e) => {
                error!("Couldn't Send Precommit {:?}", e);
            }
        }

        match timeout(Duration::from_secs(100), async {
            debug!("querying precommit");
            loop {
                // Checking Pre-commits for validators
                let res = somm_send::query_allocation_precommits(grpc_client).await;
                match res {
                    Ok(val) => {
                        if val.len() > 0 && val.contains(allocation_precommit) {
                            break val;
                        }
                    },
                    Err(err) => error!("error querying precommit: {}", err)
                }
                sleep(Duration::from_secs(1)).await;
            }
        })
        .await
        {
            Ok(_) => {
                debug!("precommit seen. sending allocation commit");
                // Sending Commits
                let response = somm_send::send_allocation(
                    &contact,
                    delegate_cosmos_address,
                    cosmos_key,
                    fee,
                    vec![allocation],
                )
                .await?;
                debug!("commit response: {:?}", response);
            }
            Err(e) => {
                error!("Couldn't Send Commit {:?}", e);
            }
        }
    }

    debug!("allocation workflow complete");
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
