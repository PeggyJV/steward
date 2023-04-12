use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::compound_c_token_adaptor_v2::CompoundCTokenAdaptorV2Calls;
use steward_proto::steward::compound_c_token_adaptor_v2;

use crate::{
    error::Error,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn compound_c_token_v2_calls(
    params: steward_proto::steward::CompoundCTokenAdaptorV2Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            compound_c_token_adaptor_v2::Function::RevokeApproval(p) => {
                let call = steward_abi::compound_c_token_adaptor_v2::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    CompoundCTokenAdaptorV2Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            compound_c_token_adaptor_v2::Function::DepositToCompound(p) => {
                let call = steward_abi::compound_c_token_adaptor_v2::DepositToCompoundCall {
                    market: sp_call_parse_address(p.market)?,
                    amount_to_deposit: string_to_u256(p.amount_to_deposit)?,
                };
                calls.push(
                    CompoundCTokenAdaptorV2Calls::DepositToCompound(call)
                        .encode()
                        .into(),
                )
            }
            compound_c_token_adaptor_v2::Function::WithdrawFromCompound(p) => {
                let call = steward_abi::compound_c_token_adaptor_v2::WithdrawFromCompoundCall {
                    market: sp_call_parse_address(p.market)?,
                    amount_to_withdraw: string_to_u256(p.amount_to_withdraw)?,
                };
                calls.push(
                    CompoundCTokenAdaptorV2Calls::WithdrawFromCompound(call)
                        .encode()
                        .into(),
                )
            }
            compound_c_token_adaptor_v2::Function::ClaimComp(_) => {
                let call = steward_abi::compound_c_token_adaptor_v2::ClaimCompCall {};
                calls.push(
                    CompoundCTokenAdaptorV2Calls::ClaimComp(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
