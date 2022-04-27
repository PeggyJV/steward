use std::convert::TryInto;

use ethers::{
    abi::AbiEncode,
    prelude::{H160, U256},
};
use steward_abi::aave_v2_stablecoin::*;
use steward_proto::steward::aave_v2_stablecoin::{
    rebalance::SwapParams,
    Function::{self, *},
};

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
            // We expect the client to pad the route to length 9
            if params.route.len() != 9 {
                return Err(sp_call_error(format!(
                    "{}: Rebalance 'route': array must contain 9 elements",
                    LOG_PREFIX
                )));
            }

            if params.swap_params.len() != 4 {
                return Err(sp_call_error(format!(
                    "{}: Rebalance 'swap_params': array must contain 4 elements",
                    LOG_PREFIX
                )));
            }

            let results: Vec<Result<H160, &String>> = params
                .route
                .iter()
                .map(|addr| match addr.parse::<H160>() {
                    Ok(addr) => Ok(addr),
                    Err(_) => Err(addr),
                })
                .collect();

            validate_route(results.clone())?;
            validate_swap_params(params.swap_params.clone())?;

            let route = results
                .iter()
                .map(|r| r.unwrap())
                .collect::<Vec<H160>>()
                .try_into()
                .expect("failed to convert 'route' addresses to array");

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
                    return Err(sp_call_error(format!(
                        "{}: Sweep 'token': argument is an invalid address: {}",
                        LOG_PREFIX, params.token
                    )))
                }
            };
            let to = match params.to.parse::<H160>() {
                Ok(t) => t,
                Err(_) => {
                    return Err(sp_call_error(format!(
                        "{}: Sweep 'to': argument is an invalid address: {}",
                        LOG_PREFIX, params.to
                    )))
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

fn validate_route(results: Vec<Result<H160, &String>>) -> Result<(), Error> {
    let mut bad_addresses = String::new();
    for r in results {
        if let Err(addr) = r {
            bad_addresses.push_str(&format!(", {}", addr))
        }
    }

    if !bad_addresses.is_empty() {
        let mut err_string = "Rebalance 'route': array contains invalid address(s)".to_string();
        err_string.push_str(&bad_addresses);
        return Err(sp_call_error(format!("{}: {}", LOG_PREFIX, err_string)));
    }

    Ok(())
}

fn validate_swap_params(swap_params: Vec<SwapParams>) -> Result<(), Error> {
    for sp in swap_params.iter() {
        if sp.in_index == 0u64 && sp.out_index == 0u64 && sp.swap_type == 0u64 {
            continue;
        }

        if sp.out_index > 8u64 {
            return Err(sp_call_error(format!(
                "{}: Rebalance 'swap_params': out_index can't be greater than 8 (the last index of 'route'). {:?}.",
                LOG_PREFIX,
                sp
            )));
        }

        if sp.in_index % 2 != 0u64 || sp.out_index % 2 != 0u64 {
            return Err(sp_call_error(format!(
                "{}: Rebalance 'swap_params': token indeces must be even. {:?}.",
                LOG_PREFIX, sp
            )));
        }

        let diff = sp.out_index.checked_sub(sp.in_index);
        if diff.is_none() || diff.unwrap() == 0u64 {
            return Err(sp_call_error(format!(
                "{}: Rebalance 'swap_params': in_index must be less than out_index. {:?}.",
                LOG_PREFIX, sp
            )));
        }

        let diff = diff.unwrap();
        if diff != 2 {
            return Err(sp_call_error(format!(
                "{}: Rebalance 'swap_params': in_index and out_index must surround a pool address (must 2 apart). {:?}.",
                LOG_PREFIX,
                sp
            )));
        }
    }

    Ok(())
}

fn sp_call_error(message: String) -> Error {
    ErrorKind::SPCallError.context(message).into()
}
