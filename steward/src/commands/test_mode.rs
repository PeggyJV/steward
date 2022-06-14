use crate::{application::APP, cork::DirectCorkHandler, prelude::*, server};
use abscissa_core::{clap::Parser, Command, Runnable};
use steward_proto::steward::contract_call_server::ContractCallServer;

/// Test mode, start Eth test mode
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Start Ethereum test module.\n This command starts Steward using the Ethereum test mode."
)]
pub struct TestModeCmd {
    key_name: String,
}

impl Runnable for TestModeCmd {
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
                .add_service(ContractCallServer::new(DirectCorkHandler {
                    key_name: self.key_name.clone(),
                }))
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
