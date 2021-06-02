//! `transfer` subcommand - this subcommand transfers ethereum from one account to another
use crate::application::APP;
use std::env;
use std::io;
/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;
use abscissa_core::{Command, Options, Runnable};
use ethers::prelude::*;
use std::convert::TryFrom;

///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>
#[derive(Command, Debug, Options)]
pub struct TransferCmd {
    #[options(free)]
    recipient: Vec<String>,
}

impl Runnable for TransferCmd {
    /// Transfer ETH from one account to another with Ganache blockchain emulator.
    fn run(&self) {
        abscissa_tokio::run(&APP, async {
            let mut provider_endpoint = String::new();
            println!("Put in your Provider endpoint");
            io::stdin()
                .read_line(&mut provider_endpoint)
                .expect("Failed to read line");
            let provider = Provider::<Http>::try_from(provider_endpoint).unwrap();
            let accounts = provider.get_accounts().await.unwrap();
            let from = accounts[1];
            let to = accounts[2];

            let tx = TransactionRequest::new().to(to).value(1000).from(from);

            let balance_before = provider.get_balance(from, None).await.unwrap();
            let tx = provider.send_transaction(tx, None).await.unwrap();

            println!("TX Hash: {:?}", tx);

            let nonce1 = provider
                .get_transaction_count(from, Some(BlockNumber::Latest.into()))
                .await
                .unwrap();

            let nonce2 = provider
                .get_transaction_count(from, Some(BlockNumber::Number(0.into()).into()))
                .await
                .unwrap();

            assert!(nonce2 < nonce1);

            let balance_after = provider.get_balance(from, None).await.unwrap();
            assert!(balance_after < balance_before);

            println!("Balance before {}", balance_before);
            println!("Balance after {}", balance_after);
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}

/**
    let args: Vec<String> = env::args().collect();
    let link = &args[1];
    let provider = Provider::<Http>::try_from(link]).unwrap();
**/
