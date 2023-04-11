use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::zero_x_adaptor_v1::ZeroXAdaptorV1Calls as AbiZeroXAdaptorV1Calls;
use steward_proto::steward::zero_x_adaptor_v1;

use crate::{
    error::Error,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn zero_x_adaptor_v1_calls(
    params: steward_proto::steward::ZeroXAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            zero_x_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::zero_x_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(AbiZeroXAdaptorV1Calls::RevokeApproval(call).encode().into())
            }
            zero_x_adaptor_v1::Function::SwapWith0x(p) => {
                let call = steward_abi::zero_x_adaptor_v1::SwapWith0XCall {
                    amount: string_to_u256(p.amount)?,
                    swap_call_data: p.swap_call_data.into(),
                    token_in: sp_call_parse_address(p.token_in)?,
                    token_out: sp_call_parse_address(p.token_out)?,
                };
                calls.push(AbiZeroXAdaptorV1Calls::SwapWith0X(call).encode().into())
            }
        }
    }

    Ok(calls)
}
