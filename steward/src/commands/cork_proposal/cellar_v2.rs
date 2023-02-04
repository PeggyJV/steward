use abscissa_core::{clap::Parser, Command, Runnable};

mod initiate_shutdown;
mod lift_shutdown;
mod set_platform_fee;
mod set_strategist_platform_cut;
mod setup_adaptor;

/// Generates scheduled cork proposal templates containing function call data for V1 cellars
#[derive(Command, Debug, Parser, Runnable)]
pub enum CellarV2Cmd {
    InitiateShutdown(initiate_shutdown::InitiateShutdownCmd),
    LiftShutdown(lift_shutdown::LiftShutdownCmd),
    SetPlatformFee(set_platform_fee::SetPlatformFeeCmd),
    SetStrategistPlatformCut(set_strategist_platform_cut::SetStrategistPlatformCutCmd),
    SetupAdaptor(setup_adaptor::SetupAdaptorCmd),
}
