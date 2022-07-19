use crate::{error::Error, prelude::APP};
use abscissa_core::Application;
use somm_proto::cork::{query_client::QueryClient, QueryCellarIDsRequest, QueryCellarIDsResponse};
use tonic::{transport::Channel, Response, Status};

/// Client wrapper used to refresh the cache
pub struct CorkQueryClient {
    inner: QueryClient<Channel>,
}

impl CorkQueryClient {
    /// Constructor
    pub async fn new() -> Result<Self, Error> {
        let config = APP.config();
        Ok(CorkQueryClient {
            inner: QueryClient::connect(config.cosmos.grpc.clone()).await?,
        })
    }

    /// Queries the Sommelier chain for approved Cellar IDs
    pub async fn get_approved_cellar_ids(
        &mut self,
    ) -> Result<Response<QueryCellarIDsResponse>, Status> {
        let request = QueryCellarIDsRequest {};
        self.inner.query_cellar_i_ds(request).await
    }
}
