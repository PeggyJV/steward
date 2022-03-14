use crate::{error::Error, gas::CellarGas, utils::get_eth_provider};
use abscissa_core::tracing::log::warn;
use ethers::prelude::*;
use std::{fmt, result::Result};

pub(crate) mod uniswapv3;

#[derive(Debug)]
pub struct CellarId {
    pub address: String,
    pub chain: String,
}

impl std::fmt::Display for CellarId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.chain, self.address)
    }
}

pub async fn get_gas_price() -> Result<U256, Error> {
    if std::env::var("ETHERSCAN_API_KEY").is_ok() {
        match CellarGas::etherscan_standard().await {
            Ok(gas) => return Ok(gas),
            Err(err) => {
                warn!("failed to retrieve gas estimate from etherscan: {}", err);
            }
        }
    }

    let provider = get_eth_provider().await?;

    provider.get_gas_price().await.map_err(|r| r.into())
}

pub fn validate_cellar_id(cellar_id: &str) -> Result<(), String> {
    if let Err(err) = cellar_id.parse::<H160>() {
        return Err(format!("invalid ethereum address: {}", err));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_cellar_id_format_errors() {
        let cellar_id = "thisaintright";
        let result = validate_cellar_id(cellar_id);

        assert!(result.is_err())
    }


    #[test]
    fn valid_cellar_id_works() {
        let cellar_id = "0x0000000000000000000000000000000000000000";
        let result = validate_cellar_id(cellar_id);

        assert!(result.is_ok());
    }
}
