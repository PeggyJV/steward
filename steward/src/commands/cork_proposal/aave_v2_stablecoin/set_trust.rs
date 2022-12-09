use crate::{application::APP, cellars, commands::cork_proposal::print_proposal, prelude::*};
use abscissa_core::{clap::Parser, Command, Runnable};
use ethers::types::H160;
use steward_proto::steward::{
    aave_v2_stablecoin_governance::{Function, SetTrust},
    governance_call::Call,
    AaveV2StablecoinGovernance, GovernanceCall,
};

/// Trust subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nSet trust for target Cellar when chain reaches specified height, to prevent Cellar from rebalancing into an asset that has not been trusted by the users."
)]

pub struct SetTrustCmd {
    /// Asset position
    #[clap(short = 'p', long)]
    position: H160,

    /// Set to true if you trust asset
    #[clap(short = 't', long)]
    trust: bool,

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

impl Runnable for SetTrustCmd {
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
                    function: Some(Function::SetTrust(SetTrust {
                        position: format!("{:#x}", self.position),
                        trust: self.trust,
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
