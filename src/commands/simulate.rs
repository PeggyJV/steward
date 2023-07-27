//! `Cosmos mode signer` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::{
    application::APP,
    prelude::*,
    proto::simulate_contract_call_service_server::SimulateContractCallServiceServer,
    server::{ServerConfig, FILE_DESCRIPTOR_SET},
    simulate::{self, SimulateHandler},
    tenderly::validate_tenderly_config,
};
use abscissa_core::{clap::Parser, Command, Runnable};

/// Runs the Simulate server which uses Tenderly to simulate contract calls
#[derive(Command, Debug, Default, Parser)]
#[clap(
    long_about = "Simulates contract calls using Tenderly. Tenderly credentials must be set in the config file to use."
)]
pub struct SimulateCmd {
    /// Run the gRPC server with TLS enabled
    #[clap(long)]
    pub use_tls: bool,
}

impl Runnable for SimulateCmd {
    /// Simulate the application.
    fn run(&self) {
        let config = APP.config();
        validate_tenderly_config(&config);

        let contents = FILE_DESCRIPTOR_SET.to_vec();
        let proto_descriptor_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(contents.as_slice())
            .build()
            .unwrap_or_else(|err| {
                status_err!("failed to build descriptor service: {}", err);
                std::process::exit(1)
            });
        info!("Starting application");
        abscissa_tokio::run(&APP, async move {

             let server_config: ServerConfig =
                 simulate::load_simulate_server_config(&config, self.use_tls)
                     .await
                     .unwrap_or_else(|err| {
                         status_err!("failed to load server config: {}", err);
                         std::process::exit(1)
                     });
 
             let mut builder = tonic::transport::Server::builder();
             if self.use_tls {
                 let tls_config = &server_config
                     .tls_config
                     .expect("tls config was not initialized");
                 builder = builder
                     .tls_config(tls_config.to_owned())
                     .unwrap_or_else(|err| {
                         panic!("{:?}", err);
                     })
             }



            info!("simulate server listening on {}", server_config.address);
            if let Err(err) = builder
                .add_service(SimulateContractCallServiceServer::new(SimulateHandler))
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
