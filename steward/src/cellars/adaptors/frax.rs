use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::{
    collateral_f_token_adaptor_v1::{
        AddCollateralCall, CollateralFTokenAdaptorV1Calls, RemoveCollateralCall,
        RevokeApprovalCall as CollateralFTokenRevokeApprovalCall,
    },
    debt_f_token_adaptor_v1::{
        BorrowFromFraxlendCall, CallAddInterestCall as DebtFTokenCallAddInterestCall,
        DebtFTokenAdaptorV1Calls, RepayFraxlendDebtCall,
        RevokeApprovalCall as DebtFTokenRevokeApprovalCall,
    },
    f_token_adaptor::{
        CallAddInterestCall, FTokenAdaptorCalls, LendFraxCall, RedeemFraxShareCall,
        RevokeApprovalCall, WithdrawFraxCall,
    },
};

use crate::{
    error::Error,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn f_token_adaptor_v1_calls(
    params: steward_proto::steward::FTokenAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            steward_proto::steward::f_token_adaptor_v1::Function::RevokeApproval(p) => {
                let call = RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(FTokenAdaptorCalls::RevokeApproval(call).encode().into());
            }
            steward_proto::steward::f_token_adaptor_v1::Function::LendFrax(p) => {
                let call = LendFraxCall {
                    f_token: sp_call_parse_address(p.f_token)?,
                    amount_to_deposit: string_to_u256(p.amount_to_deposit)?,
                };
                calls.push(FTokenAdaptorCalls::LendFrax(call).encode().into());
            }
            steward_proto::steward::f_token_adaptor_v1::Function::RedeemFraxShare(p) => {
                let call = RedeemFraxShareCall {
                    f_token: sp_call_parse_address(p.f_token)?,
                    amount_to_redeem: string_to_u256(p.amount_to_redeem)?,
                };
                calls.push(FTokenAdaptorCalls::RedeemFraxShare(call).encode().into());
            }
            steward_proto::steward::f_token_adaptor_v1::Function::WithdrawFrax(p) => {
                let call = WithdrawFraxCall {
                    f_token: sp_call_parse_address(p.f_token)?,
                    amount_to_withdraw: string_to_u256(p.amount_to_withdraw)?,
                };
                calls.push(FTokenAdaptorCalls::WithdrawFrax(call).encode().into());
            }
            steward_proto::steward::f_token_adaptor_v1::Function::CallAddInterest(p) => {
                let call = CallAddInterestCall {
                    f_token: sp_call_parse_address(p.f_token)?,
                };
                calls.push(FTokenAdaptorCalls::CallAddInterest(call).encode().into());
            }
        }
    }

    Ok(calls)
}

pub(crate) fn debt_f_token_adaptor_v1_calls(
    params: steward_proto::steward::DebtFTokenAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            steward_proto::steward::debt_f_token_adaptor_v1::Function::RevokeApproval(p) => {
                let call = DebtFTokenRevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };

                calls.push(
                    DebtFTokenAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                );
            }
            steward_proto::steward::debt_f_token_adaptor_v1::Function::BorrowFromFraxlend(p) => {
                let call = BorrowFromFraxlendCall {
                    amount_to_borrow: string_to_u256(p.amount_to_borrow)?,
                    fraxlend_pair: sp_call_parse_address(p.fraxlend_pair)?,
                };

                calls.push(
                    DebtFTokenAdaptorV1Calls::BorrowFromFraxlend(call)
                        .encode()
                        .into(),
                );
            }
            steward_proto::steward::debt_f_token_adaptor_v1::Function::RepayFraxlendDebt(p) => {
                let call = RepayFraxlendDebtCall {
                    debt_token_repay_amount: string_to_u256(p.debt_token_repay_amount)?,
                    fraxlend_pair: sp_call_parse_address(p.fraxlend_pair)?,
                };

                calls.push(
                    DebtFTokenAdaptorV1Calls::RepayFraxlendDebt(call)
                        .encode()
                        .into(),
                );
            }
            steward_proto::steward::debt_f_token_adaptor_v1::Function::CallAddInterest(p) => {
                let call = DebtFTokenCallAddInterestCall {
                    fraxlend_pair: sp_call_parse_address(p.fraxlend_pair)?,
                };

                calls.push(
                    DebtFTokenAdaptorV1Calls::CallAddInterest(call)
                        .encode()
                        .into(),
                );
            }
        }
    }

    Ok(calls)
}

pub(crate) fn collateral_f_token_adaptor_v1_calls(
    params: steward_proto::steward::CollateralFTokenAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            steward_proto::steward::collateral_f_token_adaptor_v1::Function::RevokeApproval(p) => {
                let call = CollateralFTokenRevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };

                calls.push(
                    CollateralFTokenAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                );
            }
            steward_proto::steward::collateral_f_token_adaptor_v1::Function::AddCollateral(p) => {
                let call = AddCollateralCall {
                    collateral_to_deposit: string_to_u256(p.collateral_to_deposit)?,
                    fraxlend_pair: sp_call_parse_address(p.fraxlend_pair)?,
                };

                calls.push(
                    CollateralFTokenAdaptorV1Calls::AddCollateral(call)
                        .encode()
                        .into(),
                );
            }
            steward_proto::steward::collateral_f_token_adaptor_v1::Function::RemoveCollateral(
                p,
            ) => {
                let call = RemoveCollateralCall {
                    collateral_amount: string_to_u256(p.collateral_amount)?,
                    fraxlend_pair: sp_call_parse_address(p.fraxlend_pair)?,
                };

                calls.push(
                    CollateralFTokenAdaptorV1Calls::RemoveCollateral(call)
                        .encode()
                        .into(),
                );
            }
        }
    }

    Ok(calls)
}
