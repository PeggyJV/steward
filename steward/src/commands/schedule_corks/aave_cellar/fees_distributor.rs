use abscissa_core::{clap::Parser, Command, Runnable};

/// Fees Distributor subcommand
#[derive(Command, Debug, Parser)]
pub struct FeesDistributorCmd {}

impl Runnable for FeesDistributorCmd {
    fn run(&self) {}
}
