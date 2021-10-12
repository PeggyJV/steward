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
mod sign_delegate_keys;
mod start;
mod transfer;
mod tx;
mod reinvest;

use self::{
    config_cmd::ConfigCmd, fund_cellar::FundCellarCmd, keys::KeysCmd, predictions::PredictionsCmd,
    remove_funds::RemoveFundsCmd, start::StartCmd, transfer::TransferCmd,
};

use crate::config::CellarRebalancerConfig;
use abscissa_core::{
    config::Override, Command, Configurable, FrameworkError, Clap, Runnable,
};
use std::path::PathBuf;

/// CellarRebalancer Configuration Filename
pub const CONFIG_FILE: &str = "contract_monitor.toml";

/// CellarRebalancer Subcommands
#[derive(Command, Debug, Clap, Runnable)]
pub enum CellarRebalancerCmd {
    /// The `help` subcommand
    #[clap()]
    Help(Help<Self>),

    /// The `start` subcommand
    Start(StartCmd),

    /// The `transfer` subcommand
    Transfer(TransferCmd),

    /// The `prediction` subcommand
    Predictions(PredictionsCmd),
    /// The `keys` subcommand
    #[clap(subcommand)]
    Keys(KeysCmd),

    PrintConfig(ConfigCmd),

    FundCellar(FundCellarCmd),

    RemoveFunds(RemoveFundsCmd),

    #[clap(subcommand)]
    CosmosToEth(cosmos_to_eth::CosmosToEthCmd),

    #[clap(subcommand)]
    Deploy(deploy::DeployCmd),

    #[clap(subcommand)]
    EthToCosmos(eth_to_cosmos::EthToCosmosCmd),

    #[clap(subcommand)]
    Orchestrator(orchestrator::OrchestratorCmd),

    #[clap(subcommand)]
    Query(query::QueryCmd),

    #[clap(subcommand)]
    SignDelegateKeys(sign_delegate_keys::SignDelegateKeysCmd),

    #[clap(subcommand)]
    Tx(tx::TxCmd),

    #[clap(subcommand)]
    AllowErc20(allow_erc20::AllowERC20),

    #[clap(subcommand)]
    Reinvest(reinvest::ReinvestCommand),
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
