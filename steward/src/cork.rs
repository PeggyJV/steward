use crate::{
    cellars::{self, uniswapv3, CellarId},
    error::{Error, ErrorKind},
    prelude::APP,
    somm_send,
};
use abscissa_core::{
    tracing::log::{debug, error},
    Application,
};
use deep_space::{Coin, Contact};
use gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse;
use somm_proto::cork::Cork;
use std::time::Duration;
use steward_proto::steward::{
    self,
    submit_request::ContractCallData::{self, *},
    SubmitRequest, SubmitResponse,
};
use tonic::async_trait;

pub struct CorkHandler;

#[async_trait]
impl steward::contract_call_server::ContractCall for CorkHandler {
    async fn submit(
        &self,
        request: tonic::Request<SubmitRequest>,
    ) -> Result<tonic::Response<SubmitResponse>, tonic::Status> {
        debug!("received contract call request");
        tokio::spawn(async move {
            let request = request.get_ref();
            let cork = match build_cork(request).await {
                Ok(c) => c,
                Err(err) => {
                    error!("{}", err);
                    return;
                }
            };
            debug!("cork: {:?}", cork);

            if let Err(err) = send_cork(cork).await {
                error!("{}", err);
            }

            debug!("sent cork!");
        });
        Ok(tonic::Response::new(SubmitResponse {}))
    }
}

async fn build_cork(request: &SubmitRequest) -> Result<Cork, Error> {
    let cellar_id = cellars::parse_cellar_id(request.cellar_id.as_str())?;
    let address = cellar_id.address.clone();
    let contract_call_data = match request.contract_call_data.clone() {
        Some(call) => call,
        None => return Err(ErrorKind::Http.context("empty contract call data").into()),
    };
    let encoded_call = get_encoded_call(&cellar_id, contract_call_data)?;

    Ok(Cork {
        address: address,
        body: encoded_call,
    })
}

fn get_encoded_call(cellar_id: &CellarId, data: ContractCallData) -> Result<Vec<u8>, Error> {
    let config = APP.config();
    match data {
        Uniswapv3Rebalance(params) => {
            let info = config.cork.uniswapv3.clone();
            if info.cellar_ids.contains(&cellar_id.to_string()) {
                return Ok(uniswapv3::get_encoded_call(params));
            } else {
                return Err("cellar not found in config".to_string().into());
            }
        }
    }
}

async fn send_cork(cork: Cork) -> Result<TxResponse, Error> {
    let config = APP.config();
    debug!("establishing grpc connection");
    let timeout = Duration::from_secs(10);
    let contact = Contact::new(&config.cosmos.grpc, timeout, &config.cosmos.prefix)?;

    debug!("loading the delegate (orchestrator) key and address from config");
    let name = &config.keys.rebalancer_key;
    let delegate_key = config.load_deep_space_key(name.clone());
    let delegate_address = delegate_key.to_address(&config.cosmos.prefix)?;

    debug!("getting cosmos fee");
    let cosmos_gas_price = config.cosmos.gas_price.as_tuple();
    let fee = Coin {
        amount: (cosmos_gas_price.0 as u64).into(),
        denom: cosmos_gas_price.1,
    };
    somm_send::send_cork(
        &contact,
        cork,
        delegate_address.to_string(),
        delegate_key,
        fee,
    )
    .await
    .map_err(|e| e.into())
}
