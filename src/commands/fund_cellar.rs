use std::{convert::TryFrom, ops::Add, path, sync::Arc, time::Duration};

use abscissa_core::{Application, Command, Options, Runnable};
use chrono::Utc;
use ethers::prelude::*;
use num_bigint::{BigInt, ToBigInt};
use num_traits::Zero;
use signatory::FsKeyStore;

use crate::{cellar_wrapper::{CellarAddParams, CellarState, CellarTickInfo}, erc20::Erc20State, gas::CellarGas, prelude::*, uniswap_pool::PoolState};

#[derive(Command, Debug, Options)]
pub struct FundCellarCmd {}

impl Runnable for FundCellarCmd {
    fn run(&self) {
        let config = APP.config();
        let cellar = config.cellars.get(0).expect("Could not get cellar config");

        let keystore = path::Path::new(&config.keys.keystore);
        let keystore = FsKeyStore::create_or_open(keystore).expect("Could not open keystore");

        let name = &config
            .keys
            .rebalancer_key
            .parse()
            .expect("Could not parse name");
        let key = keystore.load(name).expect("Could not load key");

        let key = key
            .to_pem()
            .parse::<k256::elliptic_curve::SecretKey<k256::Secp256k1>>()
            .expect("Could not parse key");

        let wallet: LocalWallet = Wallet::from(key);

        let eth_host = config.ethereum.rpc.clone();
        let address = wallet.address();

        abscissa_tokio::run(&APP, async {
            let client = Provider::<Http>::try_from(eth_host)
                .unwrap()
                .interval(Duration::from_secs(3000u64));


            let client = SignerMiddleware::new(client, wallet);
            let gas = CellarGas::etherscan_standard().await.unwrap();

            // MyContract expects Arc, create with client
            let client = Arc::new(client);

            let mut contract_state = CellarState::new(cellar.cellar_address, client.clone());
            contract_state.gas_price = Some(gas);
            let pool_state = PoolState::new(cellar.pool_address, client.clone());

            let mut erc20_0 = Erc20State::new(cellar.token_0.address,client.clone());
            erc20_0.gas_price=Some(gas);
            // erc20_0.approve((10000u64 * (10u64.pow(config.cellar.token_0.decimals as u32))).into(), config.cellar.cellar_address).await;
            // return;
            let mut erc20_1 = Erc20State::new(cellar.token_1.address, client.clone());
            erc20_1.gas_price = Some(gas);
            // erc20_1.approve((10u64 * (10u64.pow(config.cellar.token_1.decimals as u32))).into(), config.cellar.cellar_address).await;

            let (sqrtPriceX96, spot_tick, _, _, _, _, _) =
                pool_state.contract.slot_0().call().await.unwrap();

            info!(
                "Current sqrtPriceX96 {} tick {}",
                sqrtPriceX96.to_string(),
                spot_tick
            );

            let mut ticks = Vec::new();

            let mut i = U256::zero();
            loop {
                match contract_state.contract.cellar_tick_info(i).call().await {
                    Ok((token_id, tick_upper, tick_lower, weight)) => {
                        let tick_info = CellarTickInfo {
                            token_id: token_id,
                            tick_upper: tick_upper,
                            tick_lower: tick_lower,
                            weight: weight,
                        };
                        ticks.push(tick_info);
                        i = i.add(U256::one());
                    }
                    Err(e) => {
                        break;
                    }
                }
            }

            info!("{} ticks", ticks.len());

            let currentPrice = uniswap_v3_sdk::getSqrtRatioAtTick(spot_tick.to_bigint().unwrap());

            let mut amount_0 = BigInt::zero();
            let mut amount_1 = BigInt::zero();

            for tick in ticks {
                let Q96 = 2.to_bigint().unwrap().pow(96);
                let Q192 = Q96.pow(2);
                let priceCurrentRangeUpper =
                    uniswap_v3_sdk::getSqrtRatioAtTick(tick.tick_upper.to_bigint().unwrap());
                let priceCurrentRangeLower =
                    uniswap_v3_sdk::getSqrtRatioAtTick(tick.tick_lower.to_bigint().unwrap());

                if spot_tick <= tick.tick_lower {
                    amount_0 += tick.weight
                        * (priceCurrentRangeUpper.clone() - priceCurrentRangeLower.clone())
                        * Q192.clone()
                        / priceCurrentRangeUpper.clone()
                        / priceCurrentRangeLower.clone();
                } else if spot_tick >= tick.tick_upper {
                    amount_1 += tick.weight
                        * (priceCurrentRangeUpper.clone() - priceCurrentRangeLower.clone())
                } else {
                    amount_0 += tick.weight
                        * (priceCurrentRangeUpper.clone() - currentPrice.clone())
                        * Q192
                        / priceCurrentRangeUpper.clone()
                        / currentPrice.clone();
                    amount_1 +=
                        tick.weight * (currentPrice.clone() - priceCurrentRangeLower.clone());
                }
            }

            info!("amount_0 {} amount_1 {} ratio {}", amount_0.to_string(), amount_1.to_string(),(amount_0.clone() / amount_1.clone()).to_string());

            let params = CellarAddParams::new(
                (4000u64 * (10u64.pow(cellar.token_0.decimals as u32))).into(),
                ((1u64 * (10u64.pow(cellar.token_1.decimals as u32)))/4).into(),
                0.into(),
                0.into(),
                address,
                (Utc::now().timestamp() + 60 * 60).into(),
            );

            contract_state
                .add_liquidity_for_uni_v3(params)
                .await
                .unwrap();
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
