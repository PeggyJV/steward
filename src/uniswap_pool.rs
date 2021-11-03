use crate::error::Error;
use ethers::contract::abigen;
use ethers::prelude::*;
use std::sync::Arc;

abigen!(
    UPool,
    "./abi/uniswapv3pool_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

pub struct PoolState<T> {
    pub contract: UPool<T>,
}

impl<T: 'static + Middleware> PoolState<T> {
    pub fn new(address: H160, client: Arc<T>) -> Self {
        PoolState {
            contract: UPool::new(address, client),
        }
    }
}
