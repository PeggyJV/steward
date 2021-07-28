//! Rust Wrapper for cellar functions
/// This will convert cellar functions from tuples to Rust types
use crate::error::Error;
use ethers::contract::abigen;
use ethers::prelude::*;
use std::sync::Arc;

//use abigen macro to fetch and incorporate contract ABI
abigen!(
    UniswapRouter,
    "./uniswap_router_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

pub struct RouterState<T> {
    contract: UniswapRouter<T>,
}

impl<T: 'static + Middleware> RouterState<T>{
}
