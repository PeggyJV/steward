//! Poller
/// The collector's [`Poller`] collects information from external sources
/// which aren't capable of pushing data.
use crate::{
    cellar_wrapper::{ContractState, ContractStateUpdate},
    collector, config,
    error::Error,
    gas::CellarGas,
    prelude::*,
    time_range::TimeRange,
};
use abscissa_core::error::BoxError;
use ethers::prelude::*;
use futures::future;
use std::time::Duration;
use tokio::time;
use tokio::try_join;
use tower::Service;

// Struct poller to collect poll_interval etc. from external sources which aren't capable of pushing data
pub struct Poller<T: Middleware> {
    poll_interval: Duration,
    time_range_host: String,
    time_range: TimeRange,
    cellar_gas: CellarGas,
    contract_state: ContractState<T>,
}

// Implement poller middleware
impl<T: Middleware> Poller<T> {
    // Retrieve poll time range
    pub async fn poll_time_range(&self) -> Result<TimeRange, Error> {
        TimeRange::fetch(self.time_range_host.clone())
            .await
            .map_err(|e| e.into())
    }

    // Retrieve current standard gas price from etherscan
    pub async fn poll_cellar_gas(&self) -> Result<U256, Error> {
        CellarGas::etherscan_standard().await.map_err(|e| e.into())
    }

    // Retrieve the current contract state
    pub async fn poll_contract_state(&self) -> Result<ContractStateUpdate, Error> {
        Ok(ContractStateUpdate {})
    }

    // Update poller with time_range, gas price and contract_state
    pub fn update_poller(
        &mut self,
        time_range: TimeRange,
        gas: U256,
        contract_state: ContractStateUpdate,
    ) {
        todo!()
    }

    pub async fn decide_rebalance(&self) -> Result<(), Error> {
        todo!()
    }

    pub fn new(config: &config::ContractMonitorConfig, client: T) -> Result<Self, Error> {
        todo!()
    }

    // Route incoming requests.
    pub async fn run<S>(mut self, collector: S)
    where
        S: Service<collector::Request, Response = collector::Response, Error = BoxError>
            + Send
            + Clone
            + 'static,
    {
        info!("polling every {:?}", self.poll_interval);
        let mut interval = time::interval(self.poll_interval);

        loop {
            interval.tick().await;
            self.poll(&collector).await;
            info!("waiting for {:?}", self.poll_interval);
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
        );
        match res {
            Ok((time_range, gas, contract_state_update)) => {
                self.update_poller(time_range, gas, contract_state_update);
                self.decide_rebalance().await;
            }
            Err(e) => error!("Error fetching data {}", e),
        }
    }
}
