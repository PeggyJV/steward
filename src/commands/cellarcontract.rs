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
    recipient: U256, // since recipient takes in an address, I used the U256 type.
    deadline: U256,
}

impl CellarAddParams {
    pub fn new(amount0_desired: U256, amount1_desired: U256, amount0_min: U256, amount1_min: U256, 
        recipient: U256, deadline: U256)-> Self{
        CellarAddParams{amount0_desired, amount1_desired, amount0_min, amount1_min, recipient, deadline}
    }
}

// For 
pub struct CellarRemoveParams {
    token_amount: U256,
    amount0_min: U256,
    amount1_min: U256,
    recipient: U256,
    deadline: U256,
}

impl CellarRemoveParams {
    pub fn new(token_amount: U256, amount0_min: U256, amount1_min: U256, recipient: U256, deadline: U256,) -> Self{
        CellarRemoveParams{token_amount, amount0_min, amount1_min, recipient, deadline}
    }
}

pub struct PoolInfo {
    token_0: U256,
    token_1: U256,
    fee_level: u32,
}

impl PoolInfo {
    pub fn new(token_0: U256, token_1: U256, fee_level: u32,) -> Self {
        PoolInfo{token_0, token_1, fee_level}
    }
}

pub struct UintPair {
    a: U256,
    b: U256,
}

impl UintPair {
    pub fn new(a: U256, b: U256) -> Self {
        UintPair{a, b}
    }
}

pub struct MintParams {
    token_0: U256,
    token_1: U256,
    fee: u32,
    tick_lower: i32,
    tick_upper: i32,
    amount0_desired: U256,
    amount1_desired: U256,
    amount0_min: U256,
    amount1_min: U256,
    recipient: U256,
    deadline: U256,
}

impl MintParams {
    pub fn new(token_0: U256, token_1: U256, fee: u32, tick_lower: i32, tick_upper: i32, amount0_desired: U256, 
        amount1_desired: U256, amount0_min: U256, amount1_min: U256, recipient: U256, deadline: U256,) -> Self {
            MintParams{token_0, token_1, fee, tick_lower, tick_upper, amount0_desired, amount1_desired, amount0_min, amount1_min, recipient, deadline}
    }
}

pub struct MintResult {
    token_id: U256,
    liquidity: u128,
    amount_0: U256,
    amount_1: U256,
}

impl MintResult {
    pub fn new(token_id: U256, liquidity: u128, amount_0: U256, amount_1: U256,) -> Self {
        MintResult{token_id, liquidity, amount_0, amount_1}
    }
}

pub struct IncreaseLiquidityParams {
    token_id: U256,
    amount0_desired: U256,
    amount1_desired: U256,
    amount0_min: U256,
    amount1_min: U256,
    deadline: U256,
}

impl IncreaseLiquidityParams {
    pub fn new(token_id: U256, amount0_desired: U256, amount1_desired: U256, amount0_min: U256, amount1_min: U256, deadline: U256,) -> Self {
        IncreaseLiquidityParams {token_id, amount0_desired, amount1_desired, amount0_min, amount1_min, deadline}
    }
}

pub struct DecreaseLiquidityParams {
    token_id: U256,
    liquidity: u128,
    amount0_min: U256,
    amount1_min: U256,
    deadline: U256,
}

impl DecreaseLiquidityParams {
    pub fn new(token_id: U256, liquidity: u128, amount0_min: U256, amount1_min: U256, deadline: U256) -> Self {
        DecreaseLiquidityParams{token_id, liquidity, amount0_min, amount1_min, deadline}
    }
}

pub struct CollectParams {
    token_id: U256,
    recipient: U256,
    amount0_max: u128,
    amount1_max: u128,
}

impl CollectParams {
    pub fn new(token_id: U256, recipient: U256, amount0_max: u128, amount1_max: u128) -> Self {
        CollectParams{token_id, recipient, amount0_max, amount1_max}
    }
}

pub struct ExactInputSingleParams {
    token_in: U256,
    token_out: U256,
    fee: u32,
    recipient: U256,
    deadline: U256,
    amount_in: U256,
    amount_out_minimum: U256,
    sqrt_price_limit_x96: H160, //I'm not so sure, I got a hint from https://docs.rs/primitive-types/0.5.0/primitive_types/struct.H160.html
}

impl ExactInputSingleParams {
    pub fn new(token_in: U256, token_out: U256, fee: u32, recipient: U256, deadline: U256, amount_in: U256, amount_out_minimum: U256, sqrt_price_limit_x96: H160, ) -> Self {
        ExactInputSingleParams{token_in, token_out, fee, recipient, deadline, amount_in, amount_out_minimum, sqrt_price_limit_x96}
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
