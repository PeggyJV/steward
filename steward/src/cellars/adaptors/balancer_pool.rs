use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::balancer_pool_adaptor_v1::BalancerPoolAdaptorV1Calls;
use steward_proto::steward::balancer_pool_adaptor_v1;

use crate::{
    error::Error,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn balancer_pool_adaptor_v1_calls(
    params: steward_proto::steward::BalancerPoolAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            balancer_pool_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::balancer_pool_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    BalancerPoolAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            balancer_pool_adaptor_v1::Function::RelayerJoinPool(p) => {
                let call = steward_abi::balancer_pool_adaptor_v1::RelayerJoinPoolCall {
                    tokens_in: p
                        .tokens_in
                        .into_iter()
                        .map(sp_call_parse_address)
                        .collect::<Result<Vec<_>, _>>()?,
                    amounts_in: p
                        .amounts_in
                        .into_iter()
                        .map(|x| string_to_u256(x))
                        .collect::<Result<Vec<_>, _>>()?,
                    bpt_out: sp_call_parse_address(p.btp_out)?,
                    call_data: p.call_data.into_iter().map(Into::into).collect(),
                };
                calls.push(
                    BalancerPoolAdaptorV1Calls::RelayerJoinPool(call)
                        .encode()
                        .into(),
                )
            }
            balancer_pool_adaptor_v1::Function::RelayerExitPool(p) => {
                let call = steward_abi::balancer_pool_adaptor_v1::RelayerExitPoolCall {
                    tokens_out: p
                        .tokens_out
                        .into_iter()
                        .map(sp_call_parse_address)
                        .collect::<Result<Vec<_>, _>>()?,
                    amount_in: string_to_u256(p.amount_in)?,
                    bpt_in: sp_call_parse_address(p.bpt_in)?,
                    call_data: p.call_data.into_iter().map(Into::into).collect(),
                };
                calls.push(
                    BalancerPoolAdaptorV1Calls::RelayerExitPool(call)
                        .encode()
                        .into(),
                )
            }
            balancer_pool_adaptor_v1::Function::StakeBpt(p) => {
                let call = steward_abi::balancer_pool_adaptor_v1::StakeBPTCall {
                    bpt: sp_call_parse_address(p.bpt)?,
                    liquidity_gauge: sp_call_parse_address(p.liquidity_gauge)?,
                    amount_in: string_to_u256(p.amount_in)?,
                };
                calls.push(BalancerPoolAdaptorV1Calls::StakeBPT(call).encode().into())
            }
            balancer_pool_adaptor_v1::Function::UnstakeBpt(p) => {
                let call = steward_abi::balancer_pool_adaptor_v1::UnstakeBPTCall {
                    bpt: sp_call_parse_address(p.bpt)?,
                    liquidity_gauge: sp_call_parse_address(p.liquidity_gauge)?,
                    amount_out: string_to_u256(p.amount_out)?,
                };
                calls.push(BalancerPoolAdaptorV1Calls::UnstakeBPT(call).encode().into())
            }
            balancer_pool_adaptor_v1::Function::ClaimRewards(p) => {
                let call = steward_abi::balancer_pool_adaptor_v1::ClaimRewardsCall {
                    gauge: sp_call_parse_address(p.guage)?,
                };
                calls.push(BalancerPoolAdaptorV1Calls::ClaimRewards(call).encode().into())
            }
        }
    }

    Ok(calls)
}
