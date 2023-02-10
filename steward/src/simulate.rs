use abscissa_core::{
    tracing::log::{info, warn},
};
use steward_proto::steward::{
    self,
    schedule_request::CallData::{AaveV2Stablecoin, CellarV1, CellarV2},
    ScheduleRequest, SimulateRequest, SimulateResponse,
};
use tonic::{async_trait, Code, Request, Response, Status};

use crate::{
    cellars::{aave_v2_stablecoin, cellar_v1, cellar_v2},
    error::{Error, ErrorKind},
    tenderly,
};

pub struct SimulateHandler;

#[async_trait]
impl steward::simulate_contract_call_server::SimulateContractCall for SimulateHandler {
    async fn simulate(
        &self,
        request: Request<SimulateRequest>,
    ) -> Result<Response<SimulateResponse>, Status> {
        let request = request.get_ref().to_owned();
        let inner_request = request.request.unwrap();
        let cellar_id = inner_request.cellar_id.clone();
        let encoded_call = match get_string_encoded_call(inner_request) {
            Ok(c) => c,
            Err(err) => {
                warn!("failed to get encoded call for {}: {}", cellar_id, err);
                return Err(Status::new(Code::Aborted, err.to_string()));
            }
        };
        let response = if request.encode_only {
            String::default()
        } else {
            info!("simulating call to {cellar_id} with tenderly");
            tenderly::simulate(cellar_id, encoded_call.clone()).await?
        };

        Ok(Response::new(SimulateResponse {
            encoded_call,
            response_body: response,
        }))
    }
}

pub fn get_string_encoded_call(request: ScheduleRequest) -> Result<String, Error> {
    if request.call_data.is_none() {
        return Err(ErrorKind::Http.context("empty contract call data").into());
    }

    match request.call_data.unwrap() {
        AaveV2Stablecoin(call) => {
            if call.function.is_none() {
                return Err(ErrorKind::Http.context("empty function data").into());
            }

            aave_v2_stablecoin::get_call(call.function.unwrap(), request.cellar_id)
                .map(|v| v.to_string())
        }
        CellarV1(call) => {
            if call.function.is_none() {
                return Err(ErrorKind::Http.context("empty function data").into());
            }

            cellar_v1::get_call(call.function.unwrap(), request.cellar_id).map(|v| v.to_string())
        }
        CellarV2(call) => {
            if call.function.is_none() {
                return Err(ErrorKind::Http.context("empty function data").into());
            }

            cellar_v2::get_call(call.function.unwrap(), request.cellar_id).map(|v| v.to_string())
        }
    }
}
