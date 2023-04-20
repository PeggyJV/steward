use abscissa_core::tracing::log::{info, warn};
use steward_proto::steward::{self, ScheduleRequest, SimulateRequest, SimulateResponse};
use tonic::{async_trait, Code, Request, Response, Status};

use crate::{cork::get_encoded_call, error::Error, tenderly, utils::bytes_to_hex_str};

pub struct SimulateHandler;

#[async_trait]
impl steward::simulate_contract_call_server::SimulateContractCall for SimulateHandler {
    async fn simulate(
        &self,
        request: Request<SimulateRequest>,
    ) -> Result<Response<SimulateResponse>, Status> {
        let request = request.get_ref().to_owned();
        let inner_request = match request.request {
            Some(r) => r,
            None => {
                return Err(Status::new(
                    Code::InvalidArgument,
                    "empty request".to_string(),
                ))
            }
        };
        let cellar_id = inner_request.cellar_id.clone();
        let encoded_call = match get_string_encoded_call(inner_request) {
            Ok(c) => c,
            Err(err) => {
                warn!("failed to get encoded call for {}: {}", cellar_id, err);
                return Err(Status::new(Code::Aborted, err.to_string()));
            }
        };
        info!("encoded call: {encoded_call}");
        let response_body = if request.encode_only {
            String::default()
        } else {
            info!("simulating call to {cellar_id} with tenderly");
            let response = tenderly::simulate(cellar_id, encoded_call.clone()).await?;
            info!("response object: {response:?}");
            response.text().await.map_err(|e| {
                Status::new(
                    Code::Internal,
                    format!("failed to deserialize response body: {e}"),
                )
            })?
        };

        Ok(Response::new(SimulateResponse {
            encoded_call,
            response_body,
        }))
    }
}

pub fn get_string_encoded_call(request: ScheduleRequest) -> Result<String, Error> {
    get_encoded_call(request).map(|b| bytes_to_hex_str(&b))
}
