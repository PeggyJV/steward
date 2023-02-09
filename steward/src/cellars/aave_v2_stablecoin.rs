use crate::{
    error::Error,
    utils::{
        encode_fees_distributor_address, governance_call_error, sp_call_error, string_to_u256,
    },
};
use deep_space::Address as CosmosAddress;
use ethers::{
    abi::AbiEncode,
    contract::EthCall,
    prelude::{Address as EthereumAddress, U256},
};
use std::convert::TryInto;
use steward_abi::aave_v2_stablecoin::*;
use steward_proto::steward::{
    aave_v2_stablecoin::Function as StrategyFunction,
    aave_v2_stablecoin_governance::Function as GovernanceFunction,
};
use GovernanceFunction::*;
use StrategyFunction::*;

use super::{log_cellar_call, log_governance_cellar_call};

const CELLAR_NAME: &str = "aave_v2_stablecoin";
const LOG_PREFIX: &str = "AaveV2StablcoinCellar";

pub fn get_encoded_call(function: StrategyFunction, cellar_id: String) -> Result<Vec<u8>, Error> {
    get_call(function, cellar_id).map(|call| call.encode())
}

pub fn get_call(
    function: StrategyFunction,
    cellar_id: String,
) -> Result<AaveV2StablecoinCellarCalls, Error> {
    match function {
        Accrue(_) => {
            log_cellar_call(
                CELLAR_NAME,
                &AccrueCall::function_name(),
                cellar_id.as_str(),
            );
            let call = AccrueCall {};
            Ok(AaveV2StablecoinCellarCalls::Accrue(call))
        }
        ClaimAndUnstake(_) => {
            log_cellar_call(
                CELLAR_NAME,
                &ClaimAndUnstakeCall::function_name(),
                cellar_id.as_str(),
            );
            let call = ClaimAndUnstakeCall {};
            Ok(AaveV2StablecoinCellarCalls::ClaimAndUnstake(call))
        }
        EnterPosition(_) => {
            log_cellar_call(
                CELLAR_NAME,
                &EnterPositionCall::function_name(),
                cellar_id.as_str(),
            );
            let call = EnterPositionCall {};
            Ok(AaveV2StablecoinCellarCalls::EnterPosition(call))
        }
        EnterPositionWithAssets(params) => {
            let assets = string_to_u256(params.assets)?;
            log_cellar_call(
                CELLAR_NAME,
                &EnterPositionWithAssetsCall::function_name(),
                cellar_id.as_str(),
            );
            let call = EnterPositionWithAssetsCall { assets };
            Ok(AaveV2StablecoinCellarCalls::EnterPositionWithAssets(call))
        }
        ExitPosition(_) => {
            log_cellar_call(
                CELLAR_NAME,
                &ExitPositionCall::function_name(),
                cellar_id.as_str(),
            );
            let call = ExitPositionCall {};
            Ok(AaveV2StablecoinCellarCalls::ExitPosition(call))
        }
        ExitPositionWithAssets(params) => {
            let assets = string_to_u256(params.assets)?;
            log_cellar_call(
                CELLAR_NAME,
                &ExitPositionWithAssetsCall::function_name(),
                cellar_id.as_str(),
            );
            let call = ExitPositionWithAssetsCall { assets };
            Ok(AaveV2StablecoinCellarCalls::ExitPositionWithAssets(call))
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

            let results: Vec<Result<EthereumAddress, &String>> = params
                .route
                .iter()
                .map(|addr| match addr.parse::<EthereumAddress>() {
                    Ok(addr) => Ok(addr),
                    Err(_) => Err(addr),
                })
                .collect();

            validate_route(results.clone())?;

            let route = results
                .iter()
                .map(|r| r.unwrap())
                .collect::<Vec<EthereumAddress>>()
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
            Ok(AaveV2StablecoinCellarCalls::Rebalance(call))
        }
        Reinvest(params) => {
            let min_assets_out = string_to_u256(params.min_assets_out)?;
            log_cellar_call(
                CELLAR_NAME,
                &ReinvestCall::function_name(),
                cellar_id.as_str(),
            );
            let call = ReinvestCall { min_assets_out };
            Ok(AaveV2StablecoinCellarCalls::Reinvest(call))
        }
        SetAccrualPeriod(params) => {
            let new_accrual_period = params.new_accrual_period;
            log_cellar_call(
                CELLAR_NAME,
                &SetAccrualPeriodCall::function_name(),
                cellar_id.as_str(),
            );
            let call = SetAccrualPeriodCall { new_accrual_period };
            Ok(AaveV2StablecoinCellarCalls::SetAccrualPeriod(call))
        }
        SetDepositLimit(params) => {
            let new_limit = string_to_u256(params.limit)?;
            log_cellar_call(
                CELLAR_NAME,
                &SetDepositLimitCall::function_name(),
                cellar_id.as_str(),
            );
            let call = SetDepositLimitCall { new_limit };
            Ok(AaveV2StablecoinCellarCalls::SetDepositLimit(call))
        }
        SetLiquidityLimit(params) => {
            let new_limit = string_to_u256(params.limit)?;
            log_cellar_call(
                CELLAR_NAME,
                &SetLiquidityLimitCall::function_name(),
                cellar_id.as_str(),
            );
            let call = SetLiquidityLimitCall { new_limit };
            Ok(AaveV2StablecoinCellarCalls::SetLiquidityLimit(call))
        }
        SendFees(_) => {
            log_cellar_call(
                CELLAR_NAME,
                &SendFeesCall::function_name(),
                cellar_id.as_str(),
            );
            let call = SendFeesCall {};
            Ok(AaveV2StablecoinCellarCalls::SendFees(call))
        }
    }
}

fn validate_route(results: Vec<Result<EthereumAddress, &String>>) -> Result<(), Error> {
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

pub fn get_encoded_governance_call(
    function: GovernanceFunction,
    cellar_id: &str,
    proposal_id: u64,
) -> Result<Vec<u8>, Error> {
    match function {
        SetFeesDistributor(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &SetFeesDistributorCall::function_name(),
                cellar_id,
            );
            let address: CosmosAddress = params.new_fees_distributor.parse().map_err(|err| {
                governance_call_error(format!(
                    "{}: SetFeesDistibutor invalid address: {}",
                    LOG_PREFIX, err
                ))
            })?;
            let call = SetFeesDistributorCall {
                new_fees_distributor: encode_fees_distributor_address(address),
            };
            Ok(AaveV2StablecoinCellarCalls::SetFeesDistributor(call).encode())
        }
        InitiateShutdown(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &InitiateShutdownCall::function_name(),
                cellar_id,
            );
            let call = InitiateShutdownCall {
                empty_position: params.empty_position,
            };
            Ok(AaveV2StablecoinCellarCalls::InitiateShutdown(call).encode())
        }
        LiftShutdown(_) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &LiftShutdownCall::function_name(),
                cellar_id,
            );
            let call = LiftShutdownCall {};
            Ok(AaveV2StablecoinCellarCalls::LiftShutdown(call).encode())
        }
        SetTrust(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &SetTrustCall::function_name(),
                cellar_id,
            );
            let call = SetTrustCall {
                trust: params.trust,
                position: params.position.parse::<EthereumAddress>().map_err(|err| {
                    governance_call_error(format!(
                        "{}: SetTrust: invalid address: {}",
                        LOG_PREFIX, err
                    ))
                })?,
            };
            Ok(AaveV2StablecoinCellarCalls::SetTrust(call).encode())
        }
        Sweep(params) => {
            log_governance_cellar_call(
                proposal_id,
                CELLAR_NAME,
                &SweepCall::function_name(),
                cellar_id,
            );
            let call = SweepCall {
                token: params.token.parse::<EthereumAddress>().map_err(|err| {
                    governance_call_error(format!(
                        "{}: Sweep: invalid token address: {}",
                        LOG_PREFIX, err
                    ))
                })?,
                to: params.recipient.parse::<EthereumAddress>().map_err(|err| {
                    governance_call_error(format!(
                        "{}: Sweep: invalid recipient address: {}",
                        LOG_PREFIX, err
                    ))
                })?,
            };
            Ok(AaveV2StablecoinCellarCalls::Sweep(call).encode())
        }
    }
}
