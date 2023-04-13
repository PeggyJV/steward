use abscissa_core::tracing::log::{info, warn};
use ethers::prelude::*;
use std::result::Result;

use crate::{
    cork::cache::{self, is_approved},
    error::{Error, ErrorKind},
    gas::CellarGas,
    utils::get_eth_provider,
};

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
    "b8952ce4010cff3c74586d712a4402285a3a3afb",
    // SwapWithUniswapAdaptor v1
    "d6bc6df1ed43e3101bc27a4254593a06598a3fdd",
    // ZeroXAdaptor v1
    "1039a9b61dff6a3fb8dbf4e924aa749e5cfe35ef",
    // AaveV3ATokenAdaptor v1
    "3184cbea47ed519fa04a23c4207cd15b7545f1a6",
    // AaveATokenAdaptor v2
    "25570a77dca06fda89c1ef41fab6ee48a2377e81",
    // FeesAndReservesAdaptor v1
    "647d264d800a2461e594796af61a39b7735d8933",
    // CTokenAdaptor v2
    "9a384df333588428843d128120becd72434ec078",
];

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

/// Checks that a cellar ID is a valid Ethereum address and that it is approved by governance. If it is not found in the
/// approved cellar cache initially, we force a cache refresh and check again in case the cellar was approved on-chain
/// since the last automatic refresh.
pub async fn validate_cellar_id(cellar_id: &str) -> Result<(), Error> {
    if let Err(err) = cellar_id.parse::<Address>() {
        return Err(ErrorKind::InvalidEthereumAddress
            .context(format!("invalid cellar ID format {}: {}", cellar_id, err))
            .into());
    }

    if !is_approved(cellar_id) {
        if let Err(err) = cache::refresh_approved_cellars().await {
            return Err(ErrorKind::CacheError
                .context(format!("failed to refresh approved cellar cache: {}", err))
                .into());
        }

        if !is_approved(cellar_id) {
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

#[cfg(test)]
mod tests {
    use assay::assay;

    use super::*;

    #[assay]
    async fn invalid_cellar_id_format_errors() {
        let cellar_id = "thisaintright";
        let result = validate_cellar_id(cellar_id).await;

        assert!(result.is_err())
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
