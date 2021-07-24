//! CellarRebalancer Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use std::time::Duration;

use ethers::prelude::H160;
use serde::{Deserialize, Serialize};

/// CellarRebalancer Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CellarRebalancerConfig {
    pub key: KeyConfig,
    /// An example configuration section
    pub cellar: CellarConfig,
    /// An example configuration for keystore
    /// An example configuration for ethereum
    pub ethereum: EthereumSection,
    pub mongo: MongoSection,
}

/// Default configuration settings.
///
/// Note: if your needs are as simple as below, you can
/// use `#[derive(Default)]` on CellarRebalancerConfig instead.
impl Default for CellarRebalancerConfig {
    fn default() -> Self {
        Self {
            cellar: CellarConfig::default(),
            key: KeyConfig::default(),
            ethereum: EthereumSection::default(),
            mongo: MongoSection::default(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MongoSection {
    pub host: String,
}

impl Default for MongoSection {
    fn default() -> Self {
        Self {
            host: "mongodb://localhost:27017/".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeyConfig {
    pub keystore: String,
    pub rebalancer_key: String,
}

impl Default for KeyConfig {
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
    pub pair_id: ethers::types::H160,
    pub cellar_addresses: ethers::types::H160,
    pub token_0: TokenInfo,
    pub token_1: TokenInfo,
    pub duration: Duration,
    pub weight_factor: u32,
}

impl Default for CellarConfig {
    fn default() -> Self {
        CellarConfig {
            pair_id: ethers::types::H160::zero(),
            cellar_addresses: ethers::types::H160::zero(),
            token_0: TokenInfo::default(),
            token_1: TokenInfo::default(),
            duration: Duration::from_secs(60),
            weight_factor: 100,
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
pub struct EthereumSection {
    /// Declaring EthereumSection key_derivation_path
    pub key_derivation_path: String,
    /// Declaring EthereumSection rpc
    pub rpc: String,
}

impl Default for EthereumSection {
    fn default() -> Self {
        Self {
            key_derivation_path: "m/44'/60'/0'/0/0".to_owned(),
            rpc: "http://localhost:8545".to_owned(),
        }
    }
}
