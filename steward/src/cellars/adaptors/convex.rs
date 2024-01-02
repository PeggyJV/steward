use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::convex_curve_adaptor_v1::ConvexCurveAdaptorV1Calls as AbiConvexCurveAdaptorV1Calls;
use steward_proto::steward::{convex_curve_adaptor_v1, ConvexCurveAdaptorV1Calls};

use crate::{
    error::Error,
    utils::{parse_selector, sp_call_error, sp_call_parse_address, string_to_u256},
};

/// Encodes adaptor calls for ConvexCurveAdaptor V1
pub fn convex_curve_adaptor_v1_calls(
    params: ConvexCurveAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            convex_curve_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::convex_curve_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    AbiConvexCurveAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            convex_curve_adaptor_v1::Function::DepositLptInConvexAndStake(p) => {
                let call = steward_abi::convex_curve_adaptor_v1::DepositLPTInConvexAndStakeCall {
                    pid: string_to_u256(p.pid)?,
                    base_reward_pool: sp_call_parse_address(p.base_reward_pool)?,
                    lpt: sp_call_parse_address(p.lpt)?,
                    pool: sp_call_parse_address(p.pool)?,
                    selector: parse_selector(p.selector)?,
                    amount: string_to_u256(p.amount_to_deposit)?,
                };
                calls.push(
                    AbiConvexCurveAdaptorV1Calls::DepositLPTInConvexAndStake(call)
                        .encode()
                        .into(),
                )
            }
            convex_curve_adaptor_v1::Function::WithdrawFromBaseRewardPoolAsLpt(p) => {
                let call =
                    steward_abi::convex_curve_adaptor_v1::WithdrawFromBaseRewardPoolAsLPTCall {
                        base_reward_pool: sp_call_parse_address(p.base_reward_pool)?,
                        amount: string_to_u256(p.amount_to_withdraw)?,
                        claim: p.claim,
                    };
                calls.push(
                    AbiConvexCurveAdaptorV1Calls::WithdrawFromBaseRewardPoolAsLPT(call)
                        .encode()
                        .into(),
                )
            }
            convex_curve_adaptor_v1::Function::GetRewards(p) => {
                let call = steward_abi::convex_curve_adaptor_v1::GetRewardsCall {
                    base_reward_pool: sp_call_parse_address(p.base_reward_pool)?,
                    claim_extras: p.claim_extras,
                };
                calls.push(
                    AbiConvexCurveAdaptorV1Calls::GetRewards(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
