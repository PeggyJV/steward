use crate::error::{Error, ErrorKind};
use deep_space::error::CosmosGrpcError;
use ethers::prelude::{types::Address as EthAddress, *};
use gravity_bridge::gravity_proto::gravity::{
    query_client::QueryClient, DelegateKeysByOrchestratorRequest,
    DelegateKeysByOrchestratorResponse,
};
use std::{convert::TryFrom, sync::Arc, time::Duration};
use tonic::transport::Channel;
use url::Url;

pub type EthSignerMiddleware = SignerMiddleware<Provider<Http>, LocalWallet>;
pub type EthClient = Arc<EthSignerMiddleware>;

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

fn check_scheme(input: &Url, original_string: &str) -> Result<(), Error> {
    if input.scheme() != "http" && input.scheme() != "https" {
        return Err(
            ErrorKind::Config.context(format!("Your url {} has an invalid scheme, please chose http or https",
            original_string)).into()
        );
    }

    Ok(())
}

pub async fn get_eth_provider(eth_rpc_url: &str) -> Result<Provider<Http>, Error> {
    let url = match Url::parse(eth_rpc_url) {
        Ok(u) => u,
        Err(err) => return Err(ErrorKind::Config.context(err).into()),
    };
    check_scheme(&url, eth_rpc_url)?;
    let eth_url = eth_rpc_url.trim_end_matches('/');

    Provider::<Http>::try_from(eth_url).map_err(|err| ErrorKind::Config.context(err).into())
}
