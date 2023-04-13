use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::swap_with_uniswap_adaptor_v1::SwapWithUniswapAdaptorV1Calls as AbiSwapWithUniswapAdaptorV1Calls;
use steward_proto::steward::swap_with_uniswap_adaptor_v1;

use crate::{
    error::Error,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn swap_with_uniswap_adaptor_v1_calls(
    params: steward_proto::steward::SwapWithUniswapAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            swap_with_uniswap_adaptor_v1::Function::SwapWithUniV2(p) => {
                let call = steward_abi::swap_with_uniswap_adaptor_v1::SwapWithUniV2Call {
                    path: p
                        .path
                        .into_iter()
                        .map(sp_call_parse_address)
                        .collect::<Result<Vec<_>, _>>()?,
                    amount: string_to_u256(p.amount)?,
                    amount_out_min: string_to_u256(p.amount_out_min)?,
                };
                calls.push(
                    AbiSwapWithUniswapAdaptorV1Calls::SwapWithUniV2(call)
                        .encode()
                        .into(),
                )
            }
            swap_with_uniswap_adaptor_v1::Function::SwapWithUniV3(p) => {
                let call = steward_abi::swap_with_uniswap_adaptor_v1::SwapWithUniV3Call {
                    path: p
                        .path
                        .into_iter()
                        .map(sp_call_parse_address)
                        .collect::<Result<Vec<_>, _>>()?,
                    pool_fees: p.pool_fees,
                    amount: string_to_u256(p.amount)?,
                    amount_out_min: string_to_u256(p.amount_out_min)?,
                };
                calls.push(
                    AbiSwapWithUniswapAdaptorV1Calls::SwapWithUniV3(call)
                        .encode()
                        .into(),
                )
            }
            swap_with_uniswap_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::swap_with_uniswap_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AbiSwapWithUniswapAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
