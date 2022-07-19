use crate::{
    cork::cache::{self, is_approved},
    error::{Error, ErrorKind},
    gas::CellarGas,
    utils::get_eth_provider,
};
use abscissa_core::tracing::{info, warn};
use ethers::prelude::*;
use std::result::Result;

pub(crate) mod aave_v2_stablecoin;

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
    if let Err(err) = cellar_id.parse::<H160>() {
        return Err(ErrorKind::SPCall
            .context(format!("invalid ethereum address: {}", err))
            .into());
    }

    if !is_approved(cellar_id) {
        if let Err(err) = cache::refresh_approved_cellars().await {
            return Err(ErrorKind::Cache
                .context(format!("failed to refresh approved cellar cache while processing SubmitCork request: {}", err))
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

#[cfg(test)]
mod tests {
    use super::*;
    use assay::assay;

    #[assay]
    async fn invalid_cellar_id_format_errors() {
        let cellar_id = "thisaintright";
        let result = validate_cellar_id(cellar_id).await;

        assert!(result.is_err())
    }
}
