mod start;

use abscissa_core::{clap::Parser, Command, Runnable};

/// Orchestrator management commands
#[derive(Command, Debug, Parser, Runnable)]
pub enum OrchestratorCmd {
    Start(start::StartCommand),
}
