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
    "1eAA1a100a460f46A2032f0402Bc01FE89FaAB60",
    // Original compound C token adaptor
    "26DbA82495f6189DDe7648Ae88bEAd46C402F078",
];
pub const BLOCKED_POSITIONS: [u32; 9] = [4, 5, 6, 7, 8, 9, 10, 11, 12];
pub const ALLOWED_SETUP_ADAPTORS: [&str; 9] = [
    // UniswapV3Adaptor V2
    "dbd750f72a00d01f209ffc6c75e80301efc789c1",
    // VestingSimpleAdaptor V2
    "508E6aE090eA92Cb90571e4269B799257CD78CA1",
    // OneInchAdaptor V1
    "B8952ce4010CFF3C74586d712a4402285A3a3AFb",
    // SwapWithUniswapAdaptor V1
    "d6BC6Df1ed43e3101bC27a4254593a06598a3fDD",
    // ZeroXAdaptor V1
    "1039a9b61DFF6A3fb8dbF4e924AA749E5cFE35ef",
    // AaveV3ATokenAdaptor V1
    "3184CBEa47eD519FA04A23c4207cD15b7545F1A6",
    // AaveATokenAdaptor V2
    "25570a77dCA06fda89C1ef41FAb6eE48a2377E81",
    // FeesAndReservesAdaptor V1
    "647d264d800A2461E594796af61a39b7735d8933",
    // CTokenAdaptor V2
    "9a384Df333588428843D128120Becd72434ec078",
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
        let nonblocked = String::from("0xDbd750F72a00d01f209FFc6C75e80301eFc789C1");

        assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked1.clone()).as_str()));
        assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked2).as_str()));
        assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked3).as_str()));
        assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked4).as_str()));
        assert!(!BLOCKED_ADAPTORS.contains(&normalize_address(nonblocked.clone()).as_str()));

        assert!(ALLOWED_SETUP_ADAPTORS.contains(&normalize_address(nonblocked).as_str()));
        assert!(!ALLOWED_SETUP_ADAPTORS.contains(&normalize_address(blocked1).as_str()));
    }
}
