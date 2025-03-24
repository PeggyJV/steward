//! Gas models
use crate::{
    error::{Error, ErrorKind},
    prelude::APP,
    utils::{get_chain, get_eth_provider},
};
use abscissa_core::{tracing::log::warn, Application};
use ethers::{
    middleware::gas_oracle::{Etherscan, GasCategory, GasOracle},
    prelude::*,
};
use std::result::Result;

pub struct CellarGas {
    pub max_gas_price: U256,
    pub current_gas: Option<U256>,
}

impl CellarGas {
    pub fn apply_gas_multiplier(price: U256) -> Result<U256, Error> {
        let config = APP.config();
        let price = match gravity_bridge::gravity::utils::ethereum::downcast_to_f32(price) {
            Some(p) => p,
            None => {
                return Err(ErrorKind::GasOracle
                    .context("failed to downcast gas price estimate")
                    .into())
            }
        };
        let price = price * config.ethereum.gas_price_multiplier;

        Ok((price as u128).into())
    }

    pub async fn etherscan_standard() -> Result<U256, Error> {
        let etherscan_client = CellarGas::get_etherscan_client().await?;
        let etherscan_oracle = Etherscan::new(etherscan_client).category(GasCategory::Standard);
        let gas_estimate = etherscan_oracle.fetch().await;

        gas_estimate.map_err(|e| e.into())
    }

    async fn get_etherscan_client() -> Result<Client, Error> {
        let provider = get_eth_provider().await?;
        let chain = get_chain(provider.clone()).await?;

        Client::new_from_env(chain).map_err(|e| e.into())
    }

    pub async fn get_gas_price() -> Result<U256, Error> {
        if std::env::var("ETHERSCAN_API_KEY").is_ok() {
            match CellarGas::etherscan_standard().await {
                Ok(gas) => return Ok(gas),
                Err(err) => {
                    warn!("failed to retrieve gas estimate from etherscan: {}", err);
                }
            }
        }

        let provider = get_eth_provider().await?;

        provider.get_gas_price().await.map_err(|r| r.into())
    }
}
