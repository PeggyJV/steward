use crate::{application::APP, prelude::*};
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use ethers::prelude::*;
use gravity_bridge::gravity_utils::ethereum::downcast_to_u64;
use signatory::FsKeyStore;
use std::{convert::TryFrom, path};

/// Scheduled corks subcommand
#[derive(Command, Debug, Parser)]
pub struct Shutdown {
    cellar_id: String,
    block_height: u64,
}

impl Runnable for Shutdown {
    fn run(&self) {
        let config = APP.config();

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

        abscissa_tokio::run_with_actix(&APP, async {

            let eth_host = config.ethereum.rpc.clone();
            let provider = Provider::<Http>::try_from(eth_host.clone())
                .unwrap_or_else(|_| panic!("could not get provider from eth_host {}", eth_host));
            let chain_id = provider
                .get_chainid()
                .await
                .expect("Could not retrieve chain ID");
    
            let chain_id =
                downcast_to_u64(chain_id).expect("Chain ID overflowed when downcasting to u64");
            let _client = SignerMiddleware::new(provider, wallet.clone().with_chain_id(chain_id));
    
            let _address = self.cellar_id.clone();
        }).unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
