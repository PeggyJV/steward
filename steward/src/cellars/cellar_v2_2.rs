//! Handlers for V2.2 of the Cellar.sol contract functions
//!
//! To learn more see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol
use abscissa_core::tracing::debug;
use ethers::{abi::AbiEncode, contract::EthCall, types::Bytes};
use steward_abi::cellar_v2_2::{AdaptorCall as AbiAdaptorCall, *};
use steward_proto::steward::{
    adaptor_call::CallData::*,
    cellar_v2_2::{function_call::Function as StrategistFunction, CallType, FunctionCall},
    cellar_v2dot2_governance::Function as GovernanceFunction,
    AdaptorCall,
};
use GovernanceFunction::*;
use StrategistFunction::*;

use crate::cellars::adaptors;
use crate::{
    error::{Error, ErrorKind},
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

use super::{
    log_cellar_call, log_governance_cellar_call, normalize_address, BLOCKED_ADAPTORS,
    BLOCKED_POSITIONS,
};

const CELLAR_NAME: &str = "CellarV2.2";

/// Encodes a call to a CellarV2.2 contract
pub fn get_encoded_call(call_type: CallType, cellar_id: String) -> Result<Vec<u8>, Error> {
    match call_type {
        CallType::FunctionCall(f) => get_encoded_function(f, cellar_id),
        CallType::Multicall(m) => {
            let mut multicall = MulticallCall::default();
            m.function_calls
                .iter()
                .map(|f| get_encoded_function(f.clone(), cellar_id.clone()))
                .collect::<Result<Vec<Vec<u8>>, Error>>()?
                .iter()
                .for_each(|f| multicall.data.push(Bytes::from(f.clone())));

            Ok(multicall.encode())
        }
    }
}

pub fn get_encoded_function(call: FunctionCall, cellar_id: String) -> Result<Vec<u8>, Error> {
    let function = call
        .function
        .ok_or_else(|| sp_call_error("call data is empty".to_string()))?;
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

            Ok(CellarV2_2Calls::AddPosition(call).encode())
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
                data: get_encoded_adaptor_calls(params.data)?,
            };

            Ok(CellarV2_2Calls::CallOnAdaptor(call).encode())
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

            Ok(CellarV2_2Calls::RemovePosition(call).encode())
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

            Ok(CellarV2_2Calls::SetHoldingPosition(call).encode())
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

            Ok(CellarV2_2Calls::SetStrategistPayoutAddress(call).encode())
        }
        SwapPositions(params) => {
            log_cellar_call(CELLAR_NAME, &SwapPositionsCall::function_name(), &cellar_id);
            let call = SwapPositionsCall {
                index_1: params.index_1,
                index_2: params.index_2,
                in_debt_array: params.in_debt_array,
            };

            Ok(CellarV2_2Calls::SwapPositions(call).encode())
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

            Ok(CellarV2_2Calls::SetShareLockPeriod(call).encode())
        }
    }
}

pub fn get_encoded_governance_call(
    function: GovernanceFunction,
    cellar_id: &str,
    proposal_id: u64,
) -> Result<Vec<u8>, Error> {
    match function {
        AddPositionToCatalogue(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &AddPositionToCatalogueCall::function_name(),
                cellar_id,
            );
            let call = AddPositionToCatalogueCall {
                position_id: params.position_id,
            };
            Ok(CellarV2_2Calls::AddPositionToCatalogue(call).encode())
        }
        RemovePositionFromCatalogue(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &RemovePositionFromCatalogueCall::function_name(),
                cellar_id,
            );
            let call = RemovePositionFromCatalogueCall {
                position_id: params.position_id,
            };
            Ok(CellarV2_2Calls::RemovePositionFromCatalogue(call).encode())
        }
        AddAdaptorToCatalogue(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &AddAdaptorToCatalogueCall::function_name(),
                cellar_id,
            );
            let call = AddAdaptorToCatalogueCall {
                adaptor: sp_call_parse_address(params.adaptor)?,
            };
            Ok(CellarV2_2Calls::AddAdaptorToCatalogue(call).encode())
        }
        RemoveAdaptorFromCatalogue(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &RemoveAdaptorFromCatalogueCall::function_name(),
                cellar_id,
            );
            let call = RemoveAdaptorFromCatalogueCall {
                adaptor: sp_call_parse_address(params.adaptor)?,
            };
            Ok(CellarV2_2Calls::RemoveAdaptorFromCatalogue(call).encode())
        }
        ForcePositionOut(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &ForcePositionOutCall::function_name(),
                cellar_id,
            );
            let call = ForcePositionOutCall {
                index: params.index,
                position_id: params.position_id,
                in_debt_array: params.in_debt_array,
            };
            Ok(CellarV2_2Calls::ForcePositionOut(call).encode())
        }
        ToggleIgnorePause(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &ToggleIgnorePauseCall::function_name(),
                cellar_id,
            );
            let call = ToggleIgnorePauseCall {
                toggle: params.ignore_pause,
            };
            Ok(CellarV2_2Calls::ToggleIgnorePause(call).encode())
        }
        SetRebalanceDeviation(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &SetRebalanceDeviationCall::function_name(),
                cellar_id,
            );
            let call = SetRebalanceDeviationCall {
                new_deviation: string_to_u256(params.new_deviation)?,
            };
            Ok(CellarV2_2Calls::SetRebalanceDeviation(call).encode())
        }
        InitiateShutdown(_) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &InitiateShutdownCall::function_name(),
                cellar_id,
            );
            let call = InitiateShutdownCall {};
            Ok(CellarV2_2Calls::InitiateShutdown(call).encode())
        }
        LiftShutdown(_) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &LiftShutdownCall::function_name(),
                cellar_id,
            );
            let call = LiftShutdownCall {};
            Ok(CellarV2_2Calls::LiftShutdown(call).encode())
        }
        SetStrategistPlatformCut(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &SetStrategistPlatformCutCall::function_name(),
                cellar_id,
            );
            let call = SetStrategistPlatformCutCall {
                cut: params.new_cut,
            };
            Ok(CellarV2_2Calls::SetStrategistPlatformCut(call).encode())
        }
    }
}

/// Encodes calls to the Adaptor contracts
fn get_encoded_adaptor_calls(data: Vec<AdaptorCall>) -> Result<Vec<AbiAdaptorCall>, Error> {
    let mut result: Vec<AbiAdaptorCall> = Vec::new();
    for d in data {
        debug!("adaptor call to {}", d.adaptor);
        let mut calls: Vec<Bytes> = Vec::new();
        let call_data = d
            .call_data
            .ok_or_else(|| sp_call_error("call data is empty".to_string()))?;

        match call_data {
            UniswapV3V1Calls(params) => {
                calls.extend(adaptors::uniswap_v3::uniswap_v3_adaptor_v1_calls(params)?)
            }
            AaveATokenV1Calls(params) => {
                calls.extend(adaptors::aave_v2::aave_a_token_adaptor_v1_calls(params)?)
            }
            AaveDebtTokenV1Calls(params) => {
                calls.extend(adaptors::aave_v2::aave_debt_token_adaptor_v1_calls(params)?)
            }
            AaveATokenV2Calls(params) => {
                calls.extend(adaptors::aave_v2::aave_a_token_adaptor_v2_calls(params)?)
            }
            AaveDebtTokenV2Calls(params) => {
                calls.extend(adaptors::aave_v2::aave_debt_token_adaptor_v2_calls(params)?)
            }
            AaveV3ATokenV1Calls(params) => {
                calls.extend(adaptors::aave_v3::aave_v3_a_token_adaptor_v1_calls(params)?)
            }
            AaveV3DebtTokenV1Calls(params) => calls.extend(
                adaptors::aave_v3::aave_v3_debt_token_adaptor_v1_calls(params)?,
            ),
            OneInchV1Calls(params) => {
                calls.extend(adaptors::oneinch::one_inch_adaptor_v1_calls(params)?)
            }
            FeesAndReservesV1Calls(params) => calls
                .extend(adaptors::fees_and_reserves::fees_and_reserves_adaptor_v1_calls(params)?),
            ZeroXV1Calls(params) => {
                calls.extend(adaptors::zero_x::zero_x_adaptor_v1_calls(params)?)
            }
            SwapWithUniswapV1Calls(params) => calls
                .extend(adaptors::swap_with_uniswap::swap_with_uniswap_adaptor_v1_calls(params)?),
            CompoundCTokenV2Calls(params) => {
                calls.extend(adaptors::compound::compound_c_token_v2_calls(params)?)
            }
            VestingSimpleV2Calls(params) => calls.extend(
                adaptors::vesting_simple::vesting_simple_adaptor_v2_calls(params)?,
            ),
            CellarV1Calls(params) => {
                calls.extend(adaptors::sommelier::cellar_adaptor_v1_calls(params)?)
            }
            UniswapV3V2Calls(params) => {
                calls.extend(adaptors::uniswap_v3::uniswap_v3_adaptor_v2_calls(params)?)
            }
        };

        result.push(AbiAdaptorCall {
            adaptor: sp_call_parse_address(d.adaptor)?,
            call_data: calls,
        })
    }

    Ok(result)
}
