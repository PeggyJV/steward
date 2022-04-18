use std::net::SocketAddr;

use crate::{config::StewardConfig, error::Error};
use tonic::transport::{Certificate, Identity, ServerTlsConfig};

pub const DEFAULT_CLIENT_CA: &[u8] = include_bytes!("../../tls/peggyjv_ca.crt");
pub const DEFAULT_STEWARD_PORT: u16 = 5734;
// for gRPC reflection
pub const DESCRIPTOR: &[u8] = include_bytes!("../../steward_proto_rust/src/prost/descriptor.bin");

pub struct ServerConfig {
    pub tls_config: ServerTlsConfig,
    pub address: SocketAddr,
}

pub async fn load_server_config(
    config: &std::sync::Arc<StewardConfig>,
) -> Result<ServerConfig, Error> {
    let cert = tokio::fs::read(&config.server.server_cert_path).await?;
    let key = tokio::fs::read(&config.server.server_key_path).await?;
    let server_identity = Identity::from_pem(cert, key);
    let client_ca = match &config.server.client_ca_cert_path {
        Some(path) => tokio::fs::read(path).await?,
        None => DEFAULT_CLIENT_CA.into(),
    };
    let client_ca_cert = Certificate::from_pem(client_ca);
    let tls_config = ServerTlsConfig::new()
        .identity(server_identity.clone())
        .client_ca_root(client_ca_cert);
    let port = match config.server.port {
        Some(p) => p,
        None => DEFAULT_STEWARD_PORT,
    };
    let address = match &config.server.address {
        Some(a) => a,
        None => "0.0.0.0",
    };
    let address: SocketAddr = format!("{}:{}", address, port).parse()?;

    Ok(ServerConfig {
        tls_config,
        address,
    })
}
