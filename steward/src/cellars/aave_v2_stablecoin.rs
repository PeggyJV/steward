use ethers::{abi::AbiEncode, prelude::H160};
use steward_abi::aave_v2_stablecoin::*;
use steward_proto::steward::aave_v2_stablecoin::Function::{self, *};

pub fn get_encoded_call(function: Function) -> Vec<u8> {
    match function {
        EnterStrategy(_) => {
            let call = EnterStrategyCall {};
            let call = AaveV2StablecoinCellarCalls::EnterStrategy(call);
            call.encode()
        }
        ReinvestAmount(params) => {
            let call = ReinvestWithAmountCall {
                amount: params.amount.into(),
                min_assets_out: params.min_assets_out.into(),
            };
            AaveV2StablecoinCellarCalls::ReinvestWithAmount(call).encode()
        }
        Reinvest(params) => {
            let call = ReinvestCall {
                min_assets_out: params.min_assets_out.into(),
            };
            AaveV2StablecoinCellarCalls::Reinvest(call).encode()
        }
        ClaimAndUnstakeAmount(params) => {
            let call = ClaimAndUnstakeWithAmountCall {
                amount: params.amount.into(),
            };
            AaveV2StablecoinCellarCalls::ClaimAndUnstakeWithAmount(call).encode()
        }
        ClaimAndUnstake(_) => {
            let call = ClaimAndUnstakeCall {};
            AaveV2StablecoinCellarCalls::ClaimAndUnstake(call).encode()
        }
        Rebalance(params) => {
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
            let call = AccruePlatformFeesCall {};
            AaveV2StablecoinCellarCalls::AccruePlatformFees(call).encode()
        }
        TransferFees(_) => {
            let call = TransferFeesCall {};
            AaveV2StablecoinCellarCalls::TransferFees(call).encode()
        }
        SetInputToken(params) => {
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
            let call = RemoveLiquidityRestrictionCall {};
            AaveV2StablecoinCellarCalls::RemoveLiquidityRestriction(call).encode()
        }
        Sweep(params) => {
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
