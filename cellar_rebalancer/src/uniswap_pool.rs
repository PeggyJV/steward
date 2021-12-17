use cellar_rebalancer_abi::uniswapv3pool::UniswapV3 as UPool;
use ethers::prelude::*;
use std::sync::Arc;

pub struct PoolState<T> {
    pub contract: UPool<T>,
}

impl<T: 'static + Middleware> PoolState<T> {
    pub fn new(address: H160, client: Arc<T>) -> Self {
        PoolState {
            contract: UPool::new(address, client),
        }
    }

    pub async fn token_0(&self) -> Address {
        self.contract
            .token_0()
            .call()
            .await
            .expect("Failed to get token0 address")
    }

    pub async fn token_1(&self) -> Address {
        self.contract
            .token_1()
            .call()
            .await
            .expect("Failed to get token0 address")
    }
}
