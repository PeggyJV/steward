use crate::error::{Error, ErrorKind};
use futures::future::join_all;
use somm_proto::cork::{query_client::QueryClient as QClient, QueryCommitPeriodRequest};
use std::{thread, time::Duration};
use steward_proto::{
    self,
    steward::{
        contract_call_client::ContractCallClient as ContractClient, SubmitRequest, SubmitResponse,
    },
};
use tonic::transport::{Channel, ClientTlsConfig, Uri};

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
            let endpoint = Channel::builder(addr.parse::<Uri>().unwrap())
                .tls_config(self.tls_configuration.clone())?;

            let client = match ContractCallClient::connect(endpoint).await {
                Ok(res) => res,
                Err(err) => return Err(ErrorKind::GrpcError.context(err).into()),
            };

            clients.push(client);
        }

        Ok(clients)
    }

    pub async fn submit_request(
        &self,
        request: SubmitRequest,
    ) -> Result<(Vec<SubmitResponse>, Vec<Error>), Error> {
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
        let mut failures = Vec::new();

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
    async fn unhappy_grpc_client() {
        // Unhappy path tests
        let client = GrpcClient {
            grpc_address: String::from(""),
            steward_endpoints: Vec::new(),
            tls_configuration: ClientTlsConfig::new(),
        };

        assert!(client.get_query_client().await.is_err());
        assert!(client.get_contract_clients().await.is_err());
        assert!(client
            .submit_request(SubmitRequest {
                cellar_id: String::from("123"),
                call_data: None
            })
            .await
            .is_err());
    }

    #[assay]
    async fn happy_grpc_client() {
        // Happy path
        let client = GrpcClient {
            grpc_address: String::from("https://35.230.37.28:9090"),
            steward_endpoints: vec![String::from("https://google.com")],
            tls_configuration: ClientTlsConfig::new(),
        };

        // Expect ConnectionRefused error as this is essentially a mock call without TLS setup
        assert_eq!("grpc error: transport error: error trying to connect: tcp connect error: Connection refused (os error 61)", client.submit_request(SubmitRequest{cellar_id: String::from("123"), call_data: None}).await.err().expect("Unable to find err.").to_string());
    }
}
