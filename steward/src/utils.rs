use crate::error::Error;
use abscissa_core::tracing::{log::warn, info};
use deep_space::error::CosmosGrpcError;
use ethers::prelude::{*, types::Address as EthAddress};
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

/// Verify that a url has an http or https prefix
fn check_scheme(input: &Url, original_string: &str) {
    if !(input.scheme() == "http" || input.scheme() == "https") {
        panic!(
            "Your url {} has an invalid scheme, please chose http or https",
            original_string
        )
    }
}

pub async fn get_eth_provider(eth_rpc_url: &str) -> Result<Provider<Http>, Error> {
    let url = Url::parse(eth_rpc_url)
        .unwrap_or_else(|_| panic!("Invalid Ethereum RPC url {}", eth_rpc_url));
    check_scheme(&url, eth_rpc_url);
    let eth_url = eth_rpc_url.trim_end_matches('/');
    let base_eth_provider = Provider::<Http>::try_from(eth_url).unwrap_or_else(|_| {
        panic!("Could not instantiate Ethereum HTTP provider: {}", eth_url)
    });
    let try_base = base_eth_provider.get_block_number().await;
    match try_base {
        // it worked, lets go!
        Ok(_) => Ok(base_eth_provider),
        // did not work, now we check if it's localhost
        Err(e) => {
            warn!("Failed to access Ethereum RPC with {:?} trying fallback options", e);
            if eth_url.to_lowercase().contains("localhost") {
                let port = url.port().unwrap_or(80);
                // this should be http or https
                let prefix = url.scheme();
                let ipv6_url = format!("{}://::1:{}", prefix, port);
                let ipv4_url = format!("{}://127.0.0.1:{}", prefix, port);
                let ipv6_eth_provider = Provider::<Http>::try_from(ipv6_url.as_str())
                    .unwrap_or_else(|_| {
                        panic!(
                            "Could not instantiate Ethereum HTTP provider: {}",
                            &ipv6_url
                        )
                    });
                let ipv4_eth_provider = Provider::<Http>::try_from(ipv4_url.as_str())
                    .unwrap_or_else(|_| {
                        panic!(
                            "Could not instantiate Ethereum HTTP provider: {}",
                            &ipv4_url
                        )
                    });
                let ipv6_test = ipv6_eth_provider.get_block_number().await;
                let ipv4_test = ipv4_eth_provider.get_block_number().await;
                warn!("Trying fallback urls {} {}", ipv6_url, ipv4_url);
                match (ipv4_test, ipv6_test) {
                    (Ok(_), Err(_)) => {
                        info!("Url fallback succeeded, your Ethereum rpc url {} has been corrected to {}", eth_rpc_url, ipv4_url);
                        Ok(ipv4_eth_provider)
                    }
                    (Err(_), Ok(_)) => {
                        info!("Url fallback succeeded, your Ethereum  rpc url {} has been corrected to {}", eth_rpc_url, ipv6_url);
                        Ok(ipv6_eth_provider)
                    },
                    (Ok(_), Ok(_)) => panic!("This should never happen? Why didn't things work the first time?"),
                    (Err(_), Err(_)) => panic!("Could not connect to Ethereum rpc, are you sure it's running and on the specified port? {}", eth_rpc_url)
                }
            } else if url.port().is_none() || url.scheme() == "http" {
                let body = url.host_str().unwrap_or_else(|| {
                    panic!("Ethereum rpc url contains no host? {}", eth_rpc_url)
                });
                // transparently upgrade to https if available, we can't transparently downgrade for obvious security reasons
                let https_on_80_url = format!("https://{}:80", body);
                let https_on_443_url = format!("https://{}:443", body);
                let https_on_80_eth_provider =
                    Provider::<Http>::try_from(https_on_80_url.as_str()).unwrap_or_else(|_| {
                        panic!(
                            "Could not instantiate Ethereum HTTP provider: {}",
                            &https_on_80_url
                        )
                    });
                let https_on_443_eth_provider = Provider::<Http>::try_from(
                    https_on_443_url.as_str(),
                )
                .unwrap_or_else(|_| {
                    panic!(
                        "Could not instantiate Ethereum HTTP provider: {}",
                        &https_on_443_url
                    )
                });
                let https_on_80_test = https_on_80_eth_provider.get_block_number().await;
                let https_on_443_test = https_on_443_eth_provider.get_block_number().await;
                warn!(
                    "Trying fallback urls {} {}",
                    https_on_443_url, https_on_80_url
                );
                match (https_on_80_test, https_on_443_test) {
                    (Ok(_), Err(_)) => {
                        info!("Https upgrade succeeded, your Ethereum rpc url {} has been corrected to {}", eth_rpc_url, https_on_80_url);
                        Ok(https_on_80_eth_provider)
                    },
                    (Err(_), Ok(_)) => {
                        info!("Https upgrade succeeded, your Ethereum rpc url {} has been corrected to {}", eth_rpc_url, https_on_443_url);
                        Ok(https_on_443_eth_provider)
                    },
                    (Ok(_), Ok(_)) => panic!("This should never happen? Why didn't things work the first time?"),
                    (Err(_), Err(_)) => panic!("Could not connect to Ethereum rpc, are you sure it's running and on the specified port? {}", eth_rpc_url)
                }
            } else {
                panic!("Could not connect to Ethereum rpc! please check your grpc url {} for errors {:?}", eth_rpc_url, e)
            }
        }
    }
}
