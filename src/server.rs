use std::net::SocketAddr;

use crate::{config::StewardConfig, error::Error};
use tonic::transport::{Certificate, Identity, ServerTlsConfig};

pub const DEFAULT_CLIENT_CA: &[u8] = include_bytes!("../tls/sevenseas_ca.crt");
// for gRPC reflection
pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("gen/proto/descriptor.bin");

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
    let port = &config.server.port;
    let address = &config.server.address;
    let address: SocketAddr = format!("{}:{}", address, port).parse()?;

    Ok(ServerConfig {
        tls_config,
        address,
    })
}
