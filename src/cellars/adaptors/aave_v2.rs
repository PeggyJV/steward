use abscissa_core::tracing::log::debug;
use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::adaptors::{
    aave_v2_a_token_adaptor_v1::AaveATokenAdaptorV1Calls as AbiAaveATokenAdaptorV1Calls,
    aave_v2_a_token_adaptor_v2::AaveATokenAdaptorV2Calls as AbiAaveATokenAdaptorV2Calls,
    aave_v2_debt_token_adaptor_v1::AaveDebtTokenAdaptorV1Calls as AbiAaveDebtTokenAdaptorV1Calls,
    aave_v2_debt_token_adaptor_v2::AaveDebtTokenAdaptorV2Calls as AbiAaveDebtTokenAdaptorV2Calls,
};

use crate::{
    error::Error,
    proto::{
        aave_a_token_adaptor_v1, aave_a_token_adaptor_v2, aave_debt_token_adaptor_v1,
        aave_debt_token_adaptor_v2, AaveATokenAdaptorV1Calls, AaveDebtTokenAdaptorV1Calls,
    },
    utils::{
        convert_exchange, encode_oracle_swap_params, encode_swap_params, sp_call_error,
        sp_call_parse_address, string_to_u256,
    },
};

/// Encodes adaptor calls for AaveATokenAdaptor V1
pub fn aave_a_token_adaptor_v1_calls(
    params: AaveATokenAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            aave_a_token_adaptor_v1::Function::DepositToAave(p) => {
                let call = steward_abi::adaptors::aave_v2_a_token_adaptor_v1::DepositToAaveCall {
                    token_to_deposit: sp_call_parse_address(p.token)?,
                    amount_to_deposit: string_to_u256(p.amount)?,
                };
                calls.push(
                    AbiAaveATokenAdaptorV1Calls::DepositToAave(call)
                        .encode()
                        .into(),
                )
            }
            aave_a_token_adaptor_v1::Function::WithdrawFromAave(p) => {
                let call =
                    steward_abi::adaptors::aave_v2_a_token_adaptor_v1::WithdrawFromAaveCall {
                        token_to_withdraw: sp_call_parse_address(p.token)?,
                        amount_to_withdraw: string_to_u256(p.amount)?,
                    };
                calls.push(
                    AbiAaveATokenAdaptorV1Calls::WithdrawFromAave(call)
                        .encode()
                        .into(),
                )
            }
            aave_a_token_adaptor_v1::Function::Swap(p) => {
                let swap_params =
                    encode_swap_params(p.params.ok_or_else(|| {
                        sp_call_error("swap params cannot be empty".to_string())
                    })?)?;

                debug!("encoded: {:?}", hex::encode(&swap_params));
                let call = steward_abi::adaptors::aave_v2_a_token_adaptor_v1::SwapCall {
                    asset_in: sp_call_parse_address(p.asset_in)?,
                    asset_out: sp_call_parse_address(p.asset_out)?,
                    amount_in: string_to_u256(p.amount_in)?,
                    exchange: convert_exchange(p.exchange),
                    params: swap_params.into(),
                };
                calls.push(AbiAaveATokenAdaptorV1Calls::Swap(call).encode().into())
            }
            aave_a_token_adaptor_v1::Function::OracleSwap(p) => {
                let oracle_swap_params =
                    encode_oracle_swap_params(p.params.ok_or_else(|| {
                        sp_call_error("swap params cannot be empty".to_string())
                    })?)?;

                debug!("encoded: {:?}", hex::encode(&oracle_swap_params));
                let call = steward_abi::adaptors::aave_v2_a_token_adaptor_v1::OracleSwapCall {
                    asset_in: sp_call_parse_address(p.asset_in)?,
                    asset_out: sp_call_parse_address(p.asset_out)?,
                    amount_in: string_to_u256(p.amount_in)?,
                    exchange: convert_exchange(p.exchange),
                    params: oracle_swap_params.into(),
                    slippage: p.slippage,
                };
                calls.push(
                    AbiAaveATokenAdaptorV1Calls::OracleSwap(call)
                        .encode()
                        .into(),
                )
            }
            aave_a_token_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::adaptors::aave_v2_a_token_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AbiAaveATokenAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}

pub fn aave_debt_token_adaptor_v1_calls(
    params: AaveDebtTokenAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            aave_debt_token_adaptor_v1::Function::BorrowFromAave(p) => {
                let call =
                    steward_abi::adaptors::aave_v2_debt_token_adaptor_v1::BorrowFromAaveCall {
                        debt_token_to_borrow: sp_call_parse_address(p.token)?,
                        amount_to_borrow: string_to_u256(p.amount)?,
                    };
                calls.push(
                    AbiAaveDebtTokenAdaptorV1Calls::BorrowFromAave(call)
                        .encode()
                        .into(),
                )
            }
            aave_debt_token_adaptor_v1::Function::RepayAaveDebt(p) => {
                let call =
                    steward_abi::adaptors::aave_v2_debt_token_adaptor_v1::RepayAaveDebtCall {
                        token_to_repay: sp_call_parse_address(p.token)?,
                        amount_to_repay: string_to_u256(p.amount)?,
                    };
                calls.push(
                    AbiAaveDebtTokenAdaptorV1Calls::RepayAaveDebt(call)
                        .encode()
                        .into(),
                )
            }
            aave_debt_token_adaptor_v1::Function::SwapAndRepay(p) => {
                let swap_params =
                    encode_swap_params(p.params.ok_or_else(|| {
                        sp_call_error("swap params cannot be empty".to_string())
                    })?)?;

                debug!("encoded: {:?}", hex::encode(&swap_params));
                let call = steward_abi::adaptors::aave_v2_debt_token_adaptor_v1::SwapAndRepayCall {
                    token_in: sp_call_parse_address(p.token_in)?,
                    token_to_repay: sp_call_parse_address(p.token_to_repay)?,
                    amount_in: string_to_u256(p.amount_in)?,
                    exchange: convert_exchange(p.exchange),
                    params: swap_params.into(),
                };
                calls.push(
                    AbiAaveDebtTokenAdaptorV1Calls::SwapAndRepay(call)
                        .encode()
                        .into(),
                )
            }
            aave_debt_token_adaptor_v1::Function::Swap(p) => {
                let swap_params =
                    encode_swap_params(p.params.ok_or_else(|| {
                        sp_call_error("swap params cannot be empty".to_string())
                    })?)?;

                debug!("encoded: {:?}", hex::encode(&swap_params));
                let call = steward_abi::adaptors::aave_v2_debt_token_adaptor_v1::SwapCall {
                    asset_in: sp_call_parse_address(p.asset_in)?,
                    asset_out: sp_call_parse_address(p.asset_out)?,
                    amount_in: string_to_u256(p.amount_in)?,
                    exchange: convert_exchange(p.exchange),
                    params: swap_params.into(),
                };
                calls.push(AbiAaveDebtTokenAdaptorV1Calls::Swap(call).encode().into())
            }
            aave_debt_token_adaptor_v1::Function::OracleSwap(p) => {
                let oracle_swap_params =
                    encode_oracle_swap_params(p.params.ok_or_else(|| {
                        sp_call_error("swap params cannot be empty".to_string())
                    })?)?;

                debug!("encoded: {:?}", hex::encode(&oracle_swap_params));
                let call = steward_abi::adaptors::aave_v2_debt_token_adaptor_v1::OracleSwapCall {
                    asset_in: sp_call_parse_address(p.asset_in)?,
                    asset_out: sp_call_parse_address(p.asset_out)?,
                    amount_in: string_to_u256(p.amount_in)?,
                    exchange: convert_exchange(p.exchange),
                    params: oracle_swap_params.into(),
                    slippage: p.slippage,
                };
                calls.push(
                    AbiAaveDebtTokenAdaptorV1Calls::OracleSwap(call)
                        .encode()
                        .into(),
                )
            }
            aave_debt_token_adaptor_v1::Function::RevokeApproval(p) => {
                let call =
                    steward_abi::adaptors::aave_v2_debt_token_adaptor_v1::RevokeApprovalCall {
                        asset: sp_call_parse_address(p.asset)?,
                        spender: sp_call_parse_address(p.spender)?,
                    };
                calls.push(
                    AbiAaveDebtTokenAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}

pub(crate) fn aave_a_token_adaptor_v2_calls(
    params: crate::proto::AaveATokenAdaptorV2Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            aave_a_token_adaptor_v2::Function::RevokeApproval(p) => {
                let call = steward_abi::adaptors::aave_v2_a_token_adaptor_v2::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AbiAaveATokenAdaptorV2Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            aave_a_token_adaptor_v2::Function::DepositToAave(p) => {
                let call = steward_abi::adaptors::aave_v2_a_token_adaptor_v2::DepositToAaveCall {
                    token_to_deposit: sp_call_parse_address(p.token)?,
                    amount_to_deposit: string_to_u256(p.amount)?,
                };
                calls.push(
                    AbiAaveATokenAdaptorV2Calls::DepositToAave(call)
                        .encode()
                        .into(),
                )
            }
            aave_a_token_adaptor_v2::Function::WithdrawFromAave(p) => {
                let call =
                    steward_abi::adaptors::aave_v2_a_token_adaptor_v2::WithdrawFromAaveCall {
                        token_to_withdraw: sp_call_parse_address(p.token)?,
                        amount_to_withdraw: string_to_u256(p.amount)?,
                    };
                calls.push(
                    AbiAaveATokenAdaptorV2Calls::WithdrawFromAave(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}

pub(crate) fn aave_debt_token_adaptor_v2_calls(
    params: crate::proto::AaveDebtTokenAdaptorV2Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            aave_debt_token_adaptor_v2::Function::RevokeApproval(p) => {
                let call =
                    steward_abi::adaptors::aave_v2_debt_token_adaptor_v2::RevokeApprovalCall {
                        asset: sp_call_parse_address(p.asset)?,
                        spender: sp_call_parse_address(p.spender)?,
                    };
                calls.push(
                    AbiAaveDebtTokenAdaptorV2Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            aave_debt_token_adaptor_v2::Function::BorrowFromAave(p) => {
                let call =
                    steward_abi::adaptors::aave_v2_debt_token_adaptor_v2::BorrowFromAaveCall {
                        debt_token_to_borrow: sp_call_parse_address(p.token)?,
                        amount_to_borrow: string_to_u256(p.amount)?,
                    };
                calls.push(
                    AbiAaveDebtTokenAdaptorV2Calls::BorrowFromAave(call)
                        .encode()
                        .into(),
                )
            }
            aave_debt_token_adaptor_v2::Function::RepayAaveDebt(p) => {
                let call =
                    steward_abi::adaptors::aave_v2_debt_token_adaptor_v2::RepayAaveDebtCall {
                        token_to_repay: sp_call_parse_address(p.token)?,
                        amount_to_repay: string_to_u256(p.amount)?,
                    };
                calls.push(
                    AbiAaveDebtTokenAdaptorV2Calls::RepayAaveDebt(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
