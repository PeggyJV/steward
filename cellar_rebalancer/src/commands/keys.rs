mod cosmos;
mod eth;

use crate::commands::keys::cosmos::CosmosKeysCmd;
use crate::commands::keys::eth::EthKeysCmd;
use abscissa_core::{Clap, Command, Runnable};

#[derive(Command, Debug, Clap, Runnable)]
pub enum KeysCmd {
    #[clap(subcommand)]
    EthKeysCmd(EthKeysCmd),

    #[clap(subcommand)]
    CosmosKeysCmd(CosmosKeysCmd),
}
