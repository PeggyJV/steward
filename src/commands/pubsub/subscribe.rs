use crate::pubsub::{subscribe, validate_url};
use crate::{application::APP, prelude::*};
use abscissa_core::{clap::Parser, Command, Runnable};

/// Subscribe to handle a publisher's calls to a particular cellar
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nSubscribe to updates for a given cellar from the specified publisher."
)]
pub struct SubscribeCmd {
    /// The ID of the cellar
    #[clap(long, short)]
    cellar_id: String,

    /// The publisher's domain as defined in the pubsub module
    #[clap(long, short)]
    publisher_domain: String,
}

impl Runnable for SubscribeCmd {
    fn run(&self) {
        #[allow(deprecated)]
        openssl_probe::init_ssl_cert_env_vars();

        validate_url(&self.publisher_domain).unwrap_or_else(|e| {
            status_err!("invalid publisher domain: {}", e);
            std::process::exit(1);
        });

        abscissa_tokio::run_with_actix(&APP, async {
            subscribe(self.cellar_id.clone(), self.publisher_domain.clone())
                .await
                .unwrap();
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
