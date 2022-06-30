use crate::{application::APP, cellars, cork, prelude::*};
use abscissa_core::{clap::Parser, Command, Runnable};
use ethers::abi::AbiEncode;
use steward_abi::aave_v2_stablecoin::*;

/// Shutdown subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "\nSchedules a call to the target cellar's initiateShutdown() function, specifying whether to remove assets from their Aave lending position in the process."
)]
pub struct InitiateShutdownCmd {
    /// Target contract for scheduled cork.
    #[clap(short, long)]
    contract: String,

    /// Block height to schedule cork.
    #[clap(short = 'b', long)]
    height: u64,

    /// Set to true if you want to exit current position.
    #[clap(short = 'e', long)]
    empty_position: bool,
}

impl Runnable for InitiateShutdownCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            let call = InitiateShutdownCall {
                empty_position: self.empty_position,
            };
            let encoded_call = AaveV2StablecoinCellarCalls::InitiateShutdown(call).encode();

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
