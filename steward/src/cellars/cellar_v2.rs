//! Handlers for V2 of the Cellar.sol contract functions
//!
//! To learn more see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol
use abscissa_core::tracing::debug;
use ethers::{abi::AbiEncode, contract::EthCall, types::Bytes};
use steward_abi::cellar_v2::{AdaptorCall as AbiAdaptorCall, *};
use steward_proto::steward::{
    adaptor_call::CallData::*, cellar_v2::Function as StrategyFunction, AdaptorCall,
};
use StrategyFunction::*;

use crate::cellars::adaptors;
use crate::{
    error::{Error, ErrorKind},
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

use super::log_cellar_call;

const CELLAR_NAME: &str = "CellarV2";

// adaptors and positions associated with the deprecated UniV3 adaptor are blocked
// addresses treated as lowercase without 0x prefix to ensure valid comparisons with arbitrary input
const BLOCKED_ADAPTORS: [&str; 1] = ["7c4262f83e6775d6ff6fe8d9ab268611ed9d13ee"];
const BLOCKED_POSITIONS: [u32; 9] = [4, 5, 6, 7, 8, 9, 10, 11, 12];
const ALLOWED_SETUP_ADAPTORS: [&str; 9] = [
    // UniswapV3Adaptr V2
    "dbd750f72a00d01f209ffc6c75e80301efc789c1",
    // VestingSimpleAdaptor V2
    "508E6aE090eA92Cb90571e4269B799257CD78CA1",
    // OneInchAdaptor V1
    "B8952ce4010CFF3C74586d712a4402285A3a3AFb",
    // SwapWithUniswapAdaptor V1
    "d6BC6Df1ed43e3101bC27a4254593a06598a3fDD",
    // ZeroXAdaptor V1
    "1039a9b61DFF6A3fb8dbF4e924AA749E5cFE35ef",
    // AaveV3ATokenAdaptor V1
    "3184CBEa47eD519FA04A23c4207cD15b7545F1A6",
    // AaveATokenAdaptor V2
    "25570a77dCA06fda89C1ef41FAb6eE48a2377E81",
    // FeesAndReservesAdaptor V1
    "647d264d800A2461E594796af61a39b7735d8933",
    // CTokenAdaptor V2
    "9a384Df333588428843D128120Becd72434ec078",
];

// since a string prefixed with or without 0x is parsable, ensure the string comparison is valid
pub fn normalize_address(address: String) -> String {
    let lowercase_address = address.to_lowercase();
    return address
        .to_lowercase()
        .strip_prefix("0x")
        .unwrap_or(&lowercase_address)
        .to_string();
}

/// Encodes a call to a CellarV2 contract
pub fn get_encoded_call(function: StrategyFunction, cellar_id: String) -> Result<Vec<u8>, Error> {
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

            Ok(CellarV2Calls::AddPosition(call).encode())
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
        SetupAdaptor(params) => {
            let adaptor_address = normalize_address(params.adaptor.clone());
            if !ALLOWED_SETUP_ADAPTORS.contains(&adaptor_address.as_str()) {
                return Err(ErrorKind::SPCallError
                    .context(format!("adaptor setup not allowed: {}", params.adaptor))
                    .into());
            }

            log_cellar_call(CELLAR_NAME, &SetupAdaptorCall::function_name(), &cellar_id);
            let call = SetupAdaptorCall {
                adaptor: sp_call_parse_address(params.adaptor)?,
            };

            Ok(CellarV2Calls::SetupAdaptor(call).encode())
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
            VestingSimpleCalls(params) => calls.extend(
                adaptors::vesting_simple::vesting_simple_adaptor_v1_call(params)?,
            ),
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
