//! Handlers for V2 of the Cellar.sol contract functions
//!
//! To learn more see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol
use abscissa_core::tracing::debug;
use ethers::{abi::AbiEncode, contract::EthCall, types::Bytes};
use steward_abi::cellar_v2::AdaptorCall as AbiAdaptorCall;
use steward_abi::{
    aave_a_token_adaptor::AaveATokenAdaptorCalls,
    aave_debt_token_adaptor::AaveDebtTokenAdaptorCalls, cellar_v2::*,
    compound_c_token_adaptor::CompoundCTokenAdaptorCalls,
    uniswap_v3_adaptor::UniswapV3AdaptorCalls, vesting_simple_adaptor::VestingSimpleAdaptorCalls,
};
use steward_proto::steward::{
    aave_a_token_adaptor, aave_debt_token_adaptor,
    cellar_v2::{adaptor_call::CallData, AdaptorCall, Function as StrategyFunction},
    cellar_v2_governance::Function as GovernanceFunction,
    compound_c_token_adaptor, uniswap_v3_adaptor, vesting_simple_adaptor,
};
use GovernanceFunction::*;
use StrategyFunction::*;

use crate::utils::{encode_oracle_swap_params, encode_swap_params};
use crate::{
    error::{Error, ErrorKind},
    utils::{
        convert_exchange, sp_call_error, sp_call_parse_address, string_to_u128, string_to_u256,
    },
};

use super::{log_cellar_call, log_governance_cellar_call};

const CELLAR_NAME: &str = "CellarV2";

// adaptors and positions associated with the deprecated UniV3 adaptor are blocked
// addresses treated as lowercase without 0x prefix to ensure valid comparisons with arbitrary input
const BLOCKED_ADAPTORS: [&str; 1] = ["7c4262f83e6775d6ff6fe8d9ab268611ed9d13ee"];
const BLOCKED_POSITIONS: [u32; 2] = [4, 5];

// since a string prefixed with or without 0x is parsable, ensure the string comparison is valid
pub fn normalize_address(address: String) -> String {
    let lowercase_address = address.to_lowercase();
    return address
        .to_lowercase()
        .strip_prefix("0x")
        .unwrap_or(&lowercase_address)
        .to_string();
}

pub fn get_encoded_call(function: StrategyFunction, cellar_id: String) -> Result<Vec<u8>, Error> {
    get_call(function, cellar_id).map(|call| call.encode())
}

pub fn get_call(function: StrategyFunction, cellar_id: String) -> Result<CellarV2Calls, Error> {
    match function {
        AddPosition(params) => {
            if BLOCKED_POSITIONS.contains(&params.position_id) {
                return Err(ErrorKind::SPCallError
                    .context(format!("position is blocked: {}", params.position_id))
                    .into());
            }

            log_cellar_call(CELLAR_NAME, &AddPositionCall::function_name(), &cellar_id);

            let call = AddPositionCall {
                index: params.index,
                position_id: params.position_id,
                configuration_data: params.configuration_data.into(),
                in_debt_array: params.in_debt_array,
            };

            Ok(CellarV2Calls::AddPosition(call))
        }
        CallOnAdaptor(params) => {
            for adaptor_call in params.data.clone() {
                let adaptor_address = normalize_address(adaptor_call.adaptor.clone());
                if BLOCKED_ADAPTORS.contains(&adaptor_address.as_str()) {
                    return Err(ErrorKind::SPCallError
                        .context(format!("adaptor is blocked: {}", adaptor_call.adaptor))
                        .into());
                }
            }

            log_cellar_call(CELLAR_NAME, &CallOnAdaptorCall::function_name(), &cellar_id);
            let call = CallOnAdaptorCall {
                data: get_encoded_adaptor_call(params.data)?,
            };

            Ok(CellarV2Calls::CallOnAdaptor(call))
        }
        RemovePosition(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &RemovePositionCall::function_name(),
                &cellar_id,
            );
            let call = RemovePositionCall {
                index: params.index,
                in_debt_array: params.in_debt_array,
            };

            Ok(CellarV2Calls::RemovePosition(call))
        }
        SetHoldingPosition(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetHoldingPositionCall::function_name(),
                &cellar_id,
            );
            let call = SetHoldingPositionCall {
                position_id: params.position_id,
            };

            Ok(CellarV2Calls::SetHoldingPosition(call))
        }
        SetStrategistPayoutAddress(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetStrategistPayoutAddressCall::function_name(),
                &cellar_id,
            );
            let call = SetStrategistPayoutAddressCall {
                payout: sp_call_parse_address(params.payout)?,
            };

            Ok(CellarV2Calls::SetStrategistPayoutAddress(call))
        }
        SwapPositions(params) => {
            log_cellar_call(CELLAR_NAME, &SwapPositionsCall::function_name(), &cellar_id);
            let call = SwapPositionsCall {
                index_1: params.index_1,
                index_2: params.index_2,
                in_debt_array: params.in_debt_array,
            };

            Ok(CellarV2Calls::SwapPositions(call))
        }
        SetShareLockPeriod(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetShareLockPeriodCall::function_name(),
                &cellar_id,
            );
            let call = SetShareLockPeriodCall {
                new_lock: string_to_u256(params.new_lock)?,
            };

            Ok(CellarV2Calls::SetShareLockPeriod(call))
        }
        // This will ultimately need to be a governance function, but for Seven Sea's live testing we are keeping
        // it here until they get a feel for what an appropriate value is.
        SetRebalanceDeviation(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetRebalanceDeviationCall::function_name(),
                &cellar_id,
            );
            let call = SetRebalanceDeviationCall {
                new_deviation: string_to_u256(params.new_deviation)?,
            };

            Ok(CellarV2Calls::SetRebalanceDeviation(call))
        }
    }
}

pub fn get_encoded_governance_call(
    function: GovernanceFunction,
    cellar_id: &str,
    proposal_id: u64,
) -> Result<Vec<u8>, Error> {
    match function {
        InitiateShutdown(_) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &InitiateShutdownCall::function_name(),
                cellar_id,
            );
            let call = InitiateShutdownCall {};
            Ok(CellarV2Calls::InitiateShutdown(call).encode())
        }
        LiftShutdown(_) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &LiftShutdownCall::function_name(),
                cellar_id,
            );
            let call = LiftShutdownCall {};
            Ok(CellarV2Calls::LiftShutdown(call).encode())
        }
        SetPlatformFee(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &SetPlatformFeeCall::function_name(),
                cellar_id,
            );
            let call = SetPlatformFeeCall {
                new_platform_fee: params.amount,
            };
            Ok(CellarV2Calls::SetPlatformFee(call).encode())
        }
        SetStrategistPlatformCut(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &SetStrategistPlatformCutCall::function_name(),
                cellar_id,
            );
            let call = SetStrategistPlatformCutCall { cut: params.amount };
            Ok(CellarV2Calls::SetStrategistPlatformCut(call).encode())
        }
        SetupAdaptor(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &SetupAdaptorCall::function_name(),
                cellar_id,
            );
            let call = SetupAdaptorCall {
                adaptor: sp_call_parse_address(params.adaptor)?,
            };

            Ok(CellarV2Calls::SetupAdaptor(call).encode())
        }
    }
}

fn get_encoded_adaptor_call(data: Vec<AdaptorCall>) -> Result<Vec<AbiAdaptorCall>, Error> {
    let mut result: Vec<AbiAdaptorCall> = Vec::new();
    for d in data {
        debug!("adaptor call to {}", d.adaptor);
        let mut calls: Vec<Bytes> = Vec::new();
        let call_data = d
            .call_data
            .ok_or_else(|| sp_call_error("call data is empty".to_string()))?;

        match call_data {
            CallData::UniswapV3Calls(params) => {
                for c in params.calls {
                    let function = c
                        .function
                        .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

                    match function {
                        uniswap_v3_adaptor::Function::OpenPosition(p) => {
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
                        uniswap_v3_adaptor::Function::ClosePosition(p) => {
                            let call = steward_abi::uniswap_v3_adaptor::ClosePositionCall {
                                token_id: string_to_u256(p.token_id)?,
                                min_0: string_to_u256(p.min_0)?,
                                min_1: string_to_u256(p.min_1)?,
                            };
                            calls.push(UniswapV3AdaptorCalls::ClosePosition(call).encode().into());
                        }
                        uniswap_v3_adaptor::Function::AddToPosition(p) => {
                            let call = steward_abi::uniswap_v3_adaptor::AddToPositionCall {
                                token_id: string_to_u256(p.token_id)?,
                                amount_0: string_to_u256(p.amount_0)?,
                                amount_1: string_to_u256(p.amount_1)?,
                                min_0: string_to_u256(p.min_0)?,
                                min_1: string_to_u256(p.min_1)?,
                            };
                            calls.push(UniswapV3AdaptorCalls::AddToPosition(call).encode().into());
                        }
                        uniswap_v3_adaptor::Function::TakeFromPosition(p) => {
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
                        uniswap_v3_adaptor::Function::Swap(p) => {
                            let swap_params = encode_swap_params(p.params.ok_or_else(|| {
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
                        uniswap_v3_adaptor::Function::OracleSwap(p) => {
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
                        uniswap_v3_adaptor::Function::RevokeApproval(p) => {
                            let call = steward_abi::uniswap_v3_adaptor::RevokeApprovalCall {
                                asset: sp_call_parse_address(p.asset)?,
                                spender: sp_call_parse_address(p.spender)?,
                            };
                            calls.push(UniswapV3AdaptorCalls::RevokeApproval(call).encode().into())
                        }
                        uniswap_v3_adaptor::Function::CollectFees(p) => {
                            let call = steward_abi::uniswap_v3_adaptor::CollectFeesCall {
                                token_id: string_to_u256(p.token_id)?,
                                amount_0: string_to_u128(p.amount_0)?.as_u128(),
                                amount_1: string_to_u128(p.amount_1)?.as_u128(),
                            };
                            calls.push(UniswapV3AdaptorCalls::CollectFees(call).encode().into())
                        }
                        uniswap_v3_adaptor::Function::PurgeAllZeroLiquidityPositions(p) => {
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
                        uniswap_v3_adaptor::Function::PurgeSinglePosition(p) => {
                            let call = steward_abi::uniswap_v3_adaptor::PurgeSinglePositionCall {
                                token_id: string_to_u256(p.token_id)?,
                            };
                            calls.push(
                                UniswapV3AdaptorCalls::PurgeSinglePosition(call)
                                    .encode()
                                    .into(),
                            )
                        }
                        uniswap_v3_adaptor::Function::RemoveUnownedPositionFromTracker(p) => {
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
            }
            CallData::AaveATokenCalls(params) => {
                for c in params.calls {
                    let function = c
                        .function
                        .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

                    match function {
                        aave_a_token_adaptor::Function::DepositToAave(p) => {
                            let call = steward_abi::aave_a_token_adaptor::DepositToAaveCall {
                                token_to_deposit: sp_call_parse_address(p.token)?,
                                amount_to_deposit: string_to_u256(p.amount)?,
                            };
                            calls.push(AaveATokenAdaptorCalls::DepositToAave(call).encode().into())
                        }
                        aave_a_token_adaptor::Function::WithdrawFromAave(p) => {
                            let call = steward_abi::aave_a_token_adaptor::WithdrawFromAaveCall {
                                token_to_withdraw: sp_call_parse_address(p.token)?,
                                amount_to_withdraw: string_to_u256(p.amount)?,
                            };
                            calls.push(
                                AaveATokenAdaptorCalls::WithdrawFromAave(call)
                                    .encode()
                                    .into(),
                            )
                        }
                        aave_a_token_adaptor::Function::Swap(p) => {
                            let swap_params = encode_swap_params(p.params.ok_or_else(|| {
                                sp_call_error("swap params cannot be empty".to_string())
                            })?)?;

                            debug!("encoded: {:?}", hex::encode(&swap_params));
                            let call = steward_abi::aave_a_token_adaptor::SwapCall {
                                asset_in: sp_call_parse_address(p.asset_in)?,
                                asset_out: sp_call_parse_address(p.asset_out)?,
                                amount_in: string_to_u256(p.amount_in)?,
                                exchange: convert_exchange(p.exchange),
                                params: swap_params.into(),
                            };
                            calls.push(AaveATokenAdaptorCalls::Swap(call).encode().into())
                        }
                        aave_a_token_adaptor::Function::OracleSwap(p) => {
                            let oracle_swap_params =
                                encode_oracle_swap_params(p.params.ok_or_else(|| {
                                    sp_call_error("swap params cannot be empty".to_string())
                                })?)?;

                            debug!("encoded: {:?}", hex::encode(&oracle_swap_params));
                            let call = steward_abi::aave_a_token_adaptor::OracleSwapCall {
                                asset_in: sp_call_parse_address(p.asset_in)?,
                                asset_out: sp_call_parse_address(p.asset_out)?,
                                amount_in: string_to_u256(p.amount_in)?,
                                exchange: convert_exchange(p.exchange),
                                params: oracle_swap_params.into(),
                                slippage: p.slippage,
                            };
                            calls.push(AaveATokenAdaptorCalls::OracleSwap(call).encode().into())
                        }
                        aave_a_token_adaptor::Function::RevokeApproval(p) => {
                            let call = steward_abi::aave_a_token_adaptor::RevokeApprovalCall {
                                asset: sp_call_parse_address(p.asset)?,
                                spender: sp_call_parse_address(p.spender)?,
                            };
                            calls.push(AaveATokenAdaptorCalls::RevokeApproval(call).encode().into())
                        }
                    }
                }
            }
            CallData::AaveDebtTokenCalls(params) => {
                for c in params.calls {
                    let function = c
                        .function
                        .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

                    match function {
                        aave_debt_token_adaptor::Function::BorrowFromAave(p) => {
                            let call = steward_abi::aave_debt_token_adaptor::BorrowFromAaveCall {
                                debt_token_to_borrow: sp_call_parse_address(p.token)?,
                                amount_to_borrow: string_to_u256(p.amount)?,
                            };
                            calls.push(
                                AaveDebtTokenAdaptorCalls::BorrowFromAave(call)
                                    .encode()
                                    .into(),
                            )
                        }
                        aave_debt_token_adaptor::Function::RepayAaveDebt(p) => {
                            let call = steward_abi::aave_debt_token_adaptor::RepayAaveDebtCall {
                                token_to_repay: sp_call_parse_address(p.token)?,
                                amount_to_repay: string_to_u256(p.amount)?,
                            };
                            calls.push(
                                AaveDebtTokenAdaptorCalls::RepayAaveDebt(call)
                                    .encode()
                                    .into(),
                            )
                        }
                        aave_debt_token_adaptor::Function::SwapAndRepay(p) => {
                            let swap_params = encode_swap_params(p.params.ok_or_else(|| {
                                sp_call_error("swap params cannot be empty".to_string())
                            })?)?;

                            debug!("encoded: {:?}", hex::encode(&swap_params));
                            let call = steward_abi::aave_debt_token_adaptor::SwapAndRepayCall {
                                token_in: sp_call_parse_address(p.token_in)?,
                                token_to_repay: sp_call_parse_address(p.token_to_repay)?,
                                amount_in: string_to_u256(p.amount_in)?,
                                exchange: convert_exchange(p.exchange),
                                params: swap_params.into(),
                            };
                            calls.push(
                                AaveDebtTokenAdaptorCalls::SwapAndRepay(call)
                                    .encode()
                                    .into(),
                            )
                        }
                        aave_debt_token_adaptor::Function::Swap(p) => {
                            let swap_params = encode_swap_params(p.params.ok_or_else(|| {
                                sp_call_error("swap params cannot be empty".to_string())
                            })?)?;

                            debug!("encoded: {:?}", hex::encode(&swap_params));
                            let call = steward_abi::aave_debt_token_adaptor::SwapCall {
                                asset_in: sp_call_parse_address(p.asset_in)?,
                                asset_out: sp_call_parse_address(p.asset_out)?,
                                amount_in: string_to_u256(p.amount_in)?,
                                exchange: convert_exchange(p.exchange),
                                params: swap_params.into(),
                            };
                            calls.push(AaveDebtTokenAdaptorCalls::Swap(call).encode().into())
                        }
                        aave_debt_token_adaptor::Function::OracleSwap(p) => {
                            let oracle_swap_params =
                                encode_oracle_swap_params(p.params.ok_or_else(|| {
                                    sp_call_error("swap params cannot be empty".to_string())
                                })?)?;

                            debug!("encoded: {:?}", hex::encode(&oracle_swap_params));
                            let call = steward_abi::aave_debt_token_adaptor::OracleSwapCall {
                                asset_in: sp_call_parse_address(p.asset_in)?,
                                asset_out: sp_call_parse_address(p.asset_out)?,
                                amount_in: string_to_u256(p.amount_in)?,
                                exchange: convert_exchange(p.exchange),
                                params: oracle_swap_params.into(),
                                slippage: p.slippage,
                            };
                            calls.push(AaveDebtTokenAdaptorCalls::OracleSwap(call).encode().into())
                        }
                        aave_debt_token_adaptor::Function::RevokeApproval(p) => {
                            let call = steward_abi::aave_debt_token_adaptor::RevokeApprovalCall {
                                asset: sp_call_parse_address(p.asset)?,
                                spender: sp_call_parse_address(p.spender)?,
                            };
                            calls.push(
                                AaveDebtTokenAdaptorCalls::RevokeApproval(call)
                                    .encode()
                                    .into(),
                            )
                        }
                    }
                }
            }
            CallData::CompoundCTokenCalls(params) => {
                for c in params.calls {
                    let function = c
                        .function
                        .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

                    match function {
                        compound_c_token_adaptor::Function::DepositToCompound(p) => {
                            let call =
                                steward_abi::compound_c_token_adaptor::DepositToCompoundCall {
                                    market: sp_call_parse_address(p.market)?,
                                    amount_to_deposit: string_to_u256(p.amount_to_deposit)?,
                                };
                            calls.push(
                                CompoundCTokenAdaptorCalls::DepositToCompound(call)
                                    .encode()
                                    .into(),
                            )
                        }
                        compound_c_token_adaptor::Function::WithdrawFromCompound(p) => {
                            let call =
                                steward_abi::compound_c_token_adaptor::WithdrawFromCompoundCall {
                                    market: sp_call_parse_address(p.market)?,
                                    amount_to_withdraw: string_to_u256(p.amount_to_withdraw)?,
                                };
                            calls.push(
                                CompoundCTokenAdaptorCalls::WithdrawFromCompound(call)
                                    .encode()
                                    .into(),
                            )
                        }
                        compound_c_token_adaptor::Function::ClaimComp(_) => {
                            let call = steward_abi::compound_c_token_adaptor::ClaimCompCall {};
                            calls.push(CompoundCTokenAdaptorCalls::ClaimComp(call).encode().into())
                        }
                        compound_c_token_adaptor::Function::ClaimCompAndSwap(p) => {
                            let oracle_swap_params =
                                encode_oracle_swap_params(p.params.ok_or_else(|| {
                                    sp_call_error("swap params cannot be empty".to_string())
                                })?)?;

                            debug!("encoded: {:?}", hex::encode(&oracle_swap_params));
                            let call =
                                steward_abi::compound_c_token_adaptor::ClaimCompAndSwapCall {
                                    asset_out: sp_call_parse_address(p.asset_out)?,
                                    exchange: convert_exchange(p.exchange),
                                    params: oracle_swap_params.into(),
                                    slippage: p.slippage,
                                };
                            calls.push(
                                CompoundCTokenAdaptorCalls::ClaimCompAndSwap(call)
                                    .encode()
                                    .into(),
                            )
                        }
                        compound_c_token_adaptor::Function::Swap(p) => {
                            let swap_params = encode_swap_params(p.params.ok_or_else(|| {
                                sp_call_error("swap params cannot be empty".to_string())
                            })?)?;

                            debug!("encoded: {:?}", hex::encode(&swap_params));
                            let call = steward_abi::compound_c_token_adaptor::SwapCall {
                                asset_in: sp_call_parse_address(p.asset_in)?,
                                asset_out: sp_call_parse_address(p.asset_out)?,
                                amount_in: string_to_u256(p.amount_in)?,
                                exchange: convert_exchange(p.exchange),
                                params: swap_params.into(),
                            };
                            calls.push(CompoundCTokenAdaptorCalls::Swap(call).encode().into())
                        }
                        compound_c_token_adaptor::Function::OracleSwap(p) => {
                            let oracle_swap_params =
                                encode_oracle_swap_params(p.params.ok_or_else(|| {
                                    sp_call_error("swap params cannot be empty".to_string())
                                })?)?;

                            debug!("encoded: {:?}", hex::encode(&oracle_swap_params));
                            let call = steward_abi::compound_c_token_adaptor::OracleSwapCall {
                                asset_in: sp_call_parse_address(p.asset_in)?,
                                asset_out: sp_call_parse_address(p.asset_out)?,
                                amount_in: string_to_u256(p.amount_in)?,
                                exchange: convert_exchange(p.exchange),
                                params: oracle_swap_params.into(),
                                slippage: p.slippage,
                            };
                            calls.push(CompoundCTokenAdaptorCalls::OracleSwap(call).encode().into())
                        }
                        compound_c_token_adaptor::Function::RevokeApproval(p) => {
                            let call = steward_abi::compound_c_token_adaptor::RevokeApprovalCall {
                                asset: sp_call_parse_address(p.asset)?,
                                spender: sp_call_parse_address(p.spender)?,
                            };
                            calls.push(
                                CompoundCTokenAdaptorCalls::RevokeApproval(call)
                                    .encode()
                                    .into(),
                            )
                        }
                    }
                }
            }
            CallData::VestingSimpleCalls(params) => {
                for c in params.calls {
                    let function = c
                        .function
                        .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

                    match function {
                        vesting_simple_adaptor::Function::DepositToVesting(p) => {
                            let call = steward_abi::vesting_simple_adaptor::DepositToVestingCall {
                                vesting_contract: sp_call_parse_address(p.vesting_contract)?,
                                amount_to_deposit: string_to_u256(p.amount)?,
                            };
                            calls.push(
                                VestingSimpleAdaptorCalls::DepositToVesting(call)
                                    .encode()
                                    .into(),
                            )
                        }
                        vesting_simple_adaptor::Function::WithdrawFromVesting(p) => {
                            let call =
                                steward_abi::vesting_simple_adaptor::WithdrawFromVestingCall {
                                    deposit_id: string_to_u256(p.deposit_id)?,
                                    vesting_contract: sp_call_parse_address(p.vesting_contract)?,
                                    amount_to_withdraw: string_to_u256(p.amount)?,
                                };
                            calls.push(
                                VestingSimpleAdaptorCalls::WithdrawFromVesting(call)
                                    .encode()
                                    .into(),
                            )
                        }
                        vesting_simple_adaptor::Function::WithdrawAnyFromVesting(p) => {
                            let call =
                                steward_abi::vesting_simple_adaptor::WithdrawAnyFromVestingCall {
                                    vesting_contract: sp_call_parse_address(p.vesting_contract)?,
                                    amount_to_withdraw: string_to_u256(p.amount)?,
                                };
                            calls.push(
                                VestingSimpleAdaptorCalls::WithdrawAnyFromVesting(call)
                                    .encode()
                                    .into(),
                            )
                        }
                        vesting_simple_adaptor::Function::WithdrawAllFromVesting(p) => {
                            let call =
                                steward_abi::vesting_simple_adaptor::WithdrawAllFromVestingCall {
                                    vesting_contract: sp_call_parse_address(p.vesting_contract)?,
                                };
                            calls.push(
                                VestingSimpleAdaptorCalls::WithdrawAllFromVesting(call)
                                    .encode()
                                    .into(),
                            )
                        }
                        vesting_simple_adaptor::Function::Swap(p) => {
                            let swap_params = encode_swap_params(p.params.ok_or_else(|| {
                                sp_call_error("swap params cannot be empty".to_string())
                            })?)?;

                            debug!("encoded: {:?}", hex::encode(&swap_params));
                            let call = steward_abi::vesting_simple_adaptor::SwapCall {
                                asset_in: sp_call_parse_address(p.asset_in)?,
                                asset_out: sp_call_parse_address(p.asset_out)?,
                                amount_in: string_to_u256(p.amount_in)?,
                                exchange: convert_exchange(p.exchange),
                                params: swap_params.into(),
                            };
                            calls.push(VestingSimpleAdaptorCalls::Swap(call).encode().into())
                        }
                        vesting_simple_adaptor::Function::OracleSwap(p) => {
                            let oracle_swap_params =
                                encode_oracle_swap_params(p.params.ok_or_else(|| {
                                    sp_call_error("swap params cannot be empty".to_string())
                                })?)?;

                            debug!("encoded: {:?}", hex::encode(&oracle_swap_params));
                            let call = steward_abi::vesting_simple_adaptor::OracleSwapCall {
                                asset_in: sp_call_parse_address(p.asset_in)?,
                                asset_out: sp_call_parse_address(p.asset_out)?,
                                amount_in: string_to_u256(p.amount_in)?,
                                exchange: convert_exchange(p.exchange),
                                params: oracle_swap_params.into(),
                                slippage: p.slippage,
                            };
                            calls.push(VestingSimpleAdaptorCalls::OracleSwap(call).encode().into())
                        }
                        vesting_simple_adaptor::Function::RevokeApproval(p) => {
                            let call = steward_abi::vesting_simple_adaptor::RevokeApprovalCall {
                                asset: sp_call_parse_address(p.asset)?,
                                spender: sp_call_parse_address(p.spender)?,
                            };
                            calls.push(
                                VestingSimpleAdaptorCalls::RevokeApproval(call)
                                    .encode()
                                    .into(),
                            )
                        }
                    }
                }
            }
        };

        result.push(AbiAdaptorCall {
            adaptor: sp_call_parse_address(d.adaptor)?,
            call_data: calls,
        })
    }

    Ok(result)
}

#[test]
fn test_address_normalization() {
    let blocked1 = String::from("0x7C4262f83e6775D6ff6fE8d9ab268611Ed9d13Ee");
    let blocked2 = String::from("0X7c4262f83e6775d6ff6fe8d9ab268611ed9d13ee");
    let blocked3 = String::from("7C4262f83e6775D6ff6fE8d9ab268611Ed9d13Ee");
    let blocked4 = String::from("7c4262f83e6775d6ff6fe8d9ab268611ed9d13ee");
    let nonblocked = String::from("0xDbd750F72a00d01f209FFc6C75e80301eFc789C1");

    assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked1.clone()).as_str()));
    assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked2).as_str()));
    assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked3).as_str()));
    assert!(BLOCKED_ADAPTORS.contains(&normalize_address(blocked4).as_str()));
    assert!(!BLOCKED_ADAPTORS.contains(&normalize_address(nonblocked.clone()).as_str()));
}
