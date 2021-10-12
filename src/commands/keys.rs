mod add;
mod delete;
mod import;
mod list;
mod show;

use abscissa_core::{Command, Clap, Runnable};

#[derive(Command, Debug, Clap, Runnable)]
pub enum KeysCmd {
    #[clap(short, long)]
    Add(add::AddKeyCmd),

    #[clap(short, long)]
    Show(show::ShowKeyCmd),

    #[clap(short, long)]
    Delete(delete::DeleteKeyCmd),

    #[clap(short, long)]
    List(list::ListKeyCmd),

    #[clap(short, long)]
    Import(import::ImportEthKeyCmd),
}
