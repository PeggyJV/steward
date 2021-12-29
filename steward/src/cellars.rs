use crate::{error::Error, gas::CellarGas};
use ethers::prelude::*;
use std::{fmt, result::Result};

pub(crate) mod uniswapv3;

pub const STEWARD_PORT: u16 = 5734;
const UNISWAPV3_CELLAR: &str = "uniswapv3";

#[derive(Debug)]
pub struct CellarId {
    chain: String,
    address: H160,
}

impl std::fmt::Display for CellarId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.chain, self.address)
    }
}

pub async fn get_gas_price() -> Result<U256, Error> {
    CellarGas::etherscan_standard().await.map_err(|e| e.into())
}

fn parse_cellar_id(cellar_id: &str) -> CellarId {
    let parts = cellar_id.split_at(cellar_id.chars().position(|c| c == ':').unwrap());
    let address: H160 = H160::from_slice(parts.1.as_bytes());

    CellarId {
        chain: parts.0.to_string(),
        address: address,
    }
}
