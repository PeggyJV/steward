use std::collections::{HashMap, HashSet};

use crate::{cellars::to_checksum_address, error::Error, prelude::APP};
use abscissa_core::{tracing::log::error, Application};
use somm_proto::{
    axelar_cork::{
        query_client::QueryClient as AxelarQueryClient,
        QueryCellarIDsRequest as AxelarQueryCellarIDsRequest,
        QueryScheduledCorksByIdRequest as AxelarQueryScheduledCorksByIdRequest,
        QueryScheduledCorksByIdResponse as AxelarQueryScheduledCorksByIdResponse,
    },
    cork::{
        query_client::QueryClient, QueryCellarIDsRequest, QueryScheduledCorksByIdRequest,
        QueryScheduledCorksByIdResponse,
    },
};
use tonic::{transport::Channel, Response, Status};

/// Client wrapper used to refresh the cache
pub struct CorkQueryClient {
    inner_cork: QueryClient<Channel>,
    inner_axelar: AxelarQueryClient<Channel>,
}

impl CorkQueryClient {
    /// Constructor
    pub async fn new() -> Result<Self, Error> {
        let config = APP.config();
        Ok(CorkQueryClient {
            inner_cork: QueryClient::connect(config.cosmos.grpc.clone()).await?,
            inner_axelar: AxelarQueryClient::connect(config.cosmos.grpc.clone()).await?,
        })
    }

    /// Queries the Sommelier chain for approved Cellar IDs
    pub async fn get_approved_cellar_ids(
        &mut self,
    ) -> Result<HashMap<u64, HashSet<String>>, Status> {
        let cork_request = QueryCellarIDsRequest {};
        let cork_result = self.inner_cork.query_cellar_i_ds(cork_request).await?;
        let axelarcork_request = AxelarQueryCellarIDsRequest {};
        let axelarcork_result = self
            .inner_axelar
            .query_cellar_i_ds(axelarcork_request)
            .await?;

        let mut result: HashMap<u64, HashSet<String>> = HashMap::new();
        for id in cork_result.into_inner().cellar_ids {
            let (normalized_id, _) = match to_checksum_address(&id) {
                Ok(addr) => addr,
                Err(err) => {
                    error!("failed to get checksum of cellar ID {}, it will not be cached as approved: {}", id, err);
                    continue;
                }
            };

            result.entry(1).or_default().insert(normalized_id);
        }

        for set in axelarcork_result.into_inner().cellar_ids {
            let chain_id = set.chain_id;
            let cellar_ids = set.ids.clone();
            for id in cellar_ids.into_iter() {
                let (normalized_id, _) = match to_checksum_address(&id) {
                    Ok(addr) => addr,
                    Err(err) => {
                        error!("failed to get checksum of axelar cellar ID {}, it will not be cached as approved: {}", id, err);
                        continue;
                    }
                };

                result.entry(chain_id).or_default().insert(normalized_id);
            }
        }

        Ok(result)
    }

    /// Queries scheduled corks by ID
    pub async fn get_scheduled_corks_by_id(
        &mut self,
        id: &str,
    ) -> Result<Response<QueryScheduledCorksByIdResponse>, Status> {
        let request = QueryScheduledCorksByIdRequest { id: id.to_string() };

        self.inner_cork.query_scheduled_corks_by_id(request).await
    }

    pub async fn get_axelar_scheduled_corks_by_id(
        &mut self,
        chain_id: u64,
        id: &str,
    ) -> Result<Response<AxelarQueryScheduledCorksByIdResponse>, Status> {
        let request = AxelarQueryScheduledCorksByIdRequest {
            chain_id,
            id: id.to_string(),
        };

        self.inner_axelar.query_scheduled_corks_by_id(request).await
    }
}
