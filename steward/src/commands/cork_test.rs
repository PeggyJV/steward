use crate::{application::APP, config::StewardConfig, cork::CorkHandler, prelude::*, server, somm_send};
use crate::config::DELEGATE_ADDRESS;
use crate::config::DELEGATE_KEY;
use abscissa_core::{clap::Parser, config, Command, FrameworkError, Runnable};
use deep_space::{Contact, Coin};
use ethers::abi::AbiEncode;
use somm_proto::cork::Cork;
use steward_abi::aave_v2_stablecoin::{ShutdownCall, AaveV2StablecoinCellarCalls};
use std::{result::Result, time::Duration};
use steward_proto::steward::contract_call_server::ContractCallServer;

/// Cosmos Signer, start allocation module
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Cork test."
)]
pub struct CorkTest {
    contract: String,
    height: u64,
}

impl Runnable for CorkTest {
    fn run(&self) {
        let config = APP.config();

        abscissa_tokio::run_with_actix(&APP, async {
            let call = ShutdownCall {};
            let encoded_call = AaveV2StablecoinCellarCalls::Shutdown(call).encode();

            let cork = Cork {
                encoded_contract_call: encoded_call,
                target_contract_address: self.contract.clone(),
            };

            debug!("establishing grpc connection");
            let contact = Contact::new(&config.cosmos.grpc, Duration::from_secs(10), "somm").unwrap();

            debug!("getting cosmos fee");
            let cosmos_gas_price = config.cosmos.gas_price.as_tuple();
            let fee = Coin {
                amount: (cosmos_gas_price.0 as u64).into(),
                denom: cosmos_gas_price.1,
            };

            somm_send::schedule_cork(
                &contact,
                cork,
                DELEGATE_ADDRESS.to_string(),
                &DELEGATE_KEY,
                fee,
                self.height,
            )
            .await
            .expect("err");
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}