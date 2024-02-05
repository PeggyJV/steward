use ethers::{abi::AbiEncode, types::Bytes};

use crate::{
    abi::adaptors::staking_adaptor_v1::{self, StakingAdaptorV1Calls as AbiStakingAdaptorV1Calls},
    error::Error,
    proto::{staking_adaptor_v1::Function, StakingAdaptorV1Calls},
    utils::{hex_to_bytes, sp_call_error, sp_call_parse_address, string_to_u256},
};

/// Encodes adaptor calls for Staking Adaptor V1
pub fn staking_adaptor_v1_calls(params: StakingAdaptorV1Calls) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            Function::RevokeApproval(p) => {
                let call = staking_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AbiStakingAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            Function::Mint(p) => {
                let call = staking_adaptor_v1::MintCall {
                    amount: string_to_u256(p.amount)?,
                    min_amount_out: string_to_u256(p.min_amount_out)?,
                    wildcard: hex_to_bytes(p.wildcard)?,
                };
                calls.push(AbiStakingAdaptorV1Calls::Mint(call).encode().into())
            }
            Function::RequestBurn(p) => {
                let call = staking_adaptor_v1::RequestBurnCall {
                    amount: string_to_u256(p.amount)?,
                    wildcard: hex_to_bytes(p.wildcard)?,
                };
                calls.push(AbiStakingAdaptorV1Calls::RequestBurn(call).encode().into())
            }
            Function::CompleteBurn(p) => {
                let call = staking_adaptor_v1::CompleteBurnCall {
                    id: string_to_u256(p.id)?,
                    min_amount_out: string_to_u256(p.min_amount_out)?,
                    wildcard: hex_to_bytes(p.wildcard)?,
                };
                calls.push(AbiStakingAdaptorV1Calls::CompleteBurn(call).encode().into())
            }
            Function::CancelBurn(p) => {
                let call = staking_adaptor_v1::CancelBurnCall {
                    id: string_to_u256(p.id)?,
                    wildcard: hex_to_bytes(p.wildcard)?,
                };
                calls.push(AbiStakingAdaptorV1Calls::CancelBurn(call).encode().into())
            }
            Function::Wrap(p) => {
                let call = staking_adaptor_v1::WrapCall {
                    amount: string_to_u256(p.amount)?,
                    min_amount_out: string_to_u256(p.min_amount_out)?,
                    wildcard: hex_to_bytes(p.wildcard)?,
                };
                calls.push(AbiStakingAdaptorV1Calls::Wrap(call).encode().into())
            }
            Function::Unwrap(p) => {
                let call = staking_adaptor_v1::UnwrapCall {
                    amount: string_to_u256(p.amount)?,
                    min_amount_out: string_to_u256(p.min_amount_out)?,
                    wildcard: hex_to_bytes(p.wildcard)?,
                };
                calls.push(AbiStakingAdaptorV1Calls::Unwrap(call).encode().into())
            }
            Function::MintErc20(p) => {
                let call = staking_adaptor_v1::MintERC20Call {
                    deposit_asset: sp_call_parse_address(p.deposit_asset)?,
                    amount: string_to_u256(p.amount)?,
                    min_amount_out: string_to_u256(p.min_amount_out)?,
                    wildcard: hex_to_bytes(p.wildcard)?,
                };
                calls.push(AbiStakingAdaptorV1Calls::MintERC20(call).encode().into())
            }
            Function::RemoveClaimedRequest(p) => {
                let call = staking_adaptor_v1::RemoveClaimedRequestCall(
                    string_to_u256(p.id)?,
                    hex_to_bytes(p.wildcard)?,
                );
                calls.push(
                    AbiStakingAdaptorV1Calls::RemoveClaimedRequest(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
