use crate::{application::APP, prelude::*, cellars, cork};
use abscissa_core::{clap::Parser, Command, Runnable};
use ethers::abi::AbiEncode;
use ethers::types::*;
use steward_abi::aave_v2_stablecoin::*;

/// Sweep subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Sweep token from target Cellar.\n This command sweep tokens sent here that are not managed by the Cellar. This is useful in case the wrong tokens are accidentally sent to this contract. This is a validator only command and can only be run by validators. It Schedules sweep based on the height specified by the validators. Therefore, it'll execute the function when the chain reaches that height. This command also takes the token address and the address to sweep token into."
)]

pub struct SweepCmd {
    /// Token address to transfer out of Cellar.
    #[clap(short = 't', long)]
    token: H160,

    /// Address to sweep token into
    #[clap(short = 'd', long)]
    destination_address: H160,

    /// Target contract for scheduled cork.
    #[clap(short = 'c', long)]
    contract: String,

    /// Block height to schedule cork.
    #[clap(short = 'b', long)]
    height: u64,
}

impl Runnable for SweepCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            // Encoded call for sweep
            let call = SweepCall {
                token: self.token,
                to: self.destination_address,
            };

            let encoded_call = AaveV2StablecoinCellarCalls::Sweep(call).encode();

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
