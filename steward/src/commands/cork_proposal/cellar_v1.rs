use abscissa_core::{clap::Parser, Command, Runnable};

mod initiate_shutdown;
mod lift_shutdown;
mod reset_high_watermark;
mod set_fees_distributor;
mod set_owner;
mod set_performance_fee;
mod set_platform_fee;
mod set_strategist_performance_cut;
mod set_strategist_platform_cut;
mod trust_position;

/// Generates scheduled cork proposal templates containing function call data for V1 cellars
#[derive(Command, Debug, Parser, Runnable)]
pub enum CellarV1Cmd {
    InitiateShutdown(initiate_shutdown::InitiateShutdownCmd),
    LiftShutdown(lift_shutdown::LiftShutdownCmd),
    ResetHighWatermark(reset_high_watermark::ResetHighWatermarkCmd),
    SetFeesDistributor(set_fees_distributor::SetFeesDistributorCmd),
    SetOwner(set_owner::SetOwnerCmd),
    SetPerformanceFee(set_performance_fee::SetPerformanceFeeCmd),
    SetPlatformFee(set_platform_fee::SetPlatformFeeCmd),
    SetStrategistPerformanceCut(set_strategist_performance_cut::SetStrategistPerformanceCutCmd),
    SetStrategistPlatformCut(set_strategist_platform_cut::SetStrategistPlatformCutCmd),
    TrustPosition(trust_position::TrustPositionCmd),
}
