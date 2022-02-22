#![allow(unused_variables)]
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use chrono::Utc;
use ethers::prelude::*;
use signatory::FsKeyStore;
use std::{convert::TryFrom, ops::Add, path, sync::Arc, time::Duration};

use crate::{
    allocation,
    cellars::uniswapv3::{UniswapV3CellarState, UniswapV3CellarTickInfo},
    gas::CellarGas,
    prelude::*,
    uniswap_pool::PoolState,
};
use steward_abi::cellar_uniswap::*;

/// Command to fund Cellars
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Fund Cellar.\n This command funds the required Cellar by loading the Cellar config. \n It takes the amounts for the Cellar pool pair."
)]
pub struct FundCellarCmd {
    /// Amount for first pool pair
    #[clap(short = 'm', long)]
    pub amount_0: f64,
    /// Amount for second pool pair
    #[clap(short = 'o', long)]
    pub amount_1: f64,
    /// Cellar Address.
    #[clap(short = 'i', long)]
    pub cellar_address: H160,
}

impl Runnable for FundCellarCmd {
    fn run(&self) {
        let config = APP.config();
        let cellar = allocation::get_cellar(self.cellar_address).unwrap();
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

            let mut contract_state =
                UniswapV3CellarState::new(cellar.cellar_address, client.clone());
            contract_state.gas_price = Some(gas);
            let pool_state = PoolState::new(cellar.pool_address, client.clone());

            let (sqrt_price_x96, spot_tick, _, _, _, _, _) =
                pool_state.contract.slot_0().call().await.unwrap();

            info!(
                "Current sqrtPriceX96 {} tick {}",
                sqrt_price_x96.to_string(),
                spot_tick
            );

            let mut ticks = Vec::new();

            let mut i = U256::zero();
            while let Ok((pair_id, tick_upper, tick_lower, weight)) =
                contract_state.contract.cellar_tick_info(i).call().await
            {
                match contract_state.contract.cellar_tick_info(i).call().await {
                    Ok((pair_id, tick_upper, tick_lower, weight)) => {
                        let tick_info = UniswapV3CellarTickInfo {
                            tick_upper,
                            tick_lower,
                            weight,
                        };
                        ticks.push(tick_info);
                        i = i.add(U256::one());
                    }
                    Err(_e) => {
                        break;
                    }
                }
            }

            info!("{} ticks", ticks.len());
            for tick in ticks {
                info!("{:?}", tick);
            }

            let params = CellarAddParams {
                amount_0_desired: ((self.amount_0
                    * (10u64.pow(cellar.token_0.decimals as u32)) as f64)
                    as u128)
                    .into(),
                amount_1_desired: ((self.amount_1
                    * (10u64.pow(cellar.token_1.decimals as u32)) as f64)
                    as u128)
                    .into(),
                amount_0_min: 0.into(),
                amount_1_min: 0.into(),
                recipient: address,
                deadline: (Utc::now().timestamp() + 60 * 60).into(),
            };

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
