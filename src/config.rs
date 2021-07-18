//! ContractMonitor Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use serde::{Deserialize, Serialize};

/// ContractMonitor Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContractMonitorConfig {
    /// An example configuration section
    pub hello: ExampleSection,
    /// An example configuration for keystore
    pub keystore: String,
    /// An example configuration for ethereum
    pub ethereum: EthereumSection,
}

/// Default configuration settings.
///
/// Note: if your needs are as simple as below, you can
/// use `#[derive(Default)]` on ContractMonitorConfig instead.
impl Default for ContractMonitorConfig {
    fn default() -> Self {
        Self {
            hello: ExampleSection::default(),
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
pub struct ExampleSection {
    /// Example configuration value
    pub recipient: String,
}

impl Default for ExampleSection {
    fn default() -> Self {
        Self {
            recipient: "world".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
    /// EthereumSection for ethereum rpc and derivation path
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
