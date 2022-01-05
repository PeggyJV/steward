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

fn parse_cellar_id(cellar_id: &str) -> Result<CellarId, String> {
    let delimiter = match cellar_id.chars().position(|c| c == ':') {
        Some(d) => d,
        None => return Err("invalid cellar_id format; could not find delimiter ':'".to_string()),
    };
    // TO-DO: Eventually we need some validation of chain name/address format for that chain
    let parts = cellar_id.split_at(delimiter);
    let address: H160 = H160::from_slice(parts.1.as_bytes());

    Ok(CellarId {
        chain: parts.0.to_string(),
        address: address,
    })
}
