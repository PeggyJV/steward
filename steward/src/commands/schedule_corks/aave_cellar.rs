mod shutdown;

use abscissa_core::{clap::Parser, Command, Runnable};

/// Schedule shutdown commands
#[derive(Command, Debug, Parser, Runnable)]
pub enum AaveCellarCmd {
    Shutdown(shutdown::ShutdownCmd),
}
