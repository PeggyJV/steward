mod cosmos;
mod eth;

use crate::commands::keys::cosmos::CosmosKeysCmd;
use crate::commands::keys::eth::EthKeysCmd;
use abscissa_core::{clap::Parser, Command, Runnable};

/// Keys management command
#[derive(Command, Debug, Parser, Runnable)]
pub enum KeysCmd {
    /// Manage Ethereum keys.
    #[clap(subcommand)]
    Eth(EthKeysCmd),
    /// Manage Cosmos keys.
    #[clap(subcommand)]
    Cosmos(CosmosKeysCmd),
}
