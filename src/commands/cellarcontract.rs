//! `transfer` subcommand - this subcommand transfers ethereum from one account to another
use crate::application::APP;
/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;
use abscissa_core::{Command, Options, Runnable};
use ethers::{
    abi::Abi,
    types::{Address, H256},
    contract::Contract,
    providers::{Provider, Http},
};
use std::convert::TryFrom;
use serde_json;

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
            
            // (ugly way to write the ABI inline, you can otherwise read it from a file)
            let abi: Abi = serde_json::from_str(r#"[{"constant":True,"inputs":[],"name":"name","outputs":[{"name":"","type":"string"}],
            "payable":False,"stateMutability":"view","type":"function"},{"constant":False,"inputs":[{"name":"guy","type":"address"},
            {"name":"wad","type":"uint256"}],"name":"approve","outputs":[{"name":"","type":"bool"}],"payable":False,"stateMutability":"nonpayable","type":"function"},{"constant":True,"inputs":[],"name":"totalSupply","outputs":[{"name":"","type":"uint256"}],"payable":False,"stateMutability":"view","type":"function"},{"constant":False,"inputs":[{"name":"src","type":"address"},{"name":"dst","type":"address"},{"name":"wad","type":"uint256"}],"name":"transferFrom","outputs":[{"name":"","type":"bool"}],"payable":False,"stateMutability":"nonpayable","type":"function"},{"constant":False,"inputs":[{"name":"wad","type":"uint256"}],"name":"withdraw","outputs":[],"payable":False,"stateMutability":"nonpayable","type":"function"},{"constant":True,"inputs":[],"name":"decimals","outputs":[{"name":"","type":"uint8"}],"payable":False,"stateMutability":"view","type":"function"},{"constant":True,"inputs":[{"name":"","type":"address"}],"name":"balanceOf","outputs":[{"name":"","type":"uint256"}],"payable":False,"stateMutability":"view","type":"function"},{"constant":True,"inputs":[],"name":"symbol","outputs":[{"name":"","type":"string"}],"payable":False,"stateMutability":"view","type":"function"},{"constant":False,"inputs":[{"name":"dst","type":"address"},{"name":"wad","type":"uint256"}],"name":"transfer","outputs":[{"name":"","type":"bool"}],"payable":False,"stateMutability":"nonpayable","type":"function"},{"constant":False,"inputs":[],"name":"deposit","outputs":[],"payable":True,"stateMutability":"payable","type":"function"},{"constant":True,"inputs":[{"name":"","type":"address"},{"name":"","type":"address"}],"name":"allowance","outputs":[{"name":"","type":"uint256"}],"payable":False,"stateMutability":"view","type":"function"},{"payable":True,"stateMutability":"payable","type":"fallback"},{"anonymous":False,"inputs":[{"indexed":True,"name":"src","type":"address"},{"indexed":True,"name":"guy","type":"address"},{"indexed":False,"name":"wad","type":"uint256"}],"name":"Approval","type":"event"},{"anonymous":False,"inputs":[{"indexed":True,"name":"src","type":"address"},{"indexed":True,"name":"dst","type":"address"},{"indexed":False,"name":"wad","type":"uint256"}],"name":"Transfer","type":"event"},{"anonymous":False,"inputs":[{"indexed":True,"name":"dst","type":"address"},{"indexed":False,"name":"wad","type":"uint256"}],"name":"Deposit","type":"event"},{"anonymous":False,"inputs":[{"indexed":True,"name":"src","type":"address"},{"indexed":False,"name":"wad","type":"uint256"}],"name":"Withdrawal","type":"event"}])"#).unwrap();
            
            // connect to the network
            let client = Provider::<Http>::try_from("http://localhost:8545").unwrap();
            
            // create the contract object at the address
            let contract = Contract::new(address, abi, client);
            
            // Calling constant methods is done by calling `call()` on the method builder.
            // (if the function takes no arguments, then you must use `()` as the argument)
            let init_value: String = contract
                .method::<_, String>("getValue", ()).unwrap()
                .call()
                .await.unwrap();
            println!("{:?}", init_value);
            
            // Non-constant methods are executed via the `send()` call on the method builder.
            let call = contract
                .method::<_, H256>("setValue", "hi".to_owned()).unwrap();
            let pending_tx = call.send().await.unwrap();
            
            // `await`ing on the pending transaction resolves to a transaction receipt
            let receipt = pending_tx.confirmations(6).await.unwrap();
            println!("{:?}", receipt);
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
