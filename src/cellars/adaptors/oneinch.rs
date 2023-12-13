use ethers::{abi::AbiEncode, types::Bytes};

use crate::{
    abi::adaptors::oneinch_adaptor_v1::{
        OneInchAdaptorV1Calls as AbiOneInchAdaptorV1Calls, SwapWithOneInchCall,
    },
    error::Error,
    proto::one_inch_adaptor_v1,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn one_inch_adaptor_v1_calls(
    params: crate::proto::OneInchAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            one_inch_adaptor_v1::Function::SwapWithOneInch(p) => {
                let call = SwapWithOneInchCall {
                    amount: string_to_u256(p.amount)?,
                    swap_call_data: p.swap_call_data.into(),
                    token_in: sp_call_parse_address(p.token_in)?,
                    token_out: sp_call_parse_address(p.token_out)?,
                };
                calls.push(
                    AbiOneInchAdaptorV1Calls::SwapWithOneInch(call)
                        .encode()
                        .into(),
                )
            }
            one_inch_adaptor_v1::Function::RevokeApproval(p) => {
                let call = crate::abi::adaptors::oneinch_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AbiOneInchAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
