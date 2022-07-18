mod aave2_clr_s;

use crate::commands::schedule_corks::aave2_clr_s::AaveV2StablecoinCellarCmd;
use abscissa_core::{clap::Parser, Command, Runnable};

/// Schedule corks command
#[derive(Command, Debug, Parser, Runnable)]
pub enum ScheduleCmd {
    /// Manage Aave cellar validator functions
    #[clap(name = "aave2-clr-s", subcommand)]
    AaveV2StablecoinCellar(AaveV2StablecoinCellarCmd),
}
