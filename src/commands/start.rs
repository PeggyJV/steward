//! `start` subcommand - example of how to write a subcommand

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
use abscissa_core::{config, Command, FrameworkError, Options, Runnable};
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
#[derive(Command, Debug, Options)]
pub struct StartCmd {
    /// To whom are we saying hello?
    #[options(free)]
    recipient: Vec<String>,
}

impl StartCmd {
    /// Initialize collector poller (if configured/needed)
    async fn init_collector_poller<S>(
        &self,
        _config: CellarRebalancerConfig,
        collector: S,
    ) -> JoinHandle<()>
    where
        S: Service<Request, Response = Response, Error = BoxError> + Send + Sync + Clone + 'static,
        S::Future: Send,
    {
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

        tokio::spawn(async move {
            // Connect to the network provider (example below is for my Ganache-cli fork)
            let client = Provider::<Http>::try_from(eth_host).unwrap();
            let client = SignerMiddleware::new(client, wallet);

            // MyContract expects Arc, create with client
            let client = Arc::new(client);

            let poller = Poller::new(&config, client).await.unwrap_or_else(|e| {
                status_err!("couldn't initialize collector poller: {}", e);
                std::process::exit(1);
            });

            poller.run(collector).await;
        })
    }
}

impl Runnable for StartCmd {
    /// Start the application.
    fn run(&self) {
        info!("Starting application");

        let config = APP.config();
        abscissa_tokio::run(&APP, async {
            let mut tasks = vec![];

            let collector =
                ServiceBuilder::new()
                    .buffer(20)
                    .service(Collector::new(&config).unwrap_or_else(|e| {
                        status_err!("couldn't initialize collector service: {}", e);
                        std::process::exit(1);
                    }));

            tasks.push(
                self.init_collector_poller(config.as_ref().clone(), collector.clone())
                    .await,
            );

            future::join_all(tasks).await;
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}

impl config::Override<CellarRebalancerConfig> for StartCmd {
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
