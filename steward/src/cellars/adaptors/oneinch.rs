use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::oneinch_adaptor::{OneInchAdaptorCalls, SwapWithOneInchCall};
use steward_proto::steward::one_inch_adaptor_v1;

use crate::{
    error::Error,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn one_inch_adaptor_v1_call(
    params: steward_proto::steward::OneInchAdaptorV1Calls,
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
                calls.push(OneInchAdaptorCalls::SwapWithOneInch(call).encode().into())
            }
            one_inch_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::oneinch_adaptor::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(OneInchAdaptorCalls::RevokeApproval(call).encode().into())
            }
        }
    }

    Ok(calls)
}
