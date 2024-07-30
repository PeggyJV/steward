use crate::prelude::*;
use crate::pubsub::validate_ca_cert;
use abscissa_core::{clap::Parser, Command, Runnable};
use somm_proto::pubsub::AddPublisherProposal;
use url::Url;

/// Add a new subscriber to the pubsub module
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nGenerates a partially filled proposal JSON for an AddPublisher proposal."
)]
pub struct AddPublisherCmd {
    /// Path to the subscriber's CA certificate PEM file
    #[clap(long, short)]
    ca_path: String,

    /// The publisher's proof URL. The publisher domain will be extracted from this.
    #[clap(long, short)]
    proof_url: String,

    /// Address
    #[clap(long, short)]
    address: String,
}

impl Runnable for AddPublisherCmd {
    fn run(&self) {
        openssl_probe::init_ssl_cert_env_vars();

        let data = std::fs::read_to_string(&self.ca_path).unwrap_or_else(|e| {
            status_err!("failed to read CA cert file: {}", e);
            std::process::exit(1);
        });
        validate_ca_cert(data.as_bytes()).unwrap_or_else(|e| {
            status_err!("invalid CA cert: {}", e);
            std::process::exit(1);
        });

        let proof_url_parsed = Url::parse(&self.proof_url).expect("failed to parse proof URL");
        let domain = proof_url_parsed
            .domain()
            .expect("failed to get domain from proof URL");

        let proposal = AddPublisherProposal {
            domain: domain.to_string(),
            address: self.address.clone(),
            proof_url: self.proof_url.clone(),
            ca_cert: data,
            ..Default::default()
        };

        println!(
            "{}",
            serde_json::to_string(&proposal).expect("failed to serialize proposal")
        );
    }
}
