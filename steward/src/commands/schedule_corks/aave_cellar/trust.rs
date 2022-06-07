use crate::{application::APP, prelude::*, utils::SubmitCork};
use abscissa_core::{clap::Parser, Command, Runnable};
use ethers::abi::AbiEncode;
use ethers::types::*;
use steward_abi::aave_v2_stablecoin::*;

/// Trust subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Set trust for target Cellar.\n This command prevents Cellar from rebalancing into an asset that has not been trusted by the users. This is a validator only command and can only be run by validators. It sets trust based on the height specified by the validators. Therefore, it'll execute the function when the chain reaches that height. This command also takes the trust and position flag."
)]

pub struct TrustCmd {
    /// Asset position
    #[clap(short = 'p', long)]
    position: H160,

    #[clap(short = 't', long)]
    trust: bool,

    /// Block height to schedule cork.
    #[clap(short = 'b', long)]
    height: u64,

    /// Target contract for scheduled cork.
    #[clap(short = 'c', long)]
    contract: String,
}

impl Runnable for TrustCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            // Encoded call for trust
            let call = SetTrustCall {
                position: self.position,
                trust: self.trust,
            };

            let encoded_call = AaveV2StablecoinCellarCalls::SetTrust(call).encode();

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
