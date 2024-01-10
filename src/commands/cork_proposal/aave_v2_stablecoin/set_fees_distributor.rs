use crate::{
    application::APP,
    cellars,
    commands::cork_proposal::{get_proposal_json, print_proposal},
    prelude::*,
    proto::{
        aave_v2_stablecoin_governance::{Function, SetFeesDistributor},
        governance_call::Call,
        AaveV2StablecoinGovernance, GovernanceCall,
    },
};
use abscissa_core::{clap::Parser, Command, Runnable};
use deep_space::Address as CosmosAddress;

/// Fees Distributor subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nSet fees distributor Cosmos address of target cellar when chain reaches specified height.\nFor more information see https://github.com/PeggyJV/aave-v2-cellar/blob/main/src/AaveV2StablecoinCellar.sol."
)]
pub struct FeesDistributorCmd {
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

impl Runnable for FeesDistributorCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            cellars::validate_cellar_id(self.chain_id, &self.cellar_id)
                .await
                .unwrap_or_else(|err| {
                    status_err!("cellar ID validation failed: {}", err);
                    std::process::exit(1);
                });

            let governance_call = GovernanceCall {
                call: Some(Call::AaveV2Stablecoin(AaveV2StablecoinGovernance {
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

            print_proposal(
                proposal_json,
                self.quiet,
            )
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
