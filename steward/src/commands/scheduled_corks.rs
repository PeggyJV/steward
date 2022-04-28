mod shutdown;
use shutdown::Shutdown;

use abscissa_core::{clap::Parser, Command, Runnable};

/// Provides tools for contract deployment
#[derive(Command, Debug, Parser, Runnable)]
pub enum ScheduledCorksCmd {
    Shutdown(Shutdown),
}