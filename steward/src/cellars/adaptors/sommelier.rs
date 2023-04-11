use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::cellar_adaptor::CellarAdaptorCalls;
use steward_proto::steward::cellar_adaptor_v1;

use crate::{
    error::Error,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn cellar_adaptor_v1_call(
    params: steward_proto::steward::CellarAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            cellar_adaptor_v1::Function::DepositToCellar(p) => {
                let call = steward_abi::cellar_adaptor::DepositToCellarCall {
                    cellar: sp_call_parse_address(p.cellar)?,
                    assets: string_to_u256(p.assets)?,
                };
                calls.push(CellarAdaptorCalls::DepositToCellar(call).encode().into())
            }
            cellar_adaptor_v1::Function::WithdrawFromCellar(p) => {
                let call = steward_abi::cellar_adaptor::WithdrawFromCellarCall {
                    cellar: sp_call_parse_address(p.cellar)?,
                    assets: string_to_u256(p.assets)?,
                };
                calls.push(CellarAdaptorCalls::WithdrawFromCellar(call).encode().into())
            }
            cellar_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::cellar_adaptor::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(CellarAdaptorCalls::RevokeApproval(call).encode().into())
            }
        }
    }

    Ok(calls)
}
