use crate::{cellars, error::Error, prelude::*, somm_send, utils};
use abscissa_core::Application;
use deep_space::{Coin, Contact};
use gravity_bridge::gravity_proto::gravity::query_client::QueryClient as GravityQueryClient;
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
    validator_address: String,
    allocation: &somm::Allocation,
    cellar_address: String,
) -> Result<somm::AllocationPrecommit, Error> {
    let hasher = somm_send::data_hash(allocation, validator_address).await?;

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
        Err(err) => {
            error!("failed to create allocation client: {}", err);
            return Err(Error::from(err));
        }
    };
    Ok(Connections { grpc, contact })
}

pub async fn decide_rebalance(
    tick_range: Vec<somm::TickRange>,
    cellar_address: String,
) -> Result<(), Error> {
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
            error!("failed to get cellar gas price: {}", err);
            // TO-DO handle this better
            0.into()
        }
    };
    let config = APP.config();

    debug!("loading the delegate (orchestrator) key and address from config");
    let name = &config.keys.rebalancer_key;
    let cosmos_delegate_key = config.load_deep_space_key(name.clone());
    let cosmos_delegate_address = cosmos_delegate_key.to_address(&config.cosmos.prefix)?;

    debug!("querying the validator address based on the associated delegate address {}", cosmos_delegate_address);
    let mut gravity_client = GravityQueryClient::connect(config.cosmos.grpc.clone()).await?;
    let delegate_keys = utils::get_delegates_keys_by_orchestrator(&mut gravity_client, cosmos_delegate_address.to_string()).await?;
    let validator_address = delegate_keys.validator_address;
    debug!("precommit containing cellar {} will be signed by {} on behalf of {}", cellar_address, cosmos_delegate_address, validator_address);

    debug!("building commit and precommit objects");
    let allocation = to_allocation(
        tick_range,
        cellar_address.clone(),
        eth_gas_price.as_u64(),
    );
    let allocation_precommit = &allocation_precommit(validator_address, &allocation, cellar_address).await?;

    debug!("getting cosmos fee");
    let cosmos_gas_price = config.cosmos.gas_price.as_tuple();
    let fee = Coin {
        amount: (cosmos_gas_price.0 as u64).into(),
        denom: cosmos_gas_price.1,
    };

    match timeout(Duration::from_secs(100), async {
        debug!("waiting for new vote period to start");
        loop {
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
            let response = somm_send::send_precommit(
                &contact,
                cosmos_delegate_address.to_string(),
                cosmos_delegate_key,
                fee.clone(),
                vec![allocation_precommit.to_owned()],
            )
            .await?;
            debug!("precommit response: {:?}", response);
        }
        Err(err) => {
            error!("Couldn't Send Precommit {:?}", err);
        }
    }

    match timeout(Duration::from_secs(100), async {
        debug!("querying precommit");
        loop {
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
            debug!("precommit found! sending commit");
            let response = somm_send::send_allocation(
                &contact,
                cosmos_delegate_address.to_string(),
                cosmos_delegate_key,
                fee,
                vec![allocation],
            )
            .await?;
            debug!("commit response: {:?}", response);
        }
        Err(err) => {
            error!("couldn't send commit {:?}", err);
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
