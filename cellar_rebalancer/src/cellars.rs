use crate::{error::Error, gas::CellarGas};
use ethers::prelude::*;
use std::result::Result;

pub(crate) mod uniswapv3;

const UNISWAPV3_CELLAR: &str = "uniswapv3";

pub async fn get_gas_price() -> Result<U256, Error> {
    CellarGas::etherscan_standard().await.map_err(|e| e.into())
}

// returns (chain name, cellar address)
fn parse_cellar_id(cellar_id: &str) -> (&str, ethers::types::H160) {
    let parts = cellar_id.split_at(cellar_id.chars().position(|c| c == ':').unwrap());
    let address: H160 = H160::from_slice(parts.1.as_bytes());

    (parts.0, address)
}
