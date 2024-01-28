mod aave_v2_stablecoin;
mod cellar_v1;
mod cellar_v2;
mod cellar_v2_2;
mod cellar_v2_5;

use crate::{
    commands::cork_proposal::aave_v2_stablecoin::AaveV2StablecoinCellarCmd, proto::GovernanceCall,
};
use abscissa_core::{clap::Parser, Command, Runnable};
use somm_proto::{axelar_cork::AxelarScheduledCorkProposal, cork::ScheduledCorkProposal};

use self::cellar_v1::CellarV1Cmd;
use self::cellar_v2::CellarV2Cmd;
use self::cellar_v2_2::CellarV2_2Cmd;
use self::cellar_v2_5::CellarV2_5Cmd;

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
    /// Generates a proposal template for a V2.2 cellar
    #[clap(name = "cellar-v2.2", subcommand)]
    CellarV2_2(CellarV2_2Cmd),
    /// Generates a proposal template for a V2.5 cellar
    #[clap(name = "cellar-v2.5", subcommand)]
    CellarV2_5(CellarV2_5Cmd),
}

fn get_cork_proposal_json(
    height: u64,
    contract: String,
    governance_call: GovernanceCall,
) -> String {
    let json =
        serde_json::to_string(&governance_call).expect("failed to serialize governance call");
    let proposal = ScheduledCorkProposal {
        block_height: height,
        target_contract_address: contract,
        contract_call_proto_json: json,
        ..Default::default()
    };

    serde_json::to_string(&proposal).expect("failed to serialize proposal")
}

fn get_axelarcork_proposal_json(
    chain_id: u64,
    height: u64,
    contract: String,
    governance_call: GovernanceCall,
) -> String {
    let json =
        serde_json::to_string(&governance_call).expect("failed to serialize governance call");
    let proposal = AxelarScheduledCorkProposal {
        block_height: height,
        target_contract_address: contract,
        contract_call_proto_json: json,
        chain_id,
        ..Default::default()
    };

    serde_json::to_string(&proposal).expect("failed to serialize proposal")
}

fn get_proposal_json(
    height: u64,
    contract: String,
    governance_call: GovernanceCall,
    chain_id: u64,
) -> String {
    if chain_id == 1 {
        get_cork_proposal_json(height, contract, governance_call)
    } else {
        get_axelarcork_proposal_json(chain_id, height, contract, governance_call)
    }
}

/// Outputs the JSON formatted scheduled cork data for submitting a Scheduled Cork Proposal to Sommelier
fn print_proposal(proposal_json: String, quiet: bool) {
    if !quiet {
        println!("\nThe following JSON can be used to submit a scheduled cork governance proposal using the Sommelier CLI.\nYou must fill in the 'title' and 'description' fields before submitting.\n");
    }

    println!("{}", proposal_json);
}
