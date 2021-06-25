//! `start` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;
use abscissa_core::error::{BoxError};

use crate::application::APP;
use ethers::{prelude::*,    providers::{Provider, Http}};
use std::time::Duration;

use crate::{config::ContractMonitorConfig,collector::{Collector,Poller,Response,Request}};
use abscissa_core::{config, Command, FrameworkError, Options, Runnable};
use tokio::task::JoinHandle;
use tower::{Service, ServiceBuilder};
use std::{convert::TryFrom, sync::Arc};


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
            config: ContractMonitorConfig,
            collector: S,
        ) -> JoinHandle<()>
        where
            S: Service<Request, Response = Response, Error = BoxError>
                + Send
                + Sync
                + Clone
                + 'static,
            S::Future: Send,
        {
            tokio::spawn(async move {

                let address = "0x44692093E7A78447D8Ddbc477192934520928A5B
                ".parse::<Address>().unwrap();
                
                // Connect to the network provider (example below is for my Ganache-cli fork)
                let client = Provider::<Http>::try_from("http://localhost:7545").unwrap();
    
                // MyContract expects Arc, create with client
                let client = Arc::new(client);
    

                let poller = Poller::new(&config,client).unwrap_or_else(|e| {
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
        let config = APP.config();
        println!("Hello, {}!", &config.hello.recipient);

        abscissa_tokio::run(&APP, async {

            let mut tasks = vec![];

            if let config = APP.config().clone(){
                let collector =
                ServiceBuilder::new()
                    .buffer(20)
                    .service(Collector::new(&config).unwrap_or_else(|e| {
                        status_err!("couldn't initialize collector service: {}", e);
                        std::process::exit(1);
                    }));

                tasks.push(self.init_collector_poller(config.as_ref().clone(), collector.clone()).await)

            } 
            tasks
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}

impl config::Override<ContractMonitorConfig> for StartCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(
        &self,
        mut config: ContractMonitorConfig,
    ) -> Result<ContractMonitorConfig, FrameworkError> {
        if !self.recipient.is_empty() {
            config.hello.recipient = self.recipient.join(" ");
        }

        Ok(config)
    }
}
