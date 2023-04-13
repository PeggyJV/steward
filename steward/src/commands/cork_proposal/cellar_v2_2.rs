use abscissa_core::{clap::Parser, Command, Runnable};

mod add_adaptor_to_catalogue;
mod add_position_to_catalogue;
mod force_position_out;
mod initiate_shutdown;
mod lift_shutdown;
mod remove_adaptor_from_catalogue;
mod remove_position_from_catalogue;
mod set_strategist_platform_cut;
mod toggle_ignore_pause;

/// Generates scheduled cork proposal templates containing function call data for V1 cellars
#[derive(Command, Debug, Parser, Runnable)]
pub enum CellarV2_2Cmd {
    AddPositionToCatalogue(add_position_to_catalogue::AddPositionToCatalogueCmd),
    RemovePositionFromCatalogue(remove_position_from_catalogue::RemovePositionFromCatalogueCmd),
    AddAdaptorToCatalogue(add_adaptor_to_catalogue::AddAdaptorToCatalogueCmd),
    RemoveAdaptorFromCatalogue(remove_adaptor_from_catalogue::RemoveAdaptorFromCatalogueCmd),
    ForcePositionOut(force_position_out::ForcePositionOutCmd),
    ToggleIgnorePause(toggle_ignore_pause::ToggleIgnorePauseCmd),
    InitiateShutdown(initiate_shutdown::InitiateShutdownCmd),
    LiftShutdown(lift_shutdown::LiftShutdownCmd),
    SetStrategistPlatformCut(set_strategist_platform_cut::SetStrategistPlatformCutCmd),
}
