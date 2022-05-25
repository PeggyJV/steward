mod aave_cellar;

use crate::commands::schedule_corks::aave_cellar::AaveCellarCmd;
use abscissa_core::{clap::Parser, Command, Runnable};

/// Schedule corks command
#[derive(Command, Debug, Parser, Runnable)]
pub enum ScheduleCmd {
    /// Manage Aave cellar validator functions
    #[clap(subcommand)]
    Aave(AaveCellarCmd),
}
