use ethers::prelude::*;
use std::result::Result;

pub(crate) mod aave_v2_stablecoin;

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
