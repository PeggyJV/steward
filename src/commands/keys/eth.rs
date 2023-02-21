mod add;
mod delete;
mod import;
mod list;
mod rename;
mod show;

use abscissa_core::{clap::Parser, Command, Runnable};

#[derive(Command, Debug, Parser, Runnable)]
pub enum EthKeysCmd {
    Add(add::AddKeyCmd),

    Delete(delete::DeleteKeyCmd),

    Import(import::ImportEthKeyCmd),

    List(list::ListKeyCmd),

    Show(show::ShowKeyCmd),

    Rename(rename::RenameKeyCmd),
}
