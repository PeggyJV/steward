use crate::{cellars::{uniswapv3::UniswapV3CellarTickInfo}, somm_send, prelude::*};
use abscissa_core::Application;
use deep_space::{Contact, Coin};
use ethers::prelude::U256;
use proto::query_client::QueryClient as AllocationQueryClient;
use somm_proto::somm as proto;
use std::time::Duration;
use tokio::time::{timeout, sleep};
use tonic::transport::Channel;

pub struct Connections {
    pub grpc: Option<AllocationQueryClient<Channel>>,
    pub contact: Option<Contact>,
}

pub async fn allocation_precommit(
    cosmos_key: &deep_space::private_key::PrivateKey,
    allocation: &proto::Allocation,
    pair_id: String,
) -> proto::AllocationPrecommit {
    let config = APP.config();
    let delegate_cosmos_address = cosmos_key
        .to_address(&config.cosmos.prefix)
        .unwrap()
        .to_string();
    let hasher = somm_send::data_hash(allocation, delegate_cosmos_address)
        .await
        .unwrap();
    proto::AllocationPrecommit {
        hash: hasher.hash,
        cellar_id: pair_id,
    }
}

pub async fn cellar_query_client() -> Connections {
    let config = APP.config();
    let timeout = Duration::from_secs(10);
    let try_base = AllocationQueryClient::connect(config.cosmos.grpc.clone()).await;
    let(grpc,contact) = match try_base {
        Ok(val) => {
            (Some(val),
            Some(
                Contact::new(&config.cosmos.grpc, timeout, &config.cosmos.prefix).unwrap(),
            ))
        }
        Err(e) => {
            warn!(
                "Failed to access Cosmos gRPC with {:?} and create connections",
                e
            );
            (None,None)
        }
    };
    Connections { grpc, contact }
}

pub async fn decide_rebalance(
    tick_range: Vec<proto::TickRange>,
    pair_id: U256,
    eth_gas_price: u64
) {
    if std::env::var("CELLAR_DRY_RUN").expect("Expect CELLAR_DRY_RUN var") == "TRUE" {
        ()
    } else {
        let connections = cellar_query_client().await;
        let contact = connections.contact.unwrap();
        let grpc_client = &mut connections.grpc.unwrap();

        let config = APP.config();
        let cosmos_gas_price = config.cosmos.gas_price.as_tuple();
        let allocation = to_allocation(tick_range, pair_id.to_string(), eth_gas_price);

        let name = &config.keys.rebalancer_key;
        let cosmos_key = config.load_deep_space_key(name.clone());
        let delegate_cosmos_address = cosmos_key.to_address(&contact.get_prefix()).unwrap();

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

pub fn to_allocation(tick_ranges: Vec<proto::TickRange>, pair_id: String, eth_gas_price: u64) -> proto::Allocation {
    proto::Allocation {
        vote: Some(proto::RebalanceVote {
            cellar: Some(proto::Cellar {
                id: pair_id,
                tick_ranges: tick_ranges,
            }),
            current_price: eth_gas_price,
        }),
        salt: "".to_string(), //TODO: Add salt,
    }
}
