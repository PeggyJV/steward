mod add;
mod delete;
mod import_from_key;
mod import_from_mnemonic;
mod list;
mod rename;
mod show;

use abscissa_core::{clap::Parser, Command, Runnable};

#[derive(Command, Debug, Parser, Runnable)]
pub enum EthKeysCmd {
    Add(add::AddKeyCmd),

    Delete(delete::DeleteKeyCmd),

    ImportFromKey(import_from_key::ImportFromKeyCmd),

    ImportFromMnemonic(import_from_mnemonic::ImportFromMnemonicCmd),

    List(list::ListKeyCmd),

    Show(show::ShowKeyCmd),

    Rename(rename::RenameKeyCmd),
}
