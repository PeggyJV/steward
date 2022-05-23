use abscissa_core::{clap::Parser, Command, Runnable};

/// Sweep subcommand
#[derive(Command, Debug, Parser)]

pub struct SweepCmd {}

impl Runnable for SweepCmd {
    fn run(&self) {}
}
