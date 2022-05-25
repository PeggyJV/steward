use abscissa_core::{
    tracing::{debug, error, warn},
    Application,
};
use lazy_static::lazy_static;
use std::{collections::HashSet, sync::RwLock, time::Duration};
use tokio::task::JoinHandle;

use crate::{cork::client::get_cork_client_wrapper, error::Error, prelude::APP};

pub type ApprovedCellarsCache = RwLock<HashSet<String>>;

lazy_static! {
    static ref APPROVED_CELLARS: ApprovedCellarsCache = ApprovedCellarsCache::default();
}

/// Indicates whether the provided address is contained in the approved cellars cache.
pub fn is_approved(cellar_id: &str) -> bool {
    let cellar_id = cellar_id.trim().to_lowercase();
    APPROVED_CELLARS
        .read()
        .unwrap()
        .iter()
        .any(|id| id == &cellar_id)
}

/// Flushes and reloads cache with normalized cellar id strings.
pub async fn refresh_approved_cellars() -> Result<(), Error> {
    debug!("refreshing approved cellars cache");
    let client_wrapper = get_cork_client_wrapper().await.unwrap();
    match client_wrapper.get_approved_cellar_ids().await {
        Ok(res) => {
            let approved_cellars = res
                .into_inner()
                .cellar_ids
                .into_iter()
                .map(|id| id.trim().to_lowercase())
                .collect();
            let mut cache = APPROVED_CELLARS.write().unwrap();

            *cache = approved_cellars;
        }
        // TO-DO: remake client if error is a connection problem
        Err(err) => return Err(err.into()),
    }

    Ok(())
}

/// Spawns the thread responsible for managing the cache of approved cellar IDs. The refresh
/// period can be configured via the `cork.cache_refresh_period` field (in seconds) in the steward
/// config file. The default period is 60 seconds.
pub async fn start_approved_cellar_cache_thread() -> JoinHandle<()> {
    let config = APP.config();
    let query_period = Duration::new(config.cork.cache_refresh_period, 0);

    tokio::spawn(async move {
        let mut fail_count = 0;
        loop {
            tokio::time::sleep(query_period).await;
            if let Err(err) = refresh_approved_cellars().await {
                fail_count += 1;
                error!("{}", err);
                warn!("the cache has been unable to refresh for {} minutes (refreshes once per minute).", fail_count)
            } else {
                fail_count = 0;
            }
        }
    })
}
