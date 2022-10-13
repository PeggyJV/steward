use crate::{
    config,
    error::{Error},
    prelude::APP,
    somm_send,
};
use abscissa_core::{
    tracing::log::{debug},
    Application,
};
use deep_space::{Coin, Contact};
use gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse;
use somm_proto::cork::{Cork};
use std::time::Duration;

const MESSAGE_TIMEOUT: Duration = Duration::from_secs(10);
const CHAIN_PREFIX: &str = "somm";

pub async fn schedule_cork(
    contract: String,
    encoded_call: Vec<u8>,
    height: u64,
) -> Result<TxResponse, Error> {
    let config = APP.config();
    debug!("establishing grpc connection");
    let contact = Contact::new(&config.cosmos.grpc, MESSAGE_TIMEOUT, CHAIN_PREFIX).unwrap();

    debug!("getting cosmos fee");
    let cosmos_gas_price = config.cosmos.gas_price.as_tuple();
    let fee = Coin {
        amount: (cosmos_gas_price.0 as u64).into(),
        denom: cosmos_gas_price.1,
    };
    let cork = Cork {
        encoded_contract_call: encoded_call,
        target_contract_address: contract.clone(),
    };
    somm_send::schedule_cork(
        &contact,
        cork,
        config::DELEGATE_ADDRESS.to_string(),
        &config::DELEGATE_KEY,
        fee,
        height,
    )
    .await
    .map_err(|e| e.into())
}
