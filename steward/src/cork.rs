use crate::{
    cellars::{self, aave_v2_stablecoin},
    config,
    error::{
        Error,
        ErrorKind::{self, *},
    },
    prelude::APP,
    somm_send,
};
use abscissa_core::{
    tracing::log::{debug, error, info, warn},
    Application,
};
use deep_space::{Coin, Contact};
use gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse;
use somm_proto::cork::Cork;
use std::time::Duration;
use steward_proto::{
    self,
    steward::{self, submit_request::CallData::AaveV2Stablecoin, SubmitRequest, SubmitResponse},
};
use tonic::{self, async_trait, Code, Request, Response, Status};

pub mod cache;
pub mod client;

const MESSAGE_TIMEOUT: Duration = Duration::from_secs(10);
const CHAIN_PREFIX: &str = "somm";

pub struct CorkHandler;

#[async_trait]
impl steward::contract_call_server::ContractCall for CorkHandler {
    async fn submit(
        &self,
        request: Request<SubmitRequest>,
    ) -> Result<Response<SubmitResponse>, Status> {
        let request = request.get_ref().to_owned();
        let cellar_id = request.cellar_id.clone();
        if let Err(err) = cellars::validate_cellar_id(&cellar_id).await {
            let message = format!("cellar ID validation failed: {}", err);
            match err.kind() {
                Cache => return Err(Status::new(Code::Internal, message)),
                SPCall => return Err(Status::new(Code::InvalidArgument, message)),
                UnapprovedCellar => return Err(Status::new(Code::InvalidArgument, message)),
                _ => return Err(Status::new(Code::Unknown, message)),
            }
        }

        let cork = match build_cork(request).await {
            Ok(c) => c,
            Err(err) => {
                warn!("failed to build cork for cellar {}: {}", cellar_id, err);
                return Err(Status::new(Code::InvalidArgument, err.to_string()));
            }
        };
        debug!("cork: {:?}", cork);

        if let Err(err) = send_cork(cork).await {
            error!("failed to submit cork: {}", err);
            return Err(Status::new(
                Code::Internal,
                format!("failed to send cork to sommelier: {}", err),
            ));
        }
        info!("submitted cork for {}", cellar_id);

        Ok(Response::new(SubmitResponse {}))
    }
}
// Because of Rusts handling of enums, we have no easy way to log what cellar type and function are
// being requested before we get to the encoding step, so we pass the whole request into this method
// and the get_encoded_call() methods so logging can happen there.
async fn build_cork(request: SubmitRequest) -> Result<Cork, Error> {
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

async fn send_cork(cork: Cork) -> Result<TxResponse, Error> {
    let config = APP.config();
    debug!("establishing grpc connection");
    let contact = Contact::new(&config.cosmos.grpc, MESSAGE_TIMEOUT, CHAIN_PREFIX)?;

    debug!("getting cosmos fee");
    let cosmos_gas_price = config.cosmos.gas_price.as_tuple();
    let fee = Coin {
        amount: (cosmos_gas_price.0 as u64).into(),
        denom: cosmos_gas_price.1,
    };
    somm_send::send_cork(
        &contact,
        cork,
        config::DELEGATE_ADDRESS.to_string(),
        &config::DELEGATE_KEY,
        fee,
    )
    .await
    .map_err(|e| e.into())
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
