//! Steward Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.
use crate::{prelude::APP, somm_send::MAX_GAS_PER_BLOCK};
use abscissa_core::Application;
use deep_space::{Address as CosmosAddress, CosmosPrivateKey, PrivateKey};
use ethers::signers::LocalWallet;
use ethers_gcp_kms_signer::{GcpKeyRingRef, GcpKmsProvider, GcpKmsSigner};
use gravity_bridge::gravity::ethereum::types::SignerType;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use signatory::FsKeyStore;
use std::{net::SocketAddr, path::Path};

lazy_static! {
    static ref DELEGATE_KEY: CosmosPrivateKey = {
        let config = APP.config();
        let name = &config.keys.delegate_key;
        config.load_deep_space_key(name.clone())
    };
    static ref DELEGATE_ADDRESS: CosmosAddress = {
        let config = APP.config();
        DELEGATE_KEY
            .to_address(&config.cosmos.prefix)
            .expect("failed to derive delegate address from key. make sure the cosmos.prefix field is set in your config.")
    };
}

const GRAVITY_ADDRESS: &str = "0x69592e6f9d21989a043646fe8225da2600e5a0f7";

pub(crate) fn get_delegate_address() -> &'static CosmosAddress {
    &DELEGATE_ADDRESS
}

pub(crate) fn get_delegate_key() -> &'static CosmosPrivateKey {
    &DELEGATE_KEY
}

/// Steward Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct StewardConfig {
    pub keystore: String,
    pub cosmos: CosmosSection,
    pub ethereum: EthereumSection,
    pub gravity: GravitySection,
    pub keys: KeysConfig,
    pub cork: CorkConfig,
    pub metrics: MetricsSection,
    pub server: ServerSection,
    pub simulate: SimulateSection,
    pub pubsub: PubsubConfig,
}

impl StewardConfig {
    fn load_secret_key(&self, name: String) -> k256::elliptic_curve::SecretKey<k256::Secp256k1> {
        let keystore = Path::new(&self.keystore);
        let keystore = FsKeyStore::create_or_open(keystore).expect("Could not open keystore");
        let name = name.parse().expect("Could not parse name");
        let key = keystore.load(&name).expect("Could not load key");
        key.to_pem().parse().expect("Could not parse pem")
    }

    pub fn load_clarity_key(&self, name: String) -> clarity::PrivateKey {
        let key = self.load_secret_key(name).to_bytes();
        clarity::PrivateKey::from_slice(key.as_slice()).expect("Could not convert key")
    }

    pub fn load_deep_space_key(&self, name: String) -> CosmosPrivateKey {
        let key = self.load_secret_key(name).to_bytes();
        let key = deep_space::utils::bytes_to_hex_str(key.as_slice());
        key.parse().expect("Could not parse private key")
    }

    pub fn load_ethers_wallet(&self, name: String) -> LocalWallet {
        let secret_key = self.load_secret_key(name);
        let bytes = secret_key.to_bytes();
        let ethers_secret = ethers::core::k256::SecretKey::from_bytes(&bytes).unwrap();
        let signing_key = ethers::core::k256::ecdsa::SigningKey::from(ethers_secret);
        LocalWallet::from(signing_key)
    }

    /// Loads an Ethereum signer either from a local keystore or via GCP KMS
    pub async fn load_ethers_signer(
        &self,
        name: String,
    ) -> Result<SignerType, Box<dyn std::error::Error>> {
        if let Some(eth_remote) = &self.ethereum.remote_signer {
            if eth_remote.use_remote {
                panic!("SHouldnt be using remote");
                // Create the GCP key ring reference
                let keyring = GcpKeyRingRef::new(
                    &eth_remote.project_id,
                    &eth_remote.location,
                    &eth_remote.key_ring,
                );

                // Create the GCP KMS provider
                let provider = GcpKmsProvider::new(keyring).await?;

                // Create the remote signer with the key name and version
                let gcp_kms_signer = GcpKmsSigner::new(
                    provider,
                    eth_remote.key_name.clone(),
                    1,
                    1, // Default to 1 signature attempt
                )
                .await?;

                return Ok(SignerType::GcpKms(gcp_kms_signer));
            }
        }

        // Fallback: load from local keystore
        let local_wallet = self.load_ethers_wallet(name);
        Ok(SignerType::Local(local_wallet))
    }
}

/// Default configuration settings.
///
/// Note: if your needs are as simple as below, you can
/// use `#[derive(Default)]` on StewardConfig instead.
impl Default for StewardConfig {
    fn default() -> Self {
        Self {
            keystore: String::new(),
            cosmos: CosmosSection::default(),
            ethereum: EthereumSection::default(),
            gravity: GravitySection::default(),
            keys: KeysConfig::default(),
            cork: CorkConfig::default(),
            metrics: MetricsSection::default(),
            server: ServerSection::default(),
            simulate: SimulateSection::default(),
            pubsub: PubsubConfig::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ServerSection {
    pub address: String,
    pub port: u16,
    pub server_cert_path: String,
    pub server_key_path: String,
}

impl Default for ServerSection {
    fn default() -> Self {
        Self {
            address: "0.0.0.0".to_string(),
            port: 5734,
            server_cert_path: "".to_owned(),
            server_key_path: "".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct KeysConfig {
    pub delegate_key: String,
}

impl Default for KeysConfig {
    fn default() -> Self {
        Self {
            delegate_key: "".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct CorkConfig {
    /// Cache refresh period in seconds
    pub cache_refresh_period: u64,
    /// Proposal polling period in seconds
    pub proposal_poll_period: u64,
    /// Number of retries for failed scheduling for proposals
    pub max_scheduling_retries: u64,
}

impl Default for CorkConfig {
    fn default() -> Self {
        Self {
            cache_refresh_period: 60,
            proposal_poll_period: 300,
            max_scheduling_retries: 3,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct PubsubConfig {
    /// Cache refresh period in seconds
    pub cache_refresh_period: u64,
    /// List of publisher domains to reject
    pub publisher_domain_block_list: Vec<String>,
}

impl Default for PubsubConfig {
    fn default() -> Self {
        Self {
            cache_refresh_period: 3600,
            publisher_domain_block_list: Vec::default(),
        }
    }
}

/// EthereumSection for ethereum rpc and derivation path
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct EthereumSection {
    pub blocks_to_search: u64,
    pub gas_price_multiplier: f32,
    pub gas_multiplier: f32,
    pub key_derivation_path: String,
    pub rpc: String,
    /// Optional remote signer configuration
    pub remote_signer: Option<EthereumRemoteSignerConfig>,
}

impl Default for EthereumSection {
    fn default() -> Self {
        Self {
            blocks_to_search: 5000,
            gas_price_multiplier: 1.0f32,
            gas_multiplier: 1.0f32,
            key_derivation_path: "m/44'/60'/0'/0/0".to_owned(),
            rpc: "http://localhost:8545".to_owned(),
            remote_signer: None,
        }
    }
}

/// Configuration for GCP KMS remote signing
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct EthereumRemoteSignerConfig {
    /// Whether to use the remote signer
    pub use_remote: bool,
    /// The GCP project ID
    pub project_id: String,
    /// The GCP location (e.g., "global" or "us-central1")
    pub location: String,
    /// The key ring in GCP KMS
    pub key_ring: String,
    /// The name of the key
    pub key_name: String,
    /// Optional version of the key
    pub key_version: Option<String>,
}

impl Default for EthereumRemoteSignerConfig {
    fn default() -> Self {
        Self {
            use_remote: false,
            project_id: String::new(),
            location: String::new(),
            key_ring: String::new(),
            key_name: String::new(),
            key_version: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GravitySection {
    pub contract: String,
    pub fees_denom: String,
}

impl Default for GravitySection {
    fn default() -> Self {
        Self {
            contract: "0x0000000000000000000000000000000000000000".to_owned(),
            fees_denom: "usomm".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct CosmosSection {
    pub gas_adjustment: f64,
    pub grpc: String,
    pub key_derivation_path: String,
    pub prefix: String,
    pub msg_batch_size: u32,
    pub gas_limit_per_msg: u64,
    pub gas_price: GasPrice,
}

impl Default for CosmosSection {
    fn default() -> Self {
        Self {
            gas_adjustment: 1.1f64,
            grpc: "https://127.0.0.1:9090".to_owned(),
            key_derivation_path: "m/44'/118'/0'/0/0".to_owned(),
            prefix: "somm".to_owned(),
            msg_batch_size: 5,
            gas_limit_per_msg: MAX_GAS_PER_BLOCK,
            gas_price: GasPrice::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GasPrice {
    pub amount: f64,
    pub denom: String,
}

impl Default for GasPrice {
    fn default() -> Self {
        Self {
            amount: 0.0,
            denom: "usomm".to_owned(),
        }
    }
}

impl GasPrice {
    pub fn as_tuple(&self) -> (f64, String) {
        (self.amount, self.denom.to_owned())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct MetricsSection {
    pub listen_addr: SocketAddr,
}

impl Default for MetricsSection {
    fn default() -> Self {
        Self {
            listen_addr: "127.0.0.1:3000".parse().unwrap(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct SimulateSection {
    pub use_tls: bool,
    pub network_id: String,
    pub tenderly_access_key: String,
    pub tenderly_project_name: String,
    pub tenderly_username: String,
    pub gravity_address: String,
    pub server_cert_path: String,
    pub server_key_path: String,
    pub client_ca_cert_path: String,
}

impl Default for SimulateSection {
    fn default() -> Self {
        Self {
            use_tls: false,
            network_id: "1".to_string(),
            tenderly_access_key: "".to_string(),
            tenderly_project_name: "".to_string(),
            tenderly_username: "".to_string(),
            gravity_address: GRAVITY_ADDRESS.to_string(),
            server_cert_path: "".to_string(),
            server_key_path: "".to_string(),
            client_ca_cert_path: "".to_string(),
        }
    }
}
