use abscissa_core::{clap::Parser, Command, Runnable};

/// Trust subcommand
#[derive(Command, Debug, Parser)]

pub struct TrustCmd {}

impl Runnable for TrustCmd {
    fn run(&self) {}
}
