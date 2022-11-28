use crate::{application::APP, cellars, cork, prelude::*};
use abscissa_core::{clap::Parser, Command, Runnable};
use ethers::abi::{AbiEncode, Address};
use steward_abi::cellar::*;

/// Fees Distributor subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nCalls trustPosition() on the target cellar contract at the specified block height.\nFor more information see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol"
)]
pub struct TrustPositionCmd {
    #[clap(short = 'n', long)]
    /// Address of the contract (position) to trust
    position: Address,

    /// The position type, where 0 = ERC20, 1 = ER4626, 2 = Cellar
    #[clap(short = 't', long)]
    position_type: u8,

    /// Target cellar for scheduled cork.
    #[clap(short, long)]
    cellar_id: String,

    /// Block height to schedule cork.
    #[clap(short = 'b', long)]
    height: u64,
}

impl Runnable for TrustPositionCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            let call = TrustPositionCall {
                position: self.position,
                position_type: self.position_type,
            };
            let encoded_call = CellarCalls::TrustPosition(call).encode();

            cellars::validate_cellar_id(&self.cellar_id).unwrap_or_else(|err| {
                status_err!("invalid cellar ID: {}", err);
                std::process::exit(1);
            });

            cork::schedule_cork(&self.cellar_id, encoded_call, self.height)
                .await
                .unwrap_or_else(|err| {
                    status_err!("error while submitting scheduled cork. please verify that the scheduling failed before attempting again: {}", err);
                    std::process::exit(1);
                })
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
