mod cosmos;
mod eth;

use abscissa_core::{Command, Clap, Runnable};

use crate::commands::keys::cosmos::CosmosKeysCmd;
use crate::commands::keys::eth::EthKeysCmd;

#[derive(Command, Debug, Clap, Runnable)]
pub enum KeysCmd {
    #[options(name = "eth")]
    EthKeysCmd(EthKeysCmd),

    #[options(name = "cosmos")]
    CosmosKeysCmd(CosmosKeysCmd),
}
