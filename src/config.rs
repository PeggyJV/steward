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
    pub cellar: CellarConfig,
}

/// Default configuration settings.
///
/// Note: if your needs are as simple as below, you can
/// use `#[derive(Default)]` on ContractMonitorConfig instead.
impl Default for ContractMonitorConfig {
    fn default() -> Self {
        Self {
            cellar: CellarConfig::default(),
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
    positions: Vec<PositionConfig>,
}

impl Default for CellarConfig {
    fn default() -> Self {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PositionConfig {
    pub id: u32,
    pub upper: i32,
    pub lower: i32,
}

impl Default for PositionConfig {
    fn default() -> Self {
        todo!()
    }
}
