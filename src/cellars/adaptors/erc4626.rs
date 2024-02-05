use ethers::{abi::AbiEncode, types::Bytes};

use crate::{
    abi::adaptors::erc4626_adaptor_v1::{self, ERC4626AdaptorV1Calls as AbiERC4626AdaptorV1Calls},
    error::Error,
    proto::{erc4626_adaptor_v1::Function, Erc4626AdaptorV1Calls},
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

/// Encodes adaptor calls for ERC4626 Adaptor V1
pub fn erc4626_adaptor_v1_calls(params: Erc4626AdaptorV1Calls) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            Function::RevokeApproval(p) => {
                let call = erc4626_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AbiERC4626AdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            Function::DepositToVault(p) => {
                let call = erc4626_adaptor_v1::DepositToVaultCall {
                    erc_4626_vault: sp_call_parse_address(p.erc4626_vault)?,
                    assets: string_to_u256(p.assets)?,
                };
                calls.push(
                    AbiERC4626AdaptorV1Calls::DepositToVault(call)
                        .encode()
                        .into(),
                )
            }
            Function::WithdrawFromVault(p) => {
                let call = erc4626_adaptor_v1::WithdrawFromVaultCall {
                    erc_4626_vault: sp_call_parse_address(p.erc4626_vault)?,
                    assets: string_to_u256(p.assets)?,
                };
                calls.push(
                    AbiERC4626AdaptorV1Calls::WithdrawFromVault(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
