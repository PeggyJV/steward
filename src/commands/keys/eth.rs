mod add;
mod delete;
mod import;
mod list;
mod show;

use abscissa_core::{Command, Options, Runnable};

#[derive(Command, Debug, Options, Runnable)]
pub enum EthKeysCmd {
    #[options(help = "add [name]")]
    Add(add::AddKeyCmd),

    #[options(help = "delete [name]")]
    Delete(delete::DeleteKeyCmd),

    #[options(help = "import [name] (private-key)")]
    Import(import::ImportEthKeyCmd),

    #[options(help = "list")]
    List(list::ListKeyCmd),

    #[options(help = "show [name]")]
    Show(show::ShowKeyCmd),
}
