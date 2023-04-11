use abscissa_core::tracing::log::debug;
use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::{
    swap_with_uniswap_adaptor::SwapWithUniswapAdaptorCalls,
    uniswap_v3_adaptor::UniswapV3AdaptorCalls,
};
use steward_proto::steward::{
    swap_with_uniswap_adaptor_v1, uniswap_v3_adaptor_v1, UniswapV3AdaptorV1Calls,
};

use crate::{
    error::Error,
    utils::{
        convert_exchange, encode_oracle_swap_params, encode_swap_params, sp_call_error,
        sp_call_parse_address, string_to_u128, string_to_u256,
    },
};

/// Encodes adaptor calls for UniswapV3Adaptor V1
pub fn uniswap_v3_adaptor_v1_call(params: UniswapV3AdaptorV1Calls) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            uniswap_v3_adaptor_v1::Function::OpenPosition(p) => {
                let call = steward_abi::uniswap_v3_adaptor::OpenPositionCall {
                    token_0: sp_call_parse_address(p.token_0)?,
                    token_1: sp_call_parse_address(p.token_1)?,
                    pool_fee: p.pool_fee,
                    amount_0: string_to_u256(p.amount_0)?,
                    amount_1: string_to_u256(p.amount_1)?,
                    min_0: string_to_u256(p.min_0)?,
                    min_1: string_to_u256(p.min_1)?,
                    tick_lower: p.tick_lower,
                    tick_upper: p.tick_upper,
                };
                calls.push(UniswapV3AdaptorCalls::OpenPosition(call).encode().into());
            }
            uniswap_v3_adaptor_v1::Function::ClosePosition(p) => {
                let call = steward_abi::uniswap_v3_adaptor::ClosePositionCall {
                    token_id: string_to_u256(p.token_id)?,
                    min_0: string_to_u256(p.min_0)?,
                    min_1: string_to_u256(p.min_1)?,
                };
                calls.push(UniswapV3AdaptorCalls::ClosePosition(call).encode().into());
            }
            uniswap_v3_adaptor_v1::Function::AddToPosition(p) => {
                let call = steward_abi::uniswap_v3_adaptor::AddToPositionCall {
                    token_id: string_to_u256(p.token_id)?,
                    amount_0: string_to_u256(p.amount_0)?,
                    amount_1: string_to_u256(p.amount_1)?,
                    min_0: string_to_u256(p.min_0)?,
                    min_1: string_to_u256(p.min_1)?,
                };
                calls.push(UniswapV3AdaptorCalls::AddToPosition(call).encode().into());
            }
            uniswap_v3_adaptor_v1::Function::TakeFromPosition(p) => {
                let call = steward_abi::uniswap_v3_adaptor::TakeFromPositionCall {
                    token_id: string_to_u256(p.token_id)?,
                    liquidity: string_to_u128(p.liquidity)?.as_u128(),
                    min_0: string_to_u256(p.min_0)?,
                    min_1: string_to_u256(p.min_1)?,
                    take_fees: p.take_fees,
                };
                calls.push(
                    UniswapV3AdaptorCalls::TakeFromPosition(call)
                        .encode()
                        .into(),
                );
            }
            uniswap_v3_adaptor_v1::Function::Swap(p) => {
                let swap_params =
                    encode_swap_params(p.params.ok_or_else(|| {
                        sp_call_error("swap params cannot be empty".to_string())
                    })?)?;

                debug!("encoded: {:?}", hex::encode(&swap_params));
                let call = steward_abi::uniswap_v3_adaptor::SwapCall {
                    asset_in: sp_call_parse_address(p.asset_in)?,
                    asset_out: sp_call_parse_address(p.asset_out)?,
                    amount_in: string_to_u256(p.amount_in)?,
                    exchange: convert_exchange(p.exchange),
                    params: swap_params.into(),
                };
                calls.push(UniswapV3AdaptorCalls::Swap(call).encode().into())
            }
            uniswap_v3_adaptor_v1::Function::OracleSwap(p) => {
                let oracle_swap_params =
                    encode_oracle_swap_params(p.params.ok_or_else(|| {
                        sp_call_error("swap params cannot be empty".to_string())
                    })?)?;

                debug!("encoded: {:?}", hex::encode(&oracle_swap_params));
                let call = steward_abi::uniswap_v3_adaptor::OracleSwapCall {
                    asset_in: sp_call_parse_address(p.asset_in)?,
                    asset_out: sp_call_parse_address(p.asset_out)?,
                    amount_in: string_to_u256(p.amount_in)?,
                    exchange: convert_exchange(p.exchange),
                    params: oracle_swap_params.into(),
                    slippage: p.slippage,
                };
                calls.push(UniswapV3AdaptorCalls::OracleSwap(call).encode().into())
            }
            uniswap_v3_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::uniswap_v3_adaptor::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(UniswapV3AdaptorCalls::RevokeApproval(call).encode().into())
            }
            uniswap_v3_adaptor_v1::Function::CollectFees(p) => {
                let call = steward_abi::uniswap_v3_adaptor::CollectFeesCall {
                    token_id: string_to_u256(p.token_id)?,
                    amount_0: string_to_u128(p.amount_0)?.as_u128(),
                    amount_1: string_to_u128(p.amount_1)?.as_u128(),
                };
                calls.push(UniswapV3AdaptorCalls::CollectFees(call).encode().into())
            }
            uniswap_v3_adaptor_v1::Function::PurgeAllZeroLiquidityPositions(p) => {
                let call = steward_abi::uniswap_v3_adaptor::PurgeAllZeroLiquidityPositionsCall {
                    token_0: sp_call_parse_address(p.token_0)?,
                    token_1: sp_call_parse_address(p.token_1)?,
                };
                calls.push(
                    UniswapV3AdaptorCalls::PurgeAllZeroLiquidityPositions(call)
                        .encode()
                        .into(),
                )
            }
            uniswap_v3_adaptor_v1::Function::PurgeSinglePosition(p) => {
                let call = steward_abi::uniswap_v3_adaptor::PurgeSinglePositionCall {
                    token_id: string_to_u256(p.token_id)?,
                };
                calls.push(
                    UniswapV3AdaptorCalls::PurgeSinglePosition(call)
                        .encode()
                        .into(),
                )
            }
            uniswap_v3_adaptor_v1::Function::RemoveUnownedPositionFromTracker(p) => {
                let call = steward_abi::uniswap_v3_adaptor::RemoveUnOwnedPositionFromTrackerCall {
                    token_id: string_to_u256(p.token_id)?,
                    token_0: sp_call_parse_address(p.token_0)?,
                    token_1: sp_call_parse_address(p.token_1)?,
                };
                calls.push(
                    UniswapV3AdaptorCalls::RemoveUnOwnedPositionFromTracker(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}

pub(crate) fn swap_with_uniswap_adaptor_v1_call(
    params: steward_proto::steward::SwapWithUniswapAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            swap_with_uniswap_adaptor_v1::Function::SwapWithUniV2(p) => {
                let call = steward_abi::swap_with_uniswap_adaptor::SwapWithUniV2Call {
                    path: p
                        .path
                        .into_iter()
                        .map(sp_call_parse_address)
                        .collect::<Result<Vec<_>, _>>()?,
                    amount: string_to_u256(p.amount)?,
                    amount_out_min: string_to_u256(p.amount_out_min)?,
                };
                calls.push(
                    SwapWithUniswapAdaptorCalls::SwapWithUniV2(call)
                        .encode()
                        .into(),
                )
            }
            swap_with_uniswap_adaptor_v1::Function::SwapWithUniV3(p) => {
                let call = steward_abi::swap_with_uniswap_adaptor::SwapWithUniV3Call {
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
                    SwapWithUniswapAdaptorCalls::SwapWithUniV3(call)
                        .encode()
                        .into(),
                )
            }
            swap_with_uniswap_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::swap_with_uniswap_adaptor::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    SwapWithUniswapAdaptorCalls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
