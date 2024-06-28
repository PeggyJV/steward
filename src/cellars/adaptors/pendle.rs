use abscissa_core::tracing::log::debug;
use ethers::{abi::AbiEncode, types::Bytes};

use crate::{
    abi::adaptors::pendle_adaptor_v1::{self, PendleAdaptorV1Calls as AbiPendleAdaptorV1Calls},
    error::Error,
    proto::{pendle_adaptor_v1::Function, PendleAdaptorV1Calls},
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

/// Encodes adaptor calls for PendleAdaptorV1
pub fn pendle_adaptor_v1_calls(params: PendleAdaptorV1Calls) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            Function::RevokeApproval(p) => {
                let call = pendle_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AbiPendleAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            Function::MintSyFromToken(p) => {
                let call = pendle_adaptor_v1::MintSyFromTokenCall {
                    market: sp_call_parse_address(p.market)?,
                    min_sy_out: string_to_u256(p.min_sy_out)?,
                    input: convert_token_input(p.input)?,
                };
                calls.push(
                    AbiPendleAdaptorV1Calls::MintSyFromToken(call)
                        .encode()
                        .into(),
                )
            }
            Function::MintPyFromSy(p) => {
                let call = pendle_adaptor_v1::MintPyFromSyCall {
                    market: sp_call_parse_address(p.market)?,
                    net_sy_in: string_to_u256(p.net_sy_in)?,
                    min_py_out: string_to_u256(p.min_py_out)?,
                };
                calls.push(AbiPendleAdaptorV1Calls::MintPyFromSy(call).encode().into())
            }
            Function::SwapExactPtForYt(p) => {
                let call = pendle_adaptor_v1::SwapExactPtForYtCall {
                    market: sp_call_parse_address(p.market)?,
                    min_yt_out: string_to_u256(p.min_yt_out)?,
                    exact_pt_in: string_to_u256(p.exact_pt_in)?,
                    guess_total_yt_to_swap: convert_approx_params(p.guess_total_yt_to_swap)?,
                };
                calls.push(
                    AbiPendleAdaptorV1Calls::SwapExactPtForYt(call)
                        .encode()
                        .into(),
                )
            }
            Function::SwapExactYtForPt(p) => {
                let call = pendle_adaptor_v1::SwapExactYtForPtCall {
                    market: sp_call_parse_address(p.market)?,
                    min_pt_out: string_to_u256(p.min_pt_out)?,
                    exact_yt_in: string_to_u256(p.exact_yt_in)?,
                    guess_total_pt_from_swap: convert_approx_params(p.guess_total_pt_to_swap)?,
                };
                calls.push(
                    AbiPendleAdaptorV1Calls::SwapExactYtForPt(call)
                        .encode()
                        .into(),
                )
            }
            Function::AddLiquidityDualSyAndPt(p) => {
                let call = pendle_adaptor_v1::AddLiquidityDualSyAndPtCall {
                    market: sp_call_parse_address(p.market)?,
                    net_sy_desired: string_to_u256(p.net_sy_desired)?,
                    net_pt_desired: string_to_u256(p.net_pt_desired)?,
                    min_lp_out: string_to_u256(p.min_lp_out)?,
                };
                calls.push(
                    AbiPendleAdaptorV1Calls::AddLiquidityDualSyAndPt(call)
                        .encode()
                        .into(),
                )
            }
            Function::RemoveLiquidityDualSyAndPt(p) => {
                let call = pendle_adaptor_v1::RemoveLiquidityDualSyAndPtCall {
                    market: sp_call_parse_address(p.market)?,
                    net_lp_to_remove: string_to_u256(p.net_lp_to_remove)?,
                    min_sy_out: string_to_u256(p.min_sy_out)?,
                    min_pt_out: string_to_u256(p.min_pt_out)?,
                };
                calls.push(
                    AbiPendleAdaptorV1Calls::RemoveLiquidityDualSyAndPt(call)
                        .encode()
                        .into(),
                )
            }
            Function::RedeemPyToSy(p) => {
                let call = pendle_adaptor_v1::RedeemPyToSyCall {
                    market: sp_call_parse_address(p.market)?,
                    net_py_in: string_to_u256(p.net_py_in)?,
                    min_sy_out: string_to_u256(p.min_sy_out)?,
                };
                calls.push(AbiPendleAdaptorV1Calls::RedeemPyToSy(call).encode().into())
            }
            Function::RedeemSyToToken(p) => {
                let call = pendle_adaptor_v1::RedeemSyToTokenCall {
                    market: sp_call_parse_address(p.market)?,
                    net_sy_in: string_to_u256(p.net_sy_in)?,
                    output: convert_token_output(p.output)?,
                };
                calls.push(
                    AbiPendleAdaptorV1Calls::RedeemSyToToken(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}

fn convert_approx_params(
    params: Option<crate::proto::pendle_adaptor_v1::ApproxParams>,
) -> Result<pendle_adaptor_v1::ApproxParams, Error> {
    let params = params.ok_or_else(|| sp_call_error("params cannot be empty".to_string()))?;

    Ok(pendle_adaptor_v1::ApproxParams {
        guess_min: string_to_u256(params.guess_min)?,
        guess_max: string_to_u256(params.guess_max)?,
        guess_offchain: string_to_u256(params.guess_offchain)?,
        max_iteration: string_to_u256(params.max_iteration)?,
        eps: string_to_u256(params.eps)?,
    })
}

fn convert_token_input(
    input: Option<crate::proto::pendle_adaptor_v1::TokenInput>,
) -> Result<pendle_adaptor_v1::TokenInput, Error> {
    let input = input.ok_or_else(|| sp_call_error("input cannot be empty".to_string()))?;
    let swap_data = input
        .swap_data
        .ok_or_else(|| sp_call_error("swap_data cannot be empty".to_string()))?;

    Ok(pendle_adaptor_v1::TokenInput {
        token_in: sp_call_parse_address(input.token_in)?,
        net_token_in: string_to_u256(input.net_token_in)?,
        token_mint_sy: sp_call_parse_address(input.token_mint_sy)?,
        pendle_swap: sp_call_parse_address(input.pendle_swap)?,
        swap_data: pendle_adaptor_v1::SwapData {
            swap_type: swap_data.swap_type as u8,
            ext_router: sp_call_parse_address(swap_data.ext_router)?,
            ext_calldata: hex::decode(swap_data.ext_calldata)?.into(),
            need_scale: swap_data.need_scale,
        },
    })
}

fn convert_token_output(
    output: Option<crate::proto::pendle_adaptor_v1::TokenOutput>,
) -> Result<pendle_adaptor_v1::TokenOutput, Error> {
    let output = output.ok_or_else(|| sp_call_error("output cannot be empty".to_string()))?;
    let swap_data = output
        .swap_data
        .ok_or_else(|| sp_call_error("swap_data cannot be empty".to_string()))?;

    Ok(pendle_adaptor_v1::TokenOutput {
        token_out: sp_call_parse_address(output.token_out)?,
        min_token_out: string_to_u256(output.min_token_out)?,
        token_redeem_sy: sp_call_parse_address(output.token_redeem_sy)?,
        pendle_swap: sp_call_parse_address(output.pendle_swap)?,
        swap_data: pendle_adaptor_v1::SwapData {
            swap_type: swap_data.swap_type as u8,
            ext_router: sp_call_parse_address(swap_data.ext_router)?,
            ext_calldata: hex::decode(swap_data.ext_calldata)?.into(),
            need_scale: swap_data.need_scale,
        },
    })
}
