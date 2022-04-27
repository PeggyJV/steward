use ethers::{abi::AbiEncode, prelude::H160};
use steward_abi::aave_v2_stablecoin::*;
use steward_proto::steward::aave_v2_stablecoin::Function::{self, *};

use super::log_cellar_call;

const CELLAR_NAME: &str = "aave_v2_stablecoin";

pub fn get_encoded_call(function: Function, cellar_id: String) -> Vec<u8> {
    match function {
        EnterStrategy(_) => {
            log_cellar_call(CELLAR_NAME, "enterStrategy", cellar_id);
            let call = EnterStrategyCall {};
            let call = AaveV2StablecoinCellarCalls::EnterStrategy(call);
            call.encode()
        }
        ReinvestAmount(params) => {
            log_cellar_call(CELLAR_NAME, "reinvestAmount", cellar_id);
            let call = ReinvestWithAmountCall {
                amount: params.amount.into(),
                min_assets_out: params.min_assets_out.into(),
            };
            AaveV2StablecoinCellarCalls::ReinvestWithAmount(call).encode()
        }
        Reinvest(params) => {
            log_cellar_call(CELLAR_NAME, "reinvest", cellar_id);
            let call = ReinvestCall {
                min_assets_out: params.min_assets_out.into(),
            };
            AaveV2StablecoinCellarCalls::Reinvest(call).encode()
        }
        ClaimAndUnstakeAmount(params) => {
            log_cellar_call(CELLAR_NAME, "claimAndUnstakeAmount", cellar_id);
            let call = ClaimAndUnstakeWithAmountCall {
                amount: params.amount.into(),
            };
            AaveV2StablecoinCellarCalls::ClaimAndUnstakeWithAmount(call).encode()
        }
        ClaimAndUnstake(_) => {
            log_cellar_call(CELLAR_NAME, "claimAndUnstake", cellar_id);
            let call = ClaimAndUnstakeCall {};
            AaveV2StablecoinCellarCalls::ClaimAndUnstake(call).encode()
        }
        Rebalance(params) => {
            log_cellar_call(CELLAR_NAME, "rebalance", cellar_id);
            let call = RebalanceCall {
                new_lending_token: params
                    .address
                    .parse::<H160>()
                    .expect("failed to parse token address"),
                min_new_lending_token_amount: params.min_new_lending_token_amount.into(),
            };
            AaveV2StablecoinCellarCalls::Rebalance(call).encode()
        }
        AccruePlatformFees(_) => {
            log_cellar_call(CELLAR_NAME, "accruePlatformFees", cellar_id);
            let call = AccruePlatformFeesCall {};
            AaveV2StablecoinCellarCalls::AccruePlatformFees(call).encode()
        }
        TransferFees(_) => {
            log_cellar_call(CELLAR_NAME, "transferFees", cellar_id);
            let call = TransferFeesCall {};
            AaveV2StablecoinCellarCalls::TransferFees(call).encode()
        }
        SetInputToken(params) => {
            log_cellar_call(CELLAR_NAME, "setInputToken", cellar_id);
            let call = SetInputTokenCall {
                token: params
                    .address
                    .parse::<H160>()
                    .expect("failed to parse token address"),
                is_approved: params.is_approved,
            };
            AaveV2StablecoinCellarCalls::SetInputToken(call).encode()
        }
        RemoveLiquidityRestriction(_) => {
            log_cellar_call(CELLAR_NAME, "removeLiquidityRestriction", cellar_id);
            let call = RemoveLiquidityRestrictionCall {};
            AaveV2StablecoinCellarCalls::RemoveLiquidityRestriction(call).encode()
        }
        Sweep(params) => {
            log_cellar_call(CELLAR_NAME, "sweep", cellar_id);
            let call = SweepCall {
                token: params
                    .address
                    .parse::<H160>()
                    .expect("failed to parse token address"),
            };
            AaveV2StablecoinCellarCalls::Sweep(call).encode()
        }
    }
}
