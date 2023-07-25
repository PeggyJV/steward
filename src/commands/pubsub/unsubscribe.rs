use crate::pubsub::unsubscribe;
use crate::{application::APP, prelude::*};
use abscissa_core::{clap::Parser, Command, Runnable};

/// Unsubscribe from receiving requests for the target cellar
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nUnsubscribes from receiving updates for the target cellar by deleting on-chain subscriber intent."
)]
pub struct UnsubscribeCmd {
    /// The cellar to unsubscribe from
    #[clap(long, short)]
    cellar_id: String,
}

impl Runnable for UnsubscribeCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            unsubscribe(self.cellar_id.clone()).await.unwrap();
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
