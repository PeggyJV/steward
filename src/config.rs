//! CellarRebalancer Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use ethers::prelude::H160;
use serde::{Deserialize, Serialize};

/// CellarRebalancer Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CellarRebalancerConfig {
    /// An example configuration section
    pub cellar: CellarConfig,
    /// An example configuration for keystore
    pub keystore: String,
    /// An example configuration for ethereum
    pub ethereum: EthereumSection,
}

/// Default configuration settings.
///
/// Note: if your needs are as simple as below, you can
/// use `#[derive(Default)]` on CellarRebalancerConfig instead.
impl Default for CellarRebalancerConfig {
    fn default() -> Self {
        Self {
            cellar: CellarConfig::default(),
            keystore: "/tmp/keystore".to_owned(),
            ethereum: EthereumSection::default(),
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
    pub token_0: TokenInfo,
    pub token_1: TokenInfo,

}

impl Default for CellarConfig {
    fn default() -> Self {
        CellarConfig {
            pair_id: ethers::types::U256::zero(),
            token_0: TokenInfo::default(),
            token_1: TokenInfo::default(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TokenInfo{
   pub decimals: u8,
   pub  symbol: String,
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
