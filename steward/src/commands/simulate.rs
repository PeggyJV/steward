//! `Cosmos mode signer` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::{
    application::APP,
    config::StewardConfig,
    prelude::*,
    server::{self, with_tls},
    simulate::SimulateHandler,
};
use abscissa_core::{clap::Parser, config, Command, FrameworkError, Runnable};
use std::result::Result;
use steward_proto::steward::{
    simulate_contract_call_server::SimulateContractCallServer,
};

/// Runs the Simulate server which uses Tenderly to simulate contract calls
#[derive(Command, Debug, Default, Parser)]
#[clap(
    long_about = "Simulates contract calls using Tenderly. Tenderly credentials must be set in the config file to use."
)]
pub struct SimulateCmd {
    /// Run the gRPC server with TLS enabled
    #[clap(long)]
    pub tls: bool,
}

impl Runnable for SimulateCmd {
    /// Simulate the application.
    fn run(&self) {
        let config = APP.config();

        info!("Starting application");
        abscissa_tokio::run(&APP, async {
            // Reflection required for certain clients to function... such as grpcurl
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

            let mut builder = tonic::transport::Server::builder();
            if config.simulate.use_tls {
                let tls_config = &server_config
                    .tls_config
                    .expect("tls config was not initialized");
                builder = with_tls(builder, tls_config);
            }

            info!("simulate server listening on {}", server_config.address);
            if let Err(err) = builder
                .add_service(SimulateContractCallServer::new(SimulateHandler))
                .add_service(proto_descriptor_service)
                .serve(server_config.address)
                .await
            {
                status_err!("simulate server error: {}", err);
                std::process::exit(1)
            }
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1)
        });
    }
}

impl config::Override<StewardConfig> for SimulateCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, mut config: StewardConfig) -> Result<StewardConfig, FrameworkError> {
        info!("IN OVERRIDE CONFIG");
        if self.tls {
            config.simulate.use_tls = self.tls;
        }

        Ok(config)
    }
}
