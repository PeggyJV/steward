//! Data Collector for cellar monitoring
use std::{
    future::Future,
    pin::Pin,
    process,
    task::{Context, Poll},
};

use crate::{config,error::Error,time_range::TimeRange,gas::CellarGas,cellar_wrapper::ContractState};
use ethers::{
    prelude::*,
};
use tower::{util::ServiceExt, Service};


pub use self::{poller::Poller, request::Request, response::Response};

mod poller;
mod request;
mod response;

pub struct Collector{
}

impl  Collector {

    pub fn new(config: &config::ContractMonitorConfig) -> Result<Self,Error> {
        Ok(Collector{})

    }


}

impl Service<Request> for Collector {
    type Response = Response;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Response, Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let result = match req {
            Request::Gas(event) => todo!(),
            Request::TickData(event) => todo!(),
            Request::ContractState(event) => todo!(),
        };

        Box::pin(async { result })
    }
}