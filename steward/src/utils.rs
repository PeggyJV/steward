use deep_space::error::CosmosGrpcError;
use ethers::types::Address as EthAddress;
use gravity_bridge::gravity_proto::gravity::{
    query_client::QueryClient, DelegateKeysByOrchestratorRequest,
    DelegateKeysByOrchestratorResponse,
};
use std::time::Duration;
use tonic::transport::Channel;

pub const TIMEOUT: Duration = Duration::from_secs(60);

pub fn format_eth_address(address: EthAddress) -> String {
    format!("0x{}", bytes_to_hex_str(address.as_bytes()))
}

pub fn bytes_to_hex_str(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:0>2x?}", b))
        .fold(String::new(), |acc, x| acc + &x)
}

pub async fn get_delegates_keys_by_orchestrator(
    client: &mut QueryClient<Channel>,
    orch_address: String,
) -> Result<DelegateKeysByOrchestratorResponse, CosmosGrpcError> {
    let request = DelegateKeysByOrchestratorRequest {
        orchestrator_address: orch_address,
    };
    let response = client.delegate_keys_by_orchestrator(request).await?;
    let keys = response.into_inner();

    Ok(keys)
}
