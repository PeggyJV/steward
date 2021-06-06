//! `cellarcontract` subcommand - this subcommand transfers ethereum from one account to another
use crate::application::APP;
/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;
use abscissa_core::{Command, Options, Runnable};
use ethers::contract::abigen;
use ethers::{
    prelude::*,
    types::{Address},
    providers::{Provider, Http},
};
use std::{convert::TryFrom, sync::Arc};

//use abigen macro to fetch and incorporate contract ABI
abigen!(
    MyContract,
    "./contract_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>
#[derive(Command, Debug, Options)]
pub struct CellarcontractCmd {
    #[options(free)]
    recipient: Vec<String>,
}

/// impl Runnable for CellarcontractCmd
impl Runnable for CellarcontractCmd {
    /// Transfer ETH from one account to another with Ganache blockchain emulator.
    fn run(&self) {
        abscissa_tokio::run(&APP, async {
            // This is a fake address used just for this example
            let address = "0x1dF62f291b2E969fB0849d99D9Ce41e2F137006e".parse::<Address>().unwrap();
            
            // Connect to the network provider (example below is for Ganache)
            let client = Provider::<Http>::try_from("http://localhost:8545").unwrap();

            // MyContract expects Arc, create with client
            let client = Arc::new(client);

            // Use ethers_rs get_accounts method to get client accounts
            let _accounts = client.get_accounts().await.unwrap();

            // Create the contract object at the address
            let contract = MyContract::new(address, client);
            
            // Test that contract creation was successful by printing contract address
            let addr = contract.address();
            //println!("{:?}", addr);

            // Cellar contract method to get balance of an account
            let _get_balance = contract.method::<_, Address>("balanceOf", addr).unwrap();
            // println!("{:?}", get_balance);

            // Cellar contract method to get total supply to an address.
            let _total_supply = contract.method::<_, String>("totalSupply", ()).unwrap();
            //println!("{:?}", total_supply);
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
