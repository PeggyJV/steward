use abscissa_core::{clap::Parser, Command, Runnable};
use somm_proto::cork::AddManagedCellarIDsProposalWithDeposit;

/// Add a new subscriber to the pubsub module
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nGenerates a partially filled proposal JSON for an AddManagedCellarIDsProposalWithDeposit."
)]
pub struct AddManagedCellarIDsCmd {
    /// Cellar IDs
    cellar_ids: Vec<String>,

    /// Publisher's domain
    #[clap(long, short)]
    publisher_domain: String,
}

impl Runnable for AddManagedCellarIDsCmd {
    fn run(&self) {
        // make sure each cellar ID is a valid evm address
        for cellar_id in &self.cellar_ids {
            cellar_id
                .parse::<ethers::types::Address>()
                .expect("invalid cellar ID");
        }

        let proposal = AddManagedCellarIDsProposalWithDeposit {
            cellar_ids: self.cellar_ids.clone(),
            publisher_domain: self.publisher_domain.clone(),
            deposit: "5000000000usomm".to_string(),
            ..Default::default()
        };

        println!(
            "{}",
            serde_json::to_string(&proposal).expect("failed to serialize proposal")
        );
    }
}
