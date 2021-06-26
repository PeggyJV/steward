//! Gas models
use ethers::{
    middleware::gas_oracle::{
        Etherchain, Etherscan, GasCategory, GasNow, GasOracle, GasOracleError,
    },
    prelude::*,
};

#[allow(unused_imports)]
use crate::{collector, config, prelude::*};
use abscissa_core::error::BoxError;
#[allow(unused_imports)]
use tower::{util::ServiceExt, Service};

pub struct CellarGas {}

impl CellarGas {
    pub async fn etherscan_standard() -> Result<U256, GasOracleError> {
        let api_key = std::env::var("ETHERSCAN_API_KEY").unwrap();
        let api_key = Some(api_key.as_str());
        let etherscan_oracle = Etherscan::new(api_key).category(GasCategory::Standard);
        let data = etherscan_oracle.fetch().await;
        data
    }

    pub async fn etherscan_safelow() -> Result<U256, GasOracleError> {
        let api_key = std::env::var("ETHERSCAN_API_KEY").unwrap();
        let api_key = Some(api_key.as_str());
        let etherscan_oracle = Etherscan::new(api_key).category(GasCategory::SafeLow);
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

    #[allow(dead_code)]
    async fn sparkpool_fatest() -> Result<U256, GasOracleError> {
        let gas_now_oracle = GasNow::new().category(GasCategory::Fastest);
        let data = gas_now_oracle.fetch().await;
        data
    }

    #[allow(dead_code)]
    async fn sparkpool_fast() -> Result<U256, GasOracleError> {
        let gas_now_oracle = GasNow::new().category(GasCategory::Fast);
        let data = gas_now_oracle.fetch().await;
        data
    }

    #[allow(dead_code)]
    async fn sparkpool_standard() -> Result<U256, GasOracleError> {
        let gas_now_oracle = GasNow::new().category(GasCategory::Standard);
        let data = gas_now_oracle.fetch().await;
        data
    }

    #[allow(dead_code)]
    async fn sparkpool_safelow() -> Result<U256, GasOracleError> {
        let gas_now_oracle = GasNow::new().category(GasCategory::SafeLow);
        let data = gas_now_oracle.fetch().await;
        data
    }

    #[allow(unused_mut)]
    pub async fn poll<S>(&self, mut _collector: S)
    where
        S: Service<collector::Request, Response = collector::Response, Error = BoxError>
            + Send
            + Clone
            + 'static,
    {
        todo!()
    }
}
