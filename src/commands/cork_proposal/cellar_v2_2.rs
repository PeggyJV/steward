use abscissa_core::{clap::Parser, Command, Runnable};

mod add_adaptor_to_catalogue;
mod add_position_to_catalogue;
mod force_position_out;
mod initiate_shutdown;
mod lift_shutdown;
mod set_rebalance_deviation;
mod set_share_lock_period;
mod set_strategist_platform_cut;
mod toggle_ignore_pause;

/// Generates scheduled cork proposal templates containing function call data for V1 cellars
#[derive(Command, Debug, Parser, Runnable)]
pub enum CellarV2_2Cmd {
    AddAdaptorToCatalogue(add_adaptor_to_catalogue::AddAdaptorToCatalogueCmd),
    AddPositionToCatalogue(add_position_to_catalogue::AddPositionToCatalogueCmd),
    ForcePositionOut(force_position_out::ForcePositionOutCmd),
    InitiateShutdown(initiate_shutdown::InitiateShutdownCmd),
    LiftShutdown(lift_shutdown::LiftShutdownCmd),
    SetRebalanceDeviation(set_rebalance_deviation::SetRebalanceDeviationCmd),
    SetShareLockPeriod(set_share_lock_period::SetShareLockPeriodCmd),
    SetStrategistPlatformCut(set_strategist_platform_cut::SetStrategistPlatformCutCmd),
    ToggleIgnorePause(toggle_ignore_pause::ToggleIgnorePauseCmd),
}
