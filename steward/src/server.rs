use crate::{config::CellarRebalancerConfig, error::Error};
use std::{fs, io};
use tonic::transport::{Certificate, Identity, ServerTlsConfig};

// for gRPC reflection
pub const DESCRIPTOR: &'static [u8] = include_bytes!("../../steward_proto/src/prost/descriptor.bin");
pub const DEFAULT_CLIENT_CA: &str = "tls/volumefi_ca.crt";

pub fn get_steward_descriptor_contents() -> Result<Vec<u8>, io::Error> {
    fs::read("steward_proto/src/prost/descriptor.bin")
}

pub async fn load_server_config(config: std::sync::Arc<CellarRebalancerConfig>) -> Result<ServerTlsConfig, Error> {
    let cert = tokio::fs::read(&config.tls.server_cert_path).await?;
    let key = tokio::fs::read(&config.tls.server_key_path).await?;
    let server_identity = Identity::from_pem(cert, key);
    let client_ca_path = match &config.tls.client_ca_cert_path {
        Some(path) => path,
        None => DEFAULT_CLIENT_CA,
    };
    let client_ca_cert = tokio::fs::read(client_ca_path).await?;
    let client_ca_cert = Certificate::from_pem(client_ca_cert);
    let tls_config = ServerTlsConfig::new()
        .identity(server_identity.clone())
        .client_ca_root(client_ca_cert.clone());

    Ok(tls_config)
}
