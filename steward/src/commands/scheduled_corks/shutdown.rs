use crate::{application::APP, config, prelude::*, somm_send};
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use deep_space::{Coin, Contact};
use ethers::prelude::*;
use gravity_bridge::gravity_utils::ethereum::downcast_to_u64;
use signatory::FsKeyStore;
use somm_proto::cork::Cork;
use std::{convert::TryFrom, path, sync::Arc, time::Duration};
use steward_abi::aave_v2_stablecoin::AaveV2StablecoinCellar;
const MESSAGE_TIMEOUT: Duration = Duration::from_secs(10);
const CHAIN_PREFIX: &str = "somm";

/// Scheduled corks subcommand
#[derive(Command, Debug, Parser)]
pub struct Shutdown {
    cellar_id: H160,
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
            let client = SignerMiddleware::new(provider, wallet.clone().with_chain_id(chain_id));

            let address = self.cellar_id.clone();
            let contract = AaveV2StablecoinCellar::new(address, Arc::new(client.clone()));
            let call_data = match contract.shutdown().calldata() {
                Some(call) => call,
                None => return Err("Invalid Argument").expect("can't find call data"),
            };
            let encoded_call = call_data.to_vec();

            let cork = Cork {
                encoded_contract_call: encoded_call,
                target_contract_address: address.to_string(),
            };

            debug!("establishing grpc connection");
            let contact = Contact::new(&config.cosmos.grpc, MESSAGE_TIMEOUT, CHAIN_PREFIX).unwrap();

            debug!("getting cosmos fee");
            let cosmos_gas_price = config.cosmos.gas_price.as_tuple();
            let fee = Coin {
                amount: (cosmos_gas_price.0 as u64).into(),
                denom: cosmos_gas_price.1,
            };
            somm_send::schedule_cork(
                &contact,
                cork,
                config::DELEGATE_ADDRESS.to_string(),
                &config::DELEGATE_KEY,
                fee,
                self.block_height,
            )
            .await
            .expect("err");
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
