mod aave2_clr_s;
mod cellar_v1;

use crate::commands::schedule_corks::aave2_clr_s::AaveV2StablecoinCellarCmd;
use abscissa_core::{clap::Parser, Command, Runnable};

use self::cellar_v1::CellarV1Cmd;

/// Schedule corks command
#[derive(Command, Debug, Parser, Runnable)]
pub enum ScheduleCmd {
    /// Manage Aave cellar validator functions
    #[clap(name = "aave2-clr-s", subcommand)]
    AaveV2StablecoinCellar(AaveV2StablecoinCellarCmd),
    /// Manage CellarV1 functions
    #[clap(name = "cellar-v1", subcommand)]
    CellarV1(CellarV1Cmd),
}
