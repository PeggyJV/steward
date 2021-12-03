mod add;
mod delete;
mod import;
mod list;
mod show;

use abscissa_core::{Clap, Command, Runnable};

/// Key management commands for the rebalancer
#[derive(Command, Debug, Clap, Runnable)]
pub enum KeysCmd {
    Add(add::AddKeyCmd),
    Delete(delete::DeleteKeyCmd),
    Import(import::ImportEthKeyCmd),
    List(list::ListKeyCmd),
    Show(show::ShowKeyCmd),
}
