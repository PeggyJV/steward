use std::collections::HashMap;

use abscissa_core::{
    tracing::log::{debug, error},
    Application,
};
use somm_proto::{
    cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse,
    pubsub::{
        query_client::QueryClient, QueryDefaultSubscriptionsRequest,
        QueryDefaultSubscriptionsResponse, QueryPublisherRequest, QueryPublisherResponse,
        QuerySubscriberIntentsBySubscriberAddressRequest,
        QuerySubscriberIntentsBySubscriberAddressResponse, Subscriber,
    },
};
use url::Url;
use x509_parser::prelude::parse_x509_pem;

use crate::{
    config::get_delegate_address,
    error::{Error, ErrorKind},
    prelude::APP,
    somm_send,
};

use self::cache::PublisherTrustData;

pub(crate) mod cache;

/// Retrieves the PEM encoded CA certs for all default subscription publishers.
pub(crate) async fn get_trust_state() -> Result<Vec<PublisherTrustData<'static>>, Error> {
    debug!("collecting trust roots from default subscriptions and subscriber intents.");

    // load subscription mappings
    let mut subscription_mappings: HashMap<String, Vec<String>> = HashMap::default();
    get_default_subscriptions()
        .await?
        .default_subscriptions
        .iter()
        .for_each(|s| {
            subscription_mappings
                // if there is an entry for the key, push to the vec and dedup,
                // otherwise create a new vec with the subscription id
                .entry(s.publisher_domain.to_owned())
                .and_modify(|e| {
                    e.push(s.subscription_id.to_owned());
                    e.dedup();
                })
                .or_insert(vec![s.subscription_id.to_owned()]);
        });
    let delegate_address = get_delegate_address().to_string();
    get_subscriber_intents_by_subscriber_address(delegate_address)
        .await?
        .subscriber_intents
        .iter()
        .for_each(|si| {
            subscription_mappings
                .entry(si.publisher_domain.to_owned())
                .and_modify(|e| {
                    e.push(si.subscription_id.to_owned());
                    e.dedup();
                })
                .or_insert(vec![si.subscription_id.to_owned()]);
        });

    if subscription_mappings.is_empty() {
        return Err(ErrorKind::NoSubscriptions
            .context("did not find any subscriptions. clients will be unable to authenticate.")
            .into());
    }

    // build trust state
    let mut states: Vec<PublisherTrustData<'static>> = Vec::new();
    for m in subscription_mappings {
        let (publisher_domain_name, subscription_ids) = m;
        let publisher = match get_publisher(publisher_domain_name.clone()).await {
            Ok(p) => p.publisher.unwrap(),
            Err(e) => {
                error!("failed to get publisher with domain {publisher_domain_name}: {e}");
                continue;
            }
        };

        // The way to create a struct with a 'static lifetime parameter is by boxing the underlying
        // data and then leaking a reference to it. Otherwise, you get get an error that the struct
        // outlives the data it references. By using Box::leak, we can ensure that the memory is cleaned
        // up when the reference is dropped on cache refresh, avoiding an actual memory leak.
        let data = Box::new(publisher.ca_cert.as_bytes());
        let data = Box::leak(data);
        let pem = match parse_x509_pem(data) {
            Ok((_, pem)) => Box::new(pem),
            Err(e) => {
                error!("failed to parse x509 pem for publisher {publisher_domain_name}: {e}");
                continue;
            }
        };
        let publisher_ca_cert = Box::leak(pem).parse_x509().unwrap();

        states.push(PublisherTrustData {
            publisher,
            subscription_ids,
            publisher_ca_cert,
        });
    }

    Ok(states)
}

async fn get_subscriber_intents_by_subscriber_address(
    subscriber_address: String,
) -> Result<QuerySubscriberIntentsBySubscriberAddressResponse, Error> {
    let config = APP.config();
    let mut client = QueryClient::connect(config.cosmos.grpc.clone())
        .await
        .map_err(|e| ErrorKind::GrpcError.context(e))?;
    let request = QuerySubscriberIntentsBySubscriberAddressRequest { subscriber_address };

    Ok(client
        .query_subscriber_intents_by_subscriber_address(request)
        .await
        .map_err(|e| match e.code() {
            tonic::Code::InvalidArgument => ErrorKind::InvalidArgument.context(e),
            _ => ErrorKind::GrpcError.context(e),
        })?
        .into_inner())
}

async fn get_default_subscriptions() -> Result<QueryDefaultSubscriptionsResponse, Error> {
    let config = APP.config();
    let mut client = QueryClient::connect(config.cosmos.grpc.clone())
        .await
        .map_err(|e| ErrorKind::GrpcError.context(e))?;
    let request = QueryDefaultSubscriptionsRequest {};

    Ok(client
        .query_default_subscriptions(request)
        .await
        .map_err(|e| ErrorKind::GrpcError.context(e))?
        .into_inner())
}

pub(crate) async fn get_publisher(domain: String) -> Result<QueryPublisherResponse, Error> {
    let config = APP.config();
    let mut client = QueryClient::connect(config.cosmos.grpc.clone())
        .await
        .map_err(|e| ErrorKind::GrpcError.context(e))?;
    let request = QueryPublisherRequest {
        publisher_domain: domain,
    };

    Ok(client
        .query_publisher(request)
        .await
        .map_err(|e| match e.code() {
            tonic::Code::InvalidArgument => ErrorKind::InvalidArgument.context(e),
            tonic::Code::NotFound => ErrorKind::PublisherNotFound.context(e),
            _ => ErrorKind::GrpcError.context(e),
        })?
        .into_inner())
}

pub(crate) async fn add_subscriber(domain: String, ca_cert: String) -> Result<TxResponse, Error> {
    let subscriber = Subscriber {
        address: get_delegate_address().to_string(),
        domain,
        ca_cert,
    };

    somm_send::add_subscriber(subscriber)
        .await
        .map_err(|e| ErrorKind::GrpcError.context(e).into())
}

pub(crate) async fn remove_subscriber() -> Result<TxResponse, Error> {
    somm_send::remove_subscriber()
        .await
        .map_err(|e| ErrorKind::GrpcError.context(e).into())
}

pub(crate) async fn subscribe(
    cellar_id: String,
    publisher_domain: String,
    subscriber_url: String,
) -> Result<TxResponse, Error> {
    somm_send::subscribe(cellar_id, publisher_domain, subscriber_url)
        .await
        .map_err(|e| ErrorKind::GrpcError.context(e).into())
}

pub(crate) async fn unsubscribe(cellar_id: String) -> Result<TxResponse, Error> {
    somm_send::unsubscribe(cellar_id)
        .await
        .map_err(|e| ErrorKind::GrpcError.context(e).into())
}

/// Validates that a given domain is a valid FQDN
pub(crate) fn validate_domain_name(dn: &str) -> Result<(), Error> {
    addr::parse_domain_name(dn).map_err(|_| {
        Into::<Error>::into(
            ErrorKind::InvalidDomainName.context(format!("failed to parse domain name {dn}")),
        )
    })?;

    Ok(())
}

/// Validates a URL is parsable as such
/// (Collin): How can we make this more thorough? Seems like a step that
/// will pay dividends in avoided validator onboarding overhead.
pub(crate) fn validate_url(url: &str) -> Result<(), Error> {
    Url::parse(url).map_err(|_e| {
        Into::<Error>::into(ErrorKind::ParsingError.context(format!("failed to parse URL {url}")))
    })?;

    Ok(())
}

/// Validate PEM encoded CA cert
pub(crate) fn validate_ca_cert(data: &[u8]) -> Result<(), Error> {
    let (_, pem) = parse_x509_pem(data)
        .map_err(|e| Into::<Error>::into(ErrorKind::InvalidCertificate.context(e)))?;
    let _ = pem
        .parse_x509()
        .map_err(|e| Into::<Error>::into(ErrorKind::InvalidCertificate.context(e)))?;

    Ok(())
}