mod cosmos;
mod eth;

use abscissa_core::{Command, Options, Runnable};

use crate::commands::keys::cosmos::CosmosKeysCmd;
use crate::commands::keys::eth::EthKeysCmd;

#[derive(Command, Debug, Options, Runnable)]
pub enum KeysCmd {
    #[options(name = "eth")]
    EthKeysCmd(EthKeysCmd),

    #[options(name = "cosmos")]
    CosmosKeysCmd(CosmosKeysCmd),
}
