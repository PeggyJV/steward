use std::{net::SocketAddr, sync::Arc};

use abscissa_core::tracing::log::{debug, info, warn};
use rustls::{
    pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer},
    ServerConfig,
};
use tonic::{async_trait, Code, Request, Response, Status};

use crate::{
    config::StewardConfig,
    cork::get_encoded_call,
    error::Error,
    proto::{self, ScheduleRequest, SimulateRequest, SimulateResponse},
    tenderly,
    utils::bytes_to_hex_str,
};

#[derive(Debug)]
pub struct ConnectionInfo {
    pub address: SocketAddr,
    pub certificates: Vec<CertificateDer<'static>>,
}

#[derive(Debug)]
pub struct SimulateServerConfig {
    pub tls_acceptor: Option<Arc<ServerConfig>>,
    pub address: SocketAddr,
    pub use_tls: bool,
}

pub struct SimulateHandler {
    use_tls: bool,
}

impl SimulateHandler {
    pub fn new(use_tls: bool) -> Self {
        Self { use_tls }
    }
}

#[async_trait]
impl proto::simulate_contract_call_service_server::SimulateContractCallService for SimulateHandler {
    async fn simulate(
        &self,
        request: Request<SimulateRequest>,
    ) -> Result<Response<SimulateResponse>, Status> {
        if self.use_tls {
            let connection_info = request.extensions().get::<Arc<ConnectionInfo>>();
            debug!("connection info: {:?}", connection_info);

            if connection_info.is_none() {
                return Err(Status::new(
                    Code::Unauthenticated,
                    "no connection info provided".to_string(),
                ));
            }

            let connection_info = connection_info.unwrap();
            debug!(
                "received simulate request from {:?}: {:?}",
                connection_info.address, request
            );
            let certs = connection_info.certificates.clone();

            if certs.is_empty() {
                return Err(Status::new(
                    Code::Unauthenticated,
                    "no certificates provided".to_string(),
                ));
            }
        }

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
) -> Result<SimulateServerConfig, Error> {
    let tls_acceptor = if use_tls {
        validate_simulate_tls_config(config);
        Some(load_simulate_tls_config(config).await?)
    } else {
        None
    };
    let port = &config.server.port;
    let address = &config.server.address;
    let address: SocketAddr = format!("{}:{}", address, port).parse()?;

    Ok(SimulateServerConfig {
        tls_acceptor,
        address,
        use_tls,
    })
}

pub async fn load_simulate_tls_config(
    config: &std::sync::Arc<StewardConfig>,
) -> Result<Arc<ServerConfig>, Error> {
    let cert = tokio::fs::read(&config.simulate.server_cert_path).await?;
    let key = tokio::fs::read(&config.simulate.server_key_path).await?;
    let cert = CertificateDer::from_pem_slice(&cert).expect("failed to parse server cert");
    let key = PrivateKeyDer::from_pem_slice(&key).expect("failed to parse server key");

    // Client verifier
    let mut trust_store = rustls::RootCertStore::empty();
    let client_ca_cert = tokio::fs::read(&config.simulate.client_ca_cert_path).await?;
    let client_ca_cert =
        CertificateDer::from_pem_slice(&client_ca_cert).expect("failed to parse client CA cert");
    trust_store.add(client_ca_cert).unwrap();
    let roots = Arc::new(trust_store);
    let client_verifier = rustls::server::WebPkiClientVerifier::builder(roots)
        .build()
        .unwrap();

    // Server config
    let mut server_config = rustls::ServerConfig::builder()
        .with_client_cert_verifier(client_verifier)
        .with_single_cert(vec![cert], key)
        .expect("failed to build rustls server config");
    server_config.alpn_protocols = vec![b"h2".to_vec()];
    server_config.ignore_client_order = true;
    server_config.max_early_data_size = 0;

    Ok(Arc::new(server_config))
}
