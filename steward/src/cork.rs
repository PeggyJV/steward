use crate::{
    cellars::{self, aave_v2_stablecoin},
    config,
    cork::abi::ParamType::String as abi_type,
    error::{Error, ErrorKind},
    prelude::APP,
    somm_send,
};
use abscissa_core::{
    tracing::log::{debug, error, warn},
    Application,
};
use deep_space::{Coin, Contact};
use ethers::prelude::*;
use gravity_bridge::{
    gravity_proto::cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse,
    gravity_utils::ethereum::downcast_to_u64,
};
use signatory::FsKeyStore;
use somm_proto::cork::{query_client::QueryClient as CorkQueryClient, Cork, QueryCellarIDsRequest};
use std::{convert::TryFrom, path, time::Duration};
use steward_proto::{
    self,
    steward::{
        self,
        submit_request::CallData::{self, AaveV2Stablecoin},
        SubmitRequest, SubmitResponse,
    },
};
use tonic::{self, async_trait, Code, Request, Response, Status};

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
        let config = APP.config();

        let keystore = path::Path::new(&config.keystore);
        let keystore = FsKeyStore::create_or_open(keystore).expect("Could not open keystore");

        let name = &config
            .keys
            .rebalancer_key
            .parse()
            .expect("Could not parse name");
        let key = keystore.load(name).expect("Could not load key");

        let key = key
            .to_pem()
            .parse::<k256::elliptic_curve::SecretKey<k256::Secp256k1>>()
            .expect("Could not parse key");

        let wallet: LocalWallet = Wallet::from(key);

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

        if let Err(err) = cellars::validate_cellar_id(&cellar_address) {
            return Err(tonic::Status::invalid_argument(err));
        }

        let address: H160 = cellar_address
            .parse()
            .expect("could not parse cellar address");

        let contract_call_data = match request.call_data.clone() {
            Some(call) => call,
            None => return Err(tonic::Status::invalid_argument("err")),
        };
        let encoded_call = get_encoded_call(contract_call_data).unwrap();

        // (ugly way to write the ABI inline, you can otherwise read it from a file)
        // let abi: abi::Abi = serde_json::from_str(r#"[{"inputs":[{"internalType":"string","name":"value","type":"string"}],"stateMutability":"nonpayable","type":"constructor"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"author","type":"address"},{"indexed":true,"internalType":"address","name":"oldAuthor","type":"address"},{"indexed":false,"internalType":"string","name":"oldValue","type":"string"},{"indexed":false,"internalType":"string","name":"newValue","type":"string"}],"name":"ValueChanged","type":"event"},{"inputs":[],"name":"getValue","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"lastSender","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"string","name":"value","type":"string"}],"name":"setValue","outputs":[],"stateMutability":"nonpayable","type":"function"}]"#).unwrap();

        let abi = abi::decode(&[abi_type], &encoded_call).unwrap();

        for abi in abi {
            let abi = abi.to_string();

            let abi: abi::Abi = serde_json::from_str(&abi).unwrap();

            // create the contract object at the address
            let contract = Contract::new(address, abi, &client);

            // Calling constant methods is done by calling `call()` on the method builder.
            // (if the function takes no arguments, then you must use `()` as the argument)
            contract
                .method::<_, String>("getValue", ())
                .unwrap()
                .call()
                .await
                .unwrap();
        }

        Ok(Response::new(SubmitResponse {}))
    }
}

async fn build_cork(request: &SubmitRequest) -> Result<Cork, Error> {
    cellars::validate_cellar_id(request.cellar_id.as_str())?;
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
