//! Rust Wrapper for cellar functions
/// This will convert cellar functions from tuples to Rust types
use crate::error::Error;
use ethers::contract::abigen;
use ethers::prelude::*;
use std::sync::Arc;

//use abigen macro to fetch and incorporate contract ABI
abigen!(
    Erc20,
    "./erc20_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

//use abigen macro to fetch and incorporate contract ABI
abigen!(
    Weth,
    "./weth_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

pub struct Erc20State<T> {
    contract: Erc20<T>,
}

pub struct WethState<T> {
    contract: Weth<T>,
}
