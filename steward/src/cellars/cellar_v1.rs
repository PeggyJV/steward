//! Handlers for the Cellar.sol vault interface contract functions
//!
//! To learn more see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol
use deep_space::Address as CosmosAddress;
use ethers::{abi::AbiEncode, contract::EthCall, types::Address as EthereumAddress};
use steward_abi::cellar_v1::*;
use steward_proto::steward::{
    cellar_v1::Function as StrategyFunction,
    cellar_v1_governance::{trust_position::Position, Function as GovernanceFunction},
};
use GovernanceFunction::*;
use StrategyFunction::*;

use crate::{
    error::Error,
    utils::{
        convert_exchange, encode_fees_distributor_address, encode_swap_params,
        governance_call_error, sp_call_error, sp_call_parse_address, string_to_u256,
    },
};

use super::{log_cellar_call, log_governance_cellar_call};

const CELLAR_NAME: &str = "CellarV1";
const LOG_PREFIX: &str = CELLAR_NAME;

pub fn get_encoded_call(function: StrategyFunction, cellar_id: String) -> Result<Vec<u8>, Error> {
    get_call(function, cellar_id).map(|call| call.encode())
}

pub fn get_call(function: StrategyFunction, cellar_id: String) -> Result<CellarV1Calls, Error> {
    match function {
        AddPosition(params) => {
            log_cellar_call(CELLAR_NAME, &AddPositionCall::function_name(), &cellar_id);
            let call = AddPositionCall {
                index: string_to_u256(params.index)?,
                position: sp_call_parse_address(params.position)?,
            };

            Ok(CellarV1Calls::AddPosition(call))
        }
        PushPosition(params) => {
            log_cellar_call(CELLAR_NAME, &PushPositionCall::function_name(), &cellar_id);
            let call = PushPositionCall {
                position: sp_call_parse_address(params.position)?,
            };

            Ok(CellarV1Calls::PushPosition(call))
        }
        RemovePosition(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &RemovePositionCall::function_name(),
                &cellar_id,
            );
            let call = RemovePositionCall {
                index: string_to_u256(params.index)?,
            };

            Ok(CellarV1Calls::RemovePosition(call))
        }
        SetHoldingPosition(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetHoldingPositionCall::function_name(),
                &cellar_id,
            );
            let call = SetHoldingPositionCall {
                new_holding_position: sp_call_parse_address(params.new_holding_position)?,
            };

            Ok(CellarV1Calls::SetHoldingPosition(call))
        }
        Rebalance(params) => {
            log_cellar_call(CELLAR_NAME, &RebalanceCall::function_name(), &cellar_id);
            let swap_params = encode_swap_params(
                params
                    .params
                    .ok_or_else(|| sp_call_error("swap params cannot be empty".to_string()))?,
            )?;

            let call = RebalanceCall {
                from_position: sp_call_parse_address(params.from_position)?,
                to_position: sp_call_parse_address(params.to_position)?,
                assets_from: string_to_u256(params.assets_from)?,
                exchange: convert_exchange(params.exchange),
                params: swap_params.into(),
            };

            Ok(CellarV1Calls::Rebalance(call))
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

            Ok(CellarV1Calls::SetStrategistPayoutAddress(call))
        }
        SetWithdrawType(params) => {
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

            Ok(CellarV1Calls::SetWithdrawType(call))
        }
        SwapPositions(params) => {
            log_cellar_call(CELLAR_NAME, &SwapPositionsCall::function_name(), &cellar_id);
            let call = SwapPositionsCall {
                index_1: string_to_u256(params.index_1)?,
                index_2: string_to_u256(params.index_2)?,
            };

            Ok(CellarV1Calls::SwapPositions(call))
        }
        SetDepositLimit(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetDepositLimitCall::function_name(),
                &cellar_id,
            );
            let call = SetDepositLimitCall {
                new_limit: string_to_u256(params.new_limit)?,
            };

            Ok(CellarV1Calls::SetDepositLimit(call))
        }
        SetLiquidityLimit(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetLiquidityLimitCall::function_name(),
                &cellar_id,
            );
            let call = SetLiquidityLimitCall {
                new_limit: string_to_u256(params.new_limit)?,
            };

            Ok(CellarV1Calls::SetLiquidityLimit(call))
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

            Ok(CellarV1Calls::SetShareLockPeriod(call))
        }
        SetRebalanceDeviation(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetRebalanceDeviationCall::function_name(),
                &cellar_id,
            );
            let call = SetRebalanceDeviationCall {
                new_deviation: string_to_u256(params.new_deviation)?,
            };

            Ok(CellarV1Calls::SetRebalanceDeviation(call))
        }
    }
}
pub fn get_encoded_governance_call(
    function: GovernanceFunction,
    cellar_id: &str,
    proposal_id: u64,
) -> Result<Vec<u8>, Error> {
    match function {
        SetFeesDistributor(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &SetFeesDistributorCall::function_name(),
                cellar_id,
            );
            let address: CosmosAddress = params.new_fees_distributor.parse().map_err(|err| {
                governance_call_error(format!(
                    "{}: SetFeesDistibutor invalid address: {}",
                    LOG_PREFIX, err
                ))
            })?;
            let call = SetFeesDistributorCall {
                new_fees_distributor: encode_fees_distributor_address(address),
            };
            Ok(CellarV1Calls::SetFeesDistributor(call).encode())
        }
        InitiateShutdown(_) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &InitiateShutdownCall::function_name(),
                cellar_id,
            );
            let call = InitiateShutdownCall {};
            Ok(CellarV1Calls::InitiateShutdown(call).encode())
        }
        LiftShutdown(_) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &LiftShutdownCall::function_name(),
                cellar_id,
            );
            let call = LiftShutdownCall {};
            Ok(CellarV1Calls::LiftShutdown(call).encode())
        }
        ResetHighWatermark(_) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &ResetHighWatermarkCall::function_name(),
                cellar_id,
            );
            let call = ResetHighWatermarkCall {};
            Ok(CellarV1Calls::ResetHighWatermark(call).encode())
        }
        SetPerformanceFee(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &SetPerformanceFeeCall::function_name(),
                cellar_id,
            );
            let call = SetPerformanceFeeCall {
                new_performance_fee: params.amount,
            };
            Ok(CellarV1Calls::SetPerformanceFee(call).encode())
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
            Ok(CellarV1Calls::SetPlatformFee(call).encode())
        }
        SetStrategistPerformanceCut(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &SetStrategistPerformanceCutCall::function_name(),
                cellar_id,
            );
            let call = SetStrategistPerformanceCutCall { cut: params.amount };
            Ok(CellarV1Calls::SetStrategistPerformanceCut(call).encode())
        }
        SetStrategistPlatformCut(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &SetStrategistPlatformCutCall::function_name(),
                cellar_id,
            );
            let call = SetStrategistPlatformCutCall { cut: params.amount };
            Ok(CellarV1Calls::SetStrategistPlatformCut(call).encode())
        }
        TrustPosition(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &TrustPositionCall::function_name(),
                cellar_id,
            );
            if params.position.is_none() {
                return Err(governance_call_error(format!(
                    "{}: TrustPosition: position is required",
                    LOG_PREFIX
                )));
            }
            let position = params.position.unwrap();
            let position_address: String;
            // a little weird but necessary to avoid adding an overly complex enum to the proto
            let position_type: u8 = match position {
                Position::Erc20Address(address) => {
                    position_address = address;
                    0
                }
                Position::Erc4626Address(address) => {
                    position_address = address;
                    1
                }
                Position::CellarAddress(address) => {
                    position_address = address;
                    2
                }
            };
            let call = TrustPositionCall {
                position: position_address.parse::<EthereumAddress>().map_err(|err| {
                    governance_call_error(format!(
                        "{}: TrustPosition: invalid address: {}",
                        LOG_PREFIX, err
                    ))
                })?,
                position_type,
            };
            Ok(CellarV1Calls::TrustPosition(call).encode())
        }
    }
}
