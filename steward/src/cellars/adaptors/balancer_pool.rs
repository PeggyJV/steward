use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::balancer_pool_adaptor_v1::{
    BalancerPoolAdaptorV1Calls, ExitPoolRequest, SwapData,
};
use steward_proto::steward::balancer_pool_adaptor_v1;

use crate::{
    error::Error,
    utils::{sp_call_error, sp_call_parse_address, sp_call_parse_h256, string_to_u256},
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
            balancer_pool_adaptor_v1::Function::JoinPool(p) => {
                if !p.swaps_before_join.is_empty()
                    && p.swaps_before_join.iter().any(|s| s.kind == 0)
                {
                    return Err(sp_call_error("invalid swap kind".to_string()));
                }

                let swaps_before_join = p
                    .swaps_before_join
                    .into_iter()
                    .map(|s| {
                        Ok(steward_abi::balancer_pool_adaptor_v1::SingleSwap {
                            pool_id: sp_call_parse_h256(s.pool_id)?.into(),
                            kind: (s.kind - 1) as u8,
                            asset_in: sp_call_parse_address(s.asset_in)?,
                            asset_out: sp_call_parse_address(s.asset_out)?,
                            amount: string_to_u256(s.amount)?,
                            user_data: s.user_data.into(),
                        })
                    })
                    .collect::<Result<Vec<_>, Error>>()?;

                let swap_data = match p.swap_data {
                    Some(d) => SwapData {
                        min_amounts_for_swaps: d
                            .min_amounts_for_swaps
                            .into_iter()
                            .map(string_to_u256)
                            .collect::<Result<Vec<_>, Error>>()?,
                        swap_deadlines: d
                            .swap_deadlines
                            .into_iter()
                            .map(string_to_u256)
                            .collect::<Result<Vec<_>, Error>>()?,
                    },
                    None => return Err(sp_call_error("swap data must be set".to_string())),
                };

                let call = steward_abi::balancer_pool_adaptor_v1::JoinPoolCall {
                    target_bpt: sp_call_parse_address(p.target_bpt)?,
                    swaps_before_join,
                    swap_data,
                    minimum_bpt: string_to_u256(p.minimum_bpt)?,
                };

                calls.push(BalancerPoolAdaptorV1Calls::JoinPool(call).encode().into())
            }
            balancer_pool_adaptor_v1::Function::ExitPool(p) => {
                if !p.swaps_after_exit.is_empty() && p.swaps_after_exit.iter().any(|s| s.kind == 0)
                {
                    return Err(sp_call_error("invalid swap kind".to_string()));
                }

                let swaps_after_exit = p
                    .swaps_after_exit
                    .into_iter()
                    .map(|s| {
                        Ok(steward_abi::balancer_pool_adaptor_v1::SingleSwap {
                            pool_id: sp_call_parse_h256(s.pool_id)?.into(),
                            kind: (s.kind - 1) as u8,
                            asset_in: sp_call_parse_address(s.asset_in)?,
                            asset_out: sp_call_parse_address(s.asset_out)?,
                            amount: string_to_u256(s.amount)?,
                            user_data: s.user_data.into(),
                        })
                    })
                    .collect::<Result<Vec<_>, Error>>()?;

                let swap_data = match p.swap_data {
                    Some(d) => SwapData {
                        min_amounts_for_swaps: d
                            .min_amounts_for_swaps
                            .into_iter()
                            .map(string_to_u256)
                            .collect::<Result<Vec<_>, Error>>()?,
                        swap_deadlines: d
                            .swap_deadlines
                            .into_iter()
                            .map(string_to_u256)
                            .collect::<Result<Vec<_>, Error>>()?,
                    },
                    None => return Err(sp_call_error("swap data must be set".to_string())),
                };

                let request = match p.request {
                    Some(r) => ExitPoolRequest {
                        assets: r
                            .assets
                            .into_iter()
                            .map(sp_call_parse_address)
                            .collect::<Result<Vec<_>, Error>>()?,
                        min_amounts_out: r
                            .min_amounts_out
                            .into_iter()
                            .map(string_to_u256)
                            .collect::<Result<Vec<_>, Error>>()?,
                        user_data: r.user_data.into(),
                        to_internal_balance: r.to_internal_balance,
                    },
                    None => return Err(sp_call_error("exit pool request must be set".to_string())),
                };

                let call = steward_abi::balancer_pool_adaptor_v1::ExitPoolCall {
                    target_bpt: sp_call_parse_address(p.target_bpt)?,
                    swaps_after_exit,
                    swap_data,
                    request,
                };

                calls.push(BalancerPoolAdaptorV1Calls::ExitPool(call).encode().into())
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
                calls.push(
                    BalancerPoolAdaptorV1Calls::ClaimRewards(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
