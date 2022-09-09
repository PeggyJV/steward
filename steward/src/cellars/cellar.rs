//! Handlers for the Cellar.sol vault interface contract functions
//!
//! To learn more see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol
use ethers::{abi::AbiEncode, contract::EthCall};
use steward_abi::cellar::{AddPositionCall, CellarCalls, PopPositionCall, PushPositionCall, RemovePositionCall, ReplacePositionCall, SetHoldingPositionCall, RebalanceCall, SetStrategistPayoutAddressCall, SetWithdrawTypeCall, SwapPositionsCall};
use steward_proto::steward::{cellar::Function, Exchange, SwapParams};


use crate::{error::Error, utils::{sp_call_parse_address, string_to_u256, sp_call_error}};

use super::log_cellar_call;

const CELLAR_NAME: &str = "Cellar";

pub fn get_encoded_call(function: Function, cellar_id: String) -> Result<Vec<u8>, Error> {
    match function {
        Function::AddPosition(params) => {
            log_cellar_call(CELLAR_NAME, &AddPositionCall::function_name(), &cellar_id);
            let call = AddPositionCall {
                index: params.index.into(),
                position: sp_call_parse_address(params.position)?,
            };

            Ok(CellarCalls::AddPosition(call).encode())
        },
        Function::PopPosition(_) => {
            log_cellar_call(CELLAR_NAME, &PopPositionCall::function_name(), &cellar_id);
            let call = PopPositionCall {};

            Ok(CellarCalls::PopPosition(call).encode())
        },
        Function::PushPosition(params) => {
            log_cellar_call(CELLAR_NAME, &PushPositionCall::function_name(), &cellar_id);
            let call = PushPositionCall {
                position: sp_call_parse_address(params.position)?,
            };

            Ok(CellarCalls::PushPosition(call).encode())
        },
        Function::RemovePosition(params) => {
            log_cellar_call(CELLAR_NAME, &RemovePositionCall::function_name(), &cellar_id);
            let call = RemovePositionCall {
                index: params.index.into(),
            };

            Ok(CellarCalls::RemovePosition(call).encode())
        },
        Function::ReplacePosition(params) => {
            log_cellar_call(CELLAR_NAME, &ReplacePositionCall::function_name(), &cellar_id);
            let call = ReplacePositionCall {
                index: params.index.into(),
                new_position: sp_call_parse_address(params.new_position)?,
            };

            Ok(CellarCalls::ReplacePosition(call).encode())
        },
        Function::SetHoldingPosition(params) => {
            log_cellar_call(CELLAR_NAME, &SetHoldingPositionCall::function_name(), &cellar_id);
            let call = SetHoldingPositionCall {
                new_holding_position: sp_call_parse_address(params.new_holding_position)?,
            };

            Ok(CellarCalls::SetHoldingPosition(call).encode())
        },
        Function::Rebalance(params) => {
            todo!();
        },
        Function::SetStrategistPayoutAddress(params) => {
            log_cellar_call(CELLAR_NAME, &SetStrategistPayoutAddressCall::function_name(), &cellar_id);
            let call = SetStrategistPayoutAddressCall {
                payout: sp_call_parse_address(params.payout)?,
            };

            Ok(CellarCalls::SetStrategistPayoutAddress(call).encode())},
        Function::SetWithdrawType(params) => {
            log_cellar_call(CELLAR_NAME, &SetWithdrawTypeCall::function_name(), &cellar_id);
            let call = SetWithdrawTypeCall {
                new_withdraw_type: params.new_withdraw_type as u8,
            };

            Ok(CellarCalls::SetWithdrawType(call).encode())
        },
        Function::SwapPositions(params) => {
            log_cellar_call(CELLAR_NAME, &SwapPositionsCall::function_name(), &cellar_id);
            let call = SwapPositionsCall {
                index_1: params.index_1.into(),
                index_2: params.index_2.into(),
            };

            Ok(CellarCalls::SwapPositions(call).encode())
        },
    }
}

fn encode_swap_params(exchange: &Exchange, params: &SwapParams) -> Result<Vec<u8>, Error> {
    match exchange {
        Exchange::Univ2 => todo!(),
        Exchange::Univ3 => todo!(),
        Exchange::Unspecified => return Err(sp_call_error("exchange cannot be unspecified".to_string())),
    }
}
