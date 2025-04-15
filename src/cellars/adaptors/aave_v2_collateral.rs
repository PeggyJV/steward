use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::adaptors::aave_v2_enable_asset_as_collateral_adaptor_v1::AaveV2EnableAssetAsCollateralAdaptorV1Calls as AbiAaveV2EnableAssetAsCollateralAdaptorV1Calls;

use crate::{
    error::Error,
    proto::aave_v2_enable_asset_as_collateral_adaptor_v1,
    utils::{sp_call_error, sp_call_parse_address},
};

pub(crate) fn aave_v2_enable_asset_as_collateral_adaptor_v1_calls(
    params: crate::proto::AaveV2EnableAssetAsCollateralAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            aave_v2_enable_asset_as_collateral_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::adaptors::aave_v2_enable_asset_as_collateral_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AbiAaveV2EnableAssetAsCollateralAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            },
            aave_v2_enable_asset_as_collateral_adaptor_v1::Function::SetUserUseReserveAsCollateral(p) => {
                let call = steward_abi::adaptors::aave_v2_enable_asset_as_collateral_adaptor_v1::SetUserUseReserveAsCollateralCall {
                    asset: sp_call_parse_address(p.asset)?,
                    use_as_collateral: p.use_as_collateral,
                };
                calls.push(
                    AbiAaveV2EnableAssetAsCollateralAdaptorV1Calls::SetUserUseReserveAsCollateral(call)
                        .encode()
                        .into(),
                )
            },
        }
    }

    Ok(calls)
}
