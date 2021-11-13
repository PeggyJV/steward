//! `eth subcommands` subcommand

use crate::uniswap_pool::PoolState;
use crate::{application::APP, prelude::*};
use ethers::prelude::*;
use std::{convert::TryFrom, sync::Arc, time::Duration};
use abscissa_core::{Command, Clap, Runnable};
/// Query Eth chain
#[derive(Command, Debug, Clap, Runnable)]
pub enum Eth {
    Balance(Balance),
    Contract(Contract),
    Pair(Pair),
}

#[derive(Command, Debug, Clap)]
pub struct Balance {
    free: Vec<String>,

    #[clap(short, long)]
    help: bool,
}

impl Runnable for Balance {
    fn run(&self) {
        assert!(self.free.len() == 1);
        let _key_name = self.free[0].clone();

        abscissa_tokio::run(&APP, async { unimplemented!() }).unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}

#[derive(Command, Debug, Clap)]
pub struct Contract {
    #[clap(short, long)]
    help: bool,
}

impl Runnable for Contract {
    /// Start the application.
    fn run(&self) {}
}

#[derive(Command, Debug, Clap)]
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

