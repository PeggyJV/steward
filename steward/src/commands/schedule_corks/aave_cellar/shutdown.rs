use crate::{application::APP, prelude::*, utils::SubmitCork};
use abscissa_core::{clap::Parser, Command, Runnable};
use ethers::abi::AbiEncode;
use steward_abi::aave_v2_stablecoin::*;

/// Shutdown subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Shutdown target Cellar.\n This command schedules the shutdown of a Cellar. This is a validator only \n command and can only be run by validators. It Schedules shutdown based on the height specified by \n the validators. Therefore, it'll execute the function when the chain reaches that height. This command also takes the shutdown and exit position flag."
)]
pub struct ShutdownCmd {
    /// Target contract for scheduled cork.
    #[clap(short, long)]
    contract: String,

    /// Block height to schedule cork.
    #[clap(short = 'b', long)]
    height: u64,

    ///Set to true if you want to shutdown Aave Cellar.
    #[clap(short = 's', long)]
    shut_down: bool,

    /// Set to true if you want to exit current position.
    #[clap(short = 'e', long)]
    exit_position: bool,
}

impl Runnable for ShutdownCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            let call = SetShutdownCall {
                shutdown: self.shut_down,
                exit_position: self.exit_position,
            };
            let encoded_call = AaveV2StablecoinCellarCalls::SetShutdown(call).encode();

            let submit = SubmitCork {
                contract: self.contract.clone(),
                height: self.height,
                encoded_call,
            };

            submit.submit_cork().await.unwrap_or_else(|err| {
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
