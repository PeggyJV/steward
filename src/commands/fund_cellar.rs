use std::{convert::TryFrom, ops::Add, path, sync::Arc, time::Duration};

use abscissa_core::{Application, Command, Options, Runnable};
use chrono::Utc;
use ethers::prelude::*;
use num_bigint::{BigInt, ToBigInt};
use num_traits::Zero;
use signatory::FsKeyStore;

use crate::{cellar_wrapper::{CellarAddParams, CellarState, CellarTickInfo}, erc20::Erc20State, gas::CellarGas, prelude::*, uniswap_pool::PoolState};

#[derive(Command, Debug, Options)]
pub struct FundCellarCmd {
    #[options(help = "Cellar Id")]
    pub cellar_id: u32,
    #[options(help = "Amount Token 0")]
    pub amount_0: f64,
    #[options(help = "Amount Token 1")]
    pub amount_1: f64,

}

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
            for tick in ticks {
                info!("{:?}", tick);
            }


            let params = CellarAddParams::new(
                ((self.amount_0 * (10u64.pow(cellar.token_0.decimals as u32)) as f64) as u128).into(),
                ((self.amount_1 * (10u64.pow(cellar.token_1.decimals as u32)) as f64) as u128).into(),
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
