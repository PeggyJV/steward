//! CellarRebalancer Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.
use ethers::{prelude::H160, signers::LocalWallet as EthWallet};
use serde::{Deserialize, Serialize};
use signatory::FsKeyStore;
use std::{net::SocketAddr, path::Path, time::Duration};

/// CellarRebalancer Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct StewardConfig {
    pub keystore: String,
    pub server: ServerSection,
    pub gravity: GravitySection,
    pub ethereum: EthereumSection,
    pub cosmos: CosmosSection,
    pub metrics: MetricsSection,
    pub cellars: Vec<CellarConfig>,
    pub keys: KeysConfig,
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
        clarity::PrivateKey::from_slice(&key).expect("Could not convert key")
    }

    pub fn load_deep_space_key(&self, name: String) -> deep_space::private_key::PrivateKey {
        let key = self.load_secret_key(name).to_bytes();
        let key = deep_space::utils::bytes_to_hex_str(&key);
        key.parse().expect("Could not parse private key")
    }

    pub fn load_ethers_wallet(&self, name: String) -> EthWallet {
        EthWallet::from(self.load_secret_key(name))
    }
}

/// Default configuration settings.
///
/// Note: if your needs are as simple as below, you can
/// use `#[derive(Default)]` on CellarRebalancerConfig instead.
impl Default for StewardConfig {
    fn default() -> Self {
        Self {
            keystore: "/tmp/keystore".to_owned(),
            server: ServerSection::default(),
            gravity: GravitySection::default(),
            ethereum: EthereumSection::default(),
            cosmos: CosmosSection::default(),
            metrics: MetricsSection::default(),
            cellars: vec![CellarConfig::default()],
            keys: KeysConfig::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerSection {
    pub address: Option<String>,
    pub client_ca_cert_path: Option<String>,
    pub port: Option<u16>,
    pub server_cert_path: String,
    pub server_key_path: String,
}

impl Default for ServerSection {
    fn default() -> Self {
        Self {
            address: Some("0.0.0.0".to_owned()),
            client_ca_cert_path: Some("".to_owned()),
            port: Some(9999),
            server_cert_path: "".to_owned(),
            server_key_path: "".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeysConfig {
    pub keystore: String,
    pub rebalancer_key: String,
}

impl Default for KeysConfig {
    fn default() -> Self {
        Self {
            keystore: "/tmp/keystore".to_owned(),
            rebalancer_key: "".to_owned(),
        }
    }
}

/// Example configuration section.
///
/// Delete this and replace it with your actual configuration structs.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CellarConfig {
    /// Example configuration value
    pub pair_id: ethers::types::U256,
    pub name: String,
    pub token_id: ethers::types::U256,
    pub cellar_address: ethers::types::H160,
    pub pool_address: ethers::types::H160,
    pub weight_factor: u32,
    pub max_gas_price_gwei: u32,
    pub token_0: TokenInfo,
    pub token_1: TokenInfo,
    pub duration: Duration,
}

impl Default for CellarConfig {
    fn default() -> Self {
        CellarConfig {
            pair_id: ethers::types::U256::zero(),
            name: String::default(),
            token_id: ethers::types::U256::zero(),
            cellar_address: ethers::types::H160::zero(),
            pool_address: ethers::types::H160::zero(),
            weight_factor: 100,
            max_gas_price_gwei: 100,
            duration: Duration::from_secs(60),
            token_0: TokenInfo::default(),
            token_1: TokenInfo::default(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TokenInfo {
    pub decimals: u8,
    pub symbol: String,
    pub address: H160,
}

impl Default for TokenInfo {
    fn default() -> Self {
        TokenInfo {
            decimals: 18,
            symbol: "NA".to_string(),
            address: H160::zero(),
        }
    }
}

/// EthereumSection for ethereum rpc and derivation path
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct EthereumSection {
    /// Declaring EthereumSection key_derivation_path
    pub key_derivation_path: String,
    /// Declaring EthereumSection rpc
    pub rpc: String,
    /// Declaring EthereumSection gas multiplier
    pub gas_price_multiplier: f32,
    /// Declaring EthereumSection blocks to search
    pub blocks_to_search: u64,
}

impl Default for EthereumSection {
    fn default() -> Self {
        Self {
            key_derivation_path: "m/44'/60'/0'/0/0".to_owned(),
            rpc: "http://localhost:8545".to_owned(),
            gas_price_multiplier: 1.0f32,
            blocks_to_search: 5000,
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
            fees_denom: "stake".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct CosmosSection {
    pub key_derivation_path: String,
    pub grpc: String,
    pub prefix: String,
    pub msg_batch_size: u32,
    pub gas_adjustment: f64,
    pub gas_price: GasPrice,
}

impl Default for CosmosSection {
    fn default() -> Self {
        Self {
            key_derivation_path: "m/44'/118'/0'/0/0".to_owned(),
            grpc: "http://localhost:9090".to_owned(),
            prefix: "cosmos".to_owned(),
            gas_price: GasPrice::default(),
            gas_adjustment: 1.0f64,
            msg_batch_size: 5,
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
            amount: 0.001,
            denom: "stake".to_owned(),
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
