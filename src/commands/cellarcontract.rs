//! `cellarcontract` subcommand
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
    Cellar,
    "./contract_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// Use generic data types for CellarWrapper struct since contract will have different data types.
pub struct CellarWrapper<T>{
    pub contract:Cellar<T>
}

// Implementation for CellarWrapper. initiate new CellarWrapper and other methods.
impl<T:Middleware > CellarWrapper<T>{
    pub fn new(address: H160, client:Arc<T>)-> Self{
        CellarWrapper{contract: Cellar::new(address,client)}
    }

    pub async fn rebalance(&mut self, cellar_tick_info:Vec<CellarTickInfo>){
        self.contract.rebalance(cellar_tick_info.into_iter().map(|x|x.to_tuple()).collect());
    }

    pub async fn add_liquidity_for_uni_v3(&mut self, cellar_add_params:CellarAddParams){
        self.contract.add_liquidity_for_uni_v3(cellar_add_params.to_tuple());
    }

    pub async fn add_liquidity_eth_for_uni_v3(&mut self, cellar_add_params:CellarAddParams){
        self.contract.add_liquidity_eth_for_uni_v3(cellar_add_params.to_tuple());
    }

    pub async fn remove_liquidity_eth_from_uni_v3(&mut self, cellar_remove_params:CellarRemoveParams){
        self.contract.remove_liquidity_eth_from_uni_v3(cellar_remove_params.to_tuple());
    }

    pub async fn remove_liquidity_from_uni_v3(&mut self, cellar_remove_params:CellarRemoveParams){
        self.contract.remove_liquidity_from_uni_v3(cellar_remove_params.to_tuple());
    }
}

// Struct for CellarTickInfo
pub struct CellarTickInfo{
    token_id:U256, 
    tick_upper:i32,
    tick_lower:i32,
    weight:u32,

}

// Implement CellarTickInfo. Initiate to_tuple method, to convert Vec<CellarTickInfo> to Tuples.
impl CellarTickInfo{
    pub fn new(token_id: U256, tick_upper:i32, tick_lower:i32, weight:u32)-> Self{
        CellarTickInfo{token_id, tick_upper, tick_lower, weight}
    }

    pub fn to_tuple(self)->(U256, i32, i32, u32){
       (self.token_id, self.tick_upper, self.tick_lower, self.weight)
    }
}

// For struct CellarAddParams
pub struct CellarAddParams {
    amount0_desired: U256,
    amount1_desired: U256,
    amount0_min: U256,
    amount1_min: U256,
    recipient: H160, // since recipient takes in an address, I used the U256 type.
    deadline: U256,
}

impl CellarAddParams {
    pub fn new(amount0_desired: U256, amount1_desired: U256, amount0_min: U256, amount1_min: U256, 
        recipient: H160, deadline: U256)-> Self{
        CellarAddParams{amount0_desired, amount1_desired, amount0_min, amount1_min, recipient, deadline}
    }

    pub fn to_tuple(self)->(U256, U256, U256, U256, H160, U256){
        (self.amount0_desired, self.amount1_desired, self.amount0_min, self.amount1_min, self.recipient, self.deadline)
     }
}


pub struct CellarRemoveParams {
    token_amount: U256,
    amount0_min: U256,
    amount1_min: U256,
    recipient: H160,
    deadline: U256,
}

impl CellarRemoveParams {
    pub fn new(token_amount: U256, amount0_min: U256, amount1_min: U256, recipient: H160, deadline: U256,) -> Self{
        CellarRemoveParams{token_amount, amount0_min, amount1_min, recipient, deadline}
    }

    pub fn to_tuple(self)->(U256, U256, U256, H160, U256){
        (self.token_amount, self.amount0_min, self.amount1_min, self.recipient, self.deadline)
     }
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
            ".parse::<Address>().unwrap();
            
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
            let make_transfer = contract.transfer_from( from, to, value).from(from);
            let transfer = make_transfer.send().await.unwrap();
            println!("{:?}", transfer);
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
