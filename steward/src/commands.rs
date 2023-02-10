//! Steward Subcommands
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
mod cork_proposal;
mod cosmos_to_eth;
mod deploy;
mod eth_to_cosmos;
mod keys;
mod orchestrator;
mod sign_delegate_keys;
mod simulate;
mod start;

use self::{config_cmd::ConfigCmd, cork_proposal::CorkProposalCmd, keys::KeysCmd, start::StartCmd};

use crate::config::StewardConfig;
use abscissa_core::{clap::Parser, Command, Configurable, FrameworkError, Runnable};
use std::{env, path::PathBuf};

/// Steward Configuration Filename
pub const CONFIG_FILE: &str = "steward.toml";

/// Steward Subcommands
#[derive(Command, Debug, Parser, Runnable)]
pub enum StewardCmd {
    #[clap(subcommand)]
    CorkProposal(CorkProposalCmd),
    #[clap(subcommand)]
    Keys(KeysCmd),
    /// Print default configurations
    PrintConfig(ConfigCmd),
    CosmosToEth(cosmos_to_eth::CosmosToEthCmd),
    #[clap(subcommand)]
    Deploy(deploy::DeployCmd),
    EthToCosmos(eth_to_cosmos::EthToCosmosCmd),
    #[clap(subcommand)]
    Orchestrator(orchestrator::OrchestratorCmd),
    SignDelegateKeys(sign_delegate_keys::SignDelegateKeysCmd),
    Simulate(simulate::SimulateCmd),
    Start(StartCmd),
}

/// Entry point for the application. It needs to be a struct to allow using subcommands!
#[derive(Command, Debug, Parser)]
#[clap(author, about, version)]
pub struct EntryPoint {
    #[clap(subcommand)]
    cmd: StewardCmd,
    /// Enable verbose logging
    #[clap(short, long)]
    pub verbose: bool,
    /// Use the specified config file
    #[clap(short, long)]
    pub config: Option<String>,
}

impl Runnable for EntryPoint {
    fn run(&self) {
        self.cmd.run()
    }
}

/// This trait allows you to define how application configuration is loaded.
impl Configurable<StewardConfig> for EntryPoint {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        // If you'd like for a missing configuration file to be a hard error
        // instead, always return `Some(CONFIG_FILE)` here.
        let config_filename = self.config.as_ref();

        let default_filename = PathBuf::from(CONFIG_FILE);

        let config_env_variable = std::env::var("STEWARD_CONFIG");

        if let Some(config_filename) = config_filename {
            Some(PathBuf::from(config_filename.to_string()))
        } else if let Ok(config_env_variable) = config_env_variable {
            Some(PathBuf::from(config_env_variable))
        } else if default_filename.exists() {
            Some(default_filename)
        } else {
            None
        }
    }
    /// Apply changes to the config after it's been loaded, e.g. overriding
    /// values in a config file using command-line options.
    ///
    /// This can be safely deleted if you don't want to override config
    /// settings from command-line options.
    fn process_config(&self, config: StewardConfig) -> Result<StewardConfig, FrameworkError> {
        Ok(config)
    }
}
