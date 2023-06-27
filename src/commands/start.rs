//! `Cosmos mode signer` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::{
    application::APP,
    config::StewardConfig,
    cork::{
        cache::start_approved_cellar_cache_thread,
        proposals::start_scheduled_cork_proposal_polling_thread,
    },
    prelude::*,
    proto::contract_call_service_server::ContractCallServiceServer,
    pubsub::cache::start_publisher_trust_state_cache_thread,
    server::start_server_management_thread,
};
use abscissa_core::{clap::Parser, config, Command, FrameworkError, Runnable};
use std::result::Result;

/// Starts steward
#[derive(Command, Debug, Default, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nCosmos mode, run Steward as a server.\n This command runs Steward as a server that will send updates to the Sommelier chain."
)]
pub struct StartCmd {}

impl Runnable for StartCmd {
    /// Start the application.
    fn run(&self) {
        info!("Starting application");
        abscissa_tokio::run(&APP, async {
            // currently allows the threads to detach since we aren't capturing the JoinHandle
            start_approved_cellar_cache_thread().await;
            start_scheduled_cork_proposal_polling_thread().await;

            let (tx, rx) = tokio::sync::mpsc::channel(1);
            start_publisher_trust_state_cache_thread(tx).await;
            start_server_management_thread(rx).await;
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1)
        });
    }
}

impl config::Override<StewardConfig> for StartCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, config: StewardConfig) -> Result<StewardConfig, FrameworkError> {
        Ok(config)
    }
}
