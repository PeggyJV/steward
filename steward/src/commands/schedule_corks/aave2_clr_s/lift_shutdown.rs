use crate::{application::APP, cellars, cork, prelude::*};
use abscissa_core::{clap::Parser, Command, Runnable};
use ethers::abi::AbiEncode;
use steward_abi::aave_v2_stablecoin::*;

/// Shutdown subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Lift shutdown of target cellar when chain reaches specified height"
)]
pub struct LiftShutdownCmd {
    /// Target contract for scheduled cork.
    #[clap(short, long)]
    contract: String,

    /// Block height to schedule cork.
    #[clap(short = 'b', long)]
    height: u64,
}

impl Runnable for LiftShutdownCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            let call = LiftShutdownCall {};
            let encoded_call = AaveV2StablecoinCellarCalls::LiftShutdown(call).encode();

            cellars::validate_cellar_id(self.contract.as_str()).unwrap_or_else(|err| {
                status_err!("Can't validate contract address format: {}", err);
                std::process::exit(1);
            });

            cork::schedule_cork(self.contract.clone(), encoded_call, self.height)
                .await
                .unwrap_or_else(|err| {
                    status_err!("executor exited with error: {}", err);
                    std::process::exit(1);
                })
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
