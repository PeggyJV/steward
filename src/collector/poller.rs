//! Poller
/// The collector's [`Poller`] collects information from external sources
/// which aren't capable of pushing data.
use crate::{
    cellar_uniswap_wrapper::{UniswapV3CellarState, UniswapV3CellarTickInfo, ContractStateUpdate},
    collector, config,
    error::Error,
    gas::CellarGas,
    prelude::*,
    somm_send,
    time_range::TimeRange,
    uniswap_pool::PoolState,
};
use abscissa_core::{Application, Command, Clap, Runnable, error::BoxError};
use crate::application::APP;
use deep_space::Contact;
use deep_space::coin::Coin;
use ethers::prelude::*;
use somm_proto::somm as proto;
use std::{sync::Arc, time::Duration};
use tokio::{time, try_join};
use tower::Service;

// Struct poller to collect poll_interval etc. from external sources which aren't capable of pushing data
pub struct Poller<T: Middleware> {
    poll_interval: Duration,
    time_range: TimeRange,
    cellar_gas: CellarGas,
    contract_state: UniswapV3CellarState<T>,
    pool: PoolState<T>,
    cosmos_key: deep_space::private_key::PrivateKey,
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
                upper: tick_weight.upper_bound as u64,
                lower: tick_weight.lower_bound as u64,
                weight: tick_weight.weight as u64,
            })
            .collect();

        proto::Allocation {
            cellar: Some(proto::Cellar {
                id: self.time_range.pair_id.to_string(),
                tick_ranges: self
                    .time_range
                    .tick_weights
                    .iter()
                    .map(|tick_weight| proto::TickRange {
                        upper: tick_weight.upper_bound as u64,
                        lower: tick_weight.lower_bound as u64,
                        weight: tick_weight.weight as u64,
                    })
                    .collect(),
            }),
            salt: "".to_string(), //TODO: Add salt,
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

    pub async fn cellar_contact(&self) -> Result<Contact, Error> {
        let config = APP.config();
        let timeout = Duration::from_secs(10);
        let contact = Contact::new(
            &config.cosmos.grpc,
            timeout,
            &config.cosmos.prefix,
        )
        .expect("Could not create contact");
        Ok(contact)
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
        contact: &Contact) -> Result<(), Error> {
        let config = APP.config();
        let gas_price = config.cosmos.gas_price.as_tuple();
        let mut tick_info: Vec<UniswapV3CellarTickInfo> = Vec::new();
        for ref tick_weight in self.time_range.tick_weights.clone() {
            if tick_weight.weight > 0 {
                tick_info.push(UniswapV3CellarTickInfo::from_tick_weight(tick_weight))
            }
        }

        if std::env::var("CELLAR_DRY_RUN").expect("Expect CELLAR_DRY_RUN var") == "TRUE" {
            Ok(())
        } else {
            tick_info.reverse();
            let delegate_cosmos_address = self.cosmos_key.to_address(&contact.get_prefix()).unwrap();
            let name = &config.keys.rebalancer_key;
            let cosmos_key = config.load_deep_space_key(name.clone());

            let fee = Coin {
                denom: gas_price.1,
                amount: 0u32.into(),
            };

            // TODO(Levi) needs to be initialized
            let cellar_id = "TODO".to_owned();

            somm_send::send_allocation(
                contact,
                delegate_cosmos_address,
                cosmos_key,
                fee,
                cellar_id,
                vec![self.to_allocation()],
            )
            .await
            .unwrap();
            self.contract_state.rebalance(tick_info).await
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

        let mut interval = time::interval(self.poll_interval);
        loop {
            interval.tick().await;
            self.poll(&collector).await;
            info!(
                "{} waiting for {:?}",
                self.time_range.pair_database, self.poll_interval
            );
        }
    }

    async fn poll<S>(&mut self, collector: &S)
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
            self.cellar_contact(),
        );
        match res {
            Ok((time_range, gas, contract_state_update, contact)) => {
                self.update_poller(time_range, gas, contract_state_update);
                self.decide_rebalance(&contact).await.unwrap();
            }
            Err(e) => error!("Error fetching data {}", e),
        }
    }
}
