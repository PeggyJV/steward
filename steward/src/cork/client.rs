use crate::{error::Error, prelude::APP};
use abscissa_core::{tracing::debug, Application};
use somm_proto::cork::query_client::QueryClient as CorkQueryClient;
use tokio::sync::{Mutex, MutexGuard, OnceCell};
use tonic::transport::Channel;

pub static CORK_QUERY_CLIENT: OnceCell<CorkClientWrapper> = OnceCell::const_new();

/// Wraps a mutex guarded CorkQueryClient. This allows us to store the client behind a
/// static OnceCell and still have the option of remaking the client if the connection
/// is dropped.
pub struct CorkClientWrapper {
    inner: Mutex<CorkQueryClient<Channel>>,
}

impl CorkClientWrapper {
    pub async fn new() -> Result<Self, Error> {
        Ok(CorkClientWrapper {
            inner: new_client().await?,
        })
    }

    /// Gets a lock on the wrapped query client
    pub async fn get_client(&self) -> MutexGuard<'_, CorkQueryClient<Channel>> {
        self.inner.lock().await
    }

    /// Replaces the wrapped query client with a new one. This establishes a fresh
    /// connection with the server.
    pub async fn remake_client(&mut self) -> Result<(), Error> {
        debug!("remaking cork query client");
        self.inner = new_client().await?;
        Ok(())
    }
}

async fn new_client() -> Result<Mutex<CorkQueryClient<Channel>>, Error> {
    let config = APP.config();
    Ok(Mutex::new(
        CorkQueryClient::connect(config.cosmos.grpc.clone()).await?,
    ))
}
