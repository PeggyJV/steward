use std::net::SocketAddr;

use abscissa_core::tracing::log::{info, warn};
use tonic::{
    async_trait,
    transport::{Certificate, Identity, ServerTlsConfig},
    Code, Request, Response, Status,
};

use crate::{
    config::StewardConfig,
    cork::get_encoded_call,
    error::Error,
    proto::{self, ScheduleRequest, SimulateRequest, SimulateResponse},
    server::ServerConfig,
    tenderly,
    utils::bytes_to_hex_str,
};

pub struct SimulateHandler;

#[async_trait]
impl proto::simulate_contract_call_service_server::SimulateContractCallService for SimulateHandler {
    async fn simulate(
        &self,
        request: Request<SimulateRequest>,
    ) -> Result<Response<SimulateResponse>, Status> {
        let request = request.get_ref().to_owned();
        let inner_request = match request.request {
            Some(r) => r,
            None => {
                return Err(Status::new(
                    Code::InvalidArgument,
                    "empty request".to_string(),
                ))
            }
        };
        let cellar_id = inner_request.cellar_id.clone();
        let encoded_call = match get_string_encoded_call(inner_request) {
            Ok(c) => c,
            Err(err) => {
                warn!("failed to get encoded call for {}: {}", cellar_id, err);
                return Err(Status::new(Code::Aborted, err.to_string()));
            }
        };
        info!("encoded call: {encoded_call}");
        let response_body = if request.encode_only {
            String::default()
        } else {
            info!("simulating call to {cellar_id} with tenderly");
            let response = tenderly::simulate(cellar_id, encoded_call.clone()).await?;
            info!("response object: {response:?}");
            response.text().await.map_err(|e| {
                Status::new(
                    Code::Internal,
                    format!("failed to deserialize response body: {e}"),
                )
            })?
        };

        Ok(Response::new(SimulateResponse {
            encoded_call,
            response_body,
        }))
    }
}

pub fn get_string_encoded_call(request: ScheduleRequest) -> Result<String, Error> {
    get_encoded_call(request).map(|b| bytes_to_hex_str(&b))
}

pub fn validate_simulate_tls_config(config: &StewardConfig) {
    if config.simulate.server_cert_path.is_empty() {
        panic!("Simulate server cert key is not set");
    }
    if config.simulate.server_key_path.is_empty() {
        panic!("Simulate server key path is not set");
    }
    if config.simulate.client_ca_cert_path.is_empty() {
        panic!("Simulate client ca cert path is not set");
    }
}

pub async fn load_simulate_server_config(
    config: &std::sync::Arc<StewardConfig>,
    use_tls: bool,
) -> Result<ServerConfig, Error> {
    let tls_config = if use_tls {
        validate_simulate_tls_config(config);
        Some(load_simulate_tls_config(config).await?)
    } else {
        None
    };
    let port = &config.server.port;
    let address = &config.server.address;
    let address: SocketAddr = format!("{}:{}", address, port).parse()?;

    Ok(ServerConfig {
        tls_config,
        address,
    })
}

pub async fn load_simulate_tls_config(
    config: &std::sync::Arc<StewardConfig>,
) -> Result<ServerTlsConfig, Error> {
    let cert = tokio::fs::read(&config.simulate.server_cert_path).await?;
    let key = tokio::fs::read(&config.simulate.server_key_path).await?;
    let server_identity = Identity::from_pem(cert, key);
    let client_ca_cert = tokio::fs::read(&config.simulate.client_ca_cert_path).await?;
    let client_ca_cert = Certificate::from_pem(client_ca_cert);

    Ok(ServerTlsConfig::new()
        .identity(server_identity.clone())
        .client_ca_root(client_ca_cert))
}
