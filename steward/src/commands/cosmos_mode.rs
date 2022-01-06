//! `Cosmos mode signer` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::{
    application::APP,
    cellars::{uniswapv3::UniswapV3CellarAllocator, STEWARD_PORT},
    config::CellarRebalancerConfig,
    prelude::*,
    server,
};
use abscissa_core::{config, Clap, Command, FrameworkError, Runnable};
use std::result::Result;
use steward_proto::uniswapv3::server::UniswapV3CellarAllocatorServer;

#[derive(Command, Debug, Clap)]
pub struct CosmosSignerCmd;

impl Runnable for CosmosSignerCmd {
    /// Start the application.
    fn run(&self) {
        let config = APP.config();
        info!("Starting application");
        abscissa_tokio::run(&APP, async {
            // Reflection required for certain clients to function... such as grpcurl
            let contents = server::DESCRIPTOR.to_vec();
            let proto_descriptor_service = tonic_reflection::server::Builder::configure()
                .register_encoded_file_descriptor_set(contents.as_slice())
                .build()
                .unwrap();

            // Configure TLS
            let tls_config = server::load_server_config(config).await;

            // run it
            let addr = ([127, 0, 0, 1], STEWARD_PORT).into();
            info!("listening on {}", addr);
            tonic::transport::Server::builder()
                .tls_config(tls_config)
                .unwrap_or_else(|err| {
                    panic!("{:?}", err);
                })
                .add_service(UniswapV3CellarAllocatorServer::new(
                    UniswapV3CellarAllocator,
                ))
                .add_service(proto_descriptor_service)
                .serve(addr)
                .await
                .unwrap();
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}

impl config::Override<CellarRebalancerConfig> for CosmosSignerCmd {
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
