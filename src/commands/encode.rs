mod start;

use abscissa_core::{clap::Parser, Command, Runnable};

/// Commands for running the encoding server
#[derive(Command, Debug, Parser, Runnable)]
pub enum EncodeCmd {
    Start(start::StartCmd),
}
