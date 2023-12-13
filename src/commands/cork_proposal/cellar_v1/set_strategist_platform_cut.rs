use crate::{
    application::APP,
    cellars,
    commands::cork_proposal::print_proposal,
    prelude::*,
    proto::{
        cellar_v1_governance::{Function, SetStrategistPlatformCut},
        governance_call::Call,
        CellarV1Governance, GovernanceCall,
    },
};
use abscissa_core::{clap::Parser, Command, Runnable};

/// Fees Distributor subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nCalls setStrategistPlatformCut() on the target cellar contract at the specified block height.\nFor more information see https://github.com/PeggyJV/cellar-v1_5/blob/release/src/base/Cellar.sol"
)]
pub struct SetStrategistPlatformCutCmd {
    #[clap(short, long)]
    /// New platform cut proportion for the Strategy Provider between 0 and 1e18 representing 0% and 100% respectively.
    new_platform_cut: u64,

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

impl Runnable for SetStrategistPlatformCutCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            cellars::validate_cellar_id(&self.cellar_id)
                .await
                .unwrap_or_else(|err| {
                    status_err!("invalid cellar ID: {}", err);
                    std::process::exit(1);
                });

            let governance_call = GovernanceCall {
                call: Some(Call::CellarV1(CellarV1Governance {
                    function: Some(Function::SetStrategistPlatformCut(
                        SetStrategistPlatformCut {
                            amount: self.new_platform_cut,
                        },
                    )),
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
