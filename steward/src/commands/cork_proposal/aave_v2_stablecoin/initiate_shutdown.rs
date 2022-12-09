use crate::{application::APP, cellars, commands::cork_proposal::print_proposal, prelude::*};
use abscissa_core::{clap::Parser, Command, Runnable};
use steward_proto::steward::{
    aave_v2_stablecoin_governance::{Function, InitiateShutdown},
    governance_call::Call,
    AaveV2StablecoinGovernance, GovernanceCall,
};

/// Shutdown subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nInitiate shutdown of target cellar when chain reaches specified height"
)]
pub struct InitiateShutdownCmd {
    /// Set to true if you want to exit current position.
    #[clap(short, long)]
    empty_position: bool,

    /// Target contract for scheduled cork.
    #[clap(short, long)]
    cellar_id: String,

    /// Block height to schedule cork.
    #[clap(short, long)]
    block_height: u64,

    /// Only print JSON output, omitting explanatory text
    #[clap(short, long)]
    quiet: bool,
}

impl Runnable for InitiateShutdownCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            cellars::validate_cellar_id(self.cellar_id.as_str())
                .await
                .unwrap_or_else(|err| {
                    status_err!("Can't validate contract address format: {}", err);
                    std::process::exit(1);
                });

            let governance_call = GovernanceCall {
                call: Some(Call::AaveV2Stablecoin(AaveV2StablecoinGovernance {
                    function: Some(Function::InitiateShutdown(InitiateShutdown {
                        empty_position: self.empty_position,
                    })),
                })),
            };

            print_proposal(
                self.block_height,
                self.cellar_id.clone(),
                governance_call,
                self.quiet,
            )
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
