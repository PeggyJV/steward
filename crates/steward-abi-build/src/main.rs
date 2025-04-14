use ethers::contract::Abigen;
use std::process;

fn main() {
    generate_contract_abis();
}

fn generate_contract_abis() {
    // (JSON/Contract name, output file name)
    let contracts = vec![
        ("AaveV2StablecoinCellar", "aave_v2_stablecoin"),
        ("CellarV1", "cellar_v1"),
        ("CellarV2", "cellar_v2"),
        ("CellarV2_2", "cellar_v2_2"),
        ("UniswapV3AdaptorV1", "uniswap_v3_adaptor_v1"),
        ("UniswapV3AdaptorV2", "uniswap_v3_adaptor_v2"),
        ("AaveATokenAdaptorV1", "aave_a_token_adaptor_v1"),
        ("AaveDebtTokenAdaptorV1", "aave_debt_token_adaptor_v1"),
        ("AaveATokenAdaptorV2", "aave_a_token_adaptor_v2"),
        ("AaveDebtTokenAdaptorV2", "aave_debt_token_adaptor_v2"),
        ("AaveV3ATokenAdaptorV1", "aave_v3_a_token_adaptor_v1"),
        ("AaveV3DebtTokenAdaptorV1", "aave_v3_debt_token_adaptor_v1"),
        ("CellarAdaptorV1", "cellar_adaptor_v1"),
        ("CompoundCTokenAdaptorV2", "compound_c_token_adaptor_v2"),
        ("OneInchAdaptorV1", "oneinch_adaptor_v1"),
        ("ZeroXAdaptorV1", "zero_x_adaptor_v1"),
        ("SwapWithUniswapAdaptorV1", "swap_with_uniswap_adaptor_v1"),
        ("FeesAndReservesAdaptorV1", "fees_and_reserves_adaptor_v1"),
        ("VestingSimpleAdaptorV2", "vesting_simple_adaptor_v2"),
        (
            "AaveV2EnableAssetAsCollateralAdaptorV1",
            "aave_v2_enable_asset_as_collateral_adaptor_v1",
        ),
        ("FTokenAdaptor", "f_token_adaptor"),
        (
            "MorphoAaveV2ATokenAdaptorV1",
            "morpho_aave_v2_a_token_adaptor_v1",
        ),
        (
            "MorphoAaveV2DebtTokenAdaptorV1",
            "morpho_aave_v2_debt_token_adaptor_v1",
        ),
        (
            "MorphoAaveV3ATokenCollateralAdaptorV1",
            "morpho_aave_v3_a_token_collateral_adaptor_v1",
        ),
        (
            "MorphoAaveV3ATokenP2PAdaptorV1",
            "morpho_aave_v3_a_token_p2p_adaptor_v1",
        ),
        (
            "MorphoAaveV3DebtTokenAdaptorV1",
            "morpho_aave_v3_debt_token_adaptor_v1",
        ),
        ("MorphoRewardHandler", "morpho_reward_handler"),
        ("BalancerPoolAdaptorV1", "balancer_pool_adaptor_v1"),
        ("CellarV2_5", "cellar_v2_5"),
        (
            "CellarWithShareLockPeriodV1",
            "cellar_with_share_lock_period_v1",
        ),
        ("DebtFTokenAdaptorV1", "debt_f_token_adaptor_v1"),
        ("CollateralFTokenAdaptorV1", "collateral_f_token_adaptor_v1"),
        ("LegacyCellarAdaptorV1", "legacy_cellar_adaptor_v1"),
        ("CurveAdaptorV1", "curve_adaptor_v1"),
        ("ConvexCurveAdaptorV1", "convex_curve_adaptor_v1"),
        ("AuraERC4626AdaptorV1", "aura_erc4626_adaptor_v1"),
        (
            "CellarWithMultiAssetDepositV1",
            "cellar_with_multi_asset_deposit_v1",
        ),
        ("MorphoBlueSupplyAdaptorV1", "morpho_blue_supply_adaptor_v1"),
        (
            "MorphoBlueCollateralAdaptorV1",
            "morpho_blue_collateral_adaptor_v1",
        ),
        ("MorphoBlueDebtAdaptorV1", "morpho_blue_debt_adaptor_v1"),
        ("ERC4626AdaptorV1", "erc4626_adaptor_v1"),
        ("StakingAdaptorV1", "staking_adaptor_v1"),
        ("PendleAdaptorV1", "pendle_adaptor_v1"),
    ];

    contracts.iter().for_each(|n| {
        let name = n.0;
        let file_name = n.1;
        let abigen = match Abigen::new(name, format!("../steward-abi/abi/{}.json", name)) {
            Ok(abigen) => abigen,
            Err(e) => {
                println!("Could not open {}.json: {}", name, e);
                process::exit(1);
            }
        };

        let abi = match abigen
            .add_derive("serde::Deserialize")
            .expect("Failed to add event derive")
            .add_derive("serde::Serialize")
            .expect("Failed to add event derive")
            .generate()
        {
            Ok(abi) => abi,
            Err(e) => {
                println!("Could not generate abi from {}.json: {}", name, e);
                process::exit(1);
            }
        };

        match abi.write_to_file(format!("../../crates/steward-abi/src/{}.rs", file_name)) {
            Ok(_) => (),
            Err(e) => println!("Error writing {}.rs: {}", file_name, e),
        }
    })
}
