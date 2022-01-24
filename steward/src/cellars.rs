use crate::{
    error::{Error, ErrorKind},
    gas::CellarGas,
};
use ethers::prelude::*;
use std::{fmt, result::Result};

pub(crate) mod uniswapv3;

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
    let parts: Vec<&str> = cellar_id.split(':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "invalid cellar_id format: {}. proper format is 'chainname:address'",
            cellar_id
        )
        .to_string());
    }
    // This assumes Ethereum address format for now.
    let address = match parts[1].parse::<H160>() {
        Ok(addr) => addr,
        Err(err) => return Err(format!("error parsing ethereum address: {}", err).to_string()),
    };

    Ok(CellarId {
        chain: parts[0].to_string(),
        address: address,
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
