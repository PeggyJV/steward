mod start;

use abscissa_core::{Clap, Command, Runnable};

/// Orchestrator management commands
#[derive(Command, Debug, Clap, Runnable)]
pub enum OrchestratorCmd {
    Start(start::StartCommand),
}
