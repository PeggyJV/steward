use std::convert::TryInto;

use ethers::{
    abi::AbiEncode,
    prelude::{H160, U256},
};
use steward_abi::aave_v2_stablecoin::*;
use steward_proto::steward::aave_v2_stablecoin::Function::{self, *};

use crate::error::{Error, ErrorKind};

const LOG_PREFIX: &str = "AaveV2StablcoinCellar";

pub fn get_encoded_call(function: Function) -> Result<Vec<u8>, Error> {
    match function {
        AccrueFees(_) => {
            let call = AccrueFeesCall {};
            Ok(AaveV2StablecoinCellarCalls::AccrueFees(call).encode())
        }
        ClaimAndUnstake(_) => {
            let call = ClaimAndUnstakeCall {};
            Ok(AaveV2StablecoinCellarCalls::ClaimAndUnstake(call).encode())
        }
        EnterPosition(_) => {
            let call = EnterPositionCall {};
            let call = AaveV2StablecoinCellarCalls::EnterPosition(call);
            Ok(call.encode())
        }
        Rebalance(params) => {
            if params.route.len() != 9 {
                return Err(ErrorKind::SPCallError
                    .context(format!(
                        "{}: Rebalance 'route' array must contain 9 elements",
                        LOG_PREFIX
                    ))
                    .into());
            }

            let results_iter = params.route.iter().map(|addr| match addr.parse::<H160>() {
                Ok(addr) => Ok(addr),
                Err(_) => Err(addr),
            });

            // If any addresses failed to parse, return error containing the bad addresses
            let mut err_string =
                "Rebalance 'route' argument contained invalid address(s)".to_string();
            for r in results_iter.clone() {
                if let Err(addr) = r {
                    err_string.push_str(&format!(", {}", addr))
                }
            }

            if !err_string.is_empty() {
                return Err(ErrorKind::SPCallError
                    .context(format!("{}: {}", LOG_PREFIX, err_string))
                    .into());
            }

            let route = results_iter
                .map(|r| r.unwrap())
                .collect::<Vec<H160>>()
                .try_into()
                .expect("failed to convert 'route' addresses to array");

            if params.swap_params.len() != 4 {
                return Err(ErrorKind::SPCallError
                    .context(format!(
                        "{}: Rebalance 'swap_params' array must contain 4 elements",
                        LOG_PREFIX
                    ))
                    .into());
            }

            let swap_params = params
                .swap_params
                .iter()
                .map(|sp| {
                    let out: [U256; 3] =
                        [sp.in_index.into(), sp.out_index.into(), sp.swap_type.into()];
                    out
                })
                .collect::<Vec<[U256; 3]>>()
                .try_into()
                .expect("failed to convert 'swap_params' vec to array");

            let call = RebalanceCall {
                route,
                swap_params,
                min_assets_out: params.min_assets_out.into(),
            };
            Ok(AaveV2StablecoinCellarCalls::Rebalance(call).encode())
        }
        Reinvest(params) => {
            let call = ReinvestCall {
                min_assets_out: params.min_assets_out.into(),
            };
            Ok(AaveV2StablecoinCellarCalls::Reinvest(call).encode())
        }
        Sweep(params) => {
            let token = match params.token.parse::<H160>() {
                Ok(t) => t,
                Err(_) => {
                    return Err(ErrorKind::SPCallError
                        .context(format!(
                            "{}: Sweep 'token' argument is an invalid address: {}",
                            LOG_PREFIX, params.token
                        ))
                        .into())
                }
            };
            let to = match params.to.parse::<H160>() {
                Ok(t) => t,
                Err(_) => {
                    return Err(ErrorKind::SPCallError
                        .context(format!(
                            "{}: Sweep 'to' argument is an invalid address: {}",
                            LOG_PREFIX, params.to
                        ))
                        .into())
                }
            };
            let call = SweepCall { token, to };
            Ok(AaveV2StablecoinCellarCalls::Sweep(call).encode())
        }
        TransferFees(_) => {
            let call = TransferFeesCall {};
            Ok(AaveV2StablecoinCellarCalls::TransferFees(call).encode())
        }
    }
}
