use std::net::SocketAddr;

use crate::{
    config::StewardConfig,
    error::{Error, ErrorKind},
};
use tonic::transport::{Certificate, Identity, ServerTlsConfig};

pub const DEFAULT_CLIENT_CA: &[u8] = include_bytes!("../../tls/sevenseas_ca.crt");
// for gRPC reflection
pub const DESCRIPTOR: &[u8] = include_bytes!("../../steward_proto/src/prost/descriptor.bin");

pub struct ServerConfig {
    pub tls_config: ServerTlsConfig,
    pub address: SocketAddr,
}

pub async fn load_server_config(
    config: &std::sync::Arc<StewardConfig>,
) -> Result<ServerConfig, Error> {
    // server identity
    let cert = tokio::fs::read(&config.server.server_cert_path).await?;
    let key = tokio::fs::read(&config.server.server_key_path).await?;
    let server_identity = Identity::from_pem(cert, key);

    // client authentication
    let client_ca = match &config.server.client_ca_cert_path {
        Some(path) => tokio::fs::read(path).await?,
        None => DEFAULT_CLIENT_CA.into(),
    };
    let client_ca_cert = Certificate::from_pem(client_ca);
    let mut client_ca_cert = std::io::Cursor::new(client_ca_cert.get_ref());
    let mut client_root_cert_store = rustls::RootCertStore::empty();
    if client_root_cert_store
        .add_pem_file(&mut client_ca_cert)
        .is_err()
    {
        return Err(ErrorKind::CertfificateParsingError
            .context("failed to parse client CA cert")
            .into());
    }
    let client_auth = rustls::AllowAnyAuthenticatedClient::new(client_root_cert_store);

    // steward server config
    let server_config = rustls::ServerConfig::new(client_auth);
    let tls_config = ServerTlsConfig::new()
        .identity(server_identity.clone())
        .rustls_server_config(server_config)
        .to_owned();
    let port = &config.server.port;
    let address = &config.server.address;
    let address: SocketAddr = format!("{}:{}", address, port).parse()?;

    Ok(ServerConfig {
        tls_config,
        address,
    })
}
