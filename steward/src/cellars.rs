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
pub const ALLOWED_SETUP_ADAPTORS: [&str; 0] = [];
pub const ALLOWED_CATALOGUE_ADAPTORS: [&str; 1] = ["24eeaa1111dac1c0fe0cf3c03bba03adde1e7fe4"]; // cellar adaptor
pub const ALLOWED_CATALOGUE_POSITIONS: [u32; 1] = [143]; // RYETH position
pub const RYGOV_CELLARS: [&str; 5] = [
    "4068bdd217a45f8f668ef19f1e3a1f043e4c4934", // RYLINK
    "c7b69e15d86c5c1581dacce3cacaf5b68cd6596f", // RY1INCH
    "6a6af5393dc23d7e3db28d28ef422db7c40932b6", // RYUNI
    "cbf2250f33c4161e18d4a2fa47464520af5216b5", // RYSNX
    "18ea937aba6053bc232d9ae2c42abe7a8a2be440", // RYENS
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

        assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked1.clone()).as_str()));
        assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked2).as_str()));
        assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked3).as_str()));
        assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked4).as_str()));

        assert!(!ALLOWED_SETUP_ADAPTORS.contains(&normalize_address(blocked1).as_str()));
    }
}
