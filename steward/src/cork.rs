use crate::{
    cellars::{self, aave_v2_stablecoin, uniswapv3},
    config,
    error::{Error, ErrorKind},
    prelude::APP,
    somm_send,
};
use abscissa_core::{
    tracing::log::{debug, error, warn},
    Application,
};
use deep_space::{Coin, Contact};
use gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse;
use somm_proto::cork::Cork;
use std::time::Duration;
use steward_proto::{
    self,
    steward::{
        self,
        submit_request::CallData::{self, AaveV2Stablecoin, Uniswapv3Rebalance},
        SubmitRequest, SubmitResponse,
    },
};
use tonic::{self, async_trait, Code, Request, Response, Status};

pub mod cache;
pub mod client;

const MESSAGE_TIMEOUT: Duration = Duration::from_secs(10);

pub struct CorkHandler;

#[async_trait]
impl steward::contract_call_server::ContractCall for CorkHandler {
    async fn submit(
        &self,
        request: Request<SubmitRequest>,
    ) -> Result<Response<SubmitResponse>, Status> {
        debug!("received contract call request");
        let request = request.get_ref();

        // Build and send cork
        let cork = match build_cork(request).await {
            Ok(c) => c,
            Err(err) => {
                warn!("failed to build cork: {}", err);
                return Err(Status::new(Code::InvalidArgument, err.to_string()));
            }
        };
        debug!("cork: {:?}", cork);

        if let Err(err) = send_cork(cork).await {
            error!("failed to submit cork: {}", err);
            return Err(Status::new(
                Code::Internal,
                "failed to send cork to sommelier",
            ));
        }
        debug!("sent cork!");

        Ok(Response::new(SubmitResponse {}))
    }
}

async fn build_cork(request: &SubmitRequest) -> Result<Cork, Error> {
    cellars::validate_cellar_id(request.cellar_id.as_str()).await?;
    let address = request.cellar_id.clone();
    let contract_call_data = match request.call_data.clone() {
        Some(call) => call,
        None => return Err(ErrorKind::Http.context("empty contract call data").into()),
    };
    let encoded_call = get_encoded_call(contract_call_data)?;

    Ok(Cork {
        encoded_contract_call: encoded_call,
        target_contract_address: address,
    })
}

fn get_encoded_call(data: CallData) -> Result<Vec<u8>, Error> {
    match data {
        AaveV2Stablecoin(call) => Ok(aave_v2_stablecoin::get_encoded_call(
            call.function
                .expect("call data for Aave V2 Stablecoin cellar was empty"),
        )),
        Uniswapv3Rebalance(params) => Ok(uniswapv3::get_encoded_call(params)),
    }
}

async fn send_cork(cork: Cork) -> Result<TxResponse, Error> {
    let config = APP.config();
    debug!("establishing grpc connection");
    let contact = Contact::new(&config.cosmos.grpc, MESSAGE_TIMEOUT, &config.cosmos.prefix)?;

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
