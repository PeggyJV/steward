use abscissa_core::{clap::Parser, Command, Runnable};

use crate::{
    application::APP,
    cellars,
    commands::cork_proposal::print_proposal,
    prelude::*,
    proto::{
        cellar_v2_2governance::{Function, SetShareLockPeriod},
        governance_call::Call,
        CellarV22governance, GovernanceCall,
    },
    utils::string_to_u256,
};

#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nCalls setShareLockPeriod() on the target V2.2 cellar contract at the specified block height.\nFor more information see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol"
)]
pub struct SetShareLockPeriodCmd {
    #[clap(short = 'p', long)]
    /// New lock period
    period: String,

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

impl Runnable for SetShareLockPeriodCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            cellars::validate_cellar_id(&self.cellar_id)
                .await
                .unwrap_or_else(|err| {
                    status_err!("invalid cellar ID: {}", err);
                    std::process::exit(1);
                });

            if let Err(err) = string_to_u256(self.period.clone()) {
                status_err!("invalid period: {}", err);
                std::process::exit(1);
            }

            let governance_call = GovernanceCall {
                call: Some(Call::CellarV22(CellarV22governance {
                    function: Some(Function::SetShareLockPeriod(SetShareLockPeriod {
                        new_lock: self.period.clone(),
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
