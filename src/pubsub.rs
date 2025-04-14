use std::collections::{HashMap, HashSet};

use abscissa_core::{
    tracing::log::{debug, error},
    Application,
};
use cosmos_sdk_proto_althea::cosmos::base::abci::v1beta1::TxResponse;
use somm_proto::pubsub::{
    query_client::QueryClient, QueryDefaultSubscriptionsRequest, QueryDefaultSubscriptionsResponse,
    QueryPublisherRequest, QueryPublisherResponse,
    QuerySubscriberIntentsBySubscriberAddressRequest,
    QuerySubscriberIntentsBySubscriberAddressResponse, Subscriber,
};
use url::Url;
use x509_parser::prelude::parse_x509_pem;

use crate::{
    cellars::to_checksum_address,
    config::get_delegate_address,
    error::{Error, ErrorKind},
    metrics::{TRUST_STATE_LOAD_ERRORS, TRUST_STATE_LOAD_SUCCESS},
    prelude::APP,
    somm_send,
};

use self::cache::PublisherTrustData;

pub(crate) mod cache;

/// Retrieves the PEM encoded CA certs for all default subscription publishers.
pub(crate) async fn get_trust_state() -> Result<Vec<PublisherTrustData<'static>>, Error> {
    debug!("collecting trust roots from default subscriptions and subscriber intents.");
    let config = APP.config();

    // load subscription mappings
    let mut subscription_mappings: HashMap<String, String> = HashMap::default();
    get_default_subscriptions()
        .await?
        .default_subscriptions
        .iter()
        .for_each(|ds| {
            subscription_mappings.insert(
                ds.subscription_id.to_owned(),
                ds.publisher_domain.to_owned(),
            );
        });

    let delegate_address = get_delegate_address().to_string();
    get_subscriber_intents_by_subscriber_address(delegate_address)
        .await?
        .subscriber_intents
        .iter()
        .for_each(|si| {
            subscription_mappings.insert(
                si.subscription_id.to_owned(),
                si.publisher_domain.to_owned(),
            );
        });

    if subscription_mappings.is_empty() {
        TRUST_STATE_LOAD_ERRORS.inc();
        return Err(ErrorKind::NoSubscriptions
            .context("did not find any subscriptions. clients will be unable to authenticate.")
            .into());
    }

    // filter out blocked publishers
    let subscription_mappings: HashMap<String, String> = subscription_mappings
        .into_iter()
        .filter_map(|(sid, domain)| {
            if config.pubsub.publisher_domain_block_list.contains(&domain) {
                debug!(
                    "publisher domain {} is in the block list. skipping.",
                    domain
                );

                return None;
            }

            // normalizing here means even if the casing is different on chain, we'll still be
            // able to match it, and if we can't normalize it then it's invalid and we ignore it.
            let (chain_id, cellar_id) = match subscription_id_parts(&sid) {
                Ok((chain_id, cellar_id)) => (chain_id, cellar_id),
                Err(err) => {
                    error!(
                        "failed to parse subscription ID {}. it will not be included in trust store: {}",
                        sid, err
                    );
                    return None;
                }
            };
            let Ok((cellar_id, _)) = to_checksum_address(&cellar_id) else {
                error!("failed to convert cellarID part of subscription ID to checksum: {sid}, it will not be included in trust store");
                return None;
            };
            let sid = format!("{}:{}", chain_id, cellar_id);

            Some((sid, domain))
        })
        .collect();

    // build trust state
    let mut states: HashMap<String, PublisherTrustData<'static>> = HashMap::new();
    for m in subscription_mappings {
        let (subscription_id, publisher_domain_name) = m;

        if states.contains_key(&publisher_domain_name) {
            states.entry(publisher_domain_name).and_modify(|v| {
                v.subscription_ids.insert(subscription_id);
            });

            continue;
        }

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

        debug!(
            "trust subscription {} for publisher {}",
            subscription_id, publisher.domain
        );

        states.insert(
            publisher_domain_name,
            PublisherTrustData {
                publisher,
                subscription_ids: HashSet::from([subscription_id]),
                publisher_ca_cert,
            },
        );
    }

    TRUST_STATE_LOAD_SUCCESS.inc();
    Ok(states.values().cloned().collect())
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

pub(crate) async fn add_subscriber(push_url: String, ca_cert: String) -> Result<TxResponse, Error> {
    let subscriber = Subscriber {
        address: get_delegate_address().to_string(),
        push_url,
        ca_cert,
    };

    somm_send::add_subscriber(subscriber)
        .await
        .map_err(|e| ErrorKind::GrpcError.context(e).into())
}

pub(crate) async fn remove_subscriber() -> Result<TxResponse, Error> {
    let tx_response = somm_send::remove_subscriber()
        .await
        .map_err(|e| ErrorKind::GrpcError.context(e))?;

    Ok(tx_response)
}

pub(crate) async fn subscribe(
    cellar_id: String,
    publisher_domain: String,
) -> Result<TxResponse, Error> {
    somm_send::subscribe(cellar_id, publisher_domain)
        .await
        .map_err(|e| ErrorKind::GrpcError.context(e).into())
}

pub(crate) async fn unsubscribe(cellar_id: String) -> Result<TxResponse, Error> {
    somm_send::unsubscribe(cellar_id)
        .await
        .map_err(|e| ErrorKind::GrpcError.context(e).into())
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

pub(crate) fn subscription_id_parts(sid: &str) -> Result<(String, String), Error> {
    let parts: Vec<&str> = sid.split(':').collect();
    if parts.len() != 2 {
        return Err(ErrorKind::InvalidSubscriptionId.into());
    }

    Ok((parts[0].to_string(), parts[1].to_string()))
}
