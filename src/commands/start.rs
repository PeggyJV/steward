//! `start` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;

use crate::{application::APP, prelude::*};
use ethers::prelude::*;
use std::time::Duration;


use crate::config::ContractMonitorConfig;
use abscissa_core::{config, Command, FrameworkError, Options, Runnable};

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

impl Runnable for StartCmd {
    /// Start the application.
    fn run(&self) {
        let config = APP.config();
        println!("Hello, {}!", &config.hello.recipient);

        abscissa_tokio::run(&APP, async {

            let ws = Ws::connect(std::env::var("ETH_WS").unwrap()).await.unwrap();
            let provider = Provider::new(ws).interval(Duration::from_millis(2000));
            let mut stream = provider.watch_blocks().await.unwrap().stream();
            while let Some(block) = stream.next().await {
                dbg!(block);
            }
        




         }).unwrap_or_else(|e| {
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
