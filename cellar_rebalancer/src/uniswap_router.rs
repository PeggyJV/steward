//! Rust Wrapper for cellar functions

use ethers::prelude::*;
use rebalancer_abi::uniswap_router::UniswapRouter;
use rebalancer_abi::uniswapv3pool::UniswapV3 as UniswapPool;
use std::sync::Arc;

pub struct RouterState<T> {
    pub contract: UniswapRouter<T>,
}

impl<T: 'static + Middleware> RouterState<T> {
    pub fn new(address: H160, client: Arc<T>) -> Self {
        RouterState {
            contract: UniswapRouter::new(address, client),
        }
    }
}

pub struct PoolState<T> {
    pub contract: UniswapPool<T>,
}

impl<T: 'static + Middleware> PoolState<T> {
    pub fn new(address: H160, client: Arc<T>) -> Self {
        PoolState {
            contract: UniswapPool::new(address, client),
        }
    }
}
