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

pub struct Collector {}

// Instantiate collector with `new` function
impl Collector {
    pub fn new(config: &config::ContractMonitorConfig) -> Result<Self, Error> {
        Ok(Collector {})
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
            Request::Gas(event) => todo!(),
            Request::TickData(event) => todo!(),
            Request::ContractState(event) => todo!(),
        };

        Box::pin(async { result })
    }
}
