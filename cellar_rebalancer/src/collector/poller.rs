//! Poller
/// The collector's [`Poller`] collects information from external sources
/// which aren't capable of pushing data.
use crate::{
    cellar_uniswap_wrapper::{ContractStateUpdate, UniswapV3CellarState},
    collector, config,
    error::Error,
    gas::CellarGas,
    prelude::*,
    somm_send,
    time_range::{TickWeight, TimeRange},
    uniswap_pool::PoolState,
};
use abscissa_core::error::BoxError;
use deep_space::{coin::Coin, Contact};
use ethers::prelude::*;
use rebalancer_abi::cellar_uniswap::*;
use somm_proto::somm as proto;
use somm_proto::somm::query_client::QueryClient as AllocationQueryClient;
use std::{result::Result, sync::Arc, time::Duration};
use tokio::{
    time::{interval, sleep, timeout},
    try_join,
};
use tonic::transport::Channel;
use tower::Service;

// Struct poller to collect poll_interval etc. from external sources which aren't capable of pushing data
#[allow(dead_code)]
pub struct Poller<T: Middleware> {
    poll_interval: Duration,
    time_range: TimeRange,
    cellar_gas: CellarGas,
    contract_state: UniswapV3CellarState<T>,
    pool: PoolState<T>,
    cosmos_key: deep_space::private_key::PrivateKey,
}

pub fn from_tick_weight(tick_weight: TickWeight) -> CellarTickInfo {
    CellarTickInfo {
        token_id: U256::zero(),
        tick_upper: tick_weight.upper_bound,
        tick_lower: tick_weight.lower_bound,
        weight: tick_weight.weight,
    }
}
pub struct Connections {
    pub grpc: Option<AllocationQueryClient<Channel>>,
    pub contact: Option<Contact>,
}

// Implement poller middleware
impl<T: 'static + Middleware> Poller<T> {
    pub async fn new(
        cellar: &config::CellarConfig,
        client: Arc<T>,
        mongo: &config::MongoSection,
        cosmos_key: &deep_space::private_key::PrivateKey,
        rebalancer_config: config::CellarRebalancerConfig,
    ) -> Result<Self, Error> {
        let pool = PoolState::new(cellar.pool_address, client.clone());
        let spacing = pool
            .contract
            .tick_spacing()
            .call()
            .await
            .expect("Could not get spacing by querying contract");

        let name = &rebalancer_config.keys.rebalancer_key;

        let poller = Poller {
            poll_interval: cellar.duration,
            time_range: TimeRange {
                time: None,
                previous_update: None,
                pair_id: cellar.pair_id,
                token_info: (cellar.token_0.clone(), cellar.token_1.clone()),
                weight_factor: cellar.weight_factor,
                tick_weights: vec![],
                monogo_uri: mongo.host.clone(),
                pair_database: cellar.pair_database.clone(),
                tick_spacing: spacing,
            },
            cellar_gas: CellarGas {
                max_gas_price: ethers::utils::parse_units(cellar.max_gas_price_gwei, "gwei")
                    .unwrap(),
                current_gas: None,
            },
            contract_state: UniswapV3CellarState::new(cellar.cellar_address, client),
            pool,
            cosmos_key: rebalancer_config.load_deep_space_key(name.clone()),
        };

        Ok(poller)
    }

    fn to_allocation(&self) -> proto::Allocation {
        let tick_range: Vec<proto::TickRange> = self
            .time_range
            .tick_weights
            .iter()
            .map(|tick_weight| proto::TickRange {
                upper: tick_weight.upper_bound as i32,
                lower: tick_weight.lower_bound as i32,
                weight: tick_weight.weight as u32,
            })
            .collect();

        proto::Allocation {
            vote: Some(proto::RebalanceVote {
                cellar: Some(proto::Cellar {
                    id: self.time_range.pair_id.to_string(),
                    tick_ranges: self
                        .time_range
                        .tick_weights
                        .iter()
                        .map(|tick_weight| proto::TickRange {
                            upper: tick_weight.upper_bound as i32,
                            lower: tick_weight.lower_bound as i32,
                            weight: tick_weight.weight as u32,
                        })
                        .collect(),
                }),
                current_price: self.contract_state.gas_price.unwrap().as_u64(),
            }),
            salt: "".to_string(), //TODO: Add salt,
        }
    }

    pub async fn allocation_precommit(&self) -> proto::AllocationPrecommit {
        let config = APP.config();
        let delegate_cosmos_address = self
            .cosmos_key
            .to_address(&config.cosmos.prefix)
            .unwrap()
            .to_string();
        let hasher = somm_send::data_hash(&self.to_allocation(), delegate_cosmos_address)
            .await
            .unwrap();
        proto::AllocationPrecommit {
            hash: hasher.hash,
            cellar_id: self.time_range.pair_id.to_string(),
        }
    }

    // Retrieve poll time range
    pub async fn poll_time_range(&self) -> Result<TimeRange, Error> {
        info!("{} polling time range", self.time_range.pair_database);

        let mut time_range = self.time_range.clone();

        time_range.poll().await;
        Ok(time_range)
    }

    // Retrieve current standard gas price from etherscan
    pub async fn poll_cellar_gas(&self) -> Result<U256, Error> {
        CellarGas::etherscan_standard().await.map_err(|e| e.into())
    }

    // Retrieve the current contract state
    pub async fn poll_contract_state(&self) -> Result<ContractStateUpdate, Error> {
        Ok(ContractStateUpdate {})
    }

    pub async fn cellar_query_client(&self) -> Result<Connections, Error> {
        let mut grpc = None;
        let mut contact = None;
        let config = APP.config();
        let timeout = Duration::from_secs(10);
        let try_base = AllocationQueryClient::connect(config.cosmos.grpc.clone()).await;
        match try_base {
            Ok(val) => {
                grpc = Some(val);
                contact = Some(
                    Contact::new(&config.cosmos.grpc, timeout, &config.cosmos.prefix).unwrap(),
                );
            }
            Err(e) => {
                warn!(
                    "Failed to access Cosmos gRPC with {:?} and create connections",
                    e
                );
                return Err(e.into());
            }
        };
        Ok(Connections { grpc, contact })
    }

    // Update poller with time_range, gas price and contract_state
    pub fn update_poller(
        &mut self,
        time_range: TimeRange,
        gas: U256,
        _contract_state: ContractStateUpdate,
    ) {
        self.cellar_gas.current_gas = Some(gas);
        self.contract_state.gas_price = Some(gas);
        self.time_range = time_range;
    }

    pub async fn decide_rebalance(
        &mut self,
        contact: &Contact,
        grpc_client: &mut AllocationQueryClient<Channel>,
    ) {
        let config = APP.config();
        let gas_price = config.cosmos.gas_price.as_tuple();
        let mut tick_info: Vec<CellarTickInfo> = Vec::new();
        for ref tick_weight in self.time_range.tick_weights.clone() {
            if tick_weight.weight > 0 {
                tick_info.push(from_tick_weight(tick_weight.clone()))
            }
        }

        if std::env::var("CELLAR_DRY_RUN").expect("Expect CELLAR_DRY_RUN var") == "TRUE" {
            ()
        } else {
            tick_info.reverse();
            let delegate_cosmos_address =
                self.cosmos_key.to_address(&contact.get_prefix()).unwrap();
            let name = &config.keys.rebalancer_key;
            let cosmos_key = config.load_deep_space_key(name.clone());

            let fee = Coin {
                denom: gas_price.1,
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
                        contact,
                        delegate_cosmos_address,
                        cosmos_key,
                        fee.clone(),
                        vec![self.allocation_precommit().await],
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
                        contact,
                        delegate_cosmos_address,
                        cosmos_key,
                        fee,
                        vec![self.to_allocation()],
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

    // Route incoming requests.
    pub async fn run<S>(mut self, collector: S)
    where
        S: Service<collector::Request, Response = collector::Response, Error = BoxError>
            + Send
            + Clone
            + 'static,
    {
        info!(
            "{} polling every {:?}",
            self.time_range.pair_database, self.poll_interval
        );

        let mut interval = interval(self.poll_interval);
        loop {
            interval.tick().await;
            self.poll(&collector).await;
            info!(
                "{} waiting for {:?}",
                self.time_range.pair_database, self.poll_interval
            );
        }
    }

    async fn poll<S>(&mut self, _collector: &S)
    where
        S: Service<collector::Request, Response = collector::Response, Error = BoxError>
            + Send
            + Clone
            + 'static,
    {
        let res = try_join!(
            self.poll_time_range(),
            self.poll_cellar_gas(),
            self.poll_contract_state(),
            self.cellar_query_client(),
        );
        match res {
            Ok((time_range, gas, contract_state_update, grpc_client)) => {
                self.update_poller(time_range, gas, contract_state_update);
                self.decide_rebalance(
                    &grpc_client
                        .contact
                        .expect("Need a valid contact connection to Sommelier Chain"),
                    &mut grpc_client
                        .grpc
                        .expect("Need a valid gRPC connection to Sommelier Chain"),
                )
                .await;
            }
            Err(e) => error!("Error fetching data {}", e),
        }
    }
}
