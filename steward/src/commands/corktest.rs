//! `Cosmos mode signer` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::{application::APP, config::DELEGATE_ADDRESS, config::DELEGATE_KEY, prelude::*, somm_send};
use abscissa_core::{clap::Parser, Command, Runnable};
use deep_space::{Contact, Coin};
use ethers::abi::AbiEncode;
use somm_proto::cork::Cork;
use steward_abi::aave_v2_stablecoin::{AaveV2StablecoinCellarCalls, EnterPositionCall};
use std::time::Duration;

/// Cosmos Signer, start allocation module
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Cork test."
)]
pub struct CorkTestCmd;

impl Runnable for CorkTestCmd {
    /// Start the application.
    fn run(&self) {
        let config = APP.config();
        abscissa_tokio::run_with_actix(&APP, async {
            let contact = Contact::new(&config.cosmos.grpc, Duration::from_secs(10), "somm").unwrap();

            info!("getting cosmos fee");
            let cosmos_gas_price = config.cosmos.gas_price.as_tuple();
            let fee = Coin {
                amount: (cosmos_gas_price.0 as u64).into(),
                denom: cosmos_gas_price.1,
            };

            let cork = Cork {
                target_contract_address: "0x7bAD5DF5E11151Dc5Ee1a648800057C5c934c0d5".to_string(),
                encoded_contract_call: AaveV2StablecoinCellarCalls::EnterPosition(EnterPositionCall {}).encode(),
            };

            let res = somm_send::send_cork(
                &contact,
                cork,
                DELEGATE_ADDRESS.to_string(),
                &DELEGATE_KEY,
                fee,
            ).await;

            println!("res: {:?}", res);
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}