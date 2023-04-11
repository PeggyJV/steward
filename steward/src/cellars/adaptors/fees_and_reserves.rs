use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::fees_and_reserves_adaptor::{
    AddAssetsToReservesCall, ChangeUpkeepFrequencyCall, ChangeUpkeepMaxGasCall,
    FeesAndReservesAdaptorCalls, PrepareFeesCall, RevokeApprovalCall, SetupMetaDataCall,
    UpdateManagementFeeCall, UpdatePerformanceFeeCall, WithdrawAssetsFromReservesCall,
};
use steward_proto::steward::fees_and_reserves_adaptor_v1;

use crate::{
    error::Error,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn fees_and_reserves_adaptor_v1_call(
    params: steward_proto::steward::FeesAndReservesAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            fees_and_reserves_adaptor_v1::Function::RevokeApproval(p) => {
                let call = RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    FeesAndReservesAdaptorCalls::RevokeApproval(call)
                        .encode()
                        .into(),
                );
            }
            fees_and_reserves_adaptor_v1::Function::UpdatePerformanceFees(p) => {
                let call = UpdatePerformanceFeeCall {
                    performance_fee: p.performance_fee,
                };
                calls.push(
                    FeesAndReservesAdaptorCalls::UpdatePerformanceFee(call)
                        .encode()
                        .into(),
                );
            }
            fees_and_reserves_adaptor_v1::Function::UpdateManagementFees(p) => {
                let call = UpdateManagementFeeCall {
                    management_fee: p.management_fee,
                };
                calls.push(
                    FeesAndReservesAdaptorCalls::UpdateManagementFee(call)
                        .encode()
                        .into(),
                );
            }
            fees_and_reserves_adaptor_v1::Function::ChangeUpkeepFrequency(p) => {
                let call = ChangeUpkeepFrequencyCall {
                    new_frequency: p.new_frequency,
                };
                calls.push(
                    FeesAndReservesAdaptorCalls::ChangeUpkeepFrequency(call)
                        .encode()
                        .into(),
                );
            }
            fees_and_reserves_adaptor_v1::Function::ChangeUpkeepMaxGas(p) => {
                let call = ChangeUpkeepMaxGasCall {
                    new_max_gas: p.new_max_gas,
                };
                calls.push(
                    FeesAndReservesAdaptorCalls::ChangeUpkeepMaxGas(call)
                        .encode()
                        .into(),
                );
            }
            fees_and_reserves_adaptor_v1::Function::SetupMetaData(p) => {
                let call = SetupMetaDataCall {
                    management_fee: p.management_fee,
                    performance_fee: p.performance_fee,
                };
                calls.push(
                    FeesAndReservesAdaptorCalls::SetupMetaData(call)
                        .encode()
                        .into(),
                );
            }
            fees_and_reserves_adaptor_v1::Function::AddAssetsToReserves(p) => {
                let call = AddAssetsToReservesCall {
                    amount: string_to_u256(p.amount)?,
                };
                calls.push(
                    FeesAndReservesAdaptorCalls::AddAssetsToReserves(call)
                        .encode()
                        .into(),
                );
            }
            fees_and_reserves_adaptor_v1::Function::WithdrawAssetsFromReserves(p) => {
                let call = WithdrawAssetsFromReservesCall {
                    amount: string_to_u256(p.amount)?,
                };
                calls.push(
                    FeesAndReservesAdaptorCalls::WithdrawAssetsFromReserves(call)
                        .encode()
                        .into(),
                );
            }
            fees_and_reserves_adaptor_v1::Function::PrepareFees(p) => {
                let call = PrepareFeesCall {
                    amount: string_to_u256(p.amount)?,
                };
                calls.push(
                    FeesAndReservesAdaptorCalls::PrepareFees(call)
                        .encode()
                        .into(),
                );
            }
        }
    }

    Ok(calls)
}
