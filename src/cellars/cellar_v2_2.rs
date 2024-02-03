//! Handlers for V2.2 of the Cellar.sol contract functions
//!
//! To learn more see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol
use crate::abi::cellar_v2_2::{AdaptorCall as AbiAdaptorCall, *};
use crate::proto::{
    adaptor_call::CallData::*,
    cellar_v2_2::{function_call::Function, CallType, FunctionCall},
    cellar_v2_2governance::Function as GovernanceFunction,
    AdaptorCall,
};
use abscissa_core::tracing::{debug, info};
use ethers::{
    abi::AbiEncode,
    contract::EthCall,
    types::{Bytes, U256},
};

use crate::cellars::adaptors;
use crate::{
    error::{Error, ErrorKind},
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

use super::{
    check_blocked_adaptor, check_blocked_position, log_cellar_call, log_governance_cellar_call,
    validate_cache_price_router, validate_force_position_out, validate_new_adaptor,
    validate_new_position, V2_2_PERMISSIONS,
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
        Function::AddPosition(params) => {
            check_blocked_position(&params.position_id, &V2_2_PERMISSIONS)?;
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
                check_blocked_adaptor(&adaptor_call.adaptor)?;
            }

            log_cellar_call(CELLAR_NAME, &CallOnAdaptorCall::function_name(), &cellar_id);
            let call = CallOnAdaptorCall {
                data: get_encoded_adaptor_calls(params.data)?,
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
        Function::RemoveAdaptorFromCatalogue(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &RemoveAdaptorFromCatalogueCall::function_name(),
                &cellar_id,
            );
            let call = RemoveAdaptorFromCatalogueCall {
                adaptor: sp_call_parse_address(params.adaptor)?,
            };

            Ok(CellarV2_2Calls::RemoveAdaptorFromCatalogue(call).encode())
        }
        Function::RemovePositionFromCatalogue(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &RemovePositionFromCatalogueCall::function_name(),
                &cellar_id,
            );
            let call = RemovePositionFromCatalogueCall {
                position_id: params.position_id,
            };

            Ok(CellarV2_2Calls::RemovePositionFromCatalogue(call).encode())
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
        Function::CachePriceRouter(params) => {
            validate_cache_price_router(
                &cellar_id,
                params.check_total_assets,
                params.allowable_range,
                None,
            )?;
            log_cellar_call(
                CELLAR_NAME,
                &CachePriceRouterCall::function_name(),
                &cellar_id,
            );
            let call = CachePriceRouterCall {
                check_total_assets: params.check_total_assets,
                allowable_range: params.allowable_range as u16,
            };

            Ok(CellarV2_2Calls::CachePriceRouter(call).encode())
        }
        Function::AddAdaptorToCatalogue(params) => {
            validate_new_adaptor(&cellar_id, &params.adaptor, &V2_2_PERMISSIONS)?;
            log_cellar_call(
                CELLAR_NAME,
                &AddAdaptorToCatalogueCall::function_name(),
                &cellar_id,
            );
            let call = AddAdaptorToCatalogueCall {
                adaptor: sp_call_parse_address(params.adaptor)?,
            };

            Ok(CellarV2_2Calls::AddAdaptorToCatalogue(call).encode())
        }
        Function::AddPositionToCatalogue(params) => {
            validate_new_position(&cellar_id, params.position_id, &V2_2_PERMISSIONS)?;
            log_cellar_call(
                CELLAR_NAME,
                &AddPositionToCatalogueCall::function_name(),
                &cellar_id,
            );
            let call = AddPositionToCatalogueCall {
                position_id: params.position_id,
            };

            Ok(CellarV2_2Calls::AddPositionToCatalogue(call).encode())
        }
        Function::ForcePositionOut(params) => {
            validate_force_position_out(
                &cellar_id,
                params.index,
                params.position_id,
                params.in_debt_array,
            )?;
            log_cellar_call(
                CELLAR_NAME,
                &ForcePositionOutCall::function_name(),
                &cellar_id,
            );
            let call = ForcePositionOutCall {
                position_id: params.position_id,
                index: params.index,
                in_debt_array: params.in_debt_array,
            };

            Ok(CellarV2_2Calls::ForcePositionOut(call).encode())
        }
    }
}

pub fn get_encoded_governance_call(
    function: GovernanceFunction,
    cellar_id: &str,
    proposal_id: u64,
) -> Result<Vec<u8>, Error> {
    match function {
        GovernanceFunction::AddAdaptorToCatalogue(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &AddAdaptorToCatalogueCall::function_name(),
                cellar_id,
            );

            if let Err(err) = check_blocked_adaptor(&params.adaptor) {
                info!(
                    "did not process governance call due to blocked adaptor {}",
                    params.adaptor
                );
                return Err(err);
            }

            let call = AddAdaptorToCatalogueCall {
                adaptor: sp_call_parse_address(params.adaptor)?,
            };

            Ok(CellarV2_2Calls::AddAdaptorToCatalogue(call).encode())
        }
        GovernanceFunction::AddPositionToCatalogue(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &AddPositionToCatalogueCall::function_name(),
                cellar_id,
            );

            if let Err(err) = check_blocked_position(&params.position_id, &V2_2_PERMISSIONS) {
                info!(
                    "did not process governance call due to blocked position id {}",
                    params.position_id
                );
                return Err(err);
            }

            let call = AddPositionToCatalogueCall {
                position_id: params.position_id,
            };

            Ok(CellarV2_2Calls::AddPositionToCatalogue(call).encode())
        }
        GovernanceFunction::SetShareLockPeriod(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &SetShareLockPeriodCall::function_name(),
                cellar_id,
            );
            let call = SetShareLockPeriodCall {
                new_lock: string_to_u256(params.new_lock)?,
            };

            Ok(CellarV2_2Calls::SetShareLockPeriod(call).encode())
        }
        // This will ultimately need to be a governance function, but for Seven Sea's live testing we are keeping
        // it here until they get a feel for what an appropriate value is.
        GovernanceFunction::SetRebalanceDeviation(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &SetRebalanceDeviationCall::function_name(),
                cellar_id,
            );

            let new_deviation = string_to_u256(params.new_deviation)?;
            if new_deviation > U256::from(5000000000000000u64) {
                return Err(ErrorKind::SPCallError
                    .context("deviation must be 0.5% or less".to_string())
                    .into());
            }

            let call = SetRebalanceDeviationCall { new_deviation };

            Ok(CellarV2_2Calls::SetRebalanceDeviation(call).encode())
        }
        GovernanceFunction::InitiateShutdown(_) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &InitiateShutdownCall::function_name(),
                cellar_id,
            );
            let call = InitiateShutdownCall {};

            Ok(CellarV2_2Calls::InitiateShutdown(call).encode())
        }
        GovernanceFunction::SetStrategistPlatformCut(params) => {
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
        GovernanceFunction::LiftShutdown(_) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &LiftShutdownCall::function_name(),
                cellar_id,
            );
            let call = LiftShutdownCall {};

            Ok(CellarV2_2Calls::LiftShutdown(call).encode())
        }
        GovernanceFunction::ForcePositionOut(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &ForcePositionOutCall::function_name(),
                cellar_id,
            );
            let call = ForcePositionOutCall {
                position_id: params.position_id,
                index: params.index,
                in_debt_array: params.in_debt_array,
            };

            Ok(CellarV2_2Calls::ForcePositionOut(call).encode())
        }
        GovernanceFunction::ToggleIgnorePause(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &ToggleIgnorePauseCall::function_name(),
                cellar_id,
            );
            let call = ToggleIgnorePauseCall {
                toggle: params.ignore,
            };

            Ok(CellarV2_2Calls::ToggleIgnorePause(call).encode())
        }
        GovernanceFunction::CachePriceRouter(params) => {
            log_cellar_call(
                CELLAR_NAME,
                &CachePriceRouterCall::function_name(),
                cellar_id,
            );
            let call = CachePriceRouterCall {
                check_total_assets: params.check_total_assets,
                allowable_range: params.allowable_range as u16,
            };

            Ok(CellarV2_2Calls::CachePriceRouter(call).encode())
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
        };

        result.push(AbiAdaptorCall {
            adaptor: sp_call_parse_address(d.adaptor)?,
            call_data: calls,
        })
    }

    Ok(result)
}
