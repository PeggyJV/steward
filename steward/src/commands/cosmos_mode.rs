//! `Cosmos mode signer` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::{
    application::APP,
    cellars::uniswapv3::UniswapV3CellarAllocator,
    config::StewardConfig,
    cork::{cache::start_approved_cellar_cache_thread, client::init_cork_client, CorkHandler},
    prelude::*,
    server,
};
use abscissa_core::{clap::Parser, config, Command, FrameworkError, Runnable};
use std::result::Result;
use steward_proto::{
    steward::contract_call_server::ContractCallServer,
    uniswapv3::server::UniswapV3CellarAllocatorServer,
};

/// Cosmos Signer, start allocation module
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Cosmos mode, run Steward as a server.\n This command runs Steward as a server that will send updates to the Sommelier chain."
)]
pub struct CosmosSignerCmd;

impl Runnable for CosmosSignerCmd {
    /// Start the application.
    fn run(&self) {
        let config = APP.config();
        info!("Starting application");
        abscissa_tokio::run(&APP, async {
            if let Err(err) = init_cork_client().await {
                // SetError<T> does not implement Debug and therefore can't call unwrap() or expect()
                panic!("{}", err);
            }

            // Start approved cellar caching thread
            // currently allows the thread to detach since we aren't capturing the JoinHandle
            // TO-DO: Some kind of keep-alive mechanism in case the thread panics?
            start_approved_cellar_cache_thread().await;

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

            info!("listening on {}", server_config.address);
            if let Err(err) = tonic::transport::Server::builder()
                .tls_config(server_config.tls_config)
                .unwrap_or_else(|err| {
                    panic!("{:?}", err);
                })
                .add_service(ContractCallServer::new(CorkHandler))
                .add_service(UniswapV3CellarAllocatorServer::new(
                    UniswapV3CellarAllocator,
                ))
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

impl config::Override<StewardConfig> for CosmosSignerCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, config: StewardConfig) -> Result<StewardConfig, FrameworkError> {
        Ok(config)
    }
}
