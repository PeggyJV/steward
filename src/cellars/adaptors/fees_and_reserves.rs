use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::adaptors::fees_and_reserves_adaptor_v1::{
    AddAssetsToReservesCall, ChangeUpkeepFrequencyCall, ChangeUpkeepMaxGasCall,
    FeesAndReservesAdaptorV1Calls as AbiFeesAndReservesAdaptorV1Calls, PrepareFeesCall,
    RevokeApprovalCall, SetupMetaDataCall, UpdateManagementFeeCall, UpdatePerformanceFeeCall,
    WithdrawAssetsFromReservesCall,
};

use crate::{
    error::Error,
    proto::fees_and_reserves_adaptor_v1,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn fees_and_reserves_adaptor_v1_calls(
    params: crate::proto::FeesAndReservesAdaptorV1Calls,
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
                    AbiFeesAndReservesAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                );
            }
            fees_and_reserves_adaptor_v1::Function::UpdatePerformanceFees(p) => {
                let call = UpdatePerformanceFeeCall {
                    performance_fee: p.performance_fee,
                };
                calls.push(
                    AbiFeesAndReservesAdaptorV1Calls::UpdatePerformanceFee(call)
                        .encode()
                        .into(),
                );
            }
            fees_and_reserves_adaptor_v1::Function::UpdateManagementFees(p) => {
                let call = UpdateManagementFeeCall {
                    management_fee: p.management_fee,
                };
                calls.push(
                    AbiFeesAndReservesAdaptorV1Calls::UpdateManagementFee(call)
                        .encode()
                        .into(),
                );
            }
            fees_and_reserves_adaptor_v1::Function::ChangeUpkeepFrequency(p) => {
                let call = ChangeUpkeepFrequencyCall {
                    new_frequency: p.new_frequency,
                };
                calls.push(
                    AbiFeesAndReservesAdaptorV1Calls::ChangeUpkeepFrequency(call)
                        .encode()
                        .into(),
                );
            }
            fees_and_reserves_adaptor_v1::Function::ChangeUpkeepMaxGas(p) => {
                let call = ChangeUpkeepMaxGasCall {
                    new_max_gas: p.new_max_gas,
                };
                calls.push(
                    AbiFeesAndReservesAdaptorV1Calls::ChangeUpkeepMaxGas(call)
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
                    AbiFeesAndReservesAdaptorV1Calls::SetupMetaData(call)
                        .encode()
                        .into(),
                );
            }
            fees_and_reserves_adaptor_v1::Function::AddAssetsToReserves(p) => {
                let call = AddAssetsToReservesCall {
                    amount: string_to_u256(p.amount)?,
                };
                calls.push(
                    AbiFeesAndReservesAdaptorV1Calls::AddAssetsToReserves(call)
                        .encode()
                        .into(),
                );
            }
            fees_and_reserves_adaptor_v1::Function::WithdrawAssetsFromReserves(p) => {
                let call = WithdrawAssetsFromReservesCall {
                    amount: string_to_u256(p.amount)?,
                };
                calls.push(
                    AbiFeesAndReservesAdaptorV1Calls::WithdrawAssetsFromReserves(call)
                        .encode()
                        .into(),
                );
            }
            fees_and_reserves_adaptor_v1::Function::PrepareFees(p) => {
                let call = PrepareFeesCall {
                    amount: string_to_u256(p.amount)?,
                };
                calls.push(
                    AbiFeesAndReservesAdaptorV1Calls::PrepareFees(call)
                        .encode()
                        .into(),
                );
            }
        }
    }

    Ok(calls)
}
