//! Handlers for V2.2 of the Cellar.sol contract functions
//!
//! To learn more see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol
use abscissa_core::tracing::debug;
use ethers::{abi::AbiEncode, contract::EthCall, types::Bytes};
use steward_abi::cellar_v2_2::{self, AdaptorCall as AbiAdaptorCall, *};
use steward_proto::steward::{
    adaptor_call::CallData::*,
    cellar_v2_2::{function_call::Function, CallType, FunctionCall, *},
    AdaptorCall,
};

use crate::cellars::adaptors;
use crate::{
    error::{Error, ErrorKind},
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

use super::log_cellar_call;

const CELLAR_NAME: &str = "CellarV2.2";

// adaptors and positions associated with the deprecated UniV3 adaptor are blocked
// addresses treated as lowercase without 0x prefix to ensure valid comparisons with arbitrary input
const BLOCKED_ADAPTORS: [&str; 1] = ["7c4262f83e6775d6ff6fe8d9ab268611ed9d13ee"];
const BLOCKED_POSITIONS: [u32; 2] = [4, 5];

// address of the updated UniV3 adaptor
const ALLOWED_SETUP_ADAPTORS: [&str; 1] = ["dbd750f72a00d01f209ffc6c75e80301efc789c1"];

// since a string prefixed with or without 0x is parsable, ensure the string comparison is valid
pub fn normalize_address(address: String) -> String {
    let lowercase_address = address.to_lowercase();
    return address
        .to_lowercase()
        .strip_prefix("0x")
        .unwrap_or(&lowercase_address)
        .to_string();
}

/// Encodes a call to a CellarV2.2 contract
pub fn get_encoded_call(call_type: CallType, cellar_id: String) -> Result<Vec<u8>, Error> {
    match call_type {
        CallType::FunctionCall(f) => get_encoded_function(f, cellar_id),
        CallType::Multicall(m) => {
            let mut multicall = MulticallCall::default();
            m
                .function_calls
                .iter()
                .map(|f| get_encoded_function(f.clone(), cellar_id.clone()))
                .collect::<Result<Vec<Vec<u8>>, Error>>()?
                .iter()
                .for_each(|f| multicall.data.push(Bytes::from(f.clone())));

            Ok(multicall.encode().into())
        },
    }
}

pub fn get_encoded_function(call: FunctionCall, cellar_id: String) -> Result<Vec<u8>, Error> {
    let function = call
        .function
        .ok_or_else(|| sp_call_error("call data is empty".to_string()))?;
    match function {
        Function::AddPosition(params) => {
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

            Ok(CellarV2_2Calls::AddPosition(call).encode())
        }
        Function::CallOnAdaptor(params) => {
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

            Ok(CellarV2_2Calls::CallOnAdaptor(call).encode())
        }
        Function::RemovePosition(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &RemovePositionCall::function_name(),
                &cellar_id,
            );
            let call = RemovePositionCall {
                index: params.index,
                in_debt_array: params.in_debt_array,
            };

            Ok(CellarV2_2Calls::RemovePosition(call).encode())
        }
        Function::SetHoldingPosition(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetHoldingPositionCall::function_name(),
                &cellar_id,
            );
            let call = SetHoldingPositionCall {
                position_id: params.position_id,
            };

            Ok(CellarV2_2Calls::SetHoldingPosition(call).encode())
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

            Ok(CellarV2_2Calls::SetStrategistPayoutAddress(call).encode())
        }
        Function::SwapPositions(params) => {
            log_cellar_call(CELLAR_NAME, &SwapPositionsCall::function_name(), &cellar_id);
            let call = SwapPositionsCall {
                index_1: params.index_1,
                index_2: params.index_2,
                in_debt_array: params.in_debt_array,
            };

            Ok(CellarV2_2Calls::SwapPositions(call).encode())
        }
        Function::SetShareLockPeriod(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetShareLockPeriodCall::function_name(),
                &cellar_id,
            );
            let call = SetShareLockPeriodCall {
                new_lock: string_to_u256(params.new_lock)?,
            };

            Ok(CellarV2_2Calls::SetShareLockPeriod(call).encode())
        }
        // This will ultimately need to be a governance function, but for Seven Sea's live testing we are keeping
        // it here until they get a feel for what an appropriate value is.
        Function::SetRebalanceDeviation(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetRebalanceDeviationCall::function_name(),
                &cellar_id,
            );
            let call = SetRebalanceDeviationCall {
                new_deviation: string_to_u256(params.new_deviation)?,
            };

            Ok(CellarV2_2Calls::SetRebalanceDeviation(call).encode())
        }
        // SetupAdaptor(params) => {
        //     let adaptor_address = normalize_address(params.adaptor.clone());
        //     if !ALLOWED_SETUP_ADAPTORS.contains(&adaptor_address.as_str()) {
        //         return Err(ErrorKind::SPCallError
        //             .context(format!("adaptor setup not allowed: {}", params.adaptor))
        //             .into());
        //     }

        //     log_cellar_call(CELLAR_NAME, &SetupAdaptorCall::function_name(), &cellar_id);
        //     let call = SetupAdaptorCall {
        //         adaptor: sp_call_parse_address(params.adaptor)?,
        //     };

        //     Ok(CellarV2_2Calls::SetupAdaptor(call).encode())
        // }
        Function::InitiateShutdown(_) => {
            log_cellar_call(
                CELLAR_NAME,
                &InitiateShutdownCall::function_name(),
                &cellar_id,
            );
            let call = InitiateShutdownCall {};

            Ok(CellarV2_2Calls::InitiateShutdown(call).encode().into())
        }
    }
}

/// Encodes calls to the Adaptor contracts
fn get_encoded_adaptor_call(data: Vec<AdaptorCall>) -> Result<Vec<AbiAdaptorCall>, Error> {
    let mut result: Vec<AbiAdaptorCall> = Vec::new();
    for d in data {
        debug!("adaptor call to {}", d.adaptor);
        let mut calls: Vec<Bytes> = Vec::new();
        let call_data = d
            .call_data
            .ok_or_else(|| sp_call_error("call data is empty".to_string()))?;

        match call_data {
            UniswapV3Calls(params) => {
                calls.extend(adaptors::uniswap_v3::uniswap_v3_adaptor_v1_call(params)?)
            }
            AaveATokenV1Calls(params) => {
                calls.extend(adaptors::aave_v2::aave_a_token_adaptor_v1_call(params)?)
            }
            AaveDebtTokenV1Calls(params) => {
                calls.extend(adaptors::aave_v2::aave_debt_token_adaptor_v1_call(params)?)
            }
            CompoundCTokenV1Calls(params) => {
                calls.extend(adaptors::compound::compound_c_token_v1_call(params)?)
            }
            AaveATokenV2Calls(params) => {
                calls.extend(adaptors::aave_v2::aave_a_token_adaptor_v2_call(params)?)
            }
            AaveDebtTokenV2Calls(params) => {
                calls.extend(adaptors::aave_v2::aave_debt_token_adaptor_v2_call(params)?)
            }
            AaveV3ATokenV1Calls(params) => {
                calls.extend(adaptors::aave_v3::aave_v3_a_token_adaptor_v1_call(params)?)
            }
            AaveV3DebtTokenV1Calls(params) => calls.extend(
                adaptors::aave_v3::aave_v3_debt_token_adaptor_v1_call(params)?,
            ),
            OneInchV1Calls(params) => {
                calls.extend(adaptors::oneinch::one_inch_adaptor_v1_call(params)?)
            }
            FeesAndReservesV1Calls(params) => calls
                .extend(adaptors::fees_and_reserves::fees_and_reserves_adaptor_v1_call(params)?),
            ZeroXV1Calls(params) => calls.extend(adaptors::zero_x::zero_x_adaptor_v1_call(params)?),
            SwapWithUniswapV1Calls(params) => calls.extend(
                adaptors::uniswap_v3::swap_with_uniswap_adaptor_v1_call(params)?,
            ),
            CompoundCTokenV2Calls(params) => {
                calls.extend(adaptors::compound::compound_c_token_v2_call(params)?)
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

    assert!(ALLOWED_SETUP_ADAPTORS.contains(&normalize_address(nonblocked).as_str()));
    assert!(!ALLOWED_SETUP_ADAPTORS.contains(&normalize_address(blocked1).as_str()));
}
