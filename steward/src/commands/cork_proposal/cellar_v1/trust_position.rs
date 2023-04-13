use crate::{application::APP, cellars, commands::cork_proposal::print_proposal, prelude::*};
use abscissa_core::{clap::Parser, Command, Runnable};
use ethers::types::Address as EthereumAddress;
use steward_proto::steward::{
    cellar_v1_governance::{trust_position::Position, Function, TrustPosition},
    governance_call::Call,
    CellarV1Governance, GovernanceCall,
};

/// Fees Distributor subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nCalls trustPosition() on the target cellar contract at the specified block height.\nFor more information see https://github.com/PeggyJV/cellar-v1_5/blob/release/src/base/Cellar.sol"
)]
pub struct TrustPositionCmd {
    #[clap(short, long)]
    /// Address of the contract (position) to trust
    position: EthereumAddress,

    /// The position type, where 0 = ERC20, 1 = ER4626, 2 = Cellar
    #[clap(short = 't', long)]
    position_type: u8,

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

impl Runnable for TrustPositionCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            cellars::validate_cellar_id(&self.cellar_id)
                .await
                .unwrap_or_else(|err| {
                    status_err!("invalid cellar ID: {}", err);
                    std::process::exit(1);
                });

            let address = format!("{:#x}", self.position);
            let position = match self.position_type {
                0 => Position::Erc20Address(address),
                1 => Position::Erc4626Address(address),
                2 => Position::CellarAddress(address),
                _ => {
                    status_err!(
                        "invalid position argument {}. 0 => ERC20, 1 => ERC4626, 2 => Cellar"
                    );
                    std::process::exit(1);
                }
            };
            let governance_call = GovernanceCall {
                call: Some(Call::CellarV1Governance(CellarV1Governance {
                    function: Some(Function::TrustPosition(TrustPosition {
                        position: Some(position),
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
