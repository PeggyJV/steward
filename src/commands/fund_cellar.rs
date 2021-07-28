use std::{convert::TryFrom, path, sync::Arc};

use abscissa_core::{Application, Command, Options, Runnable};
use ethers::prelude::*;
use signatory::FsKeyStore;

use crate::{cellar_wrapper::{CellarAddParams, CellarState}, prelude::*};


#[derive(Command, Debug, Options)]
pub struct FundCellarCmd{

}

impl Runnable for FundCellarCmd{
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

        abscissa_tokio::run(&APP, async {

            let client = Provider::<Http>::try_from(eth_host).unwrap();
            let client = SignerMiddleware::new(client, wallet);

            // MyContract expects Arc, create with client
            let client = Arc::new(client);
            let contract_state =CellarState::new(config.cellar.cellar_addresses, client);

            // contract_state.add_liquidity_eth_for_uni_v3(CellarAddParams::new(
            //     amount0_desired: (),
            //     amount1_desired: (),
            //     amount0_min: (),
            //     amount1_min: (),
            //     recipient: (),
            //     deadline: (),
            // ))

        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}