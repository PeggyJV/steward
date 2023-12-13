use ethers::{abi::AbiEncode, types::Bytes};

use crate::{
    abi::adaptors::uniswap_v3_adaptor_v2::UniswapV3AdaptorV2Calls as AbiUniswapV3AdaptorV2Calls,
    error::Error,
    proto::{uniswap_v3_adaptor_v2, UniswapV3AdaptorV2Calls},
    utils::{sp_call_error, sp_call_parse_address, string_to_u128, string_to_u256},
};

/// Encodes adaptor calls for UniswapV3Adaptor V1
pub fn uniswap_v3_adaptor_v2_calls(params: UniswapV3AdaptorV2Calls) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            uniswap_v3_adaptor_v2::Function::OpenPosition(p) => {
                let call = crate::abi::adaptors::uniswap_v3_adaptor_v2::OpenPositionCall {
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
                calls.push(
                    AbiUniswapV3AdaptorV2Calls::OpenPosition(call)
                        .encode()
                        .into(),
                );
            }
            uniswap_v3_adaptor_v2::Function::ClosePosition(p) => {
                let call = crate::abi::adaptors::uniswap_v3_adaptor_v2::ClosePositionCall {
                    token_id: string_to_u256(p.token_id)?,
                    min_0: string_to_u256(p.min_0)?,
                    min_1: string_to_u256(p.min_1)?,
                };
                calls.push(
                    AbiUniswapV3AdaptorV2Calls::ClosePosition(call)
                        .encode()
                        .into(),
                );
            }
            uniswap_v3_adaptor_v2::Function::AddToPosition(p) => {
                let call = crate::abi::adaptors::uniswap_v3_adaptor_v2::AddToPositionCall {
                    token_id: string_to_u256(p.token_id)?,
                    amount_0: string_to_u256(p.amount_0)?,
                    amount_1: string_to_u256(p.amount_1)?,
                    min_0: string_to_u256(p.min_0)?,
                    min_1: string_to_u256(p.min_1)?,
                };
                calls.push(
                    AbiUniswapV3AdaptorV2Calls::AddToPosition(call)
                        .encode()
                        .into(),
                );
            }
            uniswap_v3_adaptor_v2::Function::TakeFromPosition(p) => {
                let call = crate::abi::adaptors::uniswap_v3_adaptor_v2::TakeFromPositionCall {
                    token_id: string_to_u256(p.token_id)?,
                    liquidity: string_to_u128(p.liquidity)?.as_u128(),
                    min_0: string_to_u256(p.min_0)?,
                    min_1: string_to_u256(p.min_1)?,
                    take_fees: p.take_fees,
                };
                calls.push(
                    AbiUniswapV3AdaptorV2Calls::TakeFromPosition(call)
                        .encode()
                        .into(),
                );
            }
            uniswap_v3_adaptor_v2::Function::RevokeApproval(p) => {
                let call = crate::abi::adaptors::uniswap_v3_adaptor_v2::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AbiUniswapV3AdaptorV2Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            uniswap_v3_adaptor_v2::Function::CollectFees(p) => {
                let call = crate::abi::adaptors::uniswap_v3_adaptor_v2::CollectFeesCall {
                    token_id: string_to_u256(p.token_id)?,
                    amount_0: string_to_u128(p.amount_0)?.as_u128(),
                    amount_1: string_to_u128(p.amount_1)?.as_u128(),
                };
                calls.push(
                    AbiUniswapV3AdaptorV2Calls::CollectFees(call)
                        .encode()
                        .into(),
                )
            }
            uniswap_v3_adaptor_v2::Function::PurgeAllZeroLiquidityPositions(p) => {
                let call = crate::abi::adaptors::uniswap_v3_adaptor_v2::PurgeAllZeroLiquidityPositionsCall {
                    token_0: sp_call_parse_address(p.token_0)?,
                    token_1: sp_call_parse_address(p.token_1)?,
                };
                calls.push(
                    AbiUniswapV3AdaptorV2Calls::PurgeAllZeroLiquidityPositions(call)
                        .encode()
                        .into(),
                )
            }
            uniswap_v3_adaptor_v2::Function::PurgeSinglePosition(p) => {
                let call = crate::abi::adaptors::uniswap_v3_adaptor_v2::PurgeSinglePositionCall {
                    token_id: string_to_u256(p.token_id)?,
                };
                calls.push(
                    AbiUniswapV3AdaptorV2Calls::PurgeSinglePosition(call)
                        .encode()
                        .into(),
                )
            }
            uniswap_v3_adaptor_v2::Function::RemoveUnownedPositionFromTracker(p) => {
                let call =
                    crate::abi::adaptors::uniswap_v3_adaptor_v2::RemoveUnOwnedPositionFromTrackerCall {
                        token_id: string_to_u256(p.token_id)?,
                        token_0: sp_call_parse_address(p.token_0)?,
                        token_1: sp_call_parse_address(p.token_1)?,
                    };
                calls.push(
                    AbiUniswapV3AdaptorV2Calls::RemoveUnOwnedPositionFromTracker(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
