//! Start subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;
use abscissa_core::error::BoxError;
use futures::future;
use signatory::FsKeyStore;

use crate::application::APP;
use ethers::{
    prelude::*,
    providers::{Http, Provider},
};

use crate::{
    collector::{Collector, Poller, Request, Response},
    config::CellarRebalancerConfig,
};
use abscissa_core::{config, Command, FrameworkError, Clap, Runnable};
use std::{convert::TryFrom, path, sync::Arc};
use tokio::task::JoinHandle;
use tower::{Service, ServiceBuilder};

/// `start` subcommand
///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>
#[derive(Command, Debug, Clap)]
pub struct SingleSignerCmd {
    /// To whom are we saying hello?
    recipient: Vec<String>,
}

impl SingleSignerCmd {
    /// Initialize collector poller (if configured/needed)
    async fn build_pollers(
        &self,
        config: CellarRebalancerConfig,
    ) -> Vec<Poller<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>> {
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

        let mut pollers = vec![];

        for cellar in config.cellars.clone().into_iter() {
            let client = Provider::<Http>::try_from(eth_host.clone()).unwrap();
            let client = SignerMiddleware::new(client, wallet.clone());
            let client = Arc::new(client);
            let mongo = config.mongo.clone();

            let name = &config
            .keys
            .rebalancer_key;
            let cosmos_key = config.load_deep_space_key(name.clone());

            let poller = Poller::new(&cellar, client, &mongo, &cosmos_key, config.clone())
                .await
                .unwrap_or_else(|e| {
                    status_err!("couldn't initialize poller: {}", e);
                    std::process::exit(1);
                });

            pollers.push(poller);
        }

        pollers
    }
}

impl Runnable for SingleSignerCmd {
    /// Start the application.
    fn run(&self) {
        info!("Starting application");

        let config = APP.config();
        abscissa_tokio::run(&APP, async {
            let collector =
                ServiceBuilder::new()
                    .buffer(20)
                    .service(Collector::new(&config).unwrap_or_else(|e| {
                        status_err!("couldn't initialize collector service: {}", e);
                        std::process::exit(1);
                    }));

            let pollers = self.build_pollers(config.as_ref().clone()).await;

            let mut tasks = vec![];
            for poller in pollers {
                let collector = collector.clone();
                let task = tokio::spawn(async move { poller.run(collector).await });
                tasks.push(task);
            }
            future::join_all(tasks).await;
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}

impl config::Override<CellarRebalancerConfig> for SingleSignerCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(
        &self,
        config: CellarRebalancerConfig,
    ) -> Result<CellarRebalancerConfig, FrameworkError> {
        Ok(config)
    }
}