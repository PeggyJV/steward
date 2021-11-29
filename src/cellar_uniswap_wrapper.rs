//! Rust Wrapper for cellar functions
/// This will convert cellar functions from tuples to Rust types
use crate::error::Error;
use crate::prelude::*;
use ethers::prelude::*;
use rebalancer_abi::cellar_uniswap::*;
use std::result::Result;
use std::sync::Arc;

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
    pub async fn rebalance(&mut self, cellar_tick_info: Vec<CellarTickInfo>) -> Result<(), Error> {
        let mut ticks: Vec<CellarTickInfo> = cellar_tick_info
            .into_iter()
            .map(|x| CellarTickInfo {
                token_id: x.token_id,
                tick_upper: x.tick_upper,
                tick_lower: x.tick_lower,
                weight: x.weight,
            })
            .collect();
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
        cellar_add_params: CellarAddParams,
    ) -> Result<(), Error> {
        let mut call = self
            .contract
            .add_liquidity_for_uni_v3(cellar_add_params);

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
        cellar_add_params: CellarAddParams,
    ) -> Result<(), Error> {
        let call = self
            .contract
            .add_liquidity_eth_for_uni_v3(cellar_add_params);
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
        cellar_remove_params: CellarRemoveParams,
    ) -> Result<(), Error> {
        let call = self
            .contract
            .remove_liquidity_eth_from_uni_v3(cellar_remove_params);
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
        cellar_remove_params: CellarRemoveParams,
    ) -> Result<(), Error> {
        let call = self
            .contract
            .remove_liquidity_from_uni_v3(cellar_remove_params);
        let pending = call.send().await?;
        dbg!(&pending);
        let receipt = pending.confirmations(6).await?;
        match receipt {
            Some(receipt) => info!("Added liquidity for uniswap version 3, {:?}", receipt),
            None => info!("No pending transaction for add liquidity"),
        }

        Ok(())
    }

    pub async fn set_validator(&mut self, validator: H160, value: bool) -> Result<(), Error> {
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
