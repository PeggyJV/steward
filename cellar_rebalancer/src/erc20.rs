//! Rust Wrapper for cellar functions
/// This will convert cellar functions from tuples to Rust types
use crate::error::Error;
use crate::prelude::*;
use ethers::prelude::*;
use std::sync::Arc;
use rebalancer_abi::erc20::Erc20;
use rebalancer_abi::weth::Weth;

pub struct Erc20State<T> {
    pub contract: Erc20<T>,
    pub gas_price: Option<U256>,
}

impl<T: 'static + Middleware> Erc20State<T> {
    // Instantiate `new` ContractState
    pub fn new(address: H160, client: Arc<T>) -> Self {
        Erc20State {
            contract: Erc20::new(address, client),
            gas_price: None,
        }
    }

    pub async fn approve(&self, amount: U256, cellar_address: H160) {
        let call = self.contract.approve(cellar_address, amount);
        let gas_increase = call.gas(80_000);
        let pending = gas_increase.send().await.unwrap();
        info!("Approve transaction {:?}", pending);
        return;
    }
}
pub struct WethState<T> {
    contract: Weth<T>,
}
