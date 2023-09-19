use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::cellar_adaptor_v1::CellarAdaptorV1Calls as AbiCellarAdaptorV1Calls;
use steward_abi::legacy_cellar_adaptor_v1::LegacyCellarAdaptorV1Calls as AbiLegacyCellarAdaptorV1Calls;
use steward_proto::steward::{cellar_adaptor_v1, legacy_cellar_adaptor_v1};

use crate::{
    error::Error,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn cellar_adaptor_v1_calls(
    params: steward_proto::steward::CellarAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            cellar_adaptor_v1::Function::DepositToCellar(p) => {
                let call = steward_abi::cellar_adaptor_v1::DepositToCellarCall {
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
                let call = steward_abi::cellar_adaptor_v1::WithdrawFromCellarCall {
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
                let call = steward_abi::cellar_adaptor_v1::RevokeApprovalCall {
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

pub(crate) fn legacy_cellar_adaptor_v1_calls(
    params: steward_proto::steward::LegacyCellarAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            legacy_cellar_adaptor_v1::Function::DepositToCellar(p) => {
                let call = steward_abi::legacy_cellar_adaptor_v1::DepositToCellarCall {
                    cellar: sp_call_parse_address(p.cellar)?,
                    assets: string_to_u256(p.assets)?,
                    oracle: sp_call_parse_address(p.oracle)?,
                };
                calls.push(
                    AbiLegacyCellarAdaptorV1Calls::DepositToCellar(call)
                        .encode()
                        .into(),
                )
            }
            legacy_cellar_adaptor_v1::Function::WithdrawFromCellar(p) => {
                let call = steward_abi::legacy_cellar_adaptor_v1::WithdrawFromCellarCall {
                    cellar: sp_call_parse_address(p.cellar)?,
                    assets: string_to_u256(p.assets)?,
                    oracle: sp_call_parse_address(p.oracle)?,
                };
                calls.push(
                    AbiLegacyCellarAdaptorV1Calls::WithdrawFromCellar(call)
                        .encode()
                        .into(),
                )
            }
            legacy_cellar_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::legacy_cellar_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AbiLegacyCellarAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
