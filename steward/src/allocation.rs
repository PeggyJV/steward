use crate::{cellars, error::Error, prelude::*, somm_send};
use abscissa_core::Application;
use deep_space::{private_key::PrivateKey, Coin, Contact};
use ethers::{prelude::U256, types::H160};
use somm_proto::{somm, somm::query_client::QueryClient as AllocationQueryClient};
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tonic::transport::Channel;
use rand_core::{OsRng, RngCore};

// TO-DO Remove the need for options here
pub struct Connections {
    pub grpc: AllocationQueryClient<Channel>,
    pub contact: Contact,
}

fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    salt
}

pub async fn allocation_precommit(
    cosmos_key: &PrivateKey,
    allocation: &somm::Allocation,
    pair_id: String,
) -> somm::AllocationPrecommit {
    let config = APP.config();
    let delegate_cosmos_address = cosmos_key
        .to_address(&config.cosmos.prefix)
        .unwrap()
        .to_string();
    let hasher = somm_send::data_hash(allocation, delegate_cosmos_address)
        .await
        .unwrap();
    somm::AllocationPrecommit {
        hash: hasher.hash,
        cellar_id: pair_id,
    }
}

pub async fn get_connections() -> Result<Connections, Error> {
    let config = APP.config();
    let timeout = Duration::from_secs(10);
    let try_base = AllocationQueryClient::connect(config.cosmos.grpc.clone()).await;
    let (grpc, contact) = match try_base {
        Ok(val) => (
            val,
            Contact::new(&config.cosmos.grpc, timeout, &config.cosmos.prefix).unwrap(),
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

pub async fn decide_rebalance(tick_range: Vec<somm::TickRange>, cellar_address: H160, pair_id: String) {
    if std::env::var("CELLAR_DRY_RUN").expect("Expect CELLAR_DRY_RUN var") == "TRUE" {
        ()
    } else {
        let mut connections = get_connections().await.unwrap();
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
        let allocation = to_allocation(tick_range, pair_id.to_string(), eth_gas_price.as_u64());

        let name = &config.keys.rebalancer_key;
        let cosmos_key = config.load_deep_space_key(name.clone());
        let delegate_cosmos_address = cosmos_key.to_address(&contact.get_prefix()).unwrap();

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
            Ok(val) => {
                // Sending Pre-commits
                somm_send::send_precommit(
                    &contact,
                    delegate_cosmos_address,
                    cosmos_key,
                    fee.clone(),
                    vec![allocation_precommit(&cosmos_key, &allocation, pair_id.to_string()).await],
                )
                .await
                .unwrap();
            }
            Err(e) => {
                println!("Couldn't Send Precommit {:?}", e);
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
            Ok(val) => {
                // Sending Commits
                somm_send::send_allocation(
                    &contact,
                    delegate_cosmos_address,
                    cosmos_key,
                    fee,
                    vec![allocation],
                )
                .await
                .unwrap();
            }
            Err(e) => {
                println!("Couldn't Send Commit {:?}", e);
            }
        }
    }
}

pub fn to_allocation(
    tick_ranges: Vec<somm::TickRange>,
    pair_id: String,
    eth_gas_price: u64,
) -> somm::Allocation {
    let salt = &generate_salt();
    let salt = match std::str::from_utf8(salt) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    somm::Allocation {
        vote: Some(somm::RebalanceVote {
            cellar: Some(somm::Cellar {
                id: pair_id,
                tick_ranges: tick_ranges,
            }),
            current_price: eth_gas_price,
        }),
        salt: salt.to_string(),
    }
}
