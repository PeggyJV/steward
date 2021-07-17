mod add;
mod delete;
mod import;
mod list;
mod show;

use abscissa_core::{Command, Options, Runnable};

#[derive(Command, Debug, Options, Runnable)]
pub enum KeysCmd {
    #[options(help = "add [name] (password)")]
    Add(add::AddKeyCmd),
}