use crate::{
    cellars::{self, aave_v2_stablecoin, cellar_v1},
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
    steward::{self, schedule_request::CallData::*, ScheduleRequest, ScheduleResponse},
};
use tonic::{self, async_trait, Code, Request, Response, Status};

const MESSAGE_TIMEOUT: Duration = Duration::from_secs(10);
const CHAIN_PREFIX: &str = "somm";

pub struct CorkHandler;

#[async_trait]
impl steward::contract_call_server::ContractCall for CorkHandler {
    async fn schedule(
        &self,
        request: Request<ScheduleRequest>,
    ) -> Result<Response<ScheduleResponse>, Status> {
        let request = request.get_ref().to_owned();

        // Check if cellar is governance approved before building cork
        let config = APP.config();
        let mut client = match CorkQueryClient::connect(config.cosmos.grpc.clone()).await {
            Ok(c) => c,
            Err(err) => {
                error!("cork query client connection failed: {}", err);
                return Err(Status::new(
                    Code::Internal,
                    format!("failed to query chain to validate cellar id: {}", err),
                ));
            }
        };

        debug!("checking if cellar ID is approved");
        let ids = &client
            .query_cellar_i_ds(QueryCellarIDsRequest {})
            .await?
            .get_ref()
            .cellar_ids
            .clone();
        if !ids.contains(&request.cellar_id) {
            info!(
                "rejecting request for unapproved cellar {}",
                request.cellar_id
            );
            return Err(Status::new(
                Code::PermissionDenied,
                format!(
                    "cellar ID {} not approved by governance",
                    &request.cellar_id
                ),
            ));
        }

        // Build and send cork
        let cellar_id = request.cellar_id.clone();
        let height = request.block_height.clone();
        if let Err(err) = cellars::validate_cellar_id(&request.cellar_id) {
            return Err(Status::new(Code::InvalidArgument, err.to_string()))
        }
        let encoded_call = match get_encoded_call(request) {
            Ok(c) => c,
            Err(err) => {
                warn!("failed to get encoded call for {}: {}", cellar_id, err);
                return Err(Status::new(Code::InvalidArgument, err.to_string()));
            }
        };
        debug!("cork: {:?}", encoded_call);

        if let Err(err) = schedule_cork(&cellar_id, encoded_call, height).await {
            error!("failed to submit cork: {}", err);
            return Err(Status::new(
                Code::Internal,
                format!("failed to send cork to sommelier: {}", err),
            ));
        }
        info!("submitted cork for {}", cellar_id);

        Ok(Response::new(ScheduleResponse {}))
    }
}

fn get_encoded_call(request: ScheduleRequest) -> Result<Vec<u8>, Error> {
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
        CellarV1(call) => {
            if call.function.is_none() {
                return Err(ErrorKind::Http.context("empty function data").into());
            }

            cellar_v1::get_encoded_call(call.function.unwrap(), request.cellar_id)
        }
    }
}

pub async fn schedule_cork(
    contract: &str,
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
        target_contract_address: contract.to_string(),
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
