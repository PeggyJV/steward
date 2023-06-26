use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::f_token_adaptor::{
    CallAddInterestCall, FTokenAdaptorCalls, LendFraxCall, RedeemFraxShareCall, RevokeApprovalCall,
    WithdrawFraxCall,
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
