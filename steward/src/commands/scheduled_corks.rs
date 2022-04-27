use abscissa_core::{clap::Parser, Command, Runnable};
use ethers::abi::Function;

/// Scheduled corks subcommand
#[derive(Command, Debug, Parser)]
pub struct ScheduledCorksCmd {
    cellar_id: String,
    block_height: u64,
    function: String,
}

impl Runnable for ScheduledCorksCmd {
    fn run(&self) {
        let address = self.cellar_id.clone();
        let function: Function = self.function.clone().parse().unwrap;
    }
}
