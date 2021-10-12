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

    #[options(help = "remove_funds")]
    RemoveFunds(RemoveFundsCmd),

    #[options(help = "Send Cosmos to Ethereum")]
    CosmosToEth(cosmos_to_eth::CosmosToEthCmd),

    #[options(help = "tools for contract deployment")]
    Deploy(deploy::DeployCmd),

    #[options(help = "Send Ethereum to Cosmos")]
    EthToCosmos(eth_to_cosmos::EthToCosmosCmd),

    #[options(help = "orchestrator management commands")]
    Orchestrator(orchestrator::OrchestratorCmd),

    #[options(help = "query state on either ethereum or cosmos chains")]
    Query(query::QueryCmd),

    #[options(help = "sign delegate keys")]
    SignDelegateKeys(sign_delegate_keys::SignDelegateKeysCmd),

    #[options(help = "create transactions on either ethereum or cosmos chains")]
    Tx(tx::TxCmd),

    #[options(help = "Allow Erc20 Token to interact with cellar contract")]
    AllowErc20(allow_erc20::AllowERC20),

    #[options(help = "Reinvest fees on the cellar")]
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
