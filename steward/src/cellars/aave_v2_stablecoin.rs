use abscissa_core::tracing::info;
use ethers::{abi::AbiEncode, prelude::H160};
use steward_abi::aave_v2_stablecoin::*;
use steward_proto::steward::aave_v2_stablecoin::Function::{self, *};

pub fn get_encoded_call(function: Function, cellar_id: String) -> Vec<u8> {
    match function {
        EnterStrategy(_) => {
            info!(
                "encoding aave_v2_stablecoin.enterStrategy call for cellar {}",
                cellar_id
            );
            let call = EnterStrategyCall {};
            let call = AaveV2StablecoinCellarCalls::EnterStrategy(call);
            call.encode()
        }
        ReinvestAmount(params) => {
            info!(
                "encoding aave_v2_stablecoin.reinvestAmount call for cellar {}",
                cellar_id
            );
            let call = ReinvestWithAmountCall {
                amount: params.amount.into(),
                min_assets_out: params.min_assets_out.into(),
            };
            AaveV2StablecoinCellarCalls::ReinvestWithAmount(call).encode()
        }
        Reinvest(params) => {
            info!(
                "encoding aave_v2_stablecoin.reinvest call for cellar {}",
                cellar_id
            );
            let call = ReinvestCall {
                min_assets_out: params.min_assets_out.into(),
            };
            AaveV2StablecoinCellarCalls::Reinvest(call).encode()
        }
        ClaimAndUnstakeAmount(params) => {
            info!(
                "encoding aave_v2_stablecoin.claimAndUnstakeAmount call for cellar {}",
                cellar_id
            );
            let call = ClaimAndUnstakeWithAmountCall {
                amount: params.amount.into(),
            };
            AaveV2StablecoinCellarCalls::ClaimAndUnstakeWithAmount(call).encode()
        }
        ClaimAndUnstake(_) => {
            info!(
                "encoding aave_v2_stablecoin.claimAndUnstake call for cellar {}",
                cellar_id
            );
            let call = ClaimAndUnstakeCall {};
            AaveV2StablecoinCellarCalls::ClaimAndUnstake(call).encode()
        }
        Rebalance(params) => {
            info!(
                "encoding aave_v2_stablecoin.rebalance call for cellar {}",
                cellar_id
            );
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
            info!(
                "encoding aave_v2_stablecoin.accruePlatformFees call for cellar {}",
                cellar_id
            );
            let call = AccruePlatformFeesCall {};
            AaveV2StablecoinCellarCalls::AccruePlatformFees(call).encode()
        }
        TransferFees(_) => {
            info!(
                "encoding aave_v2_stablecoin.transferFees call for cellar {}",
                cellar_id
            );
            let call = TransferFeesCall {};
            AaveV2StablecoinCellarCalls::TransferFees(call).encode()
        }
        SetInputToken(params) => {
            info!(
                "encoding aave_v2_stablecoin.setInputToken call for cellar {}",
                cellar_id
            );
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
            info!(
                "encoding aave_v2_stablecoin.removeLiquidityRestriction call for cellar {}",
                cellar_id
            );
            let call = RemoveLiquidityRestrictionCall {};
            AaveV2StablecoinCellarCalls::RemoveLiquidityRestriction(call).encode()
        }
        Sweep(params) => {
            info!(
                "encoding aave_v2_stablecoin.sweep call for cellar {}",
                cellar_id
            );
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
