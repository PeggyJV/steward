use abscissa_core::{clap::Parser, Command, Runnable};

/// Scheduled corks subcommand
#[derive(Command, Debug, Parser)]
pub struct Shutdown {
    cellar_id: String,
    block_height: u64,
    function: String,
}

impl Runnable for Shutdown {
    fn run(&self) {
        let _address = self.cellar_id.clone();
        let _function = self.function.clone();
    }
}
