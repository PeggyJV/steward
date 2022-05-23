use abscissa_core::{clap::Parser, Command, Runnable};

/// Transfer Ownership subcommand
#[derive(Command, Debug, Parser)]

pub struct TransferOwnershipCmd {}

impl Runnable for TransferOwnershipCmd {
    fn run(&self) {}
}
