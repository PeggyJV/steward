//! `Cosmos mode signer` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::{
    application::APP,
    config::StewardConfig,
    cork::{
        cache::start_approved_cellar_cache_thread,
        proposals::start_scheduled_cork_proposal_polling_thread, CorkHandler,
    },
    prelude::*,
    server::{self, with_tls},
};
use abscissa_core::{clap::Parser, config, Command, FrameworkError, Runnable};
use std::result::Result;
use steward_proto::steward::contract_call_server::ContractCallServer;

/// Starts steward
#[derive(Command, Debug, Default, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nCosmos mode, run Steward as a server.\n This command runs Steward as a server that will send updates to the Sommelier chain."
)]
pub struct StartCmd {}

impl Runnable for StartCmd {
    /// Start the application.
    fn run(&self) {
        let config = APP.config();
        info!("Starting application");
        abscissa_tokio::run(&APP, async {
            // currently allows the threads to detach since we aren't capturing the JoinHandle
            start_approved_cellar_cache_thread().await;
            start_scheduled_cork_proposal_polling_thread().await;

            let contents = server::DESCRIPTOR.to_vec();
            let proto_descriptor_service = tonic_reflection::server::Builder::configure()
                .register_encoded_file_descriptor_set(contents.as_slice())
                .build()
                .unwrap_or_else(|err| {
                    status_err!("failed to build descriptor service: {}", err);
                    std::process::exit(1)
                });

            let server_config = server::load_server_config(&config)
                .await
                .unwrap_or_else(|err| {
                    status_err!("failed to load server config: {}", err);
                    std::process::exit(1)
                });

            // build appropriate server
            info!("listening on {}", server_config.address);
            let builder = tonic::transport::Server::builder();
            let tls_config = &server_config
                .tls_config
                .expect("tls config was not initialized");
            if let Err(err) = with_tls(builder, tls_config)
                .add_service(ContractCallServer::new(CorkHandler))
                .add_service(proto_descriptor_service)
                .serve(server_config.address)
                .await
            {
                status_err!("server error: {}", err);
                std::process::exit(1)
            }
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
