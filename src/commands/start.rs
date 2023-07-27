//! `Cosmos mode signer` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::{
    application::APP,
    config::StewardConfig,
    cork::{
        cache::start_approved_cellar_cache_thread,
    },
    prelude::*,
    proposals::start_approved_proposal_polling_thread,
    pubsub::cache::start_publisher_trust_state_cache_thread,
    server::start_server_management_thread,
};
use abscissa_core::{clap::Parser, config, Command, FrameworkError, Runnable};
use std::result::Result;

/// Starts steward
#[derive(Command, Debug, Default, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nRuns the Steward server to process strategists' calls to Cellars"
)]
pub struct StartCmd {}

impl Runnable for StartCmd {
    /// Start the application.
    fn run(&self) {
        info!("Starting application");
        abscissa_tokio::run(&APP, async {
            // currently allows the threads to detach since we aren't capturing the JoinHandle
            start_approved_cellar_cache_thread().await;

            let (tx, rx) = tokio::sync::mpsc::channel(1);
            start_approved_proposal_polling_thread(tx.clone()).await;
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
