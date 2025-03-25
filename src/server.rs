use std::{net::SocketAddr, sync::Arc};

use crate::{
    config::StewardConfig,
    cork::CorkHandler,
    // encode::EncodingHandler,
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
    tracing::{
        debug,
        log::{error, info},
    },
    Application,
};
use hyper::server::conn::http2::Builder;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    service::TowerToHyperService,
};
use rustls::{
    pki_types::{pem::PemObject, PrivateKeyDer},
    server::WebPkiClientVerifier,
};
use tokio::{net::TcpListener, sync::mpsc::Receiver};
use tokio_rustls::{rustls::pki_types::CertificateDer, TlsAcceptor};
use tokio_util::sync::CancellationToken;
use tonic::{body::boxed, service::Routes, Status};
use tower::ServiceExt;
use tower_http::ServiceBuilderExt;
use x509_parser::prelude::{FromDer, KeyIdentifier, ParsedExtension, X509Certificate};

// for gRPC reflection
pub const FILE_DESCRIPTOR_SET: &[u8] =
    include_bytes!("../crates/steward-proto/src/gen/descriptor.bin");

#[derive(Debug)]
pub(crate) struct ConnectionInfo {
    pub(crate) address: SocketAddr,
    pub(crate) certificates: Vec<CertificateDer<'static>>,
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
        .build_v1()
        .unwrap_or_else(|err| {
            status_err!("failed to build descriptor service: {}", err);
            std::process::exit(1)
        });
    let config = APP.config();
    let server_config = auth_config(&config)
        .await
        .expect("failed to get server config");
    let address = socket_addr(&config).expect("failed to get socket address");
    let svc = Routes::new(ContractCallServiceServer::new(CorkHandler))
        .add_service(StatusServiceServer::new(StatusHandler))
        .add_service(proto_descriptor_service)
        .prepare();
    let http = Builder::new(TokioExecutor::new());
    let listener = TcpListener::bind(address)
        .await
        .expect("failed to bind to address");
    let tls_acceptor = TlsAcceptor::from(Arc::new(server_config));

    // Start accepting connections
    info!("listening on {}", address);
    loop {
        let (connection, address) = match listener.accept().await {
            Ok(incoming) => incoming,
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
                continue;
            }
        };

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

            let connection_info = Arc::new(ConnectionInfo {
                address,
                certificates,
            });

            let svc = tower::ServiceBuilder::new()
                .add_extension(connection_info.clone())
                .service(svc);

            http.serve_connection(
                TokioIo::new(conn),
                TowerToHyperService::new(svc.map_request(move |req: http::Request<_>| {
                    let mut req = req.map(boxed);
                    req.extensions_mut().insert(connection_info.clone());
                    req
                })),
            )
            .await
            .unwrap();
        });
    }
}

// pub(crate) async fn start_encode_server() {
//     // Reflection required for certain clients to function... such as grpcurl
//     let contents = FILE_DESCRIPTOR_SET.to_vec();
//     let proto_descriptor_service = tonic_reflection::server::Builder::configure()
//         .register_encoded_file_descriptor_set(contents.as_slice())
//         .build_v1()
//         .unwrap_or_else(|err| {
//             status_err!("failed to build descriptor service: {}", err);
//             std::process::exit(1)
//         });
//     let config = APP.config();
//     let server_config = load_encoding_server_config(&config)
//         .await
//         .unwrap_or_else(|err| {
//             status_err!("failed to load server config: {}", err);
//             std::process::exit(1)
//         });

//     info!("test server listening on {}", server_config.address);
//     if let Err(err) = tonic::transport::Server::builder()
//         .add_service(EncodingServiceServer::new(EncodingHandler))
//         .add_service(proto_descriptor_service)
//         .serve(server_config.address)
//         .await
//     {
//         status_err!("server error: {}", err);
//         std::process::exit(1)
//     }
// }

pub(crate) async fn get_client_root_store() -> Result<rustls::RootCertStore, Error> {
    let mut trust_store = rustls::RootCertStore::empty();
    let cache = PUBLISHER_TRUST_STATE_CACHE.read().await;
    for td in cache.values() {
        let mut cert = td.publisher.ca_cert.as_bytes();
        let cert = match rustls_pemfile::certs(&mut cert).next() {
            Some(Ok(cert)) => cert,
            Some(Err(e)) => {
                error!("failed to parse certificate for trust data {td:?}: {e}");
                continue;
            }
            None => {
                error!("empty publisher CA certificate for trust data: {td:?}");
                continue;
            }
        };
        trust_store.add(cert).unwrap();
    }

    debug!("client root store: {:?}", trust_store);

    Ok(trust_store)
}

pub(crate) async fn auth_config(
    config: &Arc<StewardConfig>,
) -> Result<rustls::ServerConfig, Error> {
    // Set crypto provider
    let _ = rustls::crypto::ring::default_provider().install_default();

    // Server material
    let cert = tokio::fs::read(&config.server.server_cert_path).await?;
    let key = tokio::fs::read(&config.server.server_key_path).await?;
    let cert = CertificateDer::from_pem_slice(&cert).expect("failed to parse server cert");
    let key = PrivateKeyDer::from_pem_slice(&key).expect("failed to parse server key");

    // Client verifier
    let trust_store = get_client_root_store().await?;
    let roots = Arc::new(trust_store);
    let client_verifier = WebPkiClientVerifier::builder(roots).build().unwrap();

    // Server config
    let mut server_config = rustls::ServerConfig::builder()
        .with_client_cert_verifier(client_verifier)
        .with_single_cert(vec![cert], key)
        .expect("failed to build rustls server config");
    server_config.alpn_protocols = vec![b"h2".to_vec()];
    server_config.ignore_client_order = true; // Be more flexible with client preferences
    server_config.max_early_data_size = 0; // Disable early data to ensure proper handshake

    Ok(server_config)
}

pub(crate) fn socket_addr(config: &Arc<StewardConfig>) -> Result<SocketAddr, Error> {
    let port = &config.server.port;
    let address = &config.server.address;
    let address: SocketAddr = format!("{}:{}", address, port).parse()?;

    Ok(address)
}

// pub async fn load_encoding_server_config(
//     config: &Arc<StewardConfig>,
// ) -> Result<ServerConfig, Error> {
//     let address = socket_addr(config)?;

//     Ok(ServerConfig {
//         tls_config: None,
//         address,
//     })
// }

/// A client is authorized to publish to a subscription if the following criteria are met:
/// 1. The client certificate has an authority key identifier extension whose value is the key of an entry
///    in the publisher trust state cache
/// 2. The client certificate is signed by the publisher's CA certificate
/// 3. The entry's list of subscription IDs contains the subscription ID of the request
pub(crate) async fn handle_authorization(
    subscription_id: &String,
    certs: Vec<CertificateDer<'static>>,
) -> Result<(), Status> {
    let mut client_cert: Option<X509Certificate> = None;

    // In case the request contains multiple certificates, we iterate until the first valid one.
    for cert in certs.iter() {
        let (_, cert_candidate) = match X509Certificate::from_der(cert) {
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
