use crate::{
    cellars::{self, aave_v2_stablecoin},
    config,
    error::{Error, ErrorKind},
    gas::CellarGas,
    prelude::APP,
    somm_send,
    utils::string_to_u256,
};
use abscissa_core::{
    tracing::log::{debug, error, warn},
    Application,
};
use deep_space::{Coin, Contact};
use ethers::prelude::builders::ContractCall;
use ethers::prelude::*;
use gravity_bridge::{
    gravity_proto::cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse,
    gravity_utils::ethereum::downcast_to_u64,
};
use somm_proto::cork::{query_client::QueryClient as CorkQueryClient, Cork, QueryCellarIDsRequest};
use std::{
    convert::{TryFrom, TryInto},
    sync::Arc,
    time::Duration,
};
use steward_abi::aave_v2_stablecoin::AaveV2StablecoinCellar;
use steward_proto::steward::aave_v2_stablecoin::Function;
use steward_proto::{
    self,
    steward::{
        self,
        submit_request::CallData::AaveV2Stablecoin,
        SubmitRequest, SubmitResponse,
    },
};
use tonic::{self, async_trait, Code, Request, Response, Status};
pub type EthSignerMiddleware = SignerMiddleware<Provider<Http>, LocalWallet>;
use ethers::abi::Detokenize;

const MESSAGE_TIMEOUT: Duration = Duration::from_secs(10);
const CHAIN_PREFIX: &str = "somm";

pub struct CorkHandler;

#[async_trait]
impl steward::contract_call_server::ContractCall for CorkHandler {
    async fn submit(
        &self,
        request: Request<SubmitRequest>,
    ) -> Result<Response<SubmitResponse>, Status> {
        debug!("received contract call request");
        let request = request.get_ref().to_owned();

        // Check if cellar is governance approved before building cork
        let config = APP.config();
        let mut client = match CorkQueryClient::connect(config.cosmos.grpc.clone()).await {
            Ok(c) => c,
            Err(err) => {
                error!("cork query client connection failed: {}", err);
                return Err(Status::new(
                    Code::Internal,
                    "failed to query chain to validate cellar id",
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
            debug!(
                "cellar ID {} not approved by governance",
                &request.cellar_id
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

pub struct DirectCorkHandler;

#[async_trait]
impl steward::contract_call_server::ContractCall for DirectCorkHandler {
    async fn submit(
        &self,
        request: Request<SubmitRequest>,
    ) -> Result<Response<SubmitResponse>, Status> {
        debug!("received contract call request");
        let request = request.get_ref();

        let contract_call_data = match request.call_data.clone() {
            Some(call) => call,
            None => {
                return Err(tonic::Status::invalid_argument(
                    "Error, can't find call data",
                ))
            }
        };

        let config = APP.config();

        let name = &config.keys.delegate_key;
        let wallet = config.load_ethers_wallet(name.to_string());

        let eth_host = config.ethereum.rpc.clone();
        let provider = Provider::<Http>::try_from(eth_host.clone())
            .unwrap_or_else(|_| panic!("could not get provider from eth_host {}", eth_host));
        let chain_id = provider
            .get_chainid()
            .await
            .expect("Could not retrieve chain ID");

        let chain_id =
            downcast_to_u64(chain_id).expect("Chain ID overflowed when downcasting to u64");
        let client = SignerMiddleware::new(provider, wallet.clone().with_chain_id(chain_id));

        let cellar_address = request.cellar_id.clone();

        if cellars::validate_cellar_id(&cellar_address).is_err() {
            return Err(tonic::Status::invalid_argument(
                "Error, can't validate Cellar ID",
            ));
        }

        let address: Address = cellar_address
            .parse()
            .expect("could not parse cellar address");

        match contract_call_data {
            AaveV2Stablecoin(call) => {
                if call.function.is_none() {
                    return Err(tonic::Status::invalid_argument(
                        "Error, can't validate Cellar ID",
                    ));
                } else {
                let contract = AaveV2StablecoinCellar::new(address, Arc::new(client));

                match call.function.unwrap() {
                    Function::EnterPosition(_) => contract_call(contract.enter_position()).await,
                    Function::SetDepositLimit(params) => {
                        contract_call(contract.set_deposit_limit(string_to_u256(params.limit).unwrap())).await
                    }
                    Function::Reinvest(params) => {
                        contract_call(contract.reinvest(string_to_u256(params.min_assets_out).unwrap())).await
                    }
                    Function::Rebalance(params) => {
                        contract_call(
                            contract.rebalance(
                                params
                                    .route
                                    .iter()
                                    .map(|addr| match addr.parse::<H160>() {
                                        Ok(addr) => Ok(addr),
                                        Err(_) => Err(addr),
                                    })
                                    .collect::<Vec<_>>()
                                    .iter()
                                    .map(|r| r.unwrap())
                                    .collect::<Vec<H160>>()
                                    .try_into()
                                    .expect("failed to convert 'route' addresses to array"),
                                params
                                    .swap_params
                                    .iter()
                                    .map(|sp| {
                                        let out: [U256; 3] = [
                                            sp.in_index.into(),
                                            sp.out_index.into(),
                                            sp.swap_type.into(),
                                        ];
                                        out
                                    })
                                    .collect::<Vec<[U256; 3]>>()
                                    .try_into()
                                    .expect("failed to convert 'swap_params' vec to array"),
                                    string_to_u256(params.min_assets_out).unwrap(),
                            ),
                        )
                        .await
                    }
                    Function::AccrueFees(_) => contract_call(contract.accrue_fees()).await,
                    Function::TransferFees(_) => contract_call(contract.transfer_fees()).await,
                    Function::SetLiquidityLimit(params) => {
                        contract_call(contract.set_liquidity_limit(string_to_u256(params.limit).unwrap())).await
                    }
                    Function::ClaimAndUnstake(_) => {
                        contract_call(contract.claim_and_unstake()).await
                    }
                }

            }
        }
        };



        Ok(Response::new(SubmitResponse {}))
    }
}

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

async fn contract_call<T: Detokenize>(
    contract_call: ContractCall<EthSignerMiddleware, T>,
) -> Result<(), Error> {
    let gas = contract_call.estimate_gas().await?;
    let gas_price = CellarGas::get_gas_price().await?;
    let contract_call = contract_call.gas(gas).gas_price(gas_price);
    let contract_call = contract_call.send().await?;
    let _tx_hash = *contract_call;
    Ok(())
}
