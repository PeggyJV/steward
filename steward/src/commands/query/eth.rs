//! `eth subcommands` subcommand

use crate::uniswap_pool::PoolState;
use crate::{application::APP, prelude::*};
use abscissa_core::{clap::Parser, Command, Runnable};
use ethers::prelude::*;
use std::{convert::TryFrom, sync::Arc, time::Duration};
/// Query Eth chain
#[derive(Command, Debug, Parser, Runnable)]
pub enum Eth {
    Balance(Balance),
    Contract(Contract),
    Pair(Pair),
}

/// Query Eth balance
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Query Eth balance.\n This command queries the Eth balance, taking the name of the key as a String."
)]
pub struct Balance {
    /// Eth key name
    key_name: String,

    #[clap(short, long)]
    help: bool,
}

impl Runnable for Balance {
    fn run(&self) {
        assert!(self.key_name.len() == 1);
        let _key_name = self.key_name.clone();

        abscissa_tokio::run(&APP, async { unimplemented!() }).unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}

#[derive(Command, Debug, Parser)]
/// Query Eth Contract
pub struct Contract {
    #[clap(short, long)]
    help: bool,
}

impl Runnable for Contract {
    /// Start the application.
    fn run(&self) {}
}

/// Query Eth pool pair
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Query Eth pool pair.\n This command queries the Eth pool pair, printing pair addresses. It takes the pool address as H160."
)]
pub struct Pair {
    #[clap(short, long)]
    help: bool,

    #[clap(short, long)]
    pool: H160,
}

impl Runnable for Pair {
    /// Start the application.
    fn run(&self) {
        let config = APP.config();
        let eth_host = config.ethereum.rpc.clone();

        abscissa_tokio::run(&APP, async {
            let client = Provider::<Http>::try_from(eth_host)
                .unwrap()
                .interval(Duration::from_secs(3000u64));
            let client = Arc::new(client);
            let pool = PoolState::new(self.pool, client);
            let (token_0, token_1) = (pool.token_0().await, pool.token_1().await);

            println!("token_0: {:?}", token_0);
            println!("token_1: {:?}", token_1);
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
