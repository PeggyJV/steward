use crate::{
    application::APP,
    cellars,
    commands::cork_proposal::{get_proposal_json, print_proposal},
    prelude::*,
    proto::{
        cellar_v1_governance::{Function, SetFeesDistributor},
        governance_call::Call,
        CellarV1Governance, GovernanceCall,
    },
};
use abscissa_core::{clap::Parser, Command, Runnable};
use deep_space::Address as CosmosAddress;

/// Fees Distributor subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nCalls setFeesDistributor() on the target cellar contract at the specified block height.\nFor more information see https://github.com/PeggyJV/cellar-v1_5/blob/release/src/base/Cellar.sol"
)]
pub struct SetFeesDistributorCmd {
    #[clap(short, long)]
    /// Fee distributor's address
    new_fees_distributor: CosmosAddress,

    /// Target contract for scheduled cork.
    #[clap(short, long)]
    cellar_id: String,

    #[clap(short, long)]
    chain_id: u64,

    /// Block height to schedule cork.
    #[clap(short, long)]
    block_height: u64,

    /// Only print JSON output, omitting explanatory text
    #[clap(short, long)]
    quiet: bool,
}

impl Runnable for SetFeesDistributorCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            cellars::validate_cellar_id(self.chain_id, &self.cellar_id)
                .await
                .unwrap_or_else(|err| {
                    status_err!("invalid cellar ID: {}", err);
                    std::process::exit(1);
                });

            if self.new_fees_distributor.get_prefix().ne("somm") {
                status_err!("new fees distributor address must have a 'somm' prefix!");
                std::process::exit(1)
            }

            let governance_call = GovernanceCall {
                call: Some(Call::CellarV1(CellarV1Governance {
                    function: Some(Function::SetFeesDistributor(SetFeesDistributor {
                        new_fees_distributor: self.new_fees_distributor.to_string(),
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
