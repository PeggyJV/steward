use crate::{application::APP, prelude::*, server::start_encode_server};
use abscissa_core::{clap::Parser, Command, Runnable};

/// Start the Encode server
#[derive(Command, Debug, Parser)]
#[clap(long_about = "DESCRIPTION\n\n")]
pub struct StartCmd {}

impl Runnable for StartCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            start_encode_server().await;
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
