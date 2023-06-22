use std::{collections::HashMap, iter::FromIterator, time::Duration};

use abscissa_core::{
    tracing::log::{debug, error},
    Application,
};
use lazy_static::lazy_static;
use tokio::{
    sync::{mpsc::Sender, RwLock},
    task::JoinHandle,
};
use somm_proto::pubsub::Publisher;
use x509_parser::prelude::X509Certificate;

use crate::{
    error::Error, prelude::APP, pubsub::get_trust_state, server::extract_subject_key_identifier,
};

pub(crate) type PublisherTrustStateCache<'a> = RwLock<HashMap<Vec<u8>, PublisherTrustData<'a>>>;

lazy_static! {
    pub(crate) static ref PUBLISHER_TRUST_STATE_CACHE: PublisherTrustStateCache<'static> =
        PublisherTrustStateCache::default();
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PublisherTrustData<'a> {
    pub publisher: Publisher,
    pub publisher_ca_cert: X509Certificate<'a>,
    pub subscription_ids: Vec<String>,
}

pub(crate) async fn lookup_trust_data_by_subject_key_identifier(
    subject_key_identifier: &Vec<u8>,
) -> Option<PublisherTrustData> {
    let cache = PUBLISHER_TRUST_STATE_CACHE.read().await;
    cache.get(subject_key_identifier).cloned()
}

/// Overwrites the cache with the latest queried publisher subject key identifier/trust data pairs
pub async fn refresh_publisher_trust_state_cache() -> Result<bool, Error> {
    debug!("refreshing publisher trust state cache");
    let mut cache = PUBLISHER_TRUST_STATE_CACHE.write().await;
    let entries = get_trust_state().await?.into_iter().map(|td| {
        let subject_key_identifier = extract_subject_key_identifier(&td.publisher_ca_cert).unwrap();

        (subject_key_identifier.0.to_vec(), td)
    });
    let map = HashMap::from_iter(entries);
    debug!("cache map: {map:?}");

    let changed = *cache != map;

    *cache = map;

    Ok(changed)
}

/// Spawns the thread responsible for refreshing the publisher cache. The refresh
/// period can be configured via the `pubub.cache_refresh_period` field (in seconds) in the steward
/// config file. The default period is 60 seconds.
pub async fn start_publisher_trust_state_cache_thread(tx: Sender<()>) -> JoinHandle<()> {
    debug!("starting approved publisher cache thread");
    let config = APP.config();
    let query_period = Duration::from_secs(config.pubsub.cache_refresh_period);

    tokio::spawn(async move {
        let mut fail_count = 0;
        loop {
            match refresh_publisher_trust_state_cache().await {
                Ok(cache_changed) => {
                    fail_count = 0;

                    if cache_changed {
                        // signal that the server should be restarted
                        debug!(
                            "the publisher trust state cache has changed. signaling server restart"
                        );
                        tx.send(()).await.unwrap();
                    }
                }
                Err(err) => {
                    fail_count += 1;
                    error!(
                        "the publisher trust state cache has failed to refresh {} time(s): {}",
                        fail_count, err
                    );

                    // this is a bit of a hack to give the integration test time to run proposals to populate
                    // the cache
                    debug!("retrying quickly in case this is transient or an integration test.");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };
            tokio::time::sleep(query_period).await;
        }
    })
}
