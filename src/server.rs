use std::{net::SocketAddr, sync::Arc};

use crate::{
    config::StewardConfig,
    cork::CorkHandler,
    error::{Error, ErrorKind},
    prelude::APP,
    proto::{
        contract_call_service_server::ContractCallServiceServer,
        status_service_server::StatusServiceServer,
    },
    pubsub::cache::{lookup_trust_data_by_subject_key_identifier, PUBLISHER_TRUST_STATE_CACHE},
    status::StatusHandler,
};
use abscissa_core::{
    status_err,
    tracing::log::{error, info},
    Application,
};
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;
use tonic::{
    transport::{Certificate, ServerTlsConfig},
    Status,
};
use x509_parser::prelude::{FromDer, KeyIdentifier, ParsedExtension, X509Certificate};

// for gRPC reflection
pub const FILE_DESCRIPTOR_SET: &[u8] =
    include_bytes!("../crates/steward-proto/src/gen/descriptor.bin");

pub struct ServerConfig {
    pub tls_config: Option<ServerTlsConfig>,
    pub address: SocketAddr,
}

pub(crate) async fn start_server_management_thread(mut rx: Receiver<()>) {
    // wait for the cache to be initialized
    rx.recv().await;

    loop {
        let cancellation_token = CancellationToken::new();
        let server_handle = tokio::spawn(start_server(cancellation_token.clone()));

        // when trust state changes, send cancel signal and restart server
        if let Some(()) = rx.recv().await {
            info!("restarting server to build new trust store");
            cancellation_token.cancel();
            server_handle.await.unwrap();
        }
    }
}

pub(crate) async fn start_server(cancellation_token: CancellationToken) {
    // Reflection required for certain clients to function... such as grpcurl
    let contents = FILE_DESCRIPTOR_SET.to_vec();
    let proto_descriptor_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(contents.as_slice())
        .build()
        .unwrap_or_else(|err| {
            status_err!("failed to build descriptor service: {}", err);
            std::process::exit(1)
        });
    let config = APP.config();
    let server_config = load_server_config(&config).await.unwrap_or_else(|err| {
        status_err!("failed to load server config: {}", err);
        std::process::exit(1)
    });

    info!("listening on {}", server_config.address);
    if let Err(err) = tonic::transport::Server::builder()
        .tls_config(server_config.tls_config.expect("must have tls config"))
        .unwrap_or_else(|err| {
            status_err!("failed to configure server for tls: {}", err);
            std::process::exit(1)
        })
        .add_service(ContractCallServiceServer::new(CorkHandler))
        .add_service(StatusServiceServer::new(StatusHandler))
        .add_service(proto_descriptor_service)
        .serve_with_shutdown(server_config.address, cancellation_token.cancelled())
        .await
    {
        status_err!("server error: {}", err);
        std::process::exit(1)
    }
}

pub(crate) async fn auth_config(
    config: &Arc<StewardConfig>,
) -> Result<rustls::ServerConfig, Error> {
    let mut trust_store = rustls::RootCertStore::empty();
    let cache = PUBLISHER_TRUST_STATE_CACHE.read().await;
    for td in cache.values() {
        trust_store
            .add_pem_file(&mut td.publisher.ca_cert.as_bytes())
            .unwrap();
    }

    let client_auth = rustls::AllowAnyAuthenticatedClient::new(trust_store);

    let cert = tokio::fs::read(&config.server.server_cert_path).await?;
    let key = tokio::fs::read(&config.server.server_key_path).await?;
    let cert: Vec<rustls::Certificate> = rustls_pemfile::certs(&mut cert.as_slice())
        .unwrap()
        .iter()
        .map(|v| rustls::Certificate(v.clone()))
        .collect();
    let key = match rustls_pemfile::read_one(&mut key.as_slice())
        .expect("cannot parse private key .pem file")
    {
        Some(rustls_pemfile::Item::RSAKey(_)) => {
            return Err(ErrorKind::InvalidKey
                .context("must use an elliptic-curve-derived key")
                .into())
        }
        Some(rustls_pemfile::Item::PKCS8Key(key)) => rustls::PrivateKey(key),
        Some(rustls_pemfile::Item::ECKey(key)) => rustls::PrivateKey(key),
        _ => panic!("cannot parse private key .pem file"),
    };
    let mut server_config = rustls::ServerConfig::new(client_auth);
    let alpn_protocols: Vec<u8> = "h2".into();
    server_config.set_protocols(&[alpn_protocols]);
    server_config.set_single_cert(cert, key).unwrap();

    Ok(server_config)
}

pub(crate) fn socket_addr(config: &Arc<StewardConfig>) -> Result<SocketAddr, Error> {
    let port = &config.server.port;
    let address = &config.server.address;
    let address: SocketAddr = format!("{}:{}", address, port).parse()?;

    Ok(address)
}

pub async fn load_server_config(config: &Arc<StewardConfig>) -> Result<ServerConfig, Error> {
    let server_config = auth_config(config).await?;
    let tls_config = ServerTlsConfig::new()
        .rustls_server_config(server_config)
        .to_owned();
    let address = socket_addr(config)?;

    Ok(ServerConfig {
        tls_config: Some(tls_config),
        address,
    })
}

/// A client is authorized to publish to a subscription if the following criteria are met:
/// 1. The client certificate has an authority key identifier extension whose value is the key of an entry
///    in the publisher trust state cache
/// 2. The client certificate is signed by the publisher's CA certificate
/// 3. The entry's list of subscription IDs contains the subscription ID of the request
pub(crate) async fn handle_authorization(
    subscription_id: &String,
    certs: Arc<Vec<Certificate>>,
) -> Result<(), Status> {
    let mut client_cert: Option<X509Certificate> = None;

    // In case the request contains multiple certificates, we iterate until the first valid one.
    for cert in certs.iter() {
        let (_, cert_candidate) = match X509Certificate::from_der(cert.get_ref()) {
            Ok(cert) => cert,
            Err(e) => {
                error!("failed to parse certificate for authorization check: {e}");
                continue;
            }
        };

        // if the cert is a CA, it's not he client cert, so we skip
        if let Ok(Some(extension)) = cert_candidate.basic_constraints() {
            if extension.value.ca {
                continue;
            }
        }

        client_cert = Some(cert_candidate);

        break;
    }

    let client_cert = match client_cert {
        Some(cert) => cert,
        None => {
            error!("no valid client certificate found for authorization check");
            return Err(Status::unauthenticated("no valid client certificate found"));
        }
    };

    let client_cert_aki = match extract_authority_key_identifier(&client_cert) {
        Ok(aki) => aki,
        Err(e) => {
            error!("failed to extract authority key identifier for authorization check: {e}");
            return Err(Status::unauthenticated(
                "failed to extract authority key identifier",
            ));
        }
    }
    .0
    .to_vec();

    let publisher_trust_data =
        match lookup_trust_data_by_subject_key_identifier(&client_cert_aki).await {
            Some(p) => p,
            None => {
                error!(
                "no publisher trust data found for authority key identifier: {client_cert_aki:?}"
            );
                return Err(Status::permission_denied("no publisher trust data found"));
            }
        };

    let ca_pub = publisher_trust_data.publisher_ca_cert.public_key();
    if let Err(e) = client_cert.verify_signature(Some(ca_pub)) {
        info!("failed to verify signature for certificate: {e}");
        return Err(Status::permission_denied("signature verification failed"));
    }

    if !publisher_trust_data
        .subscription_ids
        .contains(subscription_id)
    {
        info!("not subscribed to this publisher for subscription {subscription_id}");
        return Err(Status::permission_denied(
            "not subscribed to this publisher",
        ));
    }

    Ok(())
}

pub(crate) fn extract_authority_key_identifier<'a>(
    x509: &'a X509Certificate,
) -> Result<KeyIdentifier<'a>, Error> {
    match x509
        .extensions_map()?
        .get(&x509_parser::oid_registry::OID_X509_EXT_AUTHORITY_KEY_IDENTIFIER)
        .ok_or(ErrorKind::InvalidCertificate.context("no subject key identifier found"))?
        .parsed_extension()
    {
        ParsedExtension::AuthorityKeyIdentifier(aki) => match aki.key_identifier.clone() {
            Some(ki) => Ok(ki),
            None => Err(ErrorKind::InvalidCertificate
                .context("no authority key identifier extension found")
                .into()),
        },
        _ => Err(ErrorKind::InvalidCertificate
            .context("no authority key identifier extension found")
            .into()),
    }
}

pub(crate) fn extract_subject_key_identifier<'a>(
    x509: &'a X509Certificate,
) -> Result<KeyIdentifier<'a>, Error> {
    match x509
        .extensions_map()?
        .get(&x509_parser::oid_registry::OID_X509_EXT_SUBJECT_KEY_IDENTIFIER)
        .ok_or(ErrorKind::InvalidCertificate.context("no subject key identifier found"))?
        .parsed_extension()
    {
        ParsedExtension::SubjectKeyIdentifier(ski) => Ok(ski.clone()),
        _ => Err(ErrorKind::InvalidCertificate
            .context("no subject key identifier extension found")
            .into()),
    }
}
