use crate::pubsub::{add_subscriber, validate_ca_cert, validate_url};
use crate::{application::APP, prelude::*};
use abscissa_core::{clap::Parser, Command, Runnable};

/// Add a new subscriber to the pubsub module
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nRegisters a new subscriber on chain for a steward instance.\nRequired for publishers to retreive a steward instance's URL and CA info for trust establishment."
)]
pub struct AddSubscriberCmd {
    /// Path to the subscriber's CA certificate PEM file
    #[clap(long, short)]
    ca_path: String,

    /// The subscriber's push URL. Use the FQDN for your steward server.
    #[clap(long, short)]
    push_url: String,
}

impl Runnable for AddSubscriberCmd {
    fn run(&self) {
        #[allow(deprecated)]
        openssl_probe::init_ssl_cert_env_vars();

        let data = std::fs::read_to_string(&self.ca_path).unwrap_or_else(|e| {
            status_err!("failed to read CA cert file: {}", e);
            std::process::exit(1);
        });
        validate_ca_cert(data.as_bytes()).unwrap_or_else(|e| {
            status_err!("invalid CA cert: {}", e);
            std::process::exit(1);
        });
        validate_url(&self.push_url).unwrap_or_else(|e| {
            status_err!("invalid push URL: {}", e);
            std::process::exit(1);
        });

        abscissa_tokio::run_with_actix(&APP, async {
            add_subscriber(self.push_url.clone(), data).await.unwrap();
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
