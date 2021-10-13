mod start;

use abscissa_core::{Command, Clap, Runnable};

#[derive(Command, Debug, Clap, Runnable)]
pub enum OrchestratorCmd {
    #[clap(short, long)]
    Start(start::StartCommand),
}