use crate::{
    application::APP,
    cellars,
    commands::cork_proposal::{get_proposal_json, print_proposal},
    prelude::*,
    proto::{
        aave_v2_stablecoin_governance::{Function, SetTrust},
        governance_call::Call,
        AaveV2StablecoinGovernance, GovernanceCall,
    },
};
use abscissa_core::{clap::Parser, Command, Runnable};
use ethers::types::Address as EthereumAddress;

/// TrustPosition subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nSet trust for target Cellar when chain reaches specified height, to prevent Cellar from rebalancing into an asset that has not been trusted by the users.\nFor more information see https://github.com/PeggyJV/aave-v2-cellar/blob/main/src/AaveV2StablecoinCellar.sol."
)]
pub struct TrustPositionCmd {
    /// Asset position
    #[clap(short, long)]
    position: EthereumAddress,

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

impl Runnable for TrustPositionCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            cellars::validate_cellar_id(self.chain_id, &self.cellar_id)
                .await
                .unwrap_or_else(|err| {
                    status_err!("cellar ID validation error:: {}", err);
                    std::process::exit(1);
                });

            let governance_call = GovernanceCall {
                call: Some(Call::AaveV2Stablecoin(AaveV2StablecoinGovernance {
                    function: Some(Function::SetTrust(SetTrust {
                        position: format!("{:#x}", self.position),
                        // Distrusting a position is handled by the SP. This approach brings the command more in line with
                        // the behavior of the trust-position command for V1 Cellars.
                        trust: true,
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
