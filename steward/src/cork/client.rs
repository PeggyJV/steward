use crate::{error::Error, prelude::APP};
use abscissa_core::{tracing::debug, Application};
use somm_proto::cork::{
    query_client::QueryClient as CorkQueryClient, QueryCellarIDsRequest, QueryCellarIDsResponse,
};
use tokio::sync::{Mutex, OnceCell, SetError};
use tonic::{transport::Channel, Response, Status};

static CORK_QUERY_CLIENT_WRAPPER: OnceCell<CorkClientWrapper> = OnceCell::const_new();

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
    pub async fn get_approved_cellar_ids(
        &self,
    ) -> Result<Response<QueryCellarIDsResponse>, Status> {
        let mut client = self.inner.lock().await;
        let request = QueryCellarIDsRequest {};
        client.query_cellar_i_ds(request).await
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

pub async fn init_cork_client() -> Result<(), SetError<CorkClientWrapper>> {
    CORK_QUERY_CLIENT_WRAPPER.set(CorkClientWrapper::new().await.unwrap())
}

pub async fn get_cork_client_wrapper() -> Option<&'static CorkClientWrapper> {
    if !CORK_QUERY_CLIENT_WRAPPER.initialized() {
        if let Err(err) = init_cork_client().await {
            panic!("failed to initialize cork query client: {}", err);
        }
    }

    CORK_QUERY_CLIENT_WRAPPER.get()
}
