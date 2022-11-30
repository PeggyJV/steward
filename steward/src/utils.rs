use crate::{
    application::APP,
    error::{Error, ErrorKind},
};
use abscissa_core::Application;
use deep_space::error::CosmosGrpcError;
use ethers::prelude::{types::Address as EthAddress, *};
use gravity_bridge::{
    gravity_proto::gravity::{
        query_client::QueryClient, DelegateKeysByOrchestratorRequest,
        DelegateKeysByOrchestratorResponse,
    },
    gravity_utils::ethereum::downcast_to_u64,
};
use std::{convert::TryFrom, time::Duration};
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

pub fn string_to_u256(value: String) -> Result<U256, Error> {
    match U256::from_dec_str(value.as_str()) {
        Ok(v) => Ok(v),
        Err(_) => Err(ErrorKind::SPCall
            .context(format!("failed to parse amount {} to U256", value))
            .into()),
    }
}

pub async fn get_chain(eth_client: Provider<Http>) -> Result<Chain, Error> {
    let chain_id_result = eth_client.get_chainid().await?;
    let chain_id = downcast_to_u64(chain_id_result);

    if chain_id.is_none() {
        return Err(ErrorKind::ClientError
            .context(format!(
                "Chain ID is larger than u64 max: {}",
                chain_id_result
            ))
            .into());
    }

    // We're only currently looking for ETHERSCAN_API_KEY, so only support
    // Ethereum networks. Returning mainnet as a default in absence of a better
    // option. Strangely there is no function in ethers to convert from a chain
    // ID to a Chain enum value.
    Ok(match chain_id.unwrap() {
        1 => Chain::Mainnet,
        3 => Chain::Ropsten,
        4 => Chain::Rinkeby,
        5 => Chain::Goerli,
        42 => Chain::Kovan,
        _ => Chain::Mainnet,
    })
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

pub async fn get_eth_provider() -> Result<Provider<Http>, Error> {
    let config = APP.config();
    let url = &config.ethereum.rpc;
    let eth_url = url.trim_end_matches('/');

    Provider::<Http>::try_from(eth_url).map_err(|err| ErrorKind::Config.context(err).into())
}

pub fn sp_call_error(message: String) -> Error {
    ErrorKind::SPCall.context(message).into()
}

pub fn sp_call_parse_address(address: String) -> Result<H160, Error> {
    match address.parse::<H160>() {
        Ok(addr) => Ok(addr),
        Err(err) => Err(sp_call_error(err.to_string())),
    }
}
