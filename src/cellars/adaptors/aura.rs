use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::adaptors::aura_erc4626_adaptor_v1::{
    self, AuraERC4626AdaptorV1Calls as AbiAuraERC4626AdaptorV1Calls,
};

use crate::{
    error::Error,
    proto::{aura_erc4626_adaptor_v1::Function, AuraErc4626AdaptorV1Calls},
    utils::{sp_call_error, sp_call_parse_address},
};

/// Encodes adaptor calls for Aura Adaptor V1
pub fn aura_erc4626_adaptor_v1_calls(
    params: AuraErc4626AdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            Function::RevokeApproval(p) => {
                let call = aura_erc4626_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AbiAuraERC4626AdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            Function::GetRewards(p) => {
                let call = aura_erc4626_adaptor_v1::GetRewardsCall {
                    aura_pool: sp_call_parse_address(p.aura_pool)?,
                    claim_extras: p.claim_extras,
                };
                calls.push(
                    AbiAuraERC4626AdaptorV1Calls::GetRewards(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
