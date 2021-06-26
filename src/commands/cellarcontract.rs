//! `cellarcontract` subcommand
use crate::application::APP;
use crate::cellar_wrapper;
/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;
use crate::time_range;
use abscissa_core::{Command, Options, Runnable};
use ethers::contract::abigen;
use ethers::{
    middleware::gas_oracle::{
        Etherchain, Etherscan, GasCategory, GasNow, GasOracle, GasOracleError,
    },
    prelude::*,
    providers::{Http, Provider},
    types::Address,
};
use std::{convert::TryFrom, sync::Arc};
//use abigen macro to fetch and incorporate contract ABI
abigen!(
    Cellar,
    "./contract_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

async fn etherscan_standard() -> Result<U256, GasOracleError> {
    let api_key = std::env::var("ETHERSCAN_API_KEY").unwrap();
    let api_key = Some(api_key.as_str());
    let etherscan_oracle = Etherscan::new(api_key).category(GasCategory::Standard);
    let data = etherscan_oracle.fetch().await;
    data
}

async fn etherscan_safelow() -> Result<U256, GasOracleError> {
    let api_key = std::env::var("ETHERSCAN_API_KEY").unwrap();
    let api_key = Some(api_key.as_str());
    let etherscan_oracle = Etherscan::new(api_key).category(GasCategory::SafeLow);
    let data = etherscan_oracle.fetch().await;
    data
}

async fn etherchain_fastest() -> Result<U256, GasOracleError> {
    let etherchain_oracle = Etherchain::new().category(GasCategory::Fastest);
    let data = etherchain_oracle.fetch().await;
    data
}

async fn etherchain_fast() -> Result<U256, GasOracleError> {
    let etherchain_oracle = Etherchain::new().category(GasCategory::Fast);
    let data = etherchain_oracle.fetch().await;
    data
}

async fn etherchain_standard() -> Result<U256, GasOracleError> {
    let etherchain_oracle = Etherchain::new().category(GasCategory::Standard);
    let data = etherchain_oracle.fetch().await;
    data
}

async fn etherchain_safelow() -> Result<U256, GasOracleError> {
    let etherchain_oracle = Etherchain::new().category(GasCategory::SafeLow);
    let data = etherchain_oracle.fetch().await;
    data
}

async fn sparkpool_fatest() -> Result<U256, GasOracleError> {
    let gas_now_oracle = GasNow::new().category(GasCategory::Fastest);
    let data = gas_now_oracle.fetch().await;
    data
}

async fn sparkpool_fast() -> Result<U256, GasOracleError> {
    let gas_now_oracle = GasNow::new().category(GasCategory::Fast);
    let data = gas_now_oracle.fetch().await;
    data
}

async fn sparkpool_standard() -> Result<U256, GasOracleError> {
    let gas_now_oracle = GasNow::new().category(GasCategory::Standard);
    let data = gas_now_oracle.fetch().await;
    data
}

async fn sparkpool_safelow() -> Result<U256, GasOracleError> {
    let gas_now_oracle = GasNow::new().category(GasCategory::SafeLow);
    let data = gas_now_oracle.fetch().await;
    data
}

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
    fn run(&self) {
        abscissa_tokio::run(&APP, async {
            // This is the address of the deployed contract.
            let address = "0x44692093E7A78447D8Ddbc477192934520928A5B
            "
            .parse::<Address>()
            .unwrap();

            // Connect to the network provider (example below is for my Ganache-cli fork)
            let client = Provider::<Http>::try_from("http://localhost:7545").unwrap();

            // MyContract expects Arc, create with client
            let client = Arc::new(client);

            // Use ethers_rs `get_accounts` method to get client accounts
            let accounts = client.get_accounts().await.unwrap();
            let from = accounts[0];
            let to = accounts[1];
            let value = U256::from(0);

            // Create the contract object at the address with the provider
            let contract = Cellar::new(address, client);

            // Test that contract creation was successful by printing contract address
            let addr = contract.address();

            // Get name of Contract with cellars `name` method
            let get_name = contract.method::<_, String>("name", ()).unwrap();
            let name: String = get_name.call().await.unwrap();
            println!("{:?}", name);

            // Get symbol of Contract with cellars `symbol` method
            let get_symbol = contract.method::<_, String>("symbol", ()).unwrap();
            let symbol: String = get_symbol.call().await.unwrap();
            println!("{:?}", symbol);

            // Get Token decimals with cellars `decimals` method
            let get_decimals = contract.method::<_, U128>("decimals", ()).unwrap();
            let decimals: U128 = get_decimals.call().await.unwrap();
            println!("{:?}", decimals);

            // Get balance of Address with cellars `balanceOf` method
            let get_balance = contract.method::<_, U256>("balanceOf", addr).unwrap();
            let balance: U256 = get_balance.call().await.unwrap();
            println!("{:?}", balance);

            //make transaction.
            let make_transfer = contract.transfer_from(from, to, value).from(from);
            let transfer = make_transfer.send().await.unwrap();
            println!("{:?}", transfer);
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
