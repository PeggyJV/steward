mod initiate_shutdown;
mod lift_shutdown;
mod set_fees_distributor;
mod set_trust;
mod sweep;

use abscissa_core::{clap::Parser, Command, Runnable};

/// Generates scheduled cork proposal templates containing calls to the Aave V2 Stablecoin cellar
#[derive(Command, Debug, Parser, Runnable)]
pub enum AaveV2StablecoinCellarCmd {
    InitiateShutdown(initiate_shutdown::InitiateShutdownCmd),
    LiftShutdown(lift_shutdown::LiftShutdownCmd),
    SetFeesDistributor(set_fees_distributor::FeesDistributorCmd),
    Sweep(sweep::SweepCmd),
    Trust(set_trust::SetTrustCmd),
}
