mod start;

use abscissa_core::{Command, Clap, Runnable};

/// Orchestrator management commands
#[derive(Command, Debug, Clap, Runnable)]
pub enum OrchestratorCmd {
    Start(start::StartCommand),
}