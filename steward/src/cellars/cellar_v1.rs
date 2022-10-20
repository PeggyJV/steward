//! Handlers for the Cellar.sol vault interface contract functions
//!
//! To learn more see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol
use abscissa_core::tracing::info;
use ethers::{
    abi::{self, AbiEncode, Token},
    contract::EthCall,
    types::Address,
};
use steward_abi::cellar::{
    AddPositionCall, CellarCalls, PushPositionCall, RebalanceCall, RemovePositionCall,
    SetDepositLimitCall, SetHoldingPositionCall, SetLiquidityLimitCall, SetRebalanceDeviationCall,
    SetShareLockPeriodCall, SetStrategistPayoutAddressCall, SetWithdrawTypeCall, SwapPositionsCall,
};
use steward_proto::steward::cellar_v1::{swap_params::Params::*, Function, SwapParams};

use crate::{
    error::{Error, ErrorKind},
    utils::{self, sp_call_error, sp_call_parse_address, string_to_u256},
};

use super::log_cellar_call;

const CELLAR_NAME: &str = "Cellar";

pub fn get_encoded_call(function: Function, cellar_id: String) -> Result<Vec<u8>, Error> {
    match function {
        Function::AddPosition(params) => {
            log_cellar_call(CELLAR_NAME, &AddPositionCall::function_name(), &cellar_id);
            let call = AddPositionCall {
                index: utils::string_to_u256(params.index)?,
                position: sp_call_parse_address(params.position)?,
            };

            Ok(CellarCalls::AddPosition(call).encode())
        }
        Function::PushPosition(params) => {
            log_cellar_call(CELLAR_NAME, &PushPositionCall::function_name(), &cellar_id);
            let call = PushPositionCall {
                position: sp_call_parse_address(params.position)?,
            };

            Ok(CellarCalls::PushPosition(call).encode())
        }
        Function::RemovePosition(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &RemovePositionCall::function_name(),
                &cellar_id,
            );
            let call = RemovePositionCall {
                index: utils::string_to_u256(params.index)?,
            };

            Ok(CellarCalls::RemovePosition(call).encode())
        }
        Function::SetHoldingPosition(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetHoldingPositionCall::function_name(),
                &cellar_id,
            );
            let call = SetHoldingPositionCall {
                new_holding_position: sp_call_parse_address(params.new_holding_position)?,
            };

            Ok(CellarCalls::SetHoldingPosition(call).encode())
        }
        Function::Rebalance(params) => {
            log_cellar_call(CELLAR_NAME, &RebalanceCall::function_name(), &cellar_id);
            let swap_params =
                encode_swap_params(params.params.ok_or_else(|| {
                    ErrorKind::SPCallError.context("swap params cannot be empty")
                })?)?;

            info!("encoded: {:?}", hex::encode(&swap_params));

            let call = RebalanceCall {
                from_position: sp_call_parse_address(params.from_position)?,
                to_position: sp_call_parse_address(params.to_position)?,
                assets_from: string_to_u256(params.assets_from)?,
                // to account for protobuf's requirement that an UNSPECIFIED enum variant be defined
                // as 0, we subtract 1 from the value
                exchange: (params.exchange - 1) as u8,
                params: swap_params.into(),
            };
            let call = CellarCalls::Rebalance(call).encode();
            info!("final call: {:?}", hex::encode(&call));
            Ok(call)
        }
        Function::SetStrategistPayoutAddress(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetStrategistPayoutAddressCall::function_name(),
                &cellar_id,
            );
            let call = SetStrategistPayoutAddressCall {
                payout: sp_call_parse_address(params.payout)?,
            };

            Ok(CellarCalls::SetStrategistPayoutAddress(call).encode())
        }
        Function::SetWithdrawType(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetWithdrawTypeCall::function_name(),
                &cellar_id,
            );
            let call = SetWithdrawTypeCall {
                // to account for protobuf's requirement that an UNSPECIFIED enum variant be defined
                // as 0, we subtract 1 from the value
                new_withdraw_type: (params.new_withdraw_type - 1) as u8,
            };

            Ok(CellarCalls::SetWithdrawType(call).encode())
        }
        Function::SwapPositions(params) => {
            log_cellar_call(CELLAR_NAME, &SwapPositionsCall::function_name(), &cellar_id);
            let call = SwapPositionsCall {
                index_1: utils::string_to_u256(params.index_1)?,
                index_2: utils::string_to_u256(params.index_2)?,
            };

            Ok(CellarCalls::SwapPositions(call).encode())
        }
        Function::SetDepositLimit(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetDepositLimitCall::function_name(),
                &cellar_id,
            );
            let call = SetDepositLimitCall {
                new_limit: utils::string_to_u256(params.new_limit)?,
            };

            Ok(CellarCalls::SetDepositLimit(call).encode())
        }
        Function::SetLiquidityLimit(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetLiquidityLimitCall::function_name(),
                &cellar_id,
            );
            let call = SetLiquidityLimitCall {
                new_limit: utils::string_to_u256(params.new_limit)?,
            };

            Ok(CellarCalls::SetLiquidityLimit(call).encode())
        }
        Function::SetShareLockPeriod(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetShareLockPeriodCall::function_name(),
                &cellar_id,
            );
            let call = SetShareLockPeriodCall {
                new_lock: utils::string_to_u256(params.new_lock)?,
            };

            Ok(CellarCalls::SetShareLockPeriod(call).encode())
        }
        Function::SetRebalanceDeviation(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetRebalanceDeviationCall::function_name(),
                &cellar_id,
            );
            let call = SetRebalanceDeviationCall {
                new_deviation: utils::string_to_u256(params.new_deviation)?,
            };

            Ok(CellarCalls::SetRebalanceDeviation(call).encode())
        }
    }
}

/// Encodes the swap params as ABI-encoded bytes to me passed as args to the underlying
/// swap function
fn encode_swap_params(params: SwapParams) -> Result<Vec<u8>, Error> {
    match params
        .params
        .ok_or_else(|| sp_call_error("swap params cannot be unspecified".to_string()))?
    {
        Univ2Params(p) => {
            let mut path = Vec::<Token>::new();

            for a in p.path {
                let address = a.parse::<Address>();
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
                let address = a.parse::<Address>();
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
