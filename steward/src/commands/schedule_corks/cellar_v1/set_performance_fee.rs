use crate::{application::APP, cellars, cork, prelude::*};
use abscissa_core::{clap::Parser, Command, Runnable};
use ethers::abi::AbiEncode;
use steward_abi::cellar::*;

/// Fees Distributor subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nCalls setPerformanceFee() on the target cellar contract at the specified block height.\nFor more information see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol"
)]
pub struct SetPerformanceFeeCmd {
    #[clap(short = 'n', long)]
    /// New performance fee proportion between 0 and 1e18 representing 0% and 100% respectively.
    new_performance_fee: u64,

    /// Target cellar for scheduled cork.
    #[clap(short, long)]
    cellar_id: String,

    /// Block height to schedule cork.
    #[clap(short = 'b', long)]
    height: u64,
}

impl Runnable for SetPerformanceFeeCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            let call = SetPerformanceFeeCall { new_performance_fee: self.new_performance_fee };
            let encoded_call = CellarCalls::SetPerformanceFee(call).encode();

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
