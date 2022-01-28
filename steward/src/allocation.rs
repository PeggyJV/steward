use crate::{cellars, error::{Error, ErrorKind}, prelude::*, somm_send, utils};
use abscissa_core::Application;
use deep_space::{Coin, Contact};
use ethers::prelude::U256;
use gravity_bridge::{gravity_proto::gravity::query_client::QueryClient as GravityQueryClient, gravity_utils};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use somm_proto::{somm, somm::query_client::QueryClient as AllocationQueryClient};
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tonic::transport::Channel;

pub struct Connections {
    pub allocation_client: AllocationQueryClient<Channel>,
    pub contact: Contact,
    pub gravity_client: GravityQueryClient<Channel>,
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

    Ok(Connections {
        allocation_client: AllocationQueryClient::connect(config.cosmos.grpc.clone()).await?,
        contact: Contact::new(&config.cosmos.grpc, timeout, &config.cosmos.prefix)?,
        gravity_client: GravityQueryClient::connect(config.cosmos.grpc.clone()).await?,
    })
}

pub async fn decide_rebalance(
    tick_range: Vec<somm::TickRange>,
    cellar_address: String,
) -> Result<(), Error> {
    debug!("deciding rebalance for cellar address {}", cellar_address);
    let config = APP.config();

    debug!("loading the delegate (orchestrator) key and address from config");
    let name = &config.keys.rebalancer_key;
    let cosmos_delegate_key = config.load_deep_space_key(name.clone());
    let cosmos_delegate_address = cosmos_delegate_key.to_address(&config.cosmos.prefix)?;

    debug!("establishing connections to validator");
    let mut connections = get_connections().await?;
    let (allocation_client, contact, gravity_client) = (
        &mut connections.allocation_client,
        connections.contact,
        &mut connections.gravity_client,
    );

    debug!(
        "querying the validator address based on the associated delegate address {}",
        cosmos_delegate_address
    );
    let delegate_keys = utils::get_delegates_keys_by_orchestrator(
        gravity_client,
        cosmos_delegate_address.to_string(),
    )
    .await?;
    let validator_address = delegate_keys.validator_address;
    debug!(
        "precommit containing cellar {} will be signed by {} on behalf of {}",
        cellar_address, cosmos_delegate_address, validator_address
    );

    debug!("getting eth gas price estimate");
    let eth_gas_price = cellars::get_gas_price().await?;
    debug!("padding eth gas price estimate");
    let eth_gas_price = match gravity_utils::ethereum::downcast_to_f32(eth_gas_price) {
        Some(gp) => gp,
        None => return Err(ErrorKind::AllocationError
            .context("failed to downcast gas price for padding".to_string())
            .into()),
    };
    let eth_gas_price = eth_gas_price * config.ethereum.gas_price_multiplier;
    let eth_gas_price: U256 = (eth_gas_price as u64).into();
    debug!("gas price: {}", eth_gas_price);

    debug!("building commit and precommit objects");
    let allocation = to_allocation(tick_range, cellar_address.clone(), eth_gas_price.as_u64());
    let allocation_precommit =
        &allocation_precommit(validator_address, &allocation, cellar_address).await?;

    debug!("getting cosmos fee");
    let cosmos_gas_price = config.cosmos.gas_price.as_tuple();
    let fee = Coin {
        amount: (cosmos_gas_price.0 as u64).into(),
        denom: cosmos_gas_price.1,
    };

    match timeout(Duration::from_secs(100), async {
        debug!("waiting for new vote period to start");
        loop {
            let res = somm_send::query_commit_period(allocation_client).await;
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
            let res = somm_send::query_allocation_precommits(allocation_client).await;
            match res {
                Ok(val) => {
                    if val.contains(allocation_precommit) {
                        break val;
                    }
                }
                Err(err) => error!("error querying precommit: {}", err),
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
                tick_ranges,
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
