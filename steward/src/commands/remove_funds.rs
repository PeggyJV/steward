use std::{convert::TryFrom, path, sync::Arc, time::Duration};

use abscissa_core::{clap::Parser, Application, Command, Runnable};
use chrono::Utc;
use ethers::prelude::*;
use signatory::FsKeyStore;

use crate::{allocation, cellars::uniswapv3::UniswapV3CellarState, prelude::*};
use steward_abi::cellar_uniswap::*;

/// Remove funds from Cellars
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Remove funds from Cellar.\n This command remove funds from the required Cellar by loading the Cellar config."
)]
pub struct RemoveFundsCmd {
    /// Cellar Address
    cellar_address: H160,
}

impl Runnable for RemoveFundsCmd {
    fn run(&self) {
        let config = APP.config();
        let cellar = allocation::get_cellar(self.cellar_address).unwrap();

        let keystore = path::Path::new(&config.keystore);
        let keystore = FsKeyStore::create_or_open(keystore).expect("Could not open keystore");

        let name = &config
            .keys
            .delegate_key
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

            // MyContract expects Arc, create with client
            let client = Arc::new(client);
            let mut contract_state =
                UniswapV3CellarState::new(cellar.cellar_address, client.clone());

            let balance = contract_state
                .contract
                .balance_of(address)
                .call()
                .await
                .unwrap();
            dbg!(balance.to_string());

            let params = CellarRemoveParams {
                token_amount: balance,
                amount_0_min: U256::zero(),
                amount_1_min: U256::zero(),
                recipient: address,
                deadline: (Utc::now().timestamp() + 60 * 60).into(),
            };

            contract_state
                .remove_liquidity_from_uni_v3(params)
                .await
                .unwrap();

            // let params = CellarAddParams::new(
            //     0.into(),
            //     (7000u64 * (10u64.pow(config.cellar.token_1.decimals as u32))).into(),
            //     0.into(),
            //     0.into(),
            //     address,
            //     (Utc::now().timestamp() + 60 * 60).into(),
            // );

            // contract_state
            //     .add_liquidity_for_uni_v3(params)
            //     .await
            //     .unwrap();
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
