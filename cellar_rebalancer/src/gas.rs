//! Gas models
use crate::{collector, prelude::*};
use abscissa_core::error::BoxError;
use ethers::{
    middleware::gas_oracle::{Etherchain, Etherscan, GasCategory, GasOracle, GasOracleError},
    prelude::*,
};
use std::result::Result;
use tower::Service;

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

    #[allow(unused_mut)]
    pub async fn poll<S>(&self, mut collector: S)
    where
        S: Service<collector::Request, Response = collector::Response, Error = BoxError>
            + Send
            + Clone
            + 'static,
    {
        let gas = match CellarGas::etherscan_standard().await {
            Ok(gas) => gas,
            Err(err) => {
                warn!("Gas collection error:{}", err);
                return;
            }
        };
        collector.call(collector::Request::Gas(collector::request::GasPollEvent {
            current_gas_price: gas,
        }));
    }
}