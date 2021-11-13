//! Rust Wrapper for cellar functions
/// This will convert cellar functions from tuples to Rust types
use crate::error::Error;
use crate::prelude::*;
use ethers::contract::abigen;
use ethers::prelude::*;
use std::sync::Arc;

//use abigen macro to fetch and incorporate contract ABI
abigen!(
    Erc20,
    "./abi/erc20_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

//use abigen macro to fetch and incorporate contract ABI
abigen!(
    Weth,
    "./abi/weth_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

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
