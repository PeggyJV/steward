mod initiate_shutdown;
mod lift_shutdown;

use abscissa_core::{clap::Parser, Command, Runnable};

/// Schedule shutdown commands
#[derive(Command, Debug, Parser, Runnable)]
pub enum AaveV2StablecoinCellarCmd {
    InitiateShutdown(initiate_shutdown::InitiateShutdownCmd),
    LiftShutdown(lift_shutdown::LiftShutdownCmd),
}
