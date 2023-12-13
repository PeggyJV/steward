use abscissa_core::tracing::log::{debug, error, info, warn};
use ethers::types::H160;
use gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse;
use lazy_static::lazy_static;
use sha3::{Digest, Keccak256};
use somm_proto::cork::Cork;
use tonic::{self, async_trait, Code, Request, Response, Status};

use crate::server::handle_authorization;
use crate::{
    cellars::{self, aave_v2_stablecoin, cellar_v1, cellar_v2, cellar_v2_2},
    error::{
        Error,
        ErrorKind::{self, *},
    },
    proto::{
        self, schedule_request::CallData::*, ScheduleRequest, ScheduleResponse,
    },
    somm_send,
};

pub mod cache;
pub mod client;
pub mod proposals;

lazy_static! {
    static ref STEWARD_VERSION: &'static str = env!("CARGO_PKG_VERSION");
}

pub struct CorkHandler;

#[async_trait]
impl proto::contract_call_service_server::ContractCallService for CorkHandler {
    async fn schedule(
        &self,
        request: Request<ScheduleRequest>,
    ) -> Result<Response<ScheduleResponse>, Status> {
        let certs = request.peer_certs().unwrap();
        let request = request.get_ref().to_owned();

        let cellar_id = request.cellar_id.clone();
        if let Err(err) = cellars::validate_cellar_id(&cellar_id).await {
            let message = format!("cellar ID validation failed: {}", err);
            match err.kind() {
                CacheError => return Err(Status::new(Code::Internal, message)),
                SPCallError => return Err(Status::new(Code::InvalidArgument, message)),
                UnapprovedCellar => return Err(Status::new(Code::InvalidArgument, message)),
                _ => return Err(Status::new(Code::Unknown, message)),
            }
        }

        debug!("checking publisher authorization for {cellar_id}");
        handle_authorization(&cellar_id, certs).await?;

        // Build and send cork
        let height = request.block_height;
        let encoded_call = match get_encoded_call(request) {
            Ok(c) => c,
            Err(err) => {
                warn!("failed to get encoded call for {}: {}", cellar_id, err);
                return Err(Status::new(Code::InvalidArgument, err.to_string()));
            }
        };

        debug!("hex encoded call: {:?}", hex::encode(&encoded_call));

        match schedule_cork(&cellar_id, encoded_call.clone(), height).await {
            Ok(res) => {
                if res.code != 0 {
                    error!(
                        "failed to schedule cork for cellar {}. code {}, raw log: {}",
                        cellar_id, res.code, res.raw_log
                    );
                    return Err(Status::new(
                        Code::Internal,
                        "cork submission failed. this may be a steward configuration problem."
                            .to_string(),
                    ));
                }
                info!("cork response: {:?}", res)
            }
            Err(err) => {
                error!("failed to schedule cork for cellar {}: {}", cellar_id, err);
                return Err(Status::new(
                    Code::Internal,
                    format!("failed to send cork to sommelier: {}", err),
                ));
            }
        }
        let id = id_hash(height, &cellar_id, encoded_call);
        info!(
            "scheduled cork {} for cellar {} at height {}",
            id, cellar_id, height
        );

        Ok(Response::new(ScheduleResponse { id }))
    }
}

pub fn get_encoded_call(request: ScheduleRequest) -> Result<Vec<u8>, Error> {
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
        CellarV2(call) => {
            if call.function.is_none() {
                return Err(ErrorKind::Http.context("empty function data").into());
            }

            cellar_v2::get_encoded_call(call.function.unwrap(), request.cellar_id)
        }
        CellarV22(call) => {
            if call.call_type.is_none() {
                return Err(ErrorKind::Http.context("empty function data").into());
            }

            cellar_v2_2::get_encoded_call(call.call_type.unwrap(), request.cellar_id)
        }
    }
}

pub async fn schedule_cork(
    contract: &str,
    encoded_call: Vec<u8>,
    height: u64,
) -> Result<TxResponse, Error> {
    debug!("establishing grpc connection");
    let cork = Cork {
        encoded_contract_call: encoded_call,
        target_contract_address: contract.to_string(),
    };

    somm_send::schedule_cork(cork, height)
        .await
        .map_err(|e| e.into())
}

pub fn id_hash(block_height: u64, contract_address: &str, encoded_call: Vec<u8>) -> String {
    let mut hasher = Keccak256::new();
    let address = contract_address
        .parse::<H160>()
        .expect("failed to parse cellar ID. it should have been validated before now.");
    let input = [
        (block_height).to_be_bytes().as_slice(),
        address.as_bytes(),
        &encoded_call,
    ]
    .concat();
    hasher.update(input);

    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_hash() {
        // inputs and expected output from an integration test run
        let expected =
            "fca62587f984a777a25ff1a45c2178066c82f8631f9bf54046943604c8805c84".to_string();
        let call = hex::decode("fc4d43be00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let actual = id_hash(193, "0x0165878A594ca255338adfa4d48449f69242Eb8F", call);

        assert_eq!(expected, actual);
    }
}
