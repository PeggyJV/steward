use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::vesting_simple_adaptor_v2::VestingSimpleAdaptorV2Calls as AbiVestingSimpleAdaptorV2Calls;
use steward_proto::steward::vesting_simple_adaptor_v2;

use crate::{
    error::Error,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn vesting_simple_adaptor_v2_calls(
    params: steward_proto::steward::VestingSimpleAdaptorV2Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            vesting_simple_adaptor_v2::Function::DepositToVesting(p) => {
                let call = steward_abi::vesting_simple_adaptor_v2::DepositToVestingCall {
                    vesting_contract: sp_call_parse_address(p.vesting_contract)?,
                    amount_to_deposit: string_to_u256(p.amount)?,
                };
                calls.push(
                    AbiVestingSimpleAdaptorV2Calls::DepositToVesting(call)
                        .encode()
                        .into(),
                )
            }
            vesting_simple_adaptor_v2::Function::WithdrawFromVesting(p) => {
                let call = steward_abi::vesting_simple_adaptor_v2::WithdrawFromVestingCall {
                    deposit_id: string_to_u256(p.deposit_id)?,
                    vesting_contract: sp_call_parse_address(p.vesting_contract)?,
                    amount_to_withdraw: string_to_u256(p.amount)?,
                };
                calls.push(
                    AbiVestingSimpleAdaptorV2Calls::WithdrawFromVesting(call)
                        .encode()
                        .into(),
                )
            }
            vesting_simple_adaptor_v2::Function::WithdrawAnyFromVesting(p) => {
                let call = steward_abi::vesting_simple_adaptor_v2::WithdrawAnyFromVestingCall {
                    vesting_contract: sp_call_parse_address(p.vesting_contract)?,
                    amount_to_withdraw: string_to_u256(p.amount)?,
                };
                calls.push(
                    AbiVestingSimpleAdaptorV2Calls::WithdrawAnyFromVesting(call)
                        .encode()
                        .into(),
                )
            }
            vesting_simple_adaptor_v2::Function::WithdrawAllFromVesting(p) => {
                let call = steward_abi::vesting_simple_adaptor_v2::WithdrawAllFromVestingCall {
                    vesting_contract: sp_call_parse_address(p.vesting_contract)?,
                };
                calls.push(
                    AbiVestingSimpleAdaptorV2Calls::WithdrawAllFromVesting(call)
                        .encode()
                        .into(),
                )
            }
            vesting_simple_adaptor_v2::Function::RevokeApproval(p) => {
                let call = steward_abi::vesting_simple_adaptor_v2::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AbiVestingSimpleAdaptorV2Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
