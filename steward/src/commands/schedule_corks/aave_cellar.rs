mod fees_distributor;
mod shutdown;
mod sweep;
mod transfer_ownership;
mod trust;

use abscissa_core::{clap::Parser, Command, Runnable};

/// Schedule shutdown commands
#[derive(Command, Debug, Parser, Runnable)]
pub enum AaveCellarCmd {
    Shutdown(shutdown::ShutdownCmd),
    FeesDistributor(fees_distributor::FeesDistributorCmd),
    Sweep(sweep::SweepCmd),
    TransferOwnership(transfer_ownership::TransferOwnershipCmd),
    Trust(trust::TrustCmd),
}
