use crate::error::{Error, ErrorKind};
use steward_proto::{
    self,
    steward::{self, contract_call_client::ContractCallClient},
};

use tonic::transport::Channel;

pub type ContractCallGrpcClient = ContractCallClient<Channel>;

pub struct GrpcClient {
    pub grpc_address: String,
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
    pub async fn get_client(&self) -> Result<ContractCallGrpcClient, Error> {
        self.check_for_grpc_address()?;

        ContractCallGrpcClient::connect(self.grpc_address.clone())
            .await
            .map_err(|e| ErrorKind::GrpcError.context(e).into())
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

        assert!(client.get_client().await.is_err());

        client = GrpcClient {
            grpc_address: String::from("https://google.com"),
        };

        client
            .get_client()
            .await
            .expect("Could not retrieve client.");
    }
}
