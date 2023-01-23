use crate::{
    application::APP,
    error::{Error, ErrorKind},
};
use abscissa_core::Application;
use deep_space::error::CosmosGrpcError;
use deep_space::Address as CosmosAddress;
use ethers::{
    abi::Token,
    prelude::{types::Address as EthAddress, *},
};
use gravity_bridge::{
    gravity_proto::gravity::{
        query_client::QueryClient, DelegateKeysByOrchestratorRequest,
        DelegateKeysByOrchestratorResponse,
    },
    gravity_utils::ethereum::downcast_to_u64,
};
use std::{convert::TryFrom, time::Duration};
use steward_proto::steward::{
    oracle_swap_params::Params::{
        Univ2Params as UniV2OracleParams, Univ3Params as UniV3OracleParams,
    },
    swap_params::Params::*,
    OracleSwapParams, SwapParams,
};
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

pub fn string_to_u128(value: String) -> Result<U128, Error> {
    match U128::from_dec_str(value.as_str()) {
        Ok(v) => Ok(v),
        Err(_) => Err(ErrorKind::SPCallError
            .context(format!("failed to parse amount {} to U128", value))
            .into()),
    }
}

pub fn string_to_u256(value: String) -> Result<U256, Error> {
    match U256::from_dec_str(value.as_str()) {
        Ok(v) => Ok(v),
        Err(_) => Err(ErrorKind::SPCallError
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
    ErrorKind::SPCallError.context(message).into()
}

pub fn sp_call_parse_address(address: String) -> Result<H160, Error> {
    match address.parse::<H160>() {
        Ok(addr) => Ok(addr),
        Err(err) => Err(sp_call_error(err.to_string())),
    }
}

pub fn governance_call_error(message: String) -> Error {
    ErrorKind::GovernanceCall.context(message).into()
}

/// Encodes the Cosmos address into a big-endian 32 byte array pre-padded with zeros. Since a Cosmos address is 20
/// bytes, we copy it into a zeroed-out 32 byte array starting at index 12.
pub fn encode_fees_distributor_address(address: CosmosAddress) -> [u8; 32] {
    let mut address_bytes_slice: [u8; 32] = Default::default();
    address_bytes_slice[12..].copy_from_slice(address.as_bytes());

    address_bytes_slice
}

// to account for protobuf's requirement that an UNSPECIFIED enum variant be defined
// as 0, we subtract 1 from the value
pub fn convert_exchange(value: i32) -> u8 {
    (value - 1) as u8
}

/// Encodes the swap params as ABI-encoded bytes to be passed as args to the underlying
/// swap function
pub fn encode_swap_params(params: SwapParams) -> Result<Vec<u8>, Error> {
    match params
        .params
        .ok_or_else(|| sp_call_error("swap params cannot be unspecified".to_string()))?
    {
        Univ2Params(p) => {
            let mut path = Vec::<Token>::new();

            for a in p.path {
                let address = a.parse::<EthAddress>();
                if address.is_err() {
                    return Err(sp_call_error(format!(
                        "could not parse swap params path address: {}",
                        a
                    )));
                }

                path.push(Token::Address(address.unwrap()))
            }

            let path = Token::Array(path);
            let amount = Token::Uint(string_to_u256(p.amount)?);
            let amount_out_min = Token::Uint(string_to_u256(p.amount_out_min)?);
            Ok(abi::encode(&[path, amount, amount_out_min]))
        }
        Univ3Params(p) => {
            let mut path = Vec::<Token>::new();
            for a in p.path {
                let address = a.parse::<EthAddress>();
                if address.is_err() {
                    return Err(sp_call_error(format!(
                        "could not parse swap params path address: {}",
                        a
                    )));
                }

                path.push(Token::Address(address.unwrap()))
            }

            let path = Token::Array(path);

            let mut pool = Vec::<Token>::new();
            for f in p.pool_fees {
                pool.push(Token::Uint(f.into()))
            }

            let pool = Token::Array(pool);
            let amount = Token::Uint(string_to_u256(p.amount)?);
            let amount_out_min = Token::Uint(string_to_u256(p.amount_out_min)?);

            Ok(abi::encode(&[path, pool, amount, amount_out_min]))
        }
    }
}

/// Encodes the oracle swap params as ABI-encoded bytes to be passed as args to the underlying
/// oracle swap function
pub fn encode_oracle_swap_params(params: OracleSwapParams) -> Result<Vec<u8>, Error> {
    match params
        .params
        .ok_or_else(|| sp_call_error("swap params cannot be unspecified".to_string()))?
    {
        UniV2OracleParams(p) => {
            let mut path = Vec::<Token>::new();

            for a in p.path {
                let address = a.parse::<EthAddress>();
                if address.is_err() {
                    return Err(sp_call_error(format!(
                        "could not parse swap params path address: {}",
                        a
                    )));
                }

                path.push(Token::Address(address.unwrap()))
            }

            let path = Token::Array(path);
            Ok(abi::encode(&[path]))
        }
        UniV3OracleParams(p) => {
            let mut path = Vec::<Token>::new();
            for a in p.path {
                let address = a.parse::<EthAddress>();
                if address.is_err() {
                    return Err(sp_call_error(format!(
                        "could not parse swap params path address: {}",
                        a
                    )));
                }

                path.push(Token::Address(address.unwrap()))
            }

            let path = Token::Array(path);

            let mut pool = Vec::<Token>::new();
            for f in p.pool_fees {
                pool.push(Token::Uint(f.into()))
            }

            let pool = Token::Array(pool);

            Ok(abi::encode(&[path, pool]))
        }
    }
}
