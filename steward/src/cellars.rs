use abscissa_core::tracing::log::info;
use ethers::prelude::*;
use std::result::Result;

use crate::error::{Error, ErrorKind};

pub(crate) mod aave_v2_stablecoin;
pub(crate) mod adaptors;
pub(crate) mod cellar_v1;
pub(crate) mod cellar_v2;
pub(crate) mod cellar_v2_2;

// adaptors and positions associated with the deprecated UniV3 adaptor are blocked
// addresses treated as lowercase without 0x prefix to ensure valid comparisons with arbitrary input
pub const BLOCKED_ADAPTORS: [&str; 3] = [
    // Original uniswap v3 adaptor
    "7c4262f83e6775d6ff6fe8d9ab268611ed9d13ee",
    // Original vesting simple adaptor
    "1eaa1a100a460f46a2032f0402bc01fe89faab60",
    // Original compound C token adaptor
    "26dba82495f6189dde7648ae88bead46c402f078",
];
pub const BLOCKED_POSITIONS: [u32; 9] = [4, 5, 6, 7, 8, 9, 10, 11, 12];
pub const ALLOWED_SETUP_ADAPTORS: [&str; 9] = [
    // UniswapV3Adaptor v2
    "0bd9a2c1917e3a932a4a712aee38ff63d35733fb",
    // VestingSimpleAdaptor v2
    "508e6ae090ea92cb90571e4269b799257cd78ca1",
    // OneInchAdaptor v1
    "a99ae6a11db22ff50bb17ca24c49a2bad8ece7de",
    // SwapWithUniswapAdaptor v1
    "2fe64ae823afc8100bdb99eead876d1e618d4814",
    // ZeroXAdaptor v1
    "b65b7e4776c3e5bc8863c357b3192257cef0962b",
    // AaveV3ATokenAdaptor v1
    "3184cbea47ed519fa04a23c4207cd15b7545f1a6",
    // AaveATokenAdaptor v2
    "25570a77dca06fda89c1ef41fab6ee48a2377e81",
    // FeesAndReservesAdaptor v1
    "647d264d800a2461e594796af61a39b7735d8933",
    // CTokenAdaptor v2
    "9a384df333588428843d128120becd72434ec078",
];

pub trait CellarAdaptorCall {}

pub fn validate_cellar_id(cellar_id: &str) -> Result<(), Error> {
    if let Err(err) = cellar_id.parse::<H160>() {
        return Err(ErrorKind::SPCallError
            .context(format!("invalid ethereum address: {}", err))
            .into());
    }

    Ok(())
}

pub fn log_cellar_call(cellar_name: &str, function_name: &str, cellar_id: &str) {
    info!(
        "encoding {}.{} call for cellar {}",
        cellar_name, function_name, cellar_id
    );
}

// since a string prefixed with or without 0x is parsable, ensure the string comparison is valid
pub fn normalize_address(address: String) -> String {
    let lowercase_address = address.to_lowercase();
    return address
        .to_lowercase()
        .strip_prefix("0x")
        .unwrap_or(&lowercase_address)
        .to_string();
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

    #[test]
    fn test_address_normalization() {
        let blocked1 = String::from("0x7C4262f83e6775D6ff6fE8d9ab268611Ed9d13Ee");
        let blocked2 = String::from("0X7c4262f83e6775d6ff6fe8d9ab268611ed9d13ee");
        let blocked3 = String::from("7C4262f83e6775D6ff6fE8d9ab268611Ed9d13Ee");
        let blocked4 = String::from("7c4262f83e6775d6ff6fe8d9ab268611ed9d13ee");
        let nonblocked = String::from("0x0bD9a2c1917e3A932a4a712aEE38ff63d35733fb");

        assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked1.clone()).as_str()));
        assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked2).as_str()));
        assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked3).as_str()));
        assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked4).as_str()));
        assert!(!BLOCKED_ADAPTORS.contains(&normalize_address(nonblocked.clone()).as_str()));

        assert!(ALLOWED_SETUP_ADAPTORS.contains(&normalize_address(nonblocked).as_str()));
        assert!(!ALLOWED_SETUP_ADAPTORS.contains(&normalize_address(blocked1).as_str()));
    }
}
