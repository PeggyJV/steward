use std::{convert::TryFrom, ops::Add, path, sync::Arc};

use abscissa_core::{Application, Command, Options, Runnable};
use ethers::prelude::*;
use num_bigint::ToBigInt;
use signatory::FsKeyStore;

use crate::{
    cellar_wrapper::{CellarAddParams, CellarState, CellarTickInfo},
    prelude::*,
    uniswap_pool::PoolState,
};

#[derive(Command, Debug, Options)]
pub struct FundCellarCmd {}

impl Runnable for FundCellarCmd {
    fn run(&self) {
        let config = APP.config();

        let keystore = path::Path::new(&config.key.keystore);
        let keystore = FsKeyStore::create_or_open(keystore).expect("Could not open keystore");

        let name = &config
            .key
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
            let client = Provider::<Http>::try_from(eth_host).unwrap();
            let client = SignerMiddleware::new(client, wallet);

            // MyContract expects Arc, create with client
            let client = Arc::new(client);
            let mut contract_state = CellarState::new(config.cellar.cellar_address, client.clone());
            let pool_state = PoolState::new(config.cellar.pool_address, client);

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

            let mut weight_below_spot = 0u32;
            let mut weight_above_spot = 0u32;
            let mut spot_weight = 0u32;
            let mut total_weight = 0u32;
            let mut spot_tick_info = CellarTickInfo::new(U256::zero(), 0, 0,0);

            for tick in ticks {
                if tick > spot_tick {
                    weight_below_spot += tick.weight;
                } else if tick < spot_tick {
                    weight_above_spot += tick.weight;
                } else if tick == spot_tick {
                    spot_tick_info = tick.clone();
                    spot_weight += tick.weight;
                }
                total_weight += tick.weight;
            }

            info!("Weight below spot:{} Weight above spot: {} Spot Weight:{}",weight_above_spot as f64 / total_weight as f64 , weight_below_spot as f64 / total_weight as f64, spot_weight as f64 / total_weight as f64); 

            // let sqrtRatioAX96 = uniswap_v3_sdk::getSqrtRatioAtTick(spot_tick_info.tick_lower.to_bigint().unwrap());
            // let sqrtRatioBX96 = uniswap_v3_sdk::getSqrtRatioAtTick(spot_tick_info.tick_upper.to_bigint().unwrap());

            // let liquidity =uniswap_v3_sdk::maxLiquidityForAmounts(sqrtPriceX96.to_string().parse().unwrap(), sqrtRatioAX96, sqrtRatioBX96, 100.to_bigint().unwrap(),100.to_bigint().unwrap());
            
            let params = CellarAddParams::new(0.into(), (1000 * (config.cellar.token_1.decimals as u32)).into(), 0.into(), 0.into(), address, 0.into());
            

            contract_state.add_liquidity_eth_for_uni_v3(params).await.unwrap();

        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
