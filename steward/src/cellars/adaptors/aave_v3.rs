use abscissa_core::tracing::log::debug;
use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::{
    aave_v3_a_token_adaptor_v1::AaveV3ATokenAdaptorV1Calls as AbiAaveV3ATokenAdaptorV1Calls,
    aave_v3_debt_token_adaptor_v1::{
        AaveV3DebtTokenAdaptorV1Calls as AbiAaveV3DebtTokenAdaptorV1Calls, FlashLoanCall,
        RepayWithATokensCall,
    },
    cellar_v2_2::AdaptorCall as AbiAdaptorCall,
};
use steward_proto::steward::{
    aave_v3_debt_token_adaptor_v1,
    aave_v3_debt_token_adaptor_v1_flash_loan::{
        adaptor_call_for_aave_v3_flash_loan::CallData::*, AdaptorCallForAaveV3FlashLoan,
    },
    aave_v3a_token_adaptor_v1, AaveV3DebtTokenAdaptorV1Calls,
    AaveV3DebtTokenAdaptorV1FlashLoanCalls, AaveV3aTokenAdaptorV1Calls,
};

use crate::{
    cellars::adaptors,
    error::Error,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn aave_v3_a_token_adaptor_v1_calls(
    params: AaveV3aTokenAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            aave_v3a_token_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::aave_v3_a_token_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AbiAaveV3ATokenAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            aave_v3a_token_adaptor_v1::Function::DepositToAave(p) => {
                let call = steward_abi::aave_v3_a_token_adaptor_v1::DepositToAaveCall {
                    token_to_deposit: sp_call_parse_address(p.token)?,
                    amount_to_deposit: string_to_u256(p.amount)?,
                };
                calls.push(
                    AbiAaveV3ATokenAdaptorV1Calls::DepositToAave(call)
                        .encode()
                        .into(),
                )
            }
            aave_v3a_token_adaptor_v1::Function::WithdrawFromAave(p) => {
                let call = steward_abi::aave_v3_a_token_adaptor_v1::WithdrawFromAaveCall {
                    token_to_withdraw: sp_call_parse_address(p.token)?,
                    amount_to_withdraw: string_to_u256(p.amount)?,
                };
                calls.push(
                    AbiAaveV3ATokenAdaptorV1Calls::WithdrawFromAave(call)
                        .encode()
                        .into(),
                )
            }
            aave_v3a_token_adaptor_v1::Function::AdjustIsolationModeAssetAsCollateral(p) => {
                let call = steward_abi::aave_v3_a_token_adaptor_v1::AdjustIsolationModeAssetAsCollateralCall {
                    asset: sp_call_parse_address(p.asset)?,
                    use_as_collateral: p.use_as_collateral,
                };
                calls.push(
                    AbiAaveV3ATokenAdaptorV1Calls::AdjustIsolationModeAssetAsCollateral(call)
                        .encode()
                        .into(),
                )
            }
            aave_v3a_token_adaptor_v1::Function::ChangeEmode(p) => {
                let call = steward_abi::aave_v3_a_token_adaptor_v1::ChangeEModeCall {
                    category_id: p.category_id as u8,
                };
                calls.push(
                    AbiAaveV3ATokenAdaptorV1Calls::ChangeEMode(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}

pub(crate) fn aave_v3_debt_token_adaptor_v1_calls(
    params: AaveV3DebtTokenAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            aave_v3_debt_token_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::aave_v3_debt_token_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AbiAaveV3DebtTokenAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            aave_v3_debt_token_adaptor_v1::Function::BorrowFromAave(p) => {
                let call = steward_abi::aave_v3_debt_token_adaptor_v1::BorrowFromAaveCall {
                    debt_token_to_borrow: sp_call_parse_address(p.token)?,
                    amount_to_borrow: string_to_u256(p.amount)?,
                };
                calls.push(
                    AbiAaveV3DebtTokenAdaptorV1Calls::BorrowFromAave(call)
                        .encode()
                        .into(),
                )
            }
            aave_v3_debt_token_adaptor_v1::Function::RepayAaveDebt(p) => {
                let call = steward_abi::aave_v3_debt_token_adaptor_v1::RepayAaveDebtCall {
                    token_to_repay: sp_call_parse_address(p.token)?,
                    amount_to_repay: string_to_u256(p.amount)?,
                };
                calls.push(
                    AbiAaveV3DebtTokenAdaptorV1Calls::RepayAaveDebt(call)
                        .encode()
                        .into(),
                )
            }
            aave_v3_debt_token_adaptor_v1::Function::RepayWithATokens(p) => {
                let call = RepayWithATokensCall {
                    underlying: sp_call_parse_address(p.underlying_token)?,
                    amount: string_to_u256(p.amount)?,
                };
                calls.push(
                    AbiAaveV3DebtTokenAdaptorV1Calls::RepayWithATokens(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}

pub(crate) fn aave_v3_debt_token_adaptor_v1_flash_loan_calls(
    params: AaveV3DebtTokenAdaptorV1FlashLoanCalls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let Some(p) = c.flash_loan else {
            return Err(sp_call_error(
                "make flash loan function call cannot be empty".to_string(),
            ));
        };
        let call = FlashLoanCall {
            loan_token: p
                .loan_tokens
                .iter()
                .map(|t| sp_call_parse_address(t.clone()))
                .collect::<Result<Vec<_>, _>>()?,
            loan_amount: p
                .loan_amounts
                .iter()
                .map(|a| string_to_u256(a.clone()))
                .collect::<Result<Vec<_>, _>>()?,
            params: get_encoded_adaptor_calls(p.params)?.encode().into(),
        };
        calls.push(
            AbiAaveV3DebtTokenAdaptorV1Calls::FlashLoan(call)
                .encode()
                .into(),
        )
    }

    Ok(calls)
}

/// Encodes calls to the Adaptor contracts
fn get_encoded_adaptor_calls(
    data: Vec<AdaptorCallForAaveV3FlashLoan>,
) -> Result<Vec<AbiAdaptorCall>, Error> {
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
            LegacyCellarV1Calls(params) => {
                calls.extend(adaptors::sommelier::legacy_cellar_adaptor_v1_calls(params)?)
            }
            DebtFTokenV1Calls(params) => {
                calls.extend(adaptors::frax::debt_f_token_adaptor_v1_calls(params)?)
            }
            CollateralFTokenV1Calls(params) => {
                calls.extend(adaptors::frax::collateral_f_token_adaptor_v1_calls(params)?)
            }
        };

        result.push(AbiAdaptorCall {
            adaptor: sp_call_parse_address(d.adaptor)?,
            call_data: calls,
        })
    }

    Ok(result)
}
