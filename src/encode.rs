use abscissa_core::tracing::warn;
use ethers::abi::Address;
use hex::ToHex;
use tonic::{async_trait, Code, Request, Response, Status};

use crate::{
    cellars::*,
    error::ErrorKind,
    proto::{self, encode_request::CallData::*, EncodeRequest, EncodeResponse},
};

pub struct EncodingHandler;

#[async_trait]
impl proto::encoding_service_server::EncodingService for EncodingHandler {
    async fn encode(
        &self,
        request: Request<EncodeRequest>,
    ) -> Result<Response<EncodeResponse>, Status> {
        let request = request.get_ref().to_owned();
        let _ = match &request.cellar_id.parse::<Address>() {
            Ok(a) => a,
            Err(_) => {
                return Err(Status::new(
                    Code::InvalidArgument,
                    format!("invalid EVM address format {:?}", request.cellar_id),
                ));
            }
        };
        let encoded_call = match get_encoded_call(request) {
            Ok(c) => c,
            Err(err) => {
                warn!("failed to get encoded call with error {:?}", err);
                return Err(Status::new(Code::InvalidArgument, err.to_string()));
            }
        };

        Ok(Response::new(EncodeResponse { encoded_call }))
    }
}

fn get_encoded_call(request: EncodeRequest) -> Result<String, Box<dyn std::error::Error>> {
    if request.call_data.is_none() {
        return Err(ErrorKind::Http.context("empty call data").into());
    }

    let call_data = match request.call_data.unwrap() {
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
        CellarV25(call) => {
            if call.call_type.is_none() {
                return Err(ErrorKind::Http.context("empty function data").into());
            }

            cellar_v2_5::get_encoded_call(call.call_type.unwrap(), request.cellar_id)
        }
    }?;

    let encoded_call: String = ethers::types::Bytes::from(call_data).encode_hex();

    Ok(encoded_call)
}
