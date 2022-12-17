use crate::{application::APP, cellars, commands::cork_proposal::print_proposal, prelude::*};
use abscissa_core::{clap::Parser, Command, Runnable};
use ethers::types::Address as EthereumAddress;
use steward_proto::steward::{
    aave_v2_stablecoin_governance::{Function, Sweep},
    governance_call::Call,
    AaveV2StablecoinGovernance, GovernanceCall,
};

/// Sweep subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nSchedule sweep Cosmos address of target cellar when chain reaches specified height.\nFor more information see https://github.com/PeggyJV/aave-v2-cellar/blob/main/src/AaveV2StablecoinCellar.sol."
)]
pub struct SweepCmd {
    /// Token address to transfer out of Cellar.
    #[clap(short = 't', long)]
    token: EthereumAddress,

    /// Address to sweep token into
    #[clap(short = 'd', long)]
    destination_address: EthereumAddress,

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

impl Runnable for SweepCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            cellars::validate_cellar_id(&self.cellar_id)
                .await
                .unwrap_or_else(|err| {
                    status_err!("cellar ID validation error:: {}", err);
                    std::process::exit(1);
                });

            let governance_call = GovernanceCall {
                call: Some(Call::AaveV2Stablecoin(AaveV2StablecoinGovernance {
                    function: Some(Function::Sweep(Sweep {
                        token: format!("{:#x}", self.token),
                        recipient: format!("{:#x}", self.destination_address),
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
