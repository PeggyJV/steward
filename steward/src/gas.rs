//! Gas models
use abscissa_core::Application;
use ethers::{
    middleware::gas_oracle::{Etherchain, Etherscan, GasCategory, GasOracle, GasOracleError},
    prelude::*,
};
use gravity_bridge::gravity_utils;
use std::result::Result;

use crate::{
    error::{Error, ErrorKind},
    prelude::APP,
};

pub struct CellarGas {
    pub max_gas_price: U256,
    pub current_gas: Option<U256>,
}

impl CellarGas {
    pub async fn etherscan_standard() -> Result<U256, GasOracleError> {
        let etherscan_client = Client::new_from_env(Chain::Mainnet)?;
        let etherscan_oracle = Etherscan::new(etherscan_client).category(GasCategory::Standard);
        let data = etherscan_oracle.fetch().await;
        data
    }

    pub async fn etherscan_safelow() -> Result<U256, GasOracleError> {
        let etherscan_client = Client::new_from_env(Chain::Mainnet)?;
        let etherscan_oracle = Etherscan::new(etherscan_client).category(GasCategory::SafeLow);
        let data = etherscan_oracle.fetch().await;
        data
    }

    #[allow(dead_code)]
    async fn etherchain_fastest() -> Result<U256, GasOracleError> {
        let etherchain_oracle = Etherchain::new().category(GasCategory::Fastest);
        let data = etherchain_oracle.fetch().await;
        data
    }

    #[allow(dead_code)]
    async fn etherchain_fast() -> Result<U256, GasOracleError> {
        let etherchain_oracle = Etherchain::new().category(GasCategory::Fast);
        let data = etherchain_oracle.fetch().await;
        data
    }

    #[allow(dead_code)]
    async fn etherchain_standard() -> Result<U256, GasOracleError> {
        let etherchain_oracle = Etherchain::new().category(GasCategory::Standard);
        let data = etherchain_oracle.fetch().await;
        data
    }

    #[allow(dead_code)]
    async fn etherchain_safelow() -> Result<U256, GasOracleError> {
        let etherchain_oracle = Etherchain::new().category(GasCategory::SafeLow);
        let data = etherchain_oracle.fetch().await;
        data
    }

    pub fn pad(price: U256) -> Result<U256, Error> {
        let config = APP.config();
        let price = match gravity_utils::ethereum::downcast_to_f32(price) {
            Some(p) => p,
            None => {
                return Err(ErrorKind::GasOracle
                    .context("failed to downcast gas price estimate")
                    .into())
            }
        };
        let price = price * config.ethereum.gas_price_multiplier;

        Ok((price as u64).into())
    }
}
