mod initiate_shutdown;
mod lift_shutdown;
mod fees_distributor;
mod sweep;
mod trust;
mod transfer_ownership;

use abscissa_core::{clap::Parser, Command, Runnable};

/// Schedule shutdown commands
#[derive(Command, Debug, Parser, Runnable)]
pub enum AaveV2StablecoinCellarCmd {
    InitiateShutdown(initiate_shutdown::InitiateShutdownCmd),
    LiftShutdown(lift_shutdown::LiftShutdownCmd),
    FeesDistributor(fees_distributor::FeesDistributorCmd),
    Sweep(sweep::SweepCmd),
    Trust(trust::TrustCmd),
    TransferOwnership(transfer_ownership::TransferOwnershipCmd),
}
