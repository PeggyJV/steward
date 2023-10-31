#![allow(dead_code)]
use std::result::Result;

use abscissa_core::tracing::log::info;
use ethers::prelude::*;

use crate::{
    error::Error,
    utils::{sp_call_error, string_to_u256},
};

pub type CachePriceRouterArgs = (bool, u32, Option<&'static str>);

pub(crate) mod aave_v2_stablecoin;
pub(crate) mod adaptors;
pub(crate) mod cellar_v1;
pub(crate) mod cellar_v2;
pub(crate) mod cellar_v2_2;
pub(crate) mod cellar_v2_5;

// constants
// addresses are normalized by removing the 0x prefix and converting to lowercase for reliable comparison

// oracles

pub const ORACLE1: (U256, &str) = (
    U256([3, 0, 0, 0]),
    "72249f0199eacf6230def33a31e80cf76de78f67",
);
pub const ORACLE2: (U256, &str) = (
    U256([5, 0, 0, 0]),
    "c47278b65443ce71cf47e8455bb343f2db11b70e",
);
pub const ORACLE3: (U256, &str) = (
    U256([5, 0, 0, 0]),
    "26cde3f5db92ea91c84c838e664fe42dec1b6747",
);

// price routers

pub const PRICEROUTER1: &str = "a1a0bc3d59e4ee5840c9530e49bdc2d1f88aaf92";
pub const PRICEROUTER2: &str = "8e46f30b09fdfae6c97db27fecf3304f86dd88c2";

pub const ALLOWED_PRICE_ORACLES: [(U256, &str); 3] = [ORACLE1, ORACLE2, ORACLE3];
// mapping of cellar address to allowable arguments for a cachePriceRouter call.
// args map to params as (checkTotalAssets, allowableRange, expectedPriceRouterAddress). The third
// param is only present for 2.5 cellars so it's an Option<&str>.
pub const ALLOWED_CACHE_PRICE_ROUTER: [(&str, CachePriceRouterArgs); 4] = [
    (CELLAR_RYBTC, (true, 500, None)),
    (CELLAR_RYETH, (true, 500, None)),
    (CELLAR_TURBO_STETH, (false, 500, Some(PRICEROUTER1))),
    (CELLAR_TURBO_STETH, (true, 500, Some(PRICEROUTER2))),
];

// permissions

pub const ALLOWED_V2_0_SETUP_ADAPTORS: [(&str, &str); 0] = [];
pub const ALLOWED_V2_2_CATALOGUE_ADAPTORS: [(&str, &str); 0] = [];
pub const ALLOWED_V2_5_CATALOGUE_ADAPTORS: [(&str, &str); 0] = [];

// due to position size limits in v2.0, positions must be added and removed from the limited list
// and thus approved positions need to be allowed to be re-added, hence this large list
pub const ALLOWED_V2_0_POSITIONS: [(&str, u32); 20] = [
    (CELLAR_RYUSD, 1),
    (CELLAR_RYUSD, 2),
    (CELLAR_RYUSD, 3),
    (CELLAR_RYUSD, 13),
    (CELLAR_RYUSD, 14),
    (CELLAR_RYUSD, 15),
    (CELLAR_RYUSD, 16),
    (CELLAR_RYUSD, 17),
    (CELLAR_RYUSD, 18),
    (CELLAR_RYUSD, 19),
    (CELLAR_RYUSD, 20),
    (CELLAR_RYUSD, 21),
    (CELLAR_RYUSD, 22),
    (CELLAR_RYUSD, 23),
    (CELLAR_RYUSD, 24),
    (CELLAR_RYUSD, 25),
    (CELLAR_RYUSD, 26),
    (CELLAR_RYUSD, 27),
    (CELLAR_RYUSD, 28),
    (CELLAR_RYUSD, 29),
];
pub const ALLOWED_V2_2_CATALOGUE_POSITIONS: [(&str, u32); 0] = [];
pub const ALLOWED_V2_5_CATALOGUE_POSITIONS: [(&str, u32); 0] = [];

pub const BLOCKED_ADAPTORS: [&str; 3] = [
    ADAPTOR_UNIV3_V1,
    ADAPTOR_VESTING_SIMPLE_V1,
    ADAPTOR_COMPOUND_C_TOKEN_V1,
];

pub const BLOCKED_V2_0_POSITIONS: [u32; 9] = [4, 5, 6, 7, 8, 9, 10, 11, 12];
pub const BLOCKED_V2_2_POSITIONS: [u32; 0] = [];
pub const BLOCKED_V2_5_POSITIONS: [u32; 0] = [];

pub struct CellarArchPermissions {
    allowed_adaptors: &'static [(&'static str, &'static str)],
    allowed_positions: &'static [(&'static str, u32)],
    blocked_positions: &'static [u32],
}

pub const V2_0_PERMISSIONS: CellarArchPermissions = CellarArchPermissions {
    allowed_adaptors: &ALLOWED_V2_0_SETUP_ADAPTORS,
    allowed_positions: &ALLOWED_V2_0_POSITIONS,
    blocked_positions: &BLOCKED_V2_0_POSITIONS,
};
pub const V2_2_PERMISSIONS: CellarArchPermissions = CellarArchPermissions {
    allowed_adaptors: &ALLOWED_V2_2_CATALOGUE_ADAPTORS,
    allowed_positions: &ALLOWED_V2_2_CATALOGUE_POSITIONS,
    blocked_positions: &BLOCKED_V2_2_POSITIONS,
};
pub const V2_5_PERMISSIONS: CellarArchPermissions = CellarArchPermissions {
    allowed_adaptors: &ALLOWED_V2_5_CATALOGUE_ADAPTORS,
    allowed_positions: &ALLOWED_V2_5_CATALOGUE_POSITIONS,
    blocked_positions: &BLOCKED_V2_5_POSITIONS,
};

// cellars

pub const CELLAR_RY1INCH: &str = "c7b69e15d86c5c1581dacce3cacaf5b68cd6596f";
pub const CELLAR_RYENS: &str = "18ea937aba6053bc232d9ae2c42abe7a8a2be440";
pub const CELLAR_RYETH: &str = "b5b29320d2dde5ba5bafa1ebcd270052070483ec";
pub const CELLAR_RYLINK: &str = "4068bdd217a45f8f668ef19f1e3a1f043e4c4934";
pub const CELLAR_RYSNX: &str = "cbf2250f33c4161e18d4a2fa47464520af5216b5";
pub const CELLAR_RYUNI: &str = "6a6af5393dc23d7e3db28d28ef422db7c40932b6";
pub const CELLAR_RYUSD: &str = "97e6e0a40a3d02f12d1cec30ebfbae04e37c119e";
pub const CELLAR_RYBTC: &str = "0274a704a6d9129f90a62ddc6f6024b33ecdad36";
pub const CELLAR_TURBO_SWETH: &str = "d33dad974b938744dac81fe00ac67cb5aa13958e";
pub const CELLAR_TURBO_GHO: &str = "0c190ded9be5f512bd72827bdad4003e9cc7975c";
pub const CELLAR_TURBO_STETH: &str = "fd6db5011b171b05e1ea3b92f9eacaeeb055e971";

// deprecated adaptors

pub const ADAPTOR_UNIV3_V1: &str = "7c4262f83e6775d6ff6fe8d9ab268611ed9d13ee";
pub const ADAPTOR_VESTING_SIMPLE_V1: &str = "1eaa1a100a460f46a2032f0402bc01fe89faab60";
pub const ADAPTOR_COMPOUND_C_TOKEN_V1: &str = "26dba82495f6189dde7648ae88bead46c402f078";

// adaptors

pub const ADAPTOR_AAVE_V3_A_TOKEN_V1: &str = "76cef5606c8b6ba38fe2e3c639e1659afa530b47";
pub const ADAPTOR_CELLAR_V2: &str = "3b5ca5de4d808cd793d3a7b3a731d3e67e707b27";
pub const ADAPTOR_COLLATERAL_F_TOKEN_V1: &str = "0055cf6a99eba1405d100c7dfaa88a35521a0037";
pub const ADAPTOR_DEBT_F_TOKEN_V1: &str = "50d8f70a5da95021dab86579db4751a863c1b87c";
pub const ADAPTOR_LEGACY_CELLAR_V1: &str = "1e22adf9e63ef8f2a3626841ddddd19683e31068";
pub const ADAPTOR_MORPHO_AAVE_V2_A_TOKEN_V1: &str = "1a4cb53edb8c65c3df6aa9d88c1ab4cf35312b73";
pub const ADAPTOR_MORPHO_AAVE_V2_DEBT_TOKEN_V1: &str = "407d5489f201013ee6a6ca20fccb05047c548138";
pub const ADAPTOR_MORPHO_AAVE_V3_A_TOKEN_COLLATERAL_V1: &str =
    "b46e8a03b1aafffb50f281397c57b5b87080363e";
pub const ADAPTOR_MORPHO_AAVE_V3_DEBT_TOKEN_V1: &str = "25a61f771af9a38c10ddd93c2bbab39a88926fa9";
pub const ADAPTOR_MORPHO_AAVE_V3_P2P_V1: &str = "4fe068caad05b82bf3f86e1f7d1a7b8bbf516111";
pub const ADAPTOR_UNIV3_V3: &str = "92611574ec9bc13c6137917481dab7bb7b173c9b";
pub const ADAPTOR_VESTING_SIMPLE_V2: &str = "3b98ba00f981342664969e609fb88280704ac479";

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

pub fn validate_new_adaptor(
    cellar_id: &str,
    adaptor_id: &str,
    permissions: &CellarArchPermissions,
) -> Result<(), Error> {
    let adaptor_id = normalize_address(adaptor_id.to_string());
    check_blocked_adaptor(&adaptor_id)?;

    let cellar_id = normalize_address(cellar_id.to_string());
    if !permissions
        .allowed_adaptors
        .contains(&(&cellar_id, &adaptor_id))
    {
        return Err(sp_call_error(format!(
            "new adaptor {adaptor_id} not allowed to be added for cellar {cellar_id}"
        )));
    }

    Ok(())
}

pub fn validate_new_position(
    cellar_id: &str,
    position: u32,
    permissions: &CellarArchPermissions,
) -> Result<(), Error> {
    check_blocked_position(&position, permissions)?;

    let cellar_id = normalize_address(cellar_id.to_string());
    if !permissions
        .allowed_positions
        .contains(&(&cellar_id, position))
    {
        return Err(sp_call_error(format!(
            "new position {position} not allowed to be added for cellar {cellar_id}"
        )));
    }

    Ok(())
}

pub fn validate_oracle(
    cellar_id: &str,
    registry_id_in: &str,
    oracle_in: &str,
) -> Result<(), Error> {
    let cellar_id_normalized = normalize_address(cellar_id.to_string());
    let oracle_in = normalize_address(oracle_in.to_string());
    let registry_id_in = string_to_u256(registry_id_in.to_string())?;
    if !cellar_id_normalized.eq(CELLAR_TURBO_SWETH)
        || !ALLOWED_PRICE_ORACLES.contains(&(registry_id_in, oracle_in.as_str()))
    {
        return Err(sp_call_error("unauthorized oracle update".to_string()));
    }

    Ok(())
}

pub fn validate_cache_price_router(
    cellar_id: &str,
    check_total_assets_value: bool,
    allowable_range_value: u32,
    expected_price_router: Option<&str>,
) -> Result<(), Error> {
    let cellar_id_normalized = normalize_address(cellar_id.to_string());

    if !ALLOWED_CACHE_PRICE_ROUTER
        .iter()
        .any(|(cellar_id, permissions)| {
            cellar_id_normalized.eq(cellar_id)
                && permissions.0 == check_total_assets_value
                && permissions.1 >= allowable_range_value
                && permissions.2 == expected_price_router
        })
    {
        return Err(sp_call_error("call not authorized for cellar".to_string()));
    }

    Ok(())
}

pub fn check_blocked_adaptor(adaptor_id: &str) -> Result<(), Error> {
    let adaptor_id = normalize_address(adaptor_id.to_string());
    if BLOCKED_ADAPTORS.contains(&adaptor_id.as_str()) {
        return Err(sp_call_error(format!("adaptor {adaptor_id} is blocked")));
    }

    Ok(())
}

pub fn check_blocked_position(
    position: &u32,
    permissions: &CellarArchPermissions,
) -> Result<(), Error> {
    if permissions.blocked_positions.contains(position) {
        return Err(sp_call_error(format!("position {position} is blocked")));
    }

    Ok(())
}

pub fn validate_cellar_id(cellar_id: &str) -> Result<(), Error> {
    if let Err(err) = cellar_id.parse::<H160>() {
        return Err(sp_call_error(format!("invalid ethereum address: {err}")));
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

        assert!(!ALLOWED_V2_0_SETUP_ADAPTORS
            .contains(&(CELLAR_RYUSD, normalize_address(blocked1.clone()).as_ref())));

        // idempotent
        let once = normalize_address(blocked1);
        let twice = normalize_address(once.clone());
        assert_eq!(&once, &twice);
    }

    #[test]
    fn test_check_blocked_position() {
        let error_prefix = "SP call error: ".to_string();

        // allows unblocked position ID
        let unblocked_pos = 25;
        assert!(check_blocked_position(&unblocked_pos, &V2_0_PERMISSIONS).is_ok());
        assert!(check_blocked_position(&unblocked_pos, &V2_2_PERMISSIONS).is_ok());
        assert!(check_blocked_position(&unblocked_pos, &V2_5_PERMISSIONS).is_ok());

        // rejects blocked position ID
        let v2_0_blocked_pos = 4;
        let res = check_blocked_position(&v2_0_blocked_pos, &V2_0_PERMISSIONS);
        let expected_err =
            error_prefix.clone() + &format!("position {v2_0_blocked_pos} is blocked");
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());

        // this position is only blocked on V2.0
        assert!(check_blocked_position(&v2_0_blocked_pos, &V2_2_PERMISSIONS).is_ok());
        assert!(check_blocked_position(&v2_0_blocked_pos, &V2_5_PERMISSIONS).is_ok());
    }

    #[test]
    fn test_check_blocked_adaptor() {
        let error_prefix = "SP call error: ".to_string();

        // allows unblocked adaptor ID
        let unblocked_adaptor = ADAPTOR_MORPHO_AAVE_V2_A_TOKEN_V1;
        assert!(check_blocked_adaptor(&unblocked_adaptor).is_ok());

        // rejects blocked adaptor ID
        let blocked_adaptor = ADAPTOR_UNIV3_V1;
        let res = check_blocked_adaptor(&blocked_adaptor);
        let expected_err = error_prefix.clone() + &format!("adaptor {blocked_adaptor} is blocked");
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());
    }

    #[test]
    fn test_validate_new_adaptor() {
        let error_prefix = "SP call error: ".to_string();

        // allows approved cellar/adaptor ID pairs
        let (v2_0_cellar_id, v2_0_approved_adaptor_id) = (CELLAR_RYUSD, ADAPTOR_CELLAR_V2);
        // assert!(
        //     validate_new_adaptor(v2_0_cellar_id, v2_0_approved_adaptor_id, &V2_0_PERMISSIONS)
        //         .is_ok()
        // );

        // rejects blocked adaptor ID
        let blocked_adaptor_id = ADAPTOR_UNIV3_V1;
        let res = validate_new_adaptor(v2_0_cellar_id, blocked_adaptor_id, &V2_0_PERMISSIONS);
        let expected_err =
            error_prefix.clone() + &format!("adaptor {blocked_adaptor_id} is blocked");
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());

        // rejects unapproved cellar/adaptor ID pair
        let unapproved_adaptor_id = ADAPTOR_UNIV3_V3;
        let res = validate_new_adaptor(v2_0_cellar_id, unapproved_adaptor_id, &V2_0_PERMISSIONS);
        let expected_err = error_prefix.clone()
            + &format!(
                "new adaptor {unapproved_adaptor_id} not allowed to be added for cellar {v2_0_cellar_id}"
            );
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());

        let unapproved_cellar = "0000000000000000000000000000000000000000";
        let res = validate_new_adaptor(
            unapproved_cellar,
            v2_0_approved_adaptor_id,
            &V2_0_PERMISSIONS,
        );
        let expected_err = error_prefix
            + &format!("new adaptor {v2_0_approved_adaptor_id} not allowed to be added for cellar {unapproved_cellar}");
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());
    }

    #[test]
    fn test_validate_new_position() {
        let error_prefix = "SP call error: ".to_string();

        // allows approved cellar/position ID pairs
        let (v2_0_cellar_id, v2_0_approved_pos) = (CELLAR_RYUSD, 1);
        assert!(
            validate_new_position(v2_0_cellar_id, v2_0_approved_pos, &V2_0_PERMISSIONS).is_ok()
        );

        // rejects blocked position ID
        let blocked_pos = 4;
        let res = validate_new_position(v2_0_cellar_id, blocked_pos, &V2_0_PERMISSIONS);
        let expected_err = error_prefix.clone() + &format!("position {blocked_pos} is blocked");
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());

        // rejects unapproved cellar/position ID pair
        let unapproved_pos = 153;
        let res = validate_new_position(v2_0_cellar_id, unapproved_pos, &V2_0_PERMISSIONS);
        let expected_err = error_prefix.clone()
            + &format!(
                "new position {unapproved_pos} not allowed to be added for cellar {v2_0_cellar_id}"
            );
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());

        let unapproved_cellar = "0000000000000000000000000000000000000000";
        let res = validate_new_position(unapproved_cellar, v2_0_approved_pos, &V2_0_PERMISSIONS);
        let expected_err = error_prefix
            + &format!(
                "new position {v2_0_approved_pos} not allowed to be added for cellar {unapproved_cellar}"
            );
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());
    }

    #[test]
    fn test_u256_sanity() {
        assert_eq!(U256([5, 0, 0, 0]), U256::from(5));
    }

    #[test]
    fn test_validate_oracle() {
        assert!(validate_oracle(CELLAR_RYBTC, &ORACLE1.0.to_string(), ORACLE1.1).is_err());
        assert!(
            validate_oracle(CELLAR_TURBO_SWETH, &U256::from(6).to_string(), ORACLE2.1).is_err()
        );
        assert!(validate_oracle(CELLAR_TURBO_SWETH, &ORACLE1.0.to_string(), ORACLE1.1).is_ok());
        assert!(validate_oracle(CELLAR_TURBO_SWETH, &ORACLE2.0.to_string(), ORACLE2.1).is_ok());
    }

    #[test]
    fn test_validate_cache_price_router() {
        // valid
        assert!(
            validate_cache_price_router(CELLAR_TURBO_STETH, true, 400, Some(PRICEROUTER2)).is_ok()
        );

        // invalid
        assert!(
            validate_cache_price_router(CELLAR_TURBO_SWETH, true, 500, Some(PRICEROUTER2)).is_err()
        );
        assert!(validate_cache_price_router(CELLAR_RYETH, false, 500, None).is_err());
        assert!(validate_cache_price_router(CELLAR_RYBTC, true, 600, None).is_err());
    }
}
