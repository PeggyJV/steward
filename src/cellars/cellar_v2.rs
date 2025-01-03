//! Handlers for V2 of the Cellar.sol contract functions
//!
//! To learn more see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol
use abscissa_core::tracing::{debug, info};
use ethers::{
    abi::AbiEncode,
    contract::EthCall,
    types::{Bytes, U256},
};

use crate::{
    abi::cellar_v2::{AdaptorCall as AbiAdaptorCall, *},
    cellars::adaptors,
    error::{Error, ErrorKind},
    proto::{
        adaptor_call::CallData::*, cellar_v2::Function,
        cellar_v2_governance::Function as GovernanceFunction, AdaptorCall,
    },
    utils::{sp_call_error, sp_call_parse_address, sp_disabled_call_error, string_to_u256},
};

use super::{
    check_blocked_adaptor, log_cellar_call, log_governance_cellar_call, validate_new_adaptor,
    validate_new_position, V2_0_PERMISSIONS,
};

const CELLAR_NAME: &str = "CellarV2";

pub fn get_encoded_call(function: Function, cellar_id: String) -> Result<Vec<u8>, Error> {
    get_call(function, cellar_id).map(|call| call.encode())
}

pub fn get_call(function: Function, cellar_id: String) -> Result<CellarV2Calls, Error> {
    match function {
        Function::AddPosition(params) => {
            validate_new_position(&cellar_id, params.position_id, &V2_0_PERMISSIONS)?;
            log_cellar_call(CELLAR_NAME, &AddPositionCall::function_name(), &cellar_id);

            let call = AddPositionCall {
                index: params.index,
                position_id: params.position_id,
                configuration_data: params.configuration_data.into(),
                in_debt_array: params.in_debt_array,
            };

            Ok(CellarV2Calls::AddPosition(call))
        }
        Function::CallOnAdaptor(params) => {
            for adaptor_call in params.data.clone() {
                check_blocked_adaptor(&adaptor_call.adaptor)?;
            }

            log_cellar_call(CELLAR_NAME, &CallOnAdaptorCall::function_name(), &cellar_id);
            let call = CallOnAdaptorCall {
                data: get_encoded_adaptor_calls(params.data)?,
            };

            Ok(CellarV2Calls::CallOnAdaptor(call))
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

            Ok(CellarV2Calls::RemovePosition(call))
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

            Ok(CellarV2Calls::SetHoldingPosition(call))
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

            Ok(CellarV2Calls::SetStrategistPayoutAddress(call))
        }
        Function::SwapPositions(params) => {
            log_cellar_call(CELLAR_NAME, &SwapPositionsCall::function_name(), &cellar_id);
            let call = SwapPositionsCall {
                index_1: params.index_1,
                index_2: params.index_2,
                in_debt_array: params.in_debt_array,
            };

            Ok(CellarV2Calls::SwapPositions(call))
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

            Ok(CellarV2Calls::SetShareLockPeriod(call))
        }
        Function::SetupAdaptor(params) => {
            validate_new_adaptor(&cellar_id, &params.adaptor, &V2_0_PERMISSIONS)?;
            log_cellar_call(CELLAR_NAME, &SetupAdaptorCall::function_name(), &cellar_id);

            let call = SetupAdaptorCall {
                adaptor: sp_call_parse_address(params.adaptor)?,
            };

            Ok(CellarV2Calls::SetupAdaptor(call))
        }
        Function::InitiateShutdown(_) => {
            log_cellar_call(
                CELLAR_NAME,
                &InitiateShutdownCall::function_name(),
                &cellar_id,
            );
            let call = InitiateShutdownCall {};
            Ok(CellarV2Calls::InitiateShutdown(call))
        }
        Function::LiftShutdown(_) => {
            log_cellar_call(CELLAR_NAME, &LiftShutdownCall::function_name(), &cellar_id);
            let call = LiftShutdownCall {};
            Ok(CellarV2Calls::LiftShutdown(call))
        }
        Function::SetPlatformFee(_) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetPlatformFeeCall::function_name(),
                &cellar_id,
            );

            Err(sp_disabled_call_error("SetPlatformCut is no longer available as a strategist function".to_string()))
        }
        Function::SetStrategistPlatformCut(_) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetStrategistPlatformCutCall::function_name(),
                &cellar_id,
            );

            Err(sp_disabled_call_error("SetStrategistPlatformCut is no longer available as a strategist function".to_string()))
        }
        Function::SetRebalanceDeviation(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetRebalanceDeviationCall::function_name(),
                &cellar_id,
            );
            let new_deviation = string_to_u256(params.new_deviation)?;
            if new_deviation > U256::from(5000000000000000u64) {
                return Err(ErrorKind::SPCallError
                    .context("deviation must be 0.5% or less".to_string())
                    .into());
            }

            let call = SetRebalanceDeviationCall { new_deviation };

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
        GovernanceFunction::InitiateShutdown(_) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &InitiateShutdownCall::function_name(),
                cellar_id,
            );
            let call = InitiateShutdownCall {};
            Ok(CellarV2Calls::InitiateShutdown(call).encode())
        }
        GovernanceFunction::LiftShutdown(_) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &LiftShutdownCall::function_name(),
                cellar_id,
            );
            let call = LiftShutdownCall {};
            Ok(CellarV2Calls::LiftShutdown(call).encode())
        }
        GovernanceFunction::SetPlatformFee(params) => {
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
        GovernanceFunction::SetStrategistPlatformCut(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &SetStrategistPlatformCutCall::function_name(),
                cellar_id,
            );
            let call = SetStrategistPlatformCutCall { cut: params.amount };
            Ok(CellarV2Calls::SetStrategistPlatformCut(call).encode())
        }
        GovernanceFunction::SetupAdaptor(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &SetupAdaptorCall::function_name(),
                cellar_id,
            );

            if let Err(err) = check_blocked_adaptor(&params.adaptor) {
                info!(
                    "did not process governance call due to blocked adaptor {}",
                    params.adaptor
                );
                return Err(err);
            }

            let call = SetupAdaptorCall {
                adaptor: sp_call_parse_address(params.adaptor)?,
            };

            Ok(CellarV2Calls::SetupAdaptor(call).encode())
        }
        GovernanceFunction::SetRebalanceDeviation(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &SetRebalanceDeviationCall::function_name(),
                cellar_id,
            );
            let call = SetRebalanceDeviationCall {
                new_deviation: string_to_u256(params.new_deviation)?,
            };

            Ok(CellarV2Calls::SetRebalanceDeviation(call).encode())
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
            AaveV3DebtTokenV1FlashLoanCalls(params) => calls
                .extend(adaptors::aave_v3::aave_v3_debt_token_adaptor_v1_flash_loan_calls(params)?),
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
            AaveV2EnableAssetAsCollateralV1Calls(params) => calls.extend(
                adaptors::aave_v2_collateral::aave_v2_enable_asset_as_collateral_adaptor_v1_calls(
                    params,
                )?,
            ),
            FTokenV1Calls(params) => {
                calls.extend(adaptors::frax::f_token_adaptor_v1_calls(params)?)
            }
            MorphoAaveV2ATokenV1Calls(params) => calls.extend(
                adaptors::morpho::morpho_aave_v2_a_token_adaptor_v1_calls(params)?,
            ),
            MorphoAaveV2DebtTokenV1Calls(params) => {
                calls.extend(adaptors::morpho::morpho_aave_v2_debt_token_adaptor_v1_calls(params)?)
            }
            MorphoAaveV3ATokenCollateralV1Calls(params) => calls.extend(
                adaptors::morpho::morpho_aave_v3_a_token_collateral_adaptor_v1_calls(params)?,
            ),
            MorphoAaveV3ATokenP2pV1Calls(params) => {
                calls.extend(adaptors::morpho::morpho_aave_v3_a_token_p2p_adaptor_v1_calls(params)?)
            }
            MorphoAaveV3DebtTokenV1Calls(params) => {
                calls.extend(adaptors::morpho::morpho_aave_v3_debt_token_adaptor_v1_calls(params)?)
            }
            BalancerPoolV1Calls(params) => calls.extend(
                adaptors::balancer_pool::balancer_pool_adaptor_v1_calls(params)?,
            ),
            BalancerPoolV1FlashLoanCalls(params) => calls.extend(
                adaptors::balancer_pool::balancer_pool_adaptor_v1_flash_loan_calls(params)?,
            ),
            LegacyCellarV1Calls(params) => {
                calls.extend(adaptors::sommelier::legacy_cellar_adaptor_v1_calls(params)?)
            }
            DebtFTokenV1Calls(params) => {
                calls.extend(adaptors::frax::debt_f_token_adaptor_v1_calls(params)?)
            }
            CollateralFTokenV1Calls(params) => {
                calls.extend(adaptors::frax::collateral_f_token_adaptor_v1_calls(params)?)
            }
            ConvexCurveV1Calls(params) => {
                calls.extend(adaptors::convex::convex_curve_adaptor_v1_calls(params)?)
            }
            CurveV1Calls(params) => calls.extend(adaptors::curve::curve_adaptor_v1_calls(params)?),
            AuraErc4626V1Calls(params) => {
                calls.extend(adaptors::aura::aura_erc4626_adaptor_v1_calls(params)?)
            }
            MorphoBlueCollateralV1Calls(params) => calls.extend(
                adaptors::morpho::morpho_blue_collateral_adaptor_v1_calls(params)?,
            ),
            MorphoBlueDebtV1Calls(params) => {
                calls.extend(adaptors::morpho::morpho_blue_debt_adaptor_v1_calls(params)?)
            }
            MorphoBlueSupplyV1Calls(params) => calls.extend(
                adaptors::morpho::morpho_blue_supply_adaptor_v1_calls(params)?,
            ),
            Erc4626V1Calls(params) => {
                calls.extend(adaptors::erc4626::erc4626_adaptor_v1_calls(params)?)
            }
            StakingV1Calls(params) => {
                calls.extend(adaptors::staking::staking_adaptor_v1_calls(params)?)
            }
            PendleV1Calls(params) => {
                calls.extend(adaptors::pendle::pendle_adaptor_v1_calls(params)?)
            }
        };

        result.push(AbiAdaptorCall {
            adaptor: sp_call_parse_address(d.adaptor)?,
            call_data: calls,
        })
    }

    Ok(result)
}
