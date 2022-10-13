use crate::{
    cellars::{self, aave_v2_stablecoin},
    config,
    error::{Error, ErrorKind},
    prelude::APP,
    somm_send,
};
use abscissa_core::{
    tracing::log::{debug, error, info, warn},
    Application,
};
use deep_space::{Coin, Contact};
use gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse;
use somm_proto::cork::{query_client::QueryClient as CorkQueryClient, Cork, QueryCellarIDsRequest};
use std::time::Duration;
use steward_proto::{
    self,
    steward::{self, submit_request::CallData::AaveV2Stablecoin, SubmitRequest, SubmitResponse},
};
use tonic::{self, async_trait, Code, Request, Response, Status};

const MESSAGE_TIMEOUT: Duration = Duration::from_secs(10);
const CHAIN_PREFIX: &str = "somm";


// Because of Rusts handling of enums, we have no easy way to log what cellar type and function are
// being requested before we get to the encoding step, so we pass the whole request into this method
// and the get_encoded_call() methods so logging can happen there.
async fn build_cork(request: SubmitRequest) -> Result<Cork, Error> {
    cellars::validate_cellar_id(request.cellar_id.as_str())?;

    let address = request.cellar_id.clone();
    let encoded_call = get_encoded_call(request)?;

    Ok(Cork {
        encoded_contract_call: encoded_call,
        target_contract_address: address,
    })
}

fn get_encoded_call(request: SubmitRequest) -> Result<Vec<u8>, Error> {
    if request.call_data.is_none() {
        return Err(ErrorKind::Http.context("empty contract call data").into());
    }

    match request.call_data.unwrap() {
        AaveV2Stablecoin(call) => {
            if call.function.is_none() {
                return Err(ErrorKind::Http.context("empty function data").into());
            }

            aave_v2_stablecoin::get_encoded_call(call.function.unwrap(), request.cellar_id)
        }
    }
}

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
