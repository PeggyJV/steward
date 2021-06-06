//! `transfer` subcommand - this subcommand transfers ethereum from one account to another
use crate::application::APP;
/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;
use abscissa_core::{Command, Options, Runnable};
use ethers::contract::abigen;
use ethers::{
    prelude::*,
    abi::Abi,
    types::{Address},
    contract::Contract,
    providers::{Provider, Http},
};
use std::convert::TryFrom;
use serde_json;

abigen!(
    MyContract,
    "./contract_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

///
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

impl Runnable for CellarcontractCmd {
    /// Transfer ETH from one account to another with Ganache blockchain emulator.
    fn run(&self) {
        abscissa_tokio::run(&APP, async {
            // this is a fake address used just for this example
            let address = "0x1dF62f291b2E969fB0849d99D9Ce41e2F137006e".parse::<Address>().unwrap();
            
            // connect to the network
            let client = Provider::<Http>::try_from("http://localhost:8545").unwrap();

            let accounts = client.get_accounts().await.unwrap();
            
            // create the contract object at the address
            let contract = MyContract::new(address, abi, client);
            
            //Test that contract creation was successful by printing contract address
            let addr = contract.address();
            //println!("{:?}", addr);

            let get_supply = contract.method::<_, String>("totalSupply", addr).unwrap();
            //println!("{:?}", total_supply);

            let reinvest = contract.method::<_, String>("reinvest", ()).unwrap();
            //println!("{:?}", reinvest);

            let get_name = contract.method::<_, String>("name", ()).unwrap();
            println!("{:?}", get_name);

        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
