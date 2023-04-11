use abscissa_core::tracing::log::debug;
use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::{
    compound_c_token_adaptor::CompoundCTokenAdaptorCalls,
    compound_c_token_adaptor_v2::CompoundCTokenAdaptorV2Calls,
};
use steward_proto::steward::{
    compound_c_token_adaptor_v1, compound_c_token_adaptor_v2, CompoundCTokenAdaptorV1Calls,
};

use crate::{
    error::Error,
    utils::{
        convert_exchange, encode_oracle_swap_params, encode_swap_params, sp_call_error,
        sp_call_parse_address, string_to_u256,
    },
};

pub fn compound_c_token_v1_call(params: CompoundCTokenAdaptorV1Calls) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            compound_c_token_adaptor_v1::Function::DepositToCompound(p) => {
                let call = steward_abi::compound_c_token_adaptor::DepositToCompoundCall {
                    market: sp_call_parse_address(p.market)?,
                    amount_to_deposit: string_to_u256(p.amount_to_deposit)?,
                };
                calls.push(
                    CompoundCTokenAdaptorCalls::DepositToCompound(call)
                        .encode()
                        .into(),
                )
            }
            compound_c_token_adaptor_v1::Function::WithdrawFromCompound(p) => {
                let call = steward_abi::compound_c_token_adaptor::WithdrawFromCompoundCall {
                    market: sp_call_parse_address(p.market)?,
                    amount_to_withdraw: string_to_u256(p.amount_to_withdraw)?,
                };
                calls.push(
                    CompoundCTokenAdaptorCalls::WithdrawFromCompound(call)
                        .encode()
                        .into(),
                )
            }
            compound_c_token_adaptor_v1::Function::ClaimComp(p) => {
                let call = steward_abi::compound_c_token_adaptor::ClaimCompCall {};
                calls.push(CompoundCTokenAdaptorCalls::ClaimComp(call).encode().into())
            }
            compound_c_token_adaptor_v1::Function::ClaimCompAndSwap(p) => {
                let oracle_swap_params =
                    encode_oracle_swap_params(p.params.ok_or_else(|| {
                        sp_call_error("swap params cannot be empty".to_string())
                    })?)?;

                debug!("encoded: {:?}", hex::encode(&oracle_swap_params));
                let call = steward_abi::compound_c_token_adaptor::ClaimCompAndSwapCall {
                    asset_out: sp_call_parse_address(p.asset_out)?,
                    exchange: convert_exchange(p.exchange),
                    params: oracle_swap_params.into(),
                    slippage: p.slippage,
                };
                calls.push(
                    CompoundCTokenAdaptorCalls::ClaimCompAndSwap(call)
                        .encode()
                        .into(),
                )
            }
            compound_c_token_adaptor_v1::Function::Swap(p) => {
                let swap_params =
                    encode_swap_params(p.params.ok_or_else(|| {
                        sp_call_error("swap params cannot be empty".to_string())
                    })?)?;

                debug!("encoded: {:?}", hex::encode(&swap_params));
                let call = steward_abi::compound_c_token_adaptor::SwapCall {
                    asset_in: sp_call_parse_address(p.asset_in)?,
                    asset_out: sp_call_parse_address(p.asset_out)?,
                    amount_in: string_to_u256(p.amount_in)?,
                    exchange: convert_exchange(p.exchange),
                    params: swap_params.into(),
                };
                calls.push(CompoundCTokenAdaptorCalls::Swap(call).encode().into())
            }
            compound_c_token_adaptor_v1::Function::OracleSwap(p) => {
                let oracle_swap_params =
                    encode_oracle_swap_params(p.params.ok_or_else(|| {
                        sp_call_error("swap params cannot be empty".to_string())
                    })?)?;

                debug!("encoded: {:?}", hex::encode(&oracle_swap_params));
                let call = steward_abi::compound_c_token_adaptor::OracleSwapCall {
                    asset_in: sp_call_parse_address(p.asset_in)?,
                    asset_out: sp_call_parse_address(p.asset_out)?,
                    amount_in: string_to_u256(p.amount_in)?,
                    exchange: convert_exchange(p.exchange),
                    params: oracle_swap_params.into(),
                    slippage: p.slippage,
                };
                calls.push(CompoundCTokenAdaptorCalls::OracleSwap(call).encode().into())
            }
            compound_c_token_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::compound_c_token_adaptor::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    CompoundCTokenAdaptorCalls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}

pub(crate) fn compound_c_token_v2_call(
    params: steward_proto::steward::CompoundCTokenAdaptorV2Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            compound_c_token_adaptor_v2::Function::RevokeApproval(p) => {
                let call = steward_abi::compound_c_token_adaptor_v2::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    CompoundCTokenAdaptorV2Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            compound_c_token_adaptor_v2::Function::DepositToCompound(p) => {
                let call = steward_abi::compound_c_token_adaptor_v2::DepositToCompoundCall {
                    market: sp_call_parse_address(p.market)?,
                    amount_to_deposit: string_to_u256(p.amount_to_deposit)?,
                };
                calls.push(
                    CompoundCTokenAdaptorV2Calls::DepositToCompound(call)
                        .encode()
                        .into(),
                )
            }
            compound_c_token_adaptor_v2::Function::WithdrawFromCompound(p) => {
                let call = steward_abi::compound_c_token_adaptor_v2::WithdrawFromCompoundCall {
                    market: sp_call_parse_address(p.market)?,
                    amount_to_withdraw: string_to_u256(p.amount_to_withdraw)?,
                };
                calls.push(
                    CompoundCTokenAdaptorV2Calls::WithdrawFromCompound(call)
                        .encode()
                        .into(),
                )
            }
            compound_c_token_adaptor_v2::Function::ClaimComp(p) => {
                let call = steward_abi::compound_c_token_adaptor_v2::ClaimCompCall {};
                calls.push(
                    CompoundCTokenAdaptorV2Calls::ClaimComp(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
