//! CellarRebalancer Subcommands
//!
//! This is where you specify the subcommands of your application.
//!
//! The default application comes with two subcommands:
//!
//! - `start`: launches the application
//! - `version`: print application version
//!
//! See the `impl Configurable` below for how to specify the path to the
//! application's configuration file.

mod config_cmd;
mod keys;
mod predictions;
mod start;
mod transfer;
mod version;
mod fund_cellar;

use self::{
    config_cmd::ConfigCmd, keys::KeysCmd, predictions::PredictionsCmd, start::StartCmd,
    transfer::TransferCmd, version::VersionCmd,fund_cellar::FundCellarCmd
};

use crate::config::CellarRebalancerConfig;
use abscissa_core::{
    config::Override, Command, Configurable, FrameworkError, Help, Options, Runnable,
};
use std::path::PathBuf;

/// CellarRebalancer Configuration Filename
pub const CONFIG_FILE: &str = "contract_monitor.toml";

/// CellarRebalancer Subcommands
#[derive(Command, Debug, Options, Runnable)]
pub enum CellarRebalancerCmd {
    /// The `help` subcommand
    #[options(help = "get usage information")]
    Help(Help<Self>),

    /// The `start` subcommand
    #[options(help = "start the application")]
    Start(StartCmd),

    /// The `transfer` subcommand
    #[options(help = "transfer ETH")]
    Transfer(TransferCmd),

    /// The `version` subcommand
    #[options(help = "display version information")]
    Version(VersionCmd),

    /// The `prediction` subcommand
    #[options(help = "display lastest prediction")]
    Predictions(PredictionsCmd),
    /// The `keys` subcommand
    #[options(help = "key management commands")]
    Keys(KeysCmd),

    #[options(help = "print default config")]
    PrintConfig(ConfigCmd),

    #[options(help = "fund cellar")]
    FundCellar(FundCellarCmd),
}

/// This trait allows you to define how application configuration is loaded.
impl Configurable<CellarRebalancerConfig> for CellarRebalancerCmd {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        // If you'd like for a missing configuration file to be a hard error
        // instead, always return `Some(CONFIG_FILE)` here.
        let filename = PathBuf::from(CONFIG_FILE);

        if filename.exists() {
            Some(filename)
        } else {
            None
        }
    }

    /// Apply changes to the config after it's been loaded, e.g. overriding
    /// values in a config file using command-line options.
    ///
    /// This can be safely deleted if you don't want to override config
    /// settings from command-line options.
    fn process_config(
        &self,
        config: CellarRebalancerConfig,
    ) -> Result<CellarRebalancerConfig, FrameworkError> {
        match self {
            CellarRebalancerCmd::Start(cmd) => cmd.override_config(config),
            _ => Ok(config),
        }
    }
}
