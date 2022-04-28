mod shutdown;
use shutdown::Shutdown;

use abscissa_core::{clap::Parser, Command, Runnable};

/// Schedule corks commands
#[derive(Command, Debug, Parser, Runnable)]
#[clap(
    long_about = "DESCRIPTION \n\n Schedule corks for target contract.\n This command schedules cork by calling a function from the target Cellar. This is a validator only \n command and can only be run by validators. It Schedules cork based on the height specified by the \n validators. Therefore, it'll execute the function when the chain reaches that height."
)]
pub enum ScheduledCorksCmd {
    Shutdown(Shutdown),
}
