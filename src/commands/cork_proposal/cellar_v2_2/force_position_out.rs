use abscissa_core::{clap::Parser, Command, Runnable};

use crate::{
    application::APP,
    cellars,
    commands::cork_proposal::print_proposal,
    prelude::*,
    proto::{
        cellar_v2_2governance::{ForcePositionOut, Function},
        governance_call::Call,
        CellarV22governance, GovernanceCall,
    },
};

#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nCalls forcePositionOut() on the target V2.2 cellar contract at the specified block height.\nFor more information see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol"
)]
pub struct ForcePositionOutCmd {
    #[clap(short = 'p', long)]
    /// The ID of the position
    position_id: u32,

    #[clap(short, long)]
    /// The index of the position
    index: u32,

    /// Whether the position is a debt position
    #[clap(short, long)]
    debt: bool,

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

impl Runnable for ForcePositionOutCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            cellars::validate_cellar_id(&self.cellar_id)
                .await
                .unwrap_or_else(|err| {
                    status_err!("invalid cellar ID: {}", err);
                    std::process::exit(1);
                });

            let governance_call = GovernanceCall {
                call: Some(Call::CellarV22(CellarV22governance {
                    function: Some(Function::ForcePositionOut(ForcePositionOut {
                        position_id: self.position_id,
                        index: self.index,
                        in_debt_array: self.debt,
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
