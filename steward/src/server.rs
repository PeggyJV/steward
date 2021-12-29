use crate::config::CellarRebalancerConfig;
use std::{fs, io};
use tonic::transport::{Certificate, Identity, ServerTlsConfig};

pub fn get_steward_descriptor_contents() -> Result<Vec<u8>, io::Error> {
    fs::read("steward_proto/src/prost/descriptor.bin")
}

pub async fn load_server_config(config: std::sync::Arc<CellarRebalancerConfig>) -> ServerTlsConfig {
    let cert = tokio::fs::read(&config.tls.server_cert_path).await.unwrap();
    let key = tokio::fs::read(&config.tls.server_key_path).await.unwrap();
    let server_identity = Identity::from_pem(cert, key);
    let client_ca_cert = tokio::fs::read(&config.tls.client_ca_cert_path)
        .await
        .unwrap();
    let client_ca_cert = Certificate::from_pem(client_ca_cert);
    let tls_config = ServerTlsConfig::new()
        .identity(server_identity.clone())
        .client_ca_root(client_ca_cert.clone());

    tls_config
}
