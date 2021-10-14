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

mod allow_erc20;
mod config_cmd;
mod cosmos_to_eth;
mod deploy;
mod eth_to_cosmos;
mod fund_cellar;
mod keys;
mod orchestrator;
mod predictions;
mod query;
mod remove_funds;
mod set_validator;
mod sign_delegate_keys;
mod start;
mod transfer;
mod tx;
mod reinvest;

use self::{
    config_cmd::ConfigCmd, fund_cellar::FundCellarCmd, keys::KeysCmd, predictions::PredictionsCmd,
    remove_funds::RemoveFundsCmd, set_validator::SetValidatorCmd, start::StartCmd, transfer::TransferCmd,
};

use crate::config::CellarRebalancerConfig;
use abscissa_core::{
    Application, Clap, Command, Configurable, FrameworkError, Runnable
};
use std::path::PathBuf;

/// CellarRebalancer Configuration Filename
pub const CONFIG_FILE: &str = "contract_monitor.toml";

/// CellarRebalancer Subcommands
#[derive(Command, Debug, Clap, Runnable)]
pub enum CellarRebalancerCmd {

    Start(StartCmd),

    Transfer(TransferCmd),

    Predictions(PredictionsCmd),

    #[clap(subcommand)]
    Keys(KeysCmd),

    /// Print default configurations
    PrintConfig(ConfigCmd),

    FundCellar(FundCellarCmd),

    RemoveFunds(RemoveFundsCmd),

    CosmosToEth(cosmos_to_eth::CosmosToEthCmd),

    #[clap(subcommand)]
    Deploy(deploy::DeployCmd),

    EthToCosmos(eth_to_cosmos::EthToCosmosCmd),

    #[clap(subcommand)]
    Orchestrator(orchestrator::OrchestratorCmd),

    #[clap(subcommand)]
    Query(query::QueryCmd),

    SetValidator(SetValidatorCmd),

    SignDelegateKeys(sign_delegate_keys::SignDelegateKeysCmd),

    #[clap(subcommand)]
    Tx(tx::TxCmd),

    AllowErc20(allow_erc20::AllowERC20),

    Reinvest(reinvest::ReinvestCommand),
}

/// Entry point for the application. It needs to be a struct to allow using subcommands!
#[derive(Command, Debug, Clap)]
#[clap(author, about, version)]
pub struct EntryPoint {
    #[clap(subcommand)]
    cmd: CellarRebalancerCmd,

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
impl Configurable<CellarRebalancerConfig> for EntryPoint {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        // If you'd like for a missing configuration file to be a hard error
        // instead, always return `Some(CONFIG_FILE)` here.
        let filename = self
        .config
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| CONFIG_FILE.into());

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
        match &self.cmd {
            _ => Ok(config),
        }
    }
}