//! Data Collector for cellar monitoring
use std::{
    future::Future,
    pin::Pin,
    process,
    task::{Context, Poll},
};

use crate::{
    cellar_wrapper::ContractState, config, error::Error, gas::CellarGas, time_range::TimeRange,
};
use ethers::prelude::*;
use tower::{util::ServiceExt, Service};

pub use self::{poller::Poller, request::Request, response::Response};

mod poller;
pub(crate) mod request;
mod response;

pub struct Collector {
    recent_gas_prices: Vec<U256>,
    last_rebalance_time: chrono::DateTime<chrono::Utc>,
    time_range: TimeRange,
}

fn u256_sqrt(y: Option<U256>) -> Option<U256> {
    match y {
        Some(y) => {
            let mut z = U256::zero();
            if y.gt(&U256::from(3)) {
                z = y.clone();
                let mut x = y.checked_div(U256::from(2))?.checked_add(U256::from(1))?;
                while x.lt(&z) {
                    z = x.clone();
                    x = y
                        .checked_div(x)?
                        .checked_add(x)?
                        .checked_div(U256::from(2))?;
                }
            } else if !y.is_zero() {
                z = U256::from(1);
            }

            return Some(z);
        }
        None => return None,
    };

    // Instantiate collector with `new` function
    impl Collector {
        pub fn new(config: &config::CellarRebalancerConfig) -> Result<Self, Error> {
            Ok(Collector {
                recent_gas_prices: Vec::new(),
                last_rebalance_time: chrono::Utc::now(),
                time_range: TimeRange::default(),
            })
        }
        pub fn std_dev_mean_gas(&self) -> (Option<U256>, Option<U256>) {
            match (self.gas_mean(), self.recent_gas_prices.len()) {
                (Some(data_mean), count) if count > 0 => {
                    let variance = self
                        .recent_gas_prices
                        .iter()
                        .map(|value| {
                            let diff = data_mean - (*value);

                            diff * diff
                        })
                        .fold(U256::zero(), |sum, x| sum + x)
                        .checked_div(count.into());
                    (Some(data_mean), u256_sqrt(variance))
                }
                _ => (None, None),
            }
        }

        fn gas_mean(&self) -> Option<U256> {
            let sum = self
                .recent_gas_prices
                .iter()
                .fold(U256::zero(), |sum, x| sum + x);
            let count = self.recent_gas_prices.len();

            match count {
                positive if positive > 0 => Some(sum / count),
                _ => None,
            }
        }
    }

    // Implement service request for collector for response, error and future
    impl Service<Request> for Collector {
        type Response = Response;
        type Error = Error;
        type Future = Pin<Box<dyn Future<Output = Result<Response, Error>> + Send + 'static>>;

        fn poll_ready(&mut self, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        // Call collector for gas price, tickdata and contract state
        fn call(&mut self, req: Request) -> Self::Future {
            let result = match req {
                Request::Gas(event) => {
                    self.recent_gas_prices.push(event.current_gas_price);
                    Ok(Response::AccummulateResponse)
                }
                Request::TickData(event) => {
                    self.time_range = event.current_tick_data;
                    Ok(Response::AccummulateResponse)
                }
                Request::ContractState(event) => todo!(),
                Request::ReblanceRequest => {
                    let current_gas = self.recent_gas_prices.last();
                    todo!()
                }
            };

            Box::pin(async { result })
        }
    }
}
