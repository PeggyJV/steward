mod eth;

use abscissa_core::{Command, Clap, Runnable};
use crate::commands::keys::eth::EthKeysCmd;

#[derive(Command, Debug, Clap, Runnable)]
pub enum KeysCmd {
    #[clap(subcommand)]
    EthKeysCmd(EthKeysCmd),
}