mod eth;
mod cosmos;

use abscissa_core::{Command, Clap, Runnable};
use crate::commands::keys::eth::EthKeysCmd;
use crate::commands::keys::cosmos::CosmosKeysCmd;

#[derive(Command, Debug, Clap, Runnable)]
pub enum KeysCmd {
    #[clap(subcommand)]
    EthKeysCmd(EthKeysCmd),

    #[clap(subcommand)]
    CosmosKeysCmd(CosmosKeysCmd),
}