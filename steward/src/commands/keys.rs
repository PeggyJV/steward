mod cosmos;
mod eth;

use crate::commands::keys::cosmos::CosmosKeysCmd;
use crate::commands::keys::eth::EthKeysCmd;
use abscissa_core::{clap::Parser, Command, Runnable};

#[derive(Command, Debug, Parser, Runnable)]
pub enum KeysCmd {
    #[clap(subcommand)]
    Eth(EthKeysCmd),

    #[clap(subcommand)]
    Cosmos(CosmosKeysCmd),
}
