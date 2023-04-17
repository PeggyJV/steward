use crate::{application::APP, cellars, commands::cork_proposal::print_proposal, prelude::*};
use abscissa_core::{clap::Parser, Command, Runnable};
use steward_proto::steward::{
    cellar_v1_governance::{Function, SetPerformanceFee},
    governance_call::Call,
    CellarV1Governance, GovernanceCall,
};

/// Fees Distributor subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nCalls setPerformanceFee() on the target cellar contract at the specified block height.\nFor more information see https://github.com/PeggyJV/cellar-v1_5/blob/release/src/base/Cellar.sol"
)]
pub struct SetPerformanceFeeCmd {
    #[clap(short = 'n', long)]
    /// New performance fee proportion between 0 and 1e18 representing 0% and 100% respectively.
    new_performance_fee: u64,

    /// Target contract for scheduled cork.
    #[clap(short, long)]
    cellar_id: String,

    /// ID of the EVM chain where the cellar is deployed.
    #[clap(long)]
    chain_id: u64,

    /// Block height to schedule cork.
    #[clap(short, long)]
    block_height: u64,

    /// Only print JSON output, omitting explanatory text
    #[clap(short, long)]
    quiet: bool,
}

impl Runnable for SetPerformanceFeeCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            cellars::validate_cellar_id(&self.cellar_id, self.chain_id)
                .await
                .unwrap_or_else(|err| {
                    status_err!("invalid cellar ID: {}", err);
                    std::process::exit(1);
                });

            let governance_call = GovernanceCall {
                call: Some(Call::CellarV1(CellarV1Governance {
                    function: Some(Function::SetPerformanceFee(SetPerformanceFee {
                        amount: self.new_performance_fee,
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
