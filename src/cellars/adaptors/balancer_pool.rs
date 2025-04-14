use std::convert::TryInto;

use abscissa_core::prelude::debug;
use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::{
    adaptors::balancer_pool_adaptor_v1::{
        BalancerPoolAdaptorV1Calls, ExitPoolRequest, SingleSwap as AbiSingleSwap,
        SwapData as AbiSwapData,
    },
    cellar_v2_2::AdaptorCall as AbiAdaptorCall,
};

use crate::{
    cellars::adaptors,
    error::{Error, ErrorKind},
    proto::{
        balancer_pool_adaptor_v1::{self, SingleSwap, SwapData},
        balancer_pool_adaptor_v1_flash_loan::{
            adaptor_call_for_balancer_pool_flash_loan::CallData::*,
            AdaptorCallForBalancerPoolFlashLoan,
        },
    },
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn balancer_pool_adaptor_v1_calls(
    params: crate::proto::BalancerPoolAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            balancer_pool_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::adaptors::balancer_pool_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    BalancerPoolAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            balancer_pool_adaptor_v1::Function::JoinPool(p) => {
                if !p.swaps_before_join.is_empty()
                    && p.swaps_before_join.iter().any(|s| s.kind == 0)
                {
                    return Err(sp_call_error("invalid swap kind".to_string()));
                }

                let swaps_before_join = convert_single_swap(p.swaps_before_join)?;

                if p.swap_data.is_none() {
                    return Err(sp_call_error("swap data must be set".to_string()));
                }

                let swap_data = convert_swap_data(p.swap_data.unwrap())?;

                let call = steward_abi::adaptors::balancer_pool_adaptor_v1::JoinPoolCall {
                    target_bpt: sp_call_parse_address(p.target_bpt)?,
                    swaps_before_join,
                    swap_data,
                    minimum_bpt: string_to_u256(p.minimum_bpt)?,
                };

                calls.push(BalancerPoolAdaptorV1Calls::JoinPool(call).encode().into())
            }
            balancer_pool_adaptor_v1::Function::ExitPool(p) => {
                if !p.swaps_after_exit.is_empty() && p.swaps_after_exit.iter().any(|s| s.kind == 0)
                {
                    return Err(sp_call_error("invalid swap kind".to_string()));
                }

                let swaps_after_exit = convert_single_swap(p.swaps_after_exit)?;

                if p.swap_data.is_none() {
                    return Err(sp_call_error("swap data must be set".to_string()));
                }

                let swap_data = convert_swap_data(p.swap_data.unwrap())?;

                let request = match p.request {
                    Some(r) => ExitPoolRequest {
                        assets: r
                            .assets
                            .into_iter()
                            .map(sp_call_parse_address)
                            .collect::<Result<Vec<_>, Error>>()?,
                        min_amounts_out: r
                            .min_amounts_out
                            .into_iter()
                            .map(string_to_u256)
                            .collect::<Result<Vec<_>, Error>>()?,
                        user_data: r.user_data.into(),
                        to_internal_balance: r.to_internal_balance,
                    },
                    None => return Err(sp_call_error("exit pool request must be set".to_string())),
                };

                let call = steward_abi::adaptors::balancer_pool_adaptor_v1::ExitPoolCall {
                    target_bpt: sp_call_parse_address(p.target_bpt)?,
                    swaps_after_exit,
                    swap_data,
                    request,
                };

                calls.push(BalancerPoolAdaptorV1Calls::ExitPool(call).encode().into())
            }
            balancer_pool_adaptor_v1::Function::StakeBpt(p) => {
                let call = steward_abi::adaptors::balancer_pool_adaptor_v1::StakeBPTCall {
                    bpt: sp_call_parse_address(p.bpt)?,
                    liquidity_gauge: sp_call_parse_address(p.liquidity_gauge)?,
                    amount_in: string_to_u256(p.amount_in)?,
                };
                calls.push(BalancerPoolAdaptorV1Calls::StakeBPT(call).encode().into())
            }
            balancer_pool_adaptor_v1::Function::UnstakeBpt(p) => {
                let call = steward_abi::adaptors::balancer_pool_adaptor_v1::UnstakeBPTCall {
                    bpt: sp_call_parse_address(p.bpt)?,
                    liquidity_gauge: sp_call_parse_address(p.liquidity_gauge)?,
                    amount_out: string_to_u256(p.amount_out)?,
                };
                calls.push(BalancerPoolAdaptorV1Calls::UnstakeBPT(call).encode().into())
            }
            balancer_pool_adaptor_v1::Function::ClaimRewards(p) => {
                let call = steward_abi::adaptors::balancer_pool_adaptor_v1::ClaimRewardsCall {
                    gauge: sp_call_parse_address(p.gauge)?,
                };
                calls.push(
                    BalancerPoolAdaptorV1Calls::ClaimRewards(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}

pub(crate) fn balancer_pool_adaptor_v1_flash_loan_calls(
    params: crate::proto::BalancerPoolAdaptorV1FlashLoanCalls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let Some(p) = c.make_flash_loan else {
            return Err(sp_call_error(
                "make flash loan function call cannot be empty".to_string(),
            ));
        };
        let call = steward_abi::adaptors::balancer_pool_adaptor_v1::MakeFlashLoanCall {
            tokens: p
                .tokens
                .iter()
                .map(|t| sp_call_parse_address(t.clone()))
                .collect::<Result<Vec<_>, _>>()?,
            amounts: p
                .amounts
                .iter()
                .map(|a| string_to_u256(a.clone()))
                .collect::<Result<Vec<_>, _>>()?,
            data: get_encoded_adaptor_calls(p.data)?.encode().into(),
        };
        calls.push(
            BalancerPoolAdaptorV1Calls::MakeFlashLoan(call)
                .encode()
                .into(),
        )
    }

    Ok(calls)
}

fn convert_single_swap(swaps: Vec<SingleSwap>) -> Result<Vec<AbiSingleSwap>, Error> {
    swaps
        .into_iter()
        .map(|s| {
            let pool_id = hex::decode(s.pool_id)
                .map_err(|e| {
                    ErrorKind::SPCallError.context(format!("failed to decode pool_id: {e}"))
                })?
                .try_into()
                .map_err(|_| {
                    ErrorKind::SPCallError.context("pool ID must be 32 bytes".to_string())
                })?;

            Ok(AbiSingleSwap {
                pool_id,
                kind: (s.kind - 1) as u8,
                asset_in: sp_call_parse_address(s.asset_in)?,
                asset_out: sp_call_parse_address(s.asset_out)?,
                amount: string_to_u256(s.amount)?,
                user_data: s.user_data.into(),
            })
        })
        .collect::<Result<Vec<_>, Error>>()
}

fn convert_swap_data(data: SwapData) -> Result<AbiSwapData, Error> {
    Ok(AbiSwapData {
        min_amounts_for_swaps: data
            .min_amounts_for_swaps
            .into_iter()
            .map(string_to_u256)
            .collect::<Result<Vec<_>, Error>>()?,
        swap_deadlines: data
            .swap_deadlines
            .into_iter()
            .map(string_to_u256)
            .collect::<Result<Vec<_>, Error>>()?,
    })
}

/// Encodes calls to the Adaptor contracts
fn get_encoded_adaptor_calls(
    data: Vec<AdaptorCallForBalancerPoolFlashLoan>,
) -> Result<Vec<AbiAdaptorCall>, Error> {
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
