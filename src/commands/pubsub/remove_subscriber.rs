use crate::pubsub::remove_subscriber;
use crate::{application::APP, prelude::*};
use abscissa_core::{clap::Parser, Command, Runnable};

/// Remove steward registration from pubsub module
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nRemoves the registered subscriber (steward) associated with the current configured signer.\nRequired for existing subscribers to be replaced."
)]
pub struct RemoveSubscriberCmd;

impl Runnable for RemoveSubscriberCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            remove_subscriber().await.unwrap();
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
