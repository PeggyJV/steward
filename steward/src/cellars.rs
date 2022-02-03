use crate::{error::Error, gas::CellarGas, prelude::APP, utils};
use abscissa_core::{tracing::log::warn, Application};
use ethers::prelude::*;
use std::{fmt, result::Result};

pub(crate) mod uniswapv3;

#[derive(Debug)]
pub struct CellarId {
    chain: String,
    address: String,
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

    let config = APP.config();
    let url = &config.ethereum.rpc;
    let provider = crate::utils::get_eth_provider(url.as_str()).await?;

    provider.get_gas_price().await.map_err(|r| r.into())
}

fn parse_cellar_id(cellar_id: &str) -> Result<CellarId, String> {
    let parts: Vec<&str> = cellar_id.split(':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "invalid cellar_id format: {}. proper format is 'chainname:address'",
            cellar_id
        ));
    }
    if let Err(err) = parts[1].parse::<H160>() {
        return Err(format!("invalid ethereum address: {}", err));
    }

    Ok(CellarId {
        chain: parts[0].to_string(),
        address: parts[1].to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_cellar_id_format_errors() {
        let cellar_id = "thisaintright";
        let result = parse_cellar_id(cellar_id);

        assert!(result.is_err())
    }

    #[test]
    fn invalid_ethereum_address_errors() {
        let cellar_id = "ethereum:whatever";
        let result = parse_cellar_id(cellar_id);

        assert!(result.is_err())
    }

    #[test]
    fn valid_cellar_id_works() {
        let cellar_id = "ethereum:0x0000000000000000000000000000000000000000";
        let result = parse_cellar_id(cellar_id);

        assert!(result.is_ok());
    }
}
