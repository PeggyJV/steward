//! `start` subcommand - example of how to write a subcommand
/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::application::APP;
use crate::prelude::*;
use crate::{
    collector::{Collector, Poller},
    config::CellarRebalancerConfig,
};
use abscissa_core::{FrameworkError, Options, Runnable};
use deep_space::Contact;
use ethers::{
    prelude::*,
    providers::{Http, Provider},
};
use futures::future;
use mongodb::Client;
use signatory::FsKeyStore;
use std::{convert::TryFrom, path, sync::Arc, time::Duration};
use tower::ServiceBuilder;

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

            let poller = Poller::new(&cellar, client).await.unwrap_or_else(|e| {
                status_err!("couldn't initialize poller: {}", e);
                std::process::exit(1);
            });

            pollers.push(poller);
        }

        pollers
    }
}

impl Runnable for StartCmd {
    /// Start the application.
    fn run(&self) {
        info!("Starting application");

        let config = APP.config();

        abscissa_tokio::run(&APP, async {
            let mongo = Client::with_uri_str(config.mongo.host.clone())
                .await
                .unwrap();

            let timeout = Duration::from_secs(30);
            let chain_prefix = &config.cosmos.prefix;
            let contact = Contact::new(&config.cosmos.grpc, timeout, chain_prefix)
                .expect("Could not create contact");

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
                let contact = contact.clone();
                let collector = collector.clone();

                let cellar = config
                    .cellars
                    .get(tasks.len())
                    .expect("Could not fetch cellar");

                let database = mongo.database(&cellar.mongo_database);

                tasks.push(tokio::spawn(async move {
                    poller.run(collector, contact, database).await
                }));
            }
            future::join_all(tasks).await;
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}

impl abscissa_core::config::Override<CellarRebalancerConfig> for StartCmd {
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
