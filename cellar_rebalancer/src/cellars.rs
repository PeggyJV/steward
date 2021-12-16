use std::{path, sync::Arc, convert::TryFrom, result::Result};

use abscissa_core::Application;
use ethers::prelude::*;
use signatory::FsKeyStore;
use crate::{error::{Error, ErrorKind}, gas::CellarGas, prelude::APP};

pub(crate) mod uniswapv3;

const UNISWAPV3_CELLAR: &str = "uniswapv3";

async fn get_gas_price() -> Result<U256, Error> {
    CellarGas::etherscan_standard().await.map_err(|e| e.into())
}

fn get_signing_client() -> Arc<SignerMiddleware<Provider::<Http>, LocalWallet>> {
    let config = APP.config();
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
    let client = Provider::<Http>::try_from(eth_host.clone()).unwrap();
    let client = SignerMiddleware::new(client, wallet.clone());

    Arc::new(client)
}
