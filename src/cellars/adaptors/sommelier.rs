use ethers::{abi::AbiEncode, types::Bytes};

use crate::{
    abi::adaptors::cellar_adaptor_v1::CellarAdaptorV1Calls as AbiCellarAdaptorV1Calls,
    error::Error,
    proto::cellar_adaptor_v1,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn cellar_adaptor_v1_calls(
    params: crate::proto::CellarAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            cellar_adaptor_v1::Function::DepositToCellar(p) => {
                let call = crate::abi::adaptors::cellar_adaptor_v1::DepositToCellarCall {
                    cellar: sp_call_parse_address(p.cellar)?,
                    assets: string_to_u256(p.assets)?,
                };
                calls.push(
                    AbiCellarAdaptorV1Calls::DepositToCellar(call)
                        .encode()
                        .into(),
                )
            }
            cellar_adaptor_v1::Function::WithdrawFromCellar(p) => {
                let call = crate::abi::adaptors::cellar_adaptor_v1::WithdrawFromCellarCall {
                    cellar: sp_call_parse_address(p.cellar)?,
                    assets: string_to_u256(p.assets)?,
                };
                calls.push(
                    AbiCellarAdaptorV1Calls::WithdrawFromCellar(call)
                        .encode()
                        .into(),
                )
            }
            cellar_adaptor_v1::Function::RevokeApproval(p) => {
                let call = crate::abi::adaptors::cellar_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AbiCellarAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
