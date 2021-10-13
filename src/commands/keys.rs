mod add;
mod delete;
mod import;
mod list;
mod show;

use abscissa_core::{Command, Clap, Runnable};

#[derive(Command, Debug, Clap, Runnable)]
pub enum KeysCmd {
    Add(add::AddKeyCmd),

    Show(show::ShowKeyCmd),

    Delete(delete::DeleteKeyCmd),

    List(list::ListKeyCmd),

    Import(import::ImportEthKeyCmd),
}