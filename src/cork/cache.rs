use abscissa_core::{
    tracing::{debug, error},
    Application,
};
use lazy_static::lazy_static;
use std::{collections::HashSet, sync::RwLock, time::Duration};
use tokio::task::JoinHandle;

use crate::{cork::client::CorkQueryClient, error::Error, prelude::APP};

pub type ApprovedCellarsCache = RwLock<HashSet<String>>;

lazy_static! {
    static ref APPROVED_CELLARS: ApprovedCellarsCache = ApprovedCellarsCache::default();
}

/// Indicates whether an address is in the approved cellars cache
pub fn is_approved(cellar_id: &str) -> bool {
    let cellar_id = cellar_id.trim().to_lowercase();
    APPROVED_CELLARS.read().unwrap().contains(&cellar_id)
}

/// Overwrites the cache with the latest queried cellar IDs
pub async fn refresh_approved_cellars() -> Result<(), Error> {
    debug!("refreshing approved cellars cache");
    let mut client = CorkQueryClient::new().await?;
    match client.get_approved_cellar_ids().await {
        Ok(res) => {
            let mut cache = APPROVED_CELLARS.write().unwrap();
            *cache = res
                .into_inner()
                .cellar_ids
                .into_iter()
                .map(|id| id.trim().to_lowercase())
                .collect();
        }
        Err(err) => return Err(err.into()),
    }

    Ok(())
}

/// Spawns the thread responsible for refreshing the cache of approved cellar IDs. The refresh
/// period can be configured via the `cork.cache_refresh_period` field (in seconds) in the steward
/// config file. The default period is 60 seconds.
pub async fn start_approved_cellar_cache_thread() -> JoinHandle<()> {
    debug!("starting approved cellar cache thread");
    let config = APP.config();
    let query_period = Duration::from_secs(config.cork.cache_refresh_period);

    tokio::spawn(async move {
        let mut fail_count = 0;
        loop {
            if let Err(err) = refresh_approved_cellars().await {
                fail_count += 1;
                error!(
                    "the approved cellars cache has failed to refresh {} time(s): {}",
                    fail_count, err
                );
            } else {
                fail_count = 0;
            }

            tokio::time::sleep(query_period).await;
        }
    })
}
