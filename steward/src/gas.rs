//! Gas models
use ethers::{
    middleware::gas_oracle::{Etherchain, Etherscan, GasCategory, GasOracle, GasOracleError},
    prelude::*,
};
use std::result::Result;

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
}
