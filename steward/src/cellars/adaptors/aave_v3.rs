use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::{
    aave_v3_a_token_adaptor::AaveV3ATokenAdaptorCalls,
    aave_v3_debt_token_adaptor::AaveV3DebtTokenAdaptorCalls,
};

use crate::{
    error::Error,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn aave_v3_a_token_adaptor_v1_call(
    params: steward_proto::steward::AaveV3aTokenAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            steward_proto::steward::aave_v3a_token_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::aave_v3_a_token_adaptor::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(AaveV3ATokenAdaptorCalls::RevokeApproval(call).encode().into())
            },
            steward_proto::steward::aave_v3a_token_adaptor_v1::Function::DepositToAave(p) => {
                let call = steward_abi::aave_v3_a_token_adaptor::DepositToAaveCall {
                    token_to_deposit: sp_call_parse_address(p.token)?,
                    amount_to_deposit: string_to_u256(p.amount)?,
                };
                calls.push(AaveV3ATokenAdaptorCalls::DepositToAave(call).encode().into())
            },
            steward_proto::steward::aave_v3a_token_adaptor_v1::Function::WithdrawFromAave(p) => {
                let call = steward_abi::aave_v3_a_token_adaptor::WithdrawFromAaveCall {
                    token_to_withdraw: sp_call_parse_address(p.token)?,
                    amount_to_withdraw: string_to_u256(p.amount)?,
                };
                calls.push(AaveV3ATokenAdaptorCalls::WithdrawFromAave(call).encode().into())
            },
            steward_proto::steward::aave_v3a_token_adaptor_v1::Function::AdjustIsolationModeAssetAsCollateral(p) => {
                let call = steward_abi::aave_v3_a_token_adaptor::AdjustIsolationModeAssetAsCollateralCall {
                    asset: sp_call_parse_address(p.asset)?,
                    use_as_collateral: p.use_as_collateral,
                };
                calls.push(AaveV3ATokenAdaptorCalls::AdjustIsolationModeAssetAsCollateral(call).encode().into())
            },
            steward_proto::steward::aave_v3a_token_adaptor_v1::Function::ChangeEmode(p) => {
                let call = steward_abi::aave_v3_a_token_adaptor::ChangeEModeCall {
                    category_id: p.category_id as u8,
                };
                calls.push(AaveV3ATokenAdaptorCalls::ChangeEMode(call).encode().into())
            },
        }
    }

    Ok(calls)
}

pub(crate) fn aave_v3_debt_token_adaptor_v1_call(
    params: steward_proto::steward::AaveV3DebtTokenAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            steward_proto::steward::aave_v3_debt_token_adaptor_v1::Function::RevokeApproval(
                p,
            ) => {
                let call = steward_abi::aave_v3_debt_token_adaptor::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AaveV3DebtTokenAdaptorCalls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            steward_proto::steward::aave_v3_debt_token_adaptor_v1::Function::BorrowFromAave(
                p,
            ) => {
                let call = steward_abi::aave_v3_debt_token_adaptor::BorrowFromAaveCall {
                    debt_token_to_borrow: sp_call_parse_address(p.token)?,
                    amount_to_borrow: string_to_u256(p.amount)?,
                };
                calls.push(
                    AaveV3DebtTokenAdaptorCalls::BorrowFromAave(call)
                        .encode()
                        .into(),
                )
            }
            steward_proto::steward::aave_v3_debt_token_adaptor_v1::Function::RepayAaveDebt(
                p,
            ) => {
                let call = steward_abi::aave_v3_debt_token_adaptor::RepayAaveDebtCall {
                    token_to_repay: sp_call_parse_address(p.token)?,
                    amount_to_repay: string_to_u256(p.amount)?,
                };
                calls.push(
                    AaveV3DebtTokenAdaptorCalls::RepayAaveDebt(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
