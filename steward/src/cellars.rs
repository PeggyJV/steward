#![allow(dead_code)]
use std::result::Result;

use abscissa_core::tracing::log::info;
use ethers::prelude::*;

use crate::{error::Error, utils::sp_call_error};

pub(crate) mod aave_v2_stablecoin;
pub(crate) mod adaptors;
pub(crate) mod cellar_v1;
pub(crate) mod cellar_v2;
pub(crate) mod cellar_v2_2;

// constants
// addresses are normalized by removing the 0x prefix and converting to lowercase for reliable comparison

// allow/block lists.

pub const ALLOWED_ADD_POSITION: [(&str, u32); 3] =
    [(CELLAR_RYUSD, 25), (CELLAR_RYUSD, 26), (CELLAR_RYUSD, 27)];
pub const ALLOWED_CATALOGUE_ADAPTORS: [(&str, &str); 6] = [
    (CELLAR_RYETH, ADAPTOR_MORPHO_AAVE_V2_A_TOKEN_V1),
    (CELLAR_RYETH, ADAPTOR_MORPHO_AAVE_V2_DEBT_TOKEN_V1),
    (CELLAR_RYETH, ADAPTOR_MORPHO_AAVE_V3_A_TOKEN_V1),
    (CELLAR_RYETH, ADAPTOR_MORPHO_AAVE_V3_DEBT_TOKEN_V1),
    (CELLAR_RYETH, ADAPTOR_MORPHO_AAVE_V3_P2P_V1),
    (CELLAR_RYLINK, ADAPTOR_CELLAR_V2),
];
pub const ALLOWED_CATALOGUE_POSITIONS: [(&str, u32); 8] = [
    (CELLAR_RYETH, 155),
    (CELLAR_RYETH, 156),
    (CELLAR_RYETH, 161),
    (CELLAR_RYETH, 162),
    (CELLAR_RYETH, 163),
    (CELLAR_RYETH, 165),
    (CELLAR_RYETH, 166),
    (CELLAR_RYLINK, 154),
];
pub const ALLOWED_SETUP_ADAPTORS: [(&str, &str); 1] =
    [(CELLAR_RYUSD, ADAPTOR_MORPHO_AAVE_V2_A_TOKEN_V1)];
pub const BLOCKED_ADAPTORS: [&str; 3] = [
    CELLAR_UNIV3_V1,
    CELLAR_VESTING_SIMPLE_V1,
    CELLAR_COMPOUND_C_TOKEN_V1,
];
pub const BLOCKED_POSITIONS: [u32; 9] = [4, 5, 6, 7, 8, 9, 10, 11, 12];

// cellars

pub const CELLAR_RY1INCH: &str = "c7b69e15d86c5c1581dacce3cacaf5b68cd6596f";
pub const CELLAR_RYENS: &str = "18ea937aba6053bc232d9ae2c42abe7a8a2be440";
pub const CELLAR_RYETH: &str = "b5b29320d2dde5ba5bafa1ebcd270052070483ec";
pub const CELLAR_RYLINK: &str = "4068bdd217a45f8f668ef19f1e3a1f043e4c4934";
pub const CELLAR_RYSNX: &str = "cbf2250f33c4161e18d4a2fa47464520af5216b5";
pub const CELLAR_RYUNI: &str = "6a6af5393dc23d7e3db28d28ef422db7c40932b6";
pub const CELLAR_RYUSD: &str = "97e6e0a40a3d02f12d1cec30ebfbae04e37c119e";

// deprecated cellars

pub const CELLAR_UNIV3_V1: &str = "7c4262f83e6775d6ff6fe8d9ab268611ed9d13ee";
pub const CELLAR_VESTING_SIMPLE_V1: &str = "1eaa1a100a460f46a2032f0402bc01fe89faab60";
pub const CELLAR_COMPOUND_C_TOKEN_V1: &str = "26dba82495f6189dde7648ae88bead46c402f078";

// adaptors

pub const ADAPTOR_CELLAR_V2: &str = "24eeaa1111dac1c0fe0cf3c03bba03adde1e7fe4";
pub const ADAPTOR_MORPHO_AAVE_V2_A_TOKEN_V1: &str = "1a4cb53edb8c65c3df6aa9d88c1ab4cf35312b73";
pub const ADAPTOR_MORPHO_AAVE_V2_DEBT_TOKEN_V1: &str = "407d5489f201013ee6a6ca20fccb05047c548138";
pub const ADAPTOR_MORPHO_AAVE_V3_A_TOKEN_V1: &str = "b46e8a03b1aafffb50f281397c57b5b87080363e";
pub const ADAPTOR_MORPHO_AAVE_V3_DEBT_TOKEN_V1: &str = "25a61f771af9a38c10ddd93c2bbab39a88926fa9";
pub const ADAPTOR_MORPHO_AAVE_V3_P2P_V1: &str = "4fe068caad05b82bf3f86e1f7d1a7b8bbf516111";

// utils

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

// validation logic

pub fn check_blocked_adaptor(adaptor_id: &str) -> Result<(), Error> {
    let adaptor_id = normalize_address(adaptor_id.to_string());
    if BLOCKED_ADAPTORS.contains(&adaptor_id.as_str()) {
        return Err(sp_call_error(format!("adaptor {adaptor_id} is blocked")));
    }

    Ok(())
}

pub fn check_blocked_position(position: u32) -> Result<(), Error> {
    if BLOCKED_POSITIONS.contains(&position) {
        return Err(sp_call_error(format!("position {position} is blocked")));
    }

    Ok(())
}

pub fn validate_add_position(cellar_id: &str, position: u32) -> Result<(), Error> {
    check_blocked_position(position)?;
    let cellar_id = normalize_address(cellar_id.to_string());
    if !ALLOWED_ADD_POSITION.contains(&(&cellar_id, position)) {
        return Err(sp_call_error(format!(
            "position {position} not allowed to be added for {cellar_id}"
        )));
    }

    Ok(())
}

pub fn validate_add_adaptor_to_catalogue(
    cellar_id: &str,
    adaptor_id: &str,
) -> Result<(), Error> {
    let adaptor_id = normalize_address(adaptor_id.to_string());
    check_blocked_adaptor(&adaptor_id)?;
    let cellar_id = normalize_address(cellar_id.to_string());
    if !ALLOWED_CATALOGUE_ADAPTORS.contains(&(&cellar_id, &adaptor_id)) {
        return Err(sp_call_error(format!(
            "adaptor {adaptor_id} not allowed to be added to catalogue for {cellar_id}"
        )));
    }

    Ok(())
}

pub fn validate_add_position_to_catalogue(
    cellar_id: &str,
    position: u32,
) -> Result<(), Error> {
    let cellar_id = normalize_address(cellar_id.to_string());
    if !ALLOWED_CATALOGUE_POSITIONS.contains(&(&cellar_id, position)) {
        return Err(sp_call_error(format!(
            "position {position} not allowed to be added to catalogue for {cellar_id}"
        )));
    }

    Ok(())
}

pub fn validate_cellar_id(cellar_id: &str) -> Result<(), Error> {
    if let Err(err) = cellar_id.parse::<H160>() {
        return Err(sp_call_error(format!("invalid ethereum address: {err}")));
    }

    Ok(())
}

pub fn validate_setup_adaptor(cellar_id: &str, adaptor_id: &str) -> Result<(), Error> {
    let adaptor_id = normalize_address(adaptor_id.to_string());
    check_blocked_adaptor(&adaptor_id)?;
    let cellar_id = normalize_address(cellar_id.to_string());
    if !ALLOWED_SETUP_ADAPTORS.contains(&(&cellar_id, &adaptor_id)) {
        return Err(sp_call_error(format!(
            "adaptor {adaptor_id} not allowed to be setup for {cellar_id}"
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::cellars::{normalize_address, validate_cellar_id};

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

        assert!(
            !ALLOWED_SETUP_ADAPTORS.contains(&(CELLAR_RYUSD, normalize_address(blocked1.clone()).as_ref()))
        );

        // idempotent
        let once = normalize_address(blocked1);
        let twice = normalize_address(once.clone());
        assert_eq!(&once, &twice);
    }
}
