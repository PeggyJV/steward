mod aave_v2_stablecoin;
mod cellar_v1;
mod cellar_v2;

use crate::commands::cork_proposal::aave_v2_stablecoin::AaveV2StablecoinCellarCmd;
use abscissa_core::{clap::Parser, Command, Runnable};
use somm_proto::cork::ScheduledCorkProposal;
use steward_proto::steward::GovernanceCall;

use self::cellar_v1::CellarV1Cmd;
use self::cellar_v2::CellarV2Cmd;

/// Generates and prints a Sommelier governance ScheduledCorkProposal JSON template for the specified cellar and contract function
#[derive(Command, Debug, Parser, Runnable)]
pub enum CorkProposalCmd {
    /// Generates a proposal template for the Aave V2 Stablecoin cellar
    #[clap(name = "aave-v2-stablecoin", subcommand)]
    AaveV2StablecoinCellar(AaveV2StablecoinCellarCmd),
    /// Generates a proposal template for a V1 cellar
    #[clap(name = "cellar-v1", subcommand)]
    CellarV1(CellarV1Cmd),
    /// Generates a proposal template for a V2 cellar
    #[clap(name = "cellar-v2", subcommand)]
    CellarV2(CellarV2Cmd),
}

/// Outputs the JSON formatted scheduled cork data for submitting a Scheduled Cork Proposal to Sommelier
fn print_proposal(height: u64, contract: String, governance_call: GovernanceCall, quiet: bool) {
    let json =
        serde_json::to_string(&governance_call).expect("failed to serialize governance call");
    let proposal = ScheduledCorkProposal {
        block_height: height,
        target_contract_address: contract,
        contract_call_proto_json: json,
        ..Default::default()
    };
    let proposal = serde_json::to_string(&proposal).expect("failed to serialize proposal");

    if !quiet {
        println!("\nThe following JSON can be used to submit a scheduled cork governance proposal using the Sommelier CLI.\nYou must fill in the 'title' and 'description' fields before submitting.\n");
    }
    println!("{}", proposal);
}
