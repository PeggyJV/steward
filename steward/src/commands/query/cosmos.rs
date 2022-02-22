//! `cosmos subcommands` subcommand

use crate::{application::APP, prelude::*};
use abscissa_core::{clap::Parser, Command, Runnable};

/// Query Cosmos chain
#[derive(Command, Debug, Parser)]
pub enum Cosmos {
    Balance(Balance),
    GravityKeys(GravityKeys),
}

impl Runnable for Cosmos {
    /// Start the application.
    fn run(&self) {
        // Your code goes here
    }
}

/// Query Cosmos balance
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Query Cosmos balance.\n This command queries the Cosmos balance, taking the name of the key as a String."
)]
pub struct Balance {
    /// Cosmos key name.
    key_name: String,

    #[clap(short, long)]
    help: bool,
}

impl Runnable for Balance {
    fn run(&self) {
        assert!(self.key_name.len() == 1);
        let _key_name = self.key_name.clone();
    }
}

/// Query Cosmos Gravity keys
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Query the Cosmos Gravity keys.\n This command queries the Cosmos gravity keys, taking the name of the key."
)]
pub struct GravityKeys {
    /// Cosmos Gravity key name.
    key_name: String,
    #[clap(short, long)]
    help: bool,
}

impl Runnable for GravityKeys {
    /// Start the application.
    fn run(&self) {
        assert!(self.key_name.len() == 1);
        let _key_name = self.key_name.clone();

        abscissa_tokio::run(&APP, async { unimplemented!() }).unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
