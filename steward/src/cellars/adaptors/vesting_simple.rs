use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::vesting_simple_adaptor::VestingSimpleAdaptorCalls;
use steward_proto::steward::vesting_simple_adaptor;

use crate::{
    error::Error,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn vesting_simple_adaptor_v1_call(
    params: steward_proto::steward::VestingSimpleAdaptorCalls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            vesting_simple_adaptor::Function::DepositToVesting(p) => {
                let call = steward_abi::vesting_simple_adaptor::DepositToVestingCall {
                    vesting_contract: sp_call_parse_address(p.vesting_contract)?,
                    amount_to_deposit: string_to_u256(p.amount)?,
                };
                calls.push(
                    VestingSimpleAdaptorCalls::DepositToVesting(call)
                        .encode()
                        .into(),
                )
            }
            vesting_simple_adaptor::Function::WithdrawFromVesting(p) => {
                let call = steward_abi::vesting_simple_adaptor::WithdrawFromVestingCall {
                    deposit_id: string_to_u256(p.deposit_id)?,
                    vesting_contract: sp_call_parse_address(p.vesting_contract)?,
                    amount_to_withdraw: string_to_u256(p.amount)?,
                };
                calls.push(
                    VestingSimpleAdaptorCalls::WithdrawFromVesting(call)
                        .encode()
                        .into(),
                )
            }
            vesting_simple_adaptor::Function::WithdrawAnyFromVesting(p) => {
                let call = steward_abi::vesting_simple_adaptor::WithdrawAnyFromVestingCall {
                    vesting_contract: sp_call_parse_address(p.vesting_contract)?,
                    amount_to_withdraw: string_to_u256(p.amount)?,
                };
                calls.push(
                    VestingSimpleAdaptorCalls::WithdrawAnyFromVesting(call)
                        .encode()
                        .into(),
                )
            }
            vesting_simple_adaptor::Function::WithdrawAllFromVesting(p) => {
                let call = steward_abi::vesting_simple_adaptor::WithdrawAllFromVestingCall {
                    vesting_contract: sp_call_parse_address(p.vesting_contract)?,
                };
                calls.push(
                    VestingSimpleAdaptorCalls::WithdrawAllFromVesting(call)
                        .encode()
                        .into(),
                )
            }
            vesting_simple_adaptor::Function::RevokeApproval(p) => {
                let call = steward_abi::vesting_simple_adaptor::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    VestingSimpleAdaptorCalls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
