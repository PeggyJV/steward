//! `Cosmos mode signer` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::{
    application::APP,
    prelude::*,
    proto::simulate_contract_call_service_server::SimulateContractCallServiceServer,
    server::FILE_DESCRIPTOR_SET,
    simulate::{self, SimulateHandler, SimulateServerConfig},
    tenderly::validate_tenderly_config,
};
use abscissa_core::{clap::Parser, Command, Runnable};
use hyper::server::conn::http2::Builder;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    service::TowerToHyperService,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tonic::{body::boxed, service::Routes};
use tower::ServiceExt;
use tower_http::ServiceBuilderExt;

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
            .build_v1()
            .unwrap_or_else(|err| {
                status_err!("failed to build descriptor service: {}", err);
                std::process::exit(1)
            });

        info!("Starting application");
        abscissa_tokio::run(&APP, async move {
            let server_config: SimulateServerConfig =
                simulate::load_simulate_server_config(&config, self.use_tls)
                    .await
                    .unwrap_or_else(|err| {
                        status_err!("failed to load server config: {}", err);
                        std::process::exit(1)
                    });

            let address = server_config.address;
            let svc = Routes::new(SimulateContractCallServiceServer::new(
                SimulateHandler::new(server_config.use_tls),
            ))
            .add_service(proto_descriptor_service.clone())
            .prepare();
            let http = Builder::new(TokioExecutor::new());
            let listener = TcpListener::bind(address)
                .await
                .expect("failed to bind to address");

            if let Some(tls_config) = server_config.tls_acceptor {
                let tls_acceptor = TlsAcceptor::from(tls_config);
                info!("simulate server listening on {} with TLS", address);
                while let Ok((connection, address)) = listener.accept().await {
                    let http = http.clone();
                    let tls_acceptor = tls_acceptor.clone();
                    let svc = svc.clone();

                    tokio::spawn(async move {
                        let mut certificates = Vec::new();

                        let conn = tls_acceptor.accept(connection).await.unwrap();
                        let (_, info) = conn.get_ref();

                        if let Some(certs) = info.peer_certificates() {
                            debug!("found {} peer certificates", certs.len());
                            for cert in certs {
                                certificates.push(cert.clone());
                            }
                        } else {
                            debug!("no peer certificates found");
                        }

                        let connection_info = Arc::new(simulate::ConnectionInfo {
                            address,
                            certificates,
                        });

                        let svc = tower::ServiceBuilder::new()
                            .add_extension(connection_info.clone())
                            .service(svc);

                        http.serve_connection(
                            TokioIo::new(conn),
                            TowerToHyperService::new(svc.map_request(
                                move |req: http::Request<_>| {
                                    let mut req = req.map(boxed);
                                    req.extensions_mut().insert(connection_info.clone());
                                    req
                                },
                            )),
                        )
                        .await
                        .unwrap();
                    });
                }
            } else {
                info!("simulate server listening on {} without TLS", address);
                if let Err(err) = tonic::transport::Server::builder()
                    .add_service(SimulateContractCallServiceServer::new(
                        SimulateHandler::new(server_config.use_tls),
                    ))
                    .add_service(proto_descriptor_service)
                    .serve(address)
                    .await
                {
                    status_err!("simulate server error: {}", err);
                    std::process::exit(1)
                }
            }
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1)
        });
    }
}
