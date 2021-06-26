use ethers::contract::abigen;
use ethers::prelude::*;
use std::sync::Arc;
use crate::error::Error;

//use abigen macro to fetch and incorporate contract ABI
abigen!(
    Cellar,
    "./contract_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// Use generic data types for CellarWrapper struct since contract will have different data types.
pub struct ContractState<T> {
    contract: Cellar<T>,
}

pub struct ContractStateUpdate {}

// Implementation for CellarWrapper. initiate new CellarWrapper and other methods.
impl<T:'static+ Middleware> ContractState<T> {
    pub fn new(address: H160, client: Arc<T>) -> Self {
        ContractState {
            contract: Cellar::new(address, client),
        }
    }

    pub async fn rebalance(&mut self, cellar_tick_info: Vec<CellarTickInfo>) {
        self.contract
            .rebalance(cellar_tick_info.into_iter().map(|x| x.to_tuple()).collect());
    }

    pub async fn add_liquidity_for_uni_v3(&mut self, cellar_add_params: CellarAddParams) {
        self.contract
            .add_liquidity_for_uni_v3(cellar_add_params.to_tuple());
    }

    pub async fn add_liquidity_eth_for_uni_v3(&mut self, cellar_add_params: CellarAddParams) {
        self.contract
            .add_liquidity_eth_for_uni_v3(cellar_add_params.to_tuple());
    }

    pub async fn remove_liquidity_eth_from_uni_v3(
        &mut self,
        cellar_remove_params: CellarRemoveParams,
    )-> Result<(), Error> {
        self.contract
            .remove_liquidity_eth_from_uni_v3(cellar_remove_params.to_tuple()).call().await.map_err(|e| e.into())
    }

    pub async fn remove_liquidity_from_uni_v3(&mut self, cellar_remove_params: CellarRemoveParams) {
        self.contract
            .remove_liquidity_from_uni_v3(cellar_remove_params.to_tuple());
    }
}

// Struct for CellarTickInfo
pub struct CellarTickInfo {
    token_id: U256,
    tick_upper: i32,
    tick_lower: i32,
    weight: u32,
}

// Implement CellarTickInfo. Initiate to_tuple method, to convert Vec<CellarTickInfo> to Tuples.
impl CellarTickInfo {
    pub fn new(token_id: U256, tick_upper: i32, tick_lower: i32, weight: u32) -> Self {
        CellarTickInfo {
            token_id,
            tick_upper,
            tick_lower,
            weight,
        }
    }

    pub fn to_tuple(self) -> (U256, i32, i32, u32) {
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
    pub fn new(
        amount0_desired: U256,
        amount1_desired: U256,
        amount0_min: U256,
        amount1_min: U256,
        recipient: H160,
        deadline: U256,
    ) -> Self {
        CellarAddParams {
            amount0_desired,
            amount1_desired,
            amount0_min,
            amount1_min,
            recipient,
            deadline,
        }
    }

    pub fn to_tuple(self) -> (U256, U256, U256, U256, H160, U256) {
        (
            self.amount0_desired,
            self.amount1_desired,
            self.amount0_min,
            self.amount1_min,
            self.recipient,
            self.deadline,
        )
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
    pub fn new(
        token_amount: U256,
        amount0_min: U256,
        amount1_min: U256,
        recipient: H160,
        deadline: U256,
    ) -> Self {
        CellarRemoveParams {
            token_amount,
            amount0_min,
            amount1_min,
            recipient,
            deadline,
        }
    }

    pub fn to_tuple(self) -> (U256, U256, U256, H160, U256) {
        (
            self.token_amount,
            self.amount0_min,
            self.amount1_min,
            self.recipient,
            self.deadline,
        )
    }
}
