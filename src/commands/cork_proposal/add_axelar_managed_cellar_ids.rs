use abscissa_core::{clap::Parser, Command, Runnable};
use somm_proto::axelar_cork::AddAxelarManagedCellarIDsProposalWithDeposit;

/// Add a new subscriber to the pubsub module
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nGenerates a partially filled proposal JSON for an AddAxelarManagedCellarIDsProposal."
)]
pub struct AddAxelarManagedCellarIDsCmd {
    /// Chain ID
    #[clap(long, short)]
    chain_id: u64,

    /// Cellar IDs
    #[clap(long, short)]
    cellar_ids: Vec<String>,

    /// Publisher's domain
    #[clap(long, short)]
    publisher_domain: String,
}

impl Runnable for AddAxelarManagedCellarIDsCmd {
    fn run(&self) {
        let proposal = AddAxelarManagedCellarIDsProposalWithDeposit {
            chain_id: self.chain_id,
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
