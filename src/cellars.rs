use crate::{
    cork::cache::{self, is_approved},
    error::{Error, ErrorKind},
    utils::sp_call_error,
};
use abscissa_core::tracing::info;
use ethers::abi::Address;
use std::result::Result;

pub(crate) mod aave_v2_stablecoin;
pub(crate) mod adaptors;
pub(crate) mod cellar_v1;
pub(crate) mod cellar_v2;
pub(crate) mod cellar_v2_2;

// constants
// addresses are normalized by removing the 0x prefix and converting to lowercase for reliable comparison

// block lists

pub const BLOCKED_ADAPTORS: [&str; 3] = [
    ADAPTOR_UNIV3_V1,
    ADAPTOR_VESTING_SIMPLE_V1,
    ADAPTOR_COMPOUND_C_TOKEN_V1,
];
pub const BLOCKED_POSITIONS: [u32; 9] = [4, 5, 6, 7, 8, 9, 10, 11, 12];

// cellars

pub const CELLAR_RY1INCH: &str = "c7b69e15d86c5c1581dacce3cacaf5b68cd6596f";
pub const CELLAR_RYBTC: &str = "0274a704a6d9129f90a62ddc6f6024b33ecdad36";
pub const CELLAR_RYENS: &str = "18ea937aba6053bc232d9ae2c42abe7a8a2be440";
pub const CELLAR_RYETH: &str = "b5b29320d2dde5ba5bafa1ebcd270052070483ec";
pub const CELLAR_RYLINK: &str = "4068bdd217a45f8f668ef19f1e3a1f043e4c4934";
pub const CELLAR_RYSNX: &str = "cbf2250f33c4161e18d4a2fa47464520af5216b5";
pub const CELLAR_RYUNI: &str = "6a6af5393dc23d7e3db28d28ef422db7c40932b6";
pub const CELLAR_RYUSD: &str = "97e6e0a40a3d02f12d1cec30ebfbae04e37c119e";

// deprecated adaptors

pub const ADAPTOR_UNIV3_V1: &str = "7c4262f83e6775d6ff6fe8d9ab268611ed9d13ee";
pub const ADAPTOR_VESTING_SIMPLE_V1: &str = "1eaa1a100a460f46a2032f0402bc01fe89faab60";
pub const ADAPTOR_COMPOUND_C_TOKEN_V1: &str = "26dba82495f6189dde7648ae88bead46c402f078";

// adaptors

pub const ADAPTOR_CELLAR_V2: &str = "3b5ca5de4d808cd793d3a7b3a731d3e67e707b27";
pub const ADAPTOR_MORPHO_AAVE_V2_A_TOKEN_V1: &str = "1a4cb53edb8c65c3df6aa9d88c1ab4cf35312b73";
pub const ADAPTOR_MORPHO_AAVE_V2_DEBT_TOKEN_V1: &str = "407d5489f201013ee6a6ca20fccb05047c548138";
pub const ADAPTOR_MORPHO_AAVE_V3_A_TOKEN_COLLATERAL_V1: &str =
    "b46e8a03b1aafffb50f281397c57b5b87080363e";
pub const ADAPTOR_MORPHO_AAVE_V3_DEBT_TOKEN_V1: &str = "25a61f771af9a38c10ddd93c2bbab39a88926fa9";
pub const ADAPTOR_MORPHO_AAVE_V3_P2P_V1: &str = "4fe068caad05b82bf3f86e1f7d1a7b8bbf516111";

// utils

pub fn log_cellar_call(cellar_name: &str, function_name: &str, cellar_id: &str) {
    info!(
        "encoding {}.{} call for cellar {}",
        cellar_name, function_name, cellar_id
    );
}

pub fn log_governance_cellar_call(
    proposal_id: u64,
    cellar_name: &str,
    function_name: &str,
    cellar_id: &str,
) {
    info!(
        "[Proposal {}]: encoding {}.{} call for cellar {}",
        proposal_id, cellar_name, function_name, cellar_id
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

pub fn check_blocked_position(position: &u32) -> Result<(), Error> {
    if BLOCKED_POSITIONS.contains(position) {
        return Err(sp_call_error(format!("position {position} is blocked")));
    }

    Ok(())
}

pub fn is_evm_address(address: &str) -> bool {
    address.parse::<Address>().is_ok()
}

/// Checks that a cellar ID is a valid EVM address and that it is approved by governance. If it is not found in the
/// approved cellar cache initially, we force a cache refresh and check again in case the cellar was approved on-chain
/// since the last automatic refresh.
pub async fn validate_cellar_id(chain_id: u64, cellar_id: &str) -> Result<(), Error> {
    if !is_evm_address(cellar_id) {
        return Err(ErrorKind::InvalidEVMAddress
            .context(format!(
                "invalid cellar ID format {}: must be valid EVM address",
                cellar_id
            ))
            .into());
    }

    let cellar_id = to_checksum_address(cellar_id)?;

    if !is_approved(chain_id, &cellar_id) {
        if let Err(err) = cache::refresh_approved_cellars().await {
            return Err(ErrorKind::CacheError
                .context(format!("failed to refresh approved cellar cache: {}", err))
                .into());
        }

        if !is_approved(chain_id, &cellar_id) {
            return Err(ErrorKind::UnapprovedCellar
                .context(format!(
                    "cellar ID {} is not approved by governance",
                    cellar_id
                ))
                .into());
        }
    }

    Ok(())
}

pub(crate) fn to_checksum_address(address: &str) -> Result<String, Error> {
    let address = match address.parse::<Address>() {
        Ok(a) => a,
        Err(e) => {
            return Err(ErrorKind::InvalidEVMAddress
                .context(format!("invalid EVM address format ({}): {:?}", address, e))
                .into())
        }
    };
    let address = ethers::utils::to_checksum(&address, None);

    Ok(address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_cellar_id_format_errors() {
        let cellar_id = "thisaintright";

        assert!(!is_evm_address(cellar_id));
    }

    #[test]
    fn valid_cellar_id_format_works() {
        let cellar_id = "0x0000000000000000000000000000000000000000";

        assert!(is_evm_address(cellar_id));
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

        // idempotent
        let once = normalize_address(blocked1);
        let twice = normalize_address(once.clone());
        assert_eq!(&once, &twice);
    }

    #[test]
    fn test_check_blocked_position() {
        // allows unblocked position ID
        let unblocked_pos = 25;
        assert!(check_blocked_position(&unblocked_pos).is_ok());

        let error_prefix = "SP call error: ".to_string();

        // rejects blocked position ID
        let blocked_pos = 4;
        let res = check_blocked_position(&blocked_pos);
        let expected_err = error_prefix.clone() + &format!("position {blocked_pos} is blocked");
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());
    }

    #[test]
    fn test_check_blocked_adaptor() {
        // allows unblocked adaptor ID
        let unblocked_adaptor = ADAPTOR_MORPHO_AAVE_V2_A_TOKEN_V1;
        assert!(check_blocked_adaptor(&unblocked_adaptor).is_ok());

        let error_prefix = "SP call error: ".to_string();

        // rejects blocked adaptor ID
        let blocked_adaptor = ADAPTOR_UNIV3_V1;
        let res = check_blocked_adaptor(&blocked_adaptor);
        let expected_err = error_prefix.clone() + &format!("adaptor {blocked_adaptor} is blocked");
        assert!(res.is_err());
        assert_eq!(expected_err, res.unwrap_err().to_string());
    }
}
