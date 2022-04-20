use abscissa_core::{clap::Parser, Command, Runnable};

/// Scheduled corks subcommand
#[derive(Command, Debug, Parser)]
pub struct ScheduledCorksCmd {}

impl Runnable for ScheduledCorksCmd {
    fn run(&self) {}
}
