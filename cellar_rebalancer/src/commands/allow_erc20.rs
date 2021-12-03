use std::{convert::TryFrom, path, sync::Arc, time::Duration};

use abscissa_core::{Application, Clap, Command, Runnable};

use ethers::prelude::*;

use signatory::FsKeyStore;

use crate::{erc20::Erc20State, gas::CellarGas, prelude::*};

/// Allow Erc20 Token to interact with cellar contract
#[derive(Command, Debug, Default, Clap)]
pub struct AllowERC20 {
    #[clap(short = 'C', long)]
    cellar_address: H160,
    #[clap(short = 'A', long)]
    address: H160,
    #[clap(short = 'a', long)]
    amount: u64,
}

impl Runnable for AllowERC20 {
    fn run(&self) {
        let config = APP.config();
        let _cellar = config.cellars.get(0).expect("Could not get cellar config");

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

        abscissa_tokio::run(&APP, async {
            let client = Provider::<Http>::try_from(eth_host)
                .unwrap()
                .interval(Duration::from_secs(3000u64));

            let client = SignerMiddleware::new(client, wallet);
            let gas = CellarGas::etherscan_standard().await.unwrap();

            // MyContract expects Arc, create with client
            let client = Arc::new(client);

            let mut erc20_0 = Erc20State::new(self.address, client.clone());
            let decimals = erc20_0.contract.decimals().call().await.unwrap();
            erc20_0.gas_price = Some(gas);

            erc20_0
                .approve(
                    (self.amount * (10u64.pow(decimals as u32))).into(),
                    self.cellar_address,
                )
                .await;
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
