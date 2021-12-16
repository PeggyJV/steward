//! Start subcommand - example of how to write a subcommand

use crate::cellars::uniswapv3::UniswapV3CellarAllocator;
/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::{application::APP, config::CellarRebalancerConfig, prelude::*};
use abscissa_core::{config, Clap, Command, FrameworkError, Runnable};
use steward_proto::uniswapv3::server::UniswapV3CellarAllocatorServer;
use tonic::transport::{Identity, Certificate, ServerTlsConfig};
use std::{fs, result::Result};

#[derive(Command, Debug, Clap)]
pub struct SingleSignerCmd;

impl Runnable for SingleSignerCmd {
    /// Start the application.
    fn run(&self) {
        let config = APP.config();
        info!("Starting application");
        abscissa_tokio::run(&APP, async {
            // Reflection required for certain clients to function... such as grpcurl
            let contents = fs::read("steward_proto/src/prost/descriptor.bin").unwrap();
            let proto_descriptor_service = tonic_reflection::server::Builder::configure()
                .register_encoded_file_descriptor_set(contents.as_slice())
                .build()
                .unwrap();

            // Configure TLS
            let cert = tokio::fs::read(&config.tls.server_cert).await.unwrap();
            let key = tokio::fs::read(&config.tls.server_key).await.unwrap();
            let server_identity = Identity::from_pem(cert, key);
            let client_ca_cert = tokio::fs::read(&config.tls.client_ca_cert).await.unwrap();
            let client_ca_cert = Certificate::from_pem(client_ca_cert);
            let tls_config = ServerTlsConfig::new()
                .identity(server_identity)
                .client_ca_root(client_ca_cert);

            // run it
            let addr = ([127, 0, 0, 1], 3000).into();
            info!("listening on {}", addr);
            tonic::transport::Server::builder()
                .tls_config(tls_config)
                .unwrap_or_else(|err| {
                    panic!("{:?}", err);
                })
                .add_service(UniswapV3CellarAllocatorServer::new(UniswapV3CellarAllocator))
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
