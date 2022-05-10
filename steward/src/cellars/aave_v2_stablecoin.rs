use crate::{
    error::Error,
    utils::{sp_call_error, string_to_u256},
};
use ethers::{
    abi::AbiEncode,
    contract::EthCall,
    prelude::{H160, U256},
};
use std::convert::TryInto;
use steward_abi::aave_v2_stablecoin::*;
use steward_proto::steward::aave_v2_stablecoin::Function::{self, *};

use super::log_cellar_call;

const CELLAR_NAME: &str = "aave_v2_stablecoin";
const LOG_PREFIX: &str = "AaveV2StablcoinCellar";

pub fn get_encoded_call(function: Function, cellar_id: String) -> Result<Vec<u8>, Error> {
    match function {
        AccrueFees(_) => {
            log_cellar_call(
                CELLAR_NAME,
                &AccrueFeesCall::function_name(),
                cellar_id.as_str(),
            );
            let call = AccrueFeesCall {};
            Ok(AaveV2StablecoinCellarCalls::AccrueFees(call).encode())
        }
        ClaimAndUnstake(_) => {
            log_cellar_call(
                CELLAR_NAME,
                &ClaimAndUnstakeCall::function_name(),
                cellar_id.as_str(),
            );
            let call = ClaimAndUnstakeCall {};
            Ok(AaveV2StablecoinCellarCalls::ClaimAndUnstake(call).encode())
        }
        EnterPosition(_) => {
            log_cellar_call(
                CELLAR_NAME,
                &EnterPositionCall::function_name(),
                cellar_id.as_str(),
            );
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

            let min_assets_out = string_to_u256(params.min_assets_out)?;

            log_cellar_call(
                CELLAR_NAME,
                &RebalanceCall::function_name(),
                cellar_id.as_str(),
            );
            let call = RebalanceCall {
                route,
                swap_params,
                min_assets_out,
            };
            Ok(AaveV2StablecoinCellarCalls::Rebalance(call).encode())
        }
        Reinvest(params) => {
            let min_assets_out = string_to_u256(params.min_assets_out)?;
            log_cellar_call(
                CELLAR_NAME,
                &ReinvestCall::function_name(),
                cellar_id.as_str(),
            );
            let call = ReinvestCall { min_assets_out };
            Ok(AaveV2StablecoinCellarCalls::Reinvest(call).encode())
        }
        SetDepositLimit(params) => {
            let limit = string_to_u256(params.limit)?;
            log_cellar_call(
                CELLAR_NAME,
                &SetDepositLimitCall::function_name(),
                cellar_id.as_str(),
            );
            let call = SetDepositLimitCall { limit };
            Ok(AaveV2StablecoinCellarCalls::SetDepositLimit(call).encode())
        }
        SetLiquidityLimit(params) => {
            let limit = string_to_u256(params.limit)?;
            log_cellar_call(
                CELLAR_NAME,
                &SetLiquidityLimitCall::function_name(),
                cellar_id.as_str(),
            );
            let call = SetLiquidityLimitCall { limit };
            Ok(AaveV2StablecoinCellarCalls::SetLiquidityLimit(call).encode())
        }
        TransferFees(_) => {
            log_cellar_call(
                CELLAR_NAME,
                &TransferFeesCall::function_name(),
                cellar_id.as_str(),
            );
            let call = TransferFeesCall {};
            Ok(AaveV2StablecoinCellarCalls::TransferFees(call).encode())
        }
    }
}

fn validate_route(results: Vec<Result<H160, &String>>) -> Result<(), Error> {
    let mut bad_addresses_string = String::new();
    for r in results {
        if let Err(addr) = r {
            bad_addresses_string.push_str(&format!(", {}", addr))
        }
    }

    if !bad_addresses_string.is_empty() {
        let mut err_string = "Rebalance 'route': array contains invalid address(s)".to_string();
        err_string.push_str(&bad_addresses_string);
        return Err(sp_call_error(format!("{}: {}", LOG_PREFIX, err_string)));
    }

    Ok(())
}
