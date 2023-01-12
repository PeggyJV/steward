//! Handlers for V2 of the Cellar.sol contract functions
//!
//! To learn more see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol
use abscissa_core::tracing::{debug, info};
use ethers::{
    abi::{self, AbiEncode, Token},
    contract::EthCall,
    types::{Address as EthereumAddress, Bytes},
};
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
    compound_c_token_adaptor,
    oracle_swap_params::Params::{
        Univ2Params as UniV2OracleParams, Univ3Params as UniV3OracleParams,
    },
    swap_params::Params::*,
    uniswap_v3_adaptor, vesting_simple_adaptor, OracleSwapParams, SwapParams,
};
use GovernanceFunction::*;
use StrategyFunction::*;

use crate::{
    error::{Error, ErrorKind},
    utils::{sp_call_error, sp_call_parse_address, string_to_u128, string_to_u256},
};

use super::{log_cellar_call, log_governance_cellar_call};

const CELLAR_NAME: &str = "CellarV2";

pub fn get_encoded_call(function: StrategyFunction, cellar_id: String) -> Result<Vec<u8>, Error> {
    match function {
        AddPosition(params) => {
            log_cellar_call(CELLAR_NAME, &AddPositionCall::function_name(), &cellar_id);
            let call = AddPositionCall {
                index: params.index,
                position_id: params.position_id,
                configuration_data: params.configuration_data.into(),
                in_debt_array: params.in_debt_array,
            };

            Ok(CellarV2Calls::AddPosition(call).encode())
        }
        CallOnAdaptor(params) => {
            log_cellar_call(CELLAR_NAME, &CallOnAdaptorCall::function_name(), &cellar_id);
            let call = CallOnAdaptorCall {
                data: get_encoded_adaptor_call(params.data)?,
            };

            Ok(CellarV2Calls::CallOnAdaptor(call).encode())
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

            Ok(CellarV2Calls::RemovePosition(call).encode())
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

            Ok(CellarV2Calls::SetHoldingPosition(call).encode())
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

            Ok(CellarV2Calls::SetStrategistPayoutAddress(call).encode())
        }
        SetupAdaptor(params) => {
            log_cellar_call(CELLAR_NAME, &SetupAdaptorCall::function_name(), &cellar_id);
            let call = SetupAdaptorCall {
                adaptor: sp_call_parse_address(params.adaptor)?,
            };

            Ok(CellarV2Calls::SetupAdaptor(call).encode())
        }
        SwapPositions(params) => {
            log_cellar_call(CELLAR_NAME, &SwapPositionsCall::function_name(), &cellar_id);
            let call = SwapPositionsCall {
                index_1: params.index_1,
                index_2: params.index_2,
                in_debt_array: params.in_debt_array,
            };

            Ok(CellarV2Calls::SwapPositions(call).encode())
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

            Ok(CellarV2Calls::SetShareLockPeriod(call).encode())
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

            Ok(CellarV2Calls::SetRebalanceDeviation(call).encode())
        }
    }
}

/// Encodes the swap params as ABI-encoded bytes to be passed as args to the underlying
/// swap function
fn encode_swap_params(params: SwapParams) -> Result<Vec<u8>, Error> {
    match params
        .params
        .ok_or_else(|| sp_call_error("swap params cannot be unspecified".to_string()))?
    {
        Univ2Params(p) => {
            let mut path = Vec::<Token>::new();

            for a in p.path {
                let address = a.parse::<EthereumAddress>();
                if address.is_err() {
                    return Err(sp_call_error(format!(
                        "could not parse swap params path address: {}",
                        a
                    )));
                }

                path.push(Token::Address(address.unwrap()))
            }

            let path = Token::Array(path);
            let amount = Token::Uint(string_to_u256(p.amount)?);
            let amount_out_min = Token::Uint(string_to_u256(p.amount_out_min)?);
            Ok(abi::encode(&[path, amount, amount_out_min]))
        }
        Univ3Params(p) => {
            let mut path = Vec::<Token>::new();
            for a in p.path {
                let address = a.parse::<EthereumAddress>();
                if address.is_err() {
                    return Err(sp_call_error(format!(
                        "could not parse swap params path address: {}",
                        a
                    )));
                }

                path.push(Token::Address(address.unwrap()))
            }

            let path = Token::Array(path);

            let mut pool = Vec::<Token>::new();
            for f in p.pool_fees {
                pool.push(Token::Uint(f.into()))
            }

            let pool = Token::Array(pool);
            let amount = Token::Uint(string_to_u256(p.amount)?);
            let amount_out_min = Token::Uint(string_to_u256(p.amount_out_min)?);

            Ok(abi::encode(&[path, pool, amount, amount_out_min]))
        }
    }
}

/// Encodes the oracle swap params as ABI-encoded bytes to be passed as args to the underlying
/// oracle swap function
fn encode_oracle_swap_params(params: OracleSwapParams) -> Result<Vec<u8>, Error> {
    match params
        .params
        .ok_or_else(|| sp_call_error("swap params cannot be unspecified".to_string()))?
    {
        UniV2OracleParams(p) => {
            let mut path = Vec::<Token>::new();

            for a in p.path {
                let address = a.parse::<EthereumAddress>();
                if address.is_err() {
                    return Err(sp_call_error(format!(
                        "could not parse swap params path address: {}",
                        a
                    )));
                }

                path.push(Token::Address(address.unwrap()))
            }

            let path = Token::Array(path);
            Ok(abi::encode(&[path]))
        }
        UniV3OracleParams(p) => {
            let mut path = Vec::<Token>::new();
            for a in p.path {
                let address = a.parse::<EthereumAddress>();
                if address.is_err() {
                    return Err(sp_call_error(format!(
                        "could not parse swap params path address: {}",
                        a
                    )));
                }

                path.push(Token::Address(address.unwrap()))
            }

            let path = Token::Array(path);

            let mut pool = Vec::<Token>::new();
            for f in p.pool_fees {
                pool.push(Token::Uint(f.into()))
            }

            let pool = Token::Array(pool);

            Ok(abi::encode(&[path, pool]))
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
    }
}

fn get_encoded_adaptor_call(data: Vec<AdaptorCall>) -> Result<Vec<AbiAdaptorCall>, Error> {
    let mut result: Vec<AbiAdaptorCall> = Vec::new();
    for d in data {
        debug!("adaptor call to {}", d.adaptor);
        let mut calls: Vec<Bytes> = Vec::new();
        match d.call_data.unwrap() {
            CallData::UniswapV3Calls(params) => {
                for c in params.calls {
                    match c.function.expect("adaptor function was empty") {
                        uniswap_v3_adaptor::Function::OpenPosition(p) => {
                            let call = steward_abi::uniswap_v3_adaptor::OpenPositionCall {
                                token_0: sp_call_parse_address(p.token_0)?,
                                token_1: sp_call_parse_address(p.token_1)?,
                                pool_fee: p.pool_fee,
                                amount_0: string_to_u256(p.amount_0)?,
                                amount_1: string_to_u256(p.amount_1)?,
                                min_0: string_to_u256(p.min_0)?,
                                min_1: string_to_u256(p.min_1)?,
                                tick_lower: p.tick_lower as i32,
                                tick_upper: p.tick_upper as i32,
                            };
                            calls.push(UniswapV3AdaptorCalls::OpenPosition(call).encode().into());
                        }
                        uniswap_v3_adaptor::Function::ClosePosition(p) => {
                            let call = steward_abi::uniswap_v3_adaptor::ClosePositionCall {
                                position_id: string_to_u256(p.position_id)?,
                                min_0: string_to_u256(p.min_0)?,
                                min_1: string_to_u256(p.min_1)?,
                            };
                            calls.push(UniswapV3AdaptorCalls::ClosePosition(call).encode().into());
                        }
                        uniswap_v3_adaptor::Function::AddToPosition(p) => {
                            let call = steward_abi::uniswap_v3_adaptor::AddToPositionCall {
                                position_id: string_to_u256(p.position_id)?,
                                amount_0: string_to_u256(p.amount_0)?,
                                amount_1: string_to_u256(p.amount_1)?,
                                min_0: string_to_u256(p.min_0)?,
                                min_1: string_to_u256(p.min_1)?,
                            };
                            calls.push(UniswapV3AdaptorCalls::AddToPosition(call).encode().into());
                        }
                        uniswap_v3_adaptor::Function::TakeFromPosition(p) => {
                            let call = steward_abi::uniswap_v3_adaptor::TakeFromPositionCall {
                                position_id: string_to_u256(p.position_id)?,
                                liquidity: string_to_u128(p.liquidity)?.as_u128(),
                                min_0: string_to_u256(p.min_0)?,
                                min_1: string_to_u256(p.min_1)?,
                                collect_fees: p.collect_fees,
                            };
                            calls.push(
                                UniswapV3AdaptorCalls::TakeFromPosition(call)
                                    .encode()
                                    .into(),
                            );
                        }
                    }
                }
            }
            CallData::AaveATokenCalls(params) => {
                for c in params.calls {
                    match c.function.expect("adaptor function was empty") {
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
                    }
                }
            }
            CallData::AaveDebtTokenCalls(params) => {
                for c in params.calls {
                    match c.function.expect("adaptor function was empty") {
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
                                ErrorKind::SPCallError.context("swap params cannot be empty")
                            })?)?;

                            info!("encoded: {:?}", hex::encode(&swap_params));
                            let call = steward_abi::aave_debt_token_adaptor::SwapAndRepayCall {
                                token_in: sp_call_parse_address(p.token_in)?,
                                token_to_repay: sp_call_parse_address(p.token_to_repay)?,
                                amount_in: string_to_u256(p.amount_in)?,
                                // to account for protobuf's requirement that an UNSPECIFIED enum variant be defined
                                // as 0, we subtract 1 from the value
                                exchange: (p.exchange - 1) as u8,
                                params: swap_params.into(),
                            };
                            calls.push(
                                AaveDebtTokenAdaptorCalls::SwapAndRepay(call)
                                    .encode()
                                    .into(),
                            )
                        }
                    }
                }
            }
            CallData::CompoundCTokenCalls(params) => {
                for c in params.calls {
                    match c.function.expect("adaptor function was empty") {
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
                            let swap_params =
                                encode_oracle_swap_params(p.params.ok_or_else(|| {
                                    ErrorKind::SPCallError.context("swap params cannot be empty")
                                })?)?;

                            info!("encoded: {:?}", hex::encode(&swap_params));
                            let call =
                                steward_abi::compound_c_token_adaptor::ClaimCompAndSwapCall {
                                    asset_out: sp_call_parse_address(p.asset_out)?,
                                    // to account for protobuf's requirement that an UNSPECIFIED enum variant be defined
                                    // as 0, we subtract 1 from the value
                                    exchange: (p.exchange - 1) as u8,
                                    params: swap_params.into(),
                                    slippage: p.slippage,
                                };
                            calls.push(
                                CompoundCTokenAdaptorCalls::ClaimCompAndSwap(call)
                                    .encode()
                                    .into(),
                            )
                        }
                    }
                }
            }
            CallData::VestingSimpleCalls(params) => {
                for c in params.calls {
                    match c.function.expect("adaptor function was empty") {
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
