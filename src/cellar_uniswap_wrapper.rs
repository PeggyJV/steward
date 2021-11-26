//! Rust Wrapper for cellar functions
/// This will convert cellar functions from tuples to Rust types
use crate::error::Error;
use crate::prelude::*;
use ethers::prelude::*;
use std::sync::Arc;
use rebalancer_abi::cellar_uniswap::UniswapV3Cellar;
use std::result::Result;

// Use generic data types for CellarWrapper struct since contract will have different data types.
pub struct UniswapV3CellarState<T> {
    pub contract: UniswapV3Cellar<T>,
    pub gas_price: Option<U256>,
}

pub struct ContractStateUpdate {}

// Implementation for ContractState.
impl<T: 'static + Middleware> UniswapV3CellarState<T> {
    // Instantiate `new` ContractState
    pub fn new(address: H160, client: Arc<T>) -> Self {
        UniswapV3CellarState {
            contract: UniswapV3Cellar::new(address, client),
            gas_price: None,
        }
    }

    // Rebalance portfolio with cellar tick info
    pub async fn rebalance(&mut self, cellar_tick_info: Vec<UniswapV3CellarTickInfo>) -> Result<(), Error> {
        let mut ticks: Vec<(U256, i32, i32, u32)> =
            cellar_tick_info.into_iter().map(|x| x.to_tuple()).collect();
        ticks.reverse();

        let mut call = self.contract.rebalance(ticks);

        if let Some(gas_price) = self.gas_price {
            call = call.gas_price(gas_price)
        }

        let gased = call.gas(5_000_000);

        let pending = gased.send().await?;
        dbg!(&pending);

        Ok(())
    }

        // Rebalance portfolio with cellar tick info
        pub async fn reinvest(&mut self) -> Result<(), Error> {

            let mut call = self.contract.reinvest();
    
            if let Some(gas_price) = self.gas_price {
                call = call.gas_price(gas_price)
            }
    
            let gased = call.gas(5_000_000);
    
            let pending = gased.send().await?;
            dbg!(&pending);
    
            Ok(())
        }

    // Add liquidity for uniswap version 3 with values form struct `CellarAddParams`
    pub async fn add_liquidity_for_uni_v3(
        &mut self,
        cellar_add_params: UniswapV3CellarAddParams,
    ) -> Result<(), Error> {
        let mut call = self
            .contract
            .add_liquidity_for_uni_v3(cellar_add_params.to_tuple());

        if let Some(gas_price) = self.gas_price {
            call = call.gas_price(gas_price)
        }
        let gased = call.gas(5_000_000);

        let pending = gased.send().await?;

        info!("Pending: {:?}", pending);

        // let receipt = pending.confirmations(6).await?;
        // match receipt {
        //     Some(receipt) => info!("Added liquidity for uniswap version 3, {:?}", receipt),
        //     None => info!("No pending transaction for add liquidity"),
        // }

        Ok(())
    }

    // Add ethereum liquidity for uniswap version 3 with values form struct `CellarAddParams`
    pub async fn add_liquidity_eth_for_uni_v3(
        &mut self,
        cellar_add_params: UniswapV3CellarAddParams,
    ) -> Result<(), Error> {
        let call = self
            .contract
            .add_liquidity_eth_for_uni_v3(cellar_add_params.to_tuple());
        let pending = call.send().await?;

        let receipt = pending.confirmations(6).await?;
        match receipt {
            Some(receipt) => info!("Added liquidity for uniswap version 3, {:?}", receipt),
            None => info!("No pending transaction for add liquidity"),
        }

        Ok(())
    }

    // Remove ethereum liquidity from uniswap version 3 with values form struct `CellarAddParams`
    pub async fn remove_liquidity_eth_from_uni_v3(
        &mut self,
        cellar_remove_params: UniswapV3CellarRemoveParams,
    ) -> Result<(), Error> {
        let call = self
            .contract
            .remove_liquidity_eth_from_uni_v3(cellar_remove_params.to_tuple());
        let pending = call.send().await?;

        let receipt = pending.confirmations(6).await?;
        match receipt {
            Some(receipt) => info!("Added liquidity for uniswap version 3, {:?}", receipt),
            None => info!("No pending transaction for add liquidity"),
        }

        Ok(())
    }

    // Remove liquidity from uniswap version 3 with values form struct `CellarAddParams`
    pub async fn remove_liquidity_from_uni_v3(
        &mut self,
        cellar_remove_params: UniswapV3CellarRemoveParams,
    ) -> Result<(), Error> {
        let call = self
            .contract
            .remove_liquidity_from_uni_v3(cellar_remove_params.to_tuple());
        let pending = call.send().await?;
        dbg!(&pending);
        let receipt = pending.confirmations(6).await?;
        match receipt {
            Some(receipt) => info!("Added liquidity for uniswap version 3, {:?}", receipt),
            None => info!("No pending transaction for add liquidity"),
        }

        Ok(())
    }    

    pub async fn set_validator(
            &mut self, validator: H160,
            value: bool
    ) -> Result<(), Error> {
        let mut call = self.contract.set_validator(validator, value);

        if let Some(gas_price) = self.gas_price {
            call = call.gas_price(gas_price)
        }

        let gased = call.gas(5_000_000);

        let pending = gased.send().await?;
        dbg!(&pending);

        Ok(())
    }
}

// Struct for UniswapV3CellarTickInfo
#[derive(Clone, Debug)]
pub struct UniswapV3CellarTickInfo {
    pub(crate) token_id: U256,
    pub(crate) tick_upper: i32,
    pub(crate) tick_lower: i32,
    pub(crate) weight: u32,
}

// Implement UniswapV3CellarTickInfo. Initiate to_tuple method, to convert Vec<UniswapV3CellarTickInfo> to Tuples.
impl UniswapV3CellarTickInfo {
    pub fn new(token_id: U256, tick_upper: i32, tick_lower: i32, weight: u32) -> Self {
        UniswapV3CellarTickInfo {
            token_id,
            tick_upper,
            tick_lower,
            weight,
        }
    }

    pub fn to_tuple(self) -> (U256, i32, i32, u32) {
        (self.token_id, self.tick_upper, self.tick_lower, self.weight)
    }

    // pub fn to_abi(self)->cellar_mod::UniswapV3CellarTickInfo{
    //     cellar_mod::UniswapV3CellarTickInfo {
    //         token_id: self.token_id,
    //         tick_upper: self.tick_upper,
    //         tick_lower: self.tick_lower,
    //         weight: self.weight,
    //     }
    // }

    pub fn from_tick_weight(tick_weight: &crate::time_range::TickWeight) -> UniswapV3CellarTickInfo {
        UniswapV3CellarTickInfo {
            token_id: U256::zero(),
            tick_upper: tick_weight.upper_bound,
            tick_lower: tick_weight.lower_bound,
            weight: tick_weight.weight,
        }
    }

    pub fn valid(&self) -> bool {
        if self.tick_upper > self.tick_lower {
            true
        } else {
            false
        }
    }

    pub fn within(&self, tick: &i32) -> bool {
        if &self.tick_lower <= tick && tick <= &self.tick_upper {
            return true;
        }
        return false;
    }
}

impl std::cmp::PartialOrd<i32> for UniswapV3CellarTickInfo {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        if other < &self.tick_lower {
            Some(std::cmp::Ordering::Less)
        } else if other > &self.tick_upper {
            Some(std::cmp::Ordering::Greater)
        } else if self.within(other) {
            Some(std::cmp::Ordering::Equal)
        } else {
            None
        }
    }
}
impl std::cmp::PartialEq<i32> for UniswapV3CellarTickInfo {
    fn eq(&self, other: &i32) -> bool {
        self.within(other)
    }
}

// Struct for CellarAddParams
pub struct UniswapV3CellarAddParams {
    amount0_desired: U256,
    amount1_desired: U256,
    amount0_min: U256,
    amount1_min: U256,
    recipient: H160, // since recipient takes in an address, I used the U256 type.
    deadline: U256,
}

// Implement CellarAddParams
impl UniswapV3CellarAddParams {
    // Instantiate `new` CellarAddParams
    pub fn new(
        amount0_desired: U256,
        amount1_desired: U256,
        amount0_min: U256,
        amount1_min: U256,
        recipient: H160,
        deadline: U256,
    ) -> Self {
        UniswapV3CellarAddParams {
            amount0_desired,
            amount1_desired,
            amount0_min,
            amount1_min,
            recipient,
            deadline,
        }
    }

    // Convert CellarAddParams fields to tuple
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

    // pub fn to_abi(self) -> cellar_mod::CellarAddParams {
    //     cellar_mod::CellarAddParams{
    //         amount_0_desired: self.amount0_desired,
    //         amount_1_desired: self.amount1_desired,
    //         amount_0_min:self.amount0_min,
    //         amount_1_min:self.amount1_min,
    //         recipient: self.recipient,
    //         deadline: self.deadline,
    //     }
    // }
}

// Struct for CellarRemoveParams
pub struct UniswapV3CellarRemoveParams {
    token_amount: U256,
    amount0_min: U256,
    amount1_min: U256,
    recipient: H160,
    deadline: U256,
}

// Implement CellarRemoveParams
impl UniswapV3CellarRemoveParams {
    // Instantiate `new` CellarRemoveParams
    pub fn new(
        token_amount: U256,
        amount0_min: U256,
        amount1_min: U256,
        recipient: H160,
        deadline: U256,
    ) -> Self {
        UniswapV3CellarRemoveParams {
            token_amount,
            amount0_min,
            amount1_min,
            recipient,
            deadline,
        }
    }

    // Convert CellarRemoveParams fields to tuple
    pub fn to_tuple(self) -> (U256, U256, U256, H160, U256) {
        (
            self.token_amount,
            self.amount0_min,
            self.amount1_min,
            self.recipient,
            self.deadline,
        )
    }

    // pub fn to_abi(self) -> cellar_mod::CellarRemoveParams {
    //     cellar_mod::CellarRemoveParams {
    //         token_amount: self.token_amount,
    //         amount_0_min: self.amount0_min,
    //         amount_1_min: self.amount1_min,
    //         recipient: self.recipient,
    //         deadline: self.deadline,
    //     }

    // }to
}
