use crate::error::{Error, ErrorKind};
use futures::stream::{FuturesUnordered};
use futures::future::join_all;
use somm_proto::cork::{query_client::QueryClient as QClient, QueryCommitPeriodRequest};
use steward_proto::{
    self,
    steward::{
        contract_call_client::ContractCallClient as ContractClient, SubmitRequest, SubmitResponse,
    },
};
use tonic::transport::{Channel, Uri, ClientTlsConfig};
use std::{thread, time::Duration, sync::Arc};

pub type ContractCallClient = ContractClient<Channel>;
pub type QueryClient = QClient<Channel>;

pub struct GrpcClient {
    pub grpc_address: String,
    pub steward_endpoints: Vec<String>,
    pub tls_configuration: ClientTlsConfig,
}

impl GrpcClient {
    pub fn check_for_grpc_address(&self) -> Result<(), Error> {
        if self.grpc_address.is_empty() {
            return Err(ErrorKind::GrpcError
                .context("no grpc address available")
                .into());
        }

        Ok(())
    }

    pub fn check_for_steward_addresses(&self) -> Result<(), Error> {
        if self.steward_endpoints.is_empty() {
            return Err(ErrorKind::GrpcError
                .context("no grpc address available")
                .into());
        }

        Ok(())
    }

    pub async fn get_query_client(&self) -> Result<QueryClient, Error> {
        self.check_for_grpc_address()?;

        QueryClient::connect(self.grpc_address.clone())
            .await
            .map_err(|e| ErrorKind::GrpcError.context(e).into())
    }

    pub async fn get_contract_clients(&self) -> Result<Vec<ContractCallClient>, Error> {
        self.check_for_steward_addresses()?;

        let mut clients = Vec::new();

        for addr in &self.steward_endpoints {
            let endpoint = Channel::builder(addr.parse::<Uri>().unwrap()).tls_config(self.tls_configuration.clone())?;

            let client = match ContractCallClient::connect(endpoint).await {
                Ok(res) => res,
                Err(err) => return Err(ErrorKind::GrpcError.context(err).into()),
            };

            clients.push(client);
        }

        return Ok(clients);
    }

    pub async fn submit_request(&self, request: SubmitRequest) -> Result<(Vec<SubmitResponse>, Vec<Error>), Error> {
        // First verify voting period has started
        let mut query_client = self.get_query_client().await?;

        loop {
            let commit_period_response = match query_client
                .query_commit_period(QueryCommitPeriodRequest {})
                .await
            {
                Ok(res) => res.into_inner(),
                Err(status) => return Err(ErrorKind::GrpcError.context(status).into()),
            };

            if commit_period_response.current_height >= commit_period_response.vote_period_start
                && commit_period_response.current_height < commit_period_response.vote_period_end
            {
                break;
            }

            // Wait a few seconds before retrying
            thread::sleep(Duration::from_secs(10));
        }

        // We're now in a voting period, time to send requests.
        let mut contract_clients = self.get_contract_clients().await?;

        // Executing all requests in parallel, so need to keep track
        let mut futures = Vec::new();

        for client in &mut contract_clients {
            futures.push(client.submit(request.clone()));            
        }

        let mut responses = Vec::new();
        let mut failures  = Vec::new();

        for future in join_all(futures).await {
            match future {
                Ok(res) => responses.push(res.into_inner()),
                Err(status) => failures.push(ErrorKind::GrpcError.context(status).into()),
            }
        }
 
        Ok((responses, failures))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assay::assay;

    #[assay]
    async fn get_grpc_client() {
        let mut client = GrpcClient {
            grpc_address: String::from(""),
        };

        assert!(client.get_contract_client().await.is_err());

        client = GrpcClient {
            grpc_address: String::from("https://google.com"),
        };

        client
            .get_contract_client()
            .await
            .expect("Could not retrieve client.");
    }
}
