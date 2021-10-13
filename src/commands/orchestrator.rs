mod start;

use abscissa_core::{Command, Clap, Runnable};

#[derive(Command, Debug, Clap, Runnable)]
pub enum OrchestratorCmd {
    Start(start::StartCommand),
}