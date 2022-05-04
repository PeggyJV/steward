use crate::{
    application::APP,
    cellars::{self},
    config,
    prelude::*,
    somm_send,
};
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use deep_space::{Coin, Contact};
use ethers::abi::AbiEncode;
use somm_proto::cork::Cork;
use std::time::Duration;
use steward_abi::aave_v2_stablecoin::*;
const MESSAGE_TIMEOUT: Duration = Duration::from_secs(10);
const CHAIN_PREFIX: &str = "somm";

/// Shutdown subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Shutdown target Cellar.\n This command schedules the shutdown of a Cellar. This is a validator only \n command and can only be run by validators. It Schedules shutdown based on the height specified by \n the validators. Therefore, it'll execute the function when the chain reaches that height."
)]
pub struct ShutdownCmd {
    /// Target contract for scheduled cork.
    #[clap(short, long)]
    contract: String,

    /// Block height to schedule cork.
    #[clap(short = 'b', long)]
    height: u64,
}

impl Runnable for ShutdownCmd {
    fn run(&self) {
        let config = APP.config();

        abscissa_tokio::run_with_actix(&APP, async {
            let call = IsShutdownCall {};
            let encoded_call = AaveV2StablecoinCellarCalls::IsShutdown(call).encode();

            cellars::validate_cellar_id(self.contract.as_str()).unwrap();

            let cork = Cork {
                encoded_contract_call: encoded_call,
                target_contract_address: self.contract.clone(),
            };

            debug!("establishing grpc connection");
            let contact = Contact::new(&config.cosmos.grpc, MESSAGE_TIMEOUT, CHAIN_PREFIX).unwrap();

            debug!("getting cosmos fee");
            let cosmos_gas_price = config.cosmos.gas_price.as_tuple();
            let fee = Coin {
                amount: (cosmos_gas_price.0 as u64).into(),
                denom: cosmos_gas_price.1,
            };
            somm_send::schedule_cork(
                &contact,
                cork,
                config::DELEGATE_ADDRESS.to_string(),
                &config::DELEGATE_KEY,
                fee,
                self.height,
            )
            .await
            .expect("Error, can't schedule shutdown");
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
