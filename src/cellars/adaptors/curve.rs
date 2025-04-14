use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::adaptors::curve_adaptor_v1::{
    self, CurveAdaptorV1Calls as AbiCurveAdaptorV1Calls,
};

use crate::{
    error::Error,
    proto::{curve_adaptor_v1::Function, CurveAdaptorV1Calls},
    utils::{parse_selector, sp_call_error, sp_call_parse_address, string_to_u256},
};

/// Encodes adaptor calls for CurveAdaptor V1
pub fn curve_adaptor_v1_calls(params: CurveAdaptorV1Calls) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            Function::RevokeApproval(p) => {
                let call = curve_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(AbiCurveAdaptorV1Calls::RevokeApproval(call).encode().into())
            }
            Function::AddLiquidity(p) => {
                let call = curve_adaptor_v1::AddLiquidityCall {
                    pool: sp_call_parse_address(p.pool)?,
                    lp_token: sp_call_parse_address(p.lp_token)?,
                    ordered_underlying_token_amounts: p
                        .ordered_underlying_token_amounts
                        .into_iter()
                        .map(string_to_u256)
                        .collect::<Result<_, Error>>()?,
                    min_lp_amount: string_to_u256(p.min_lp_amount)?,
                    gauge: sp_call_parse_address(p.gauge)?,
                    selector: parse_selector(p.selector)?,
                };
                calls.push(AbiCurveAdaptorV1Calls::AddLiquidity(call).encode().into())
            }
            Function::AddLiquidityEth(p) => {
                let call = curve_adaptor_v1::AddLiquidityETHCall {
                    pool: sp_call_parse_address(p.pool)?,
                    lp_token: sp_call_parse_address(p.lp_token)?,
                    ordered_underlying_token_amounts: p
                        .ordered_underlying_token_amounts
                        .into_iter()
                        .map(string_to_u256)
                        .collect::<Result<_, Error>>()?,
                    min_lp_amount: string_to_u256(p.min_lp_amount)?,
                    use_underlying: p.use_underlying,
                    gauge: sp_call_parse_address(p.gauge)?,
                    selector: parse_selector(p.selector)?,
                };
                calls.push(
                    AbiCurveAdaptorV1Calls::AddLiquidityETH(call)
                        .encode()
                        .into(),
                )
            }
            Function::RemoveLiquidity(p) => {
                let call = curve_adaptor_v1::RemoveLiquidityCall {
                    pool: sp_call_parse_address(p.pool)?,
                    lp_token: sp_call_parse_address(p.lp_token)?,
                    lp_token_amount: string_to_u256(p.lp_token_amount)?,
                    ordered_minimum_underlying_token_amounts_out: p
                        .ordered_minimum_underlying_token_amounts_out
                        .into_iter()
                        .map(string_to_u256)
                        .collect::<Result<_, Error>>()?,
                    gauge: sp_call_parse_address(p.gauge)?,
                    selector: parse_selector(p.selector)?,
                };
                calls.push(
                    AbiCurveAdaptorV1Calls::RemoveLiquidity(call)
                        .encode()
                        .into(),
                )
            }
            Function::RemoveLiquidityEth(p) => {
                let call = curve_adaptor_v1::RemoveLiquidityETHCall {
                    pool: sp_call_parse_address(p.pool)?,
                    lp_token: sp_call_parse_address(p.lp_token)?,
                    lp_token_amount: string_to_u256(p.lp_token_amount)?,
                    ordered_minimum_underlying_token_amounts_out: p
                        .ordered_minimum_underlying_token_amounts_out
                        .into_iter()
                        .map(string_to_u256)
                        .collect::<Result<_, Error>>()?,
                    use_underlying: p.use_underlying,
                    gauge: sp_call_parse_address(p.gauge)?,
                    selector: parse_selector(p.selector)?,
                };
                calls.push(
                    AbiCurveAdaptorV1Calls::RemoveLiquidityETH(call)
                        .encode()
                        .into(),
                )
            }
            Function::StakeInGauge(p) => {
                let call = curve_adaptor_v1::StakeInGaugeCall {
                    lp_token: sp_call_parse_address(p.lp_token)?,
                    gauge: sp_call_parse_address(p.gauge)?,
                    amount: string_to_u256(p.amount)?,
                    pool: sp_call_parse_address(p.pool)?,
                    selector: parse_selector(p.selector)?,
                };
                calls.push(AbiCurveAdaptorV1Calls::StakeInGauge(call).encode().into())
            }
            Function::UnstakeFromGauge(p) => {
                let call = curve_adaptor_v1::UnStakeFromGaugeCall {
                    gauge: sp_call_parse_address(p.gauge)?,
                    amount: string_to_u256(p.amount)?,
                };
                calls.push(
                    AbiCurveAdaptorV1Calls::UnStakeFromGauge(call)
                        .encode()
                        .into(),
                )
            }
            Function::ClaimRewards(p) => {
                let call = curve_adaptor_v1::ClaimRewardsCall {
                    gauge: sp_call_parse_address(p.gauge)?,
                };
                calls.push(AbiCurveAdaptorV1Calls::ClaimRewards(call).encode().into())
            }
        }
    }

    Ok(calls)
}
