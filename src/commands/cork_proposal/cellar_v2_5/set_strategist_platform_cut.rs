use crate::{
    application::APP,
    cellars,
    commands::cork_proposal::{get_proposal_json, print_proposal},
    prelude::*,
    proto::{
        cellar_v2_5governance::SetStrategistPlatformCut, governance_call::Call,
        CellarV25governance, GovernanceCall,
    },
};
use abscissa_core::{clap::Parser, Command, Runnable};
use steward_proto::proto::cellar_v2_5governance::{
    function_call::Function, CallType, FunctionCall,
};

/// Fees Distributor subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nCalls setStrategistPlatformCut() on the target V2.5 cellar contract at the specified block height.\nFor more information see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol"
)]
pub struct SetStrategistPlatformCutCmd {
    #[clap(short, long)]
    /// New platform cut proportion for the Strategy Provider between 0 and 1e18 representing 0% and 100% respectively.
    cut: u64,

    /// Target contract for scheduled cork.
    #[clap(short, long)]
    cellar_id: String,

    #[clap(long)]
    chain_id: u64,

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
            cellars::validate_cellar_id(self.chain_id, &self.cellar_id)
                .await
                .unwrap_or_else(|err| {
                    status_err!("invalid cellar ID: {}", err);
                    std::process::exit(1);
                });

            let governance_call = GovernanceCall {
                call: Some(Call::CellarV25(CellarV25governance {
                    call_type: Some(CallType::FunctionCall(FunctionCall {
                        function: Some(Function::SetStrategistPlatformCut(
                            SetStrategistPlatformCut { new_cut: self.cut },
                        )),
                    })),
                })),
            };

            let proposal_json = get_proposal_json(
                self.block_height,
                self.cellar_id.clone(),
                governance_call,
                self.chain_id,
            );

            print_proposal(proposal_json, self.quiet)
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
