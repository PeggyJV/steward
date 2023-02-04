use crate::{
    application::APP, cellars, commands::cork_proposal::print_proposal, prelude::*, utils,
};
use abscissa_core::{clap::Parser, Command, Runnable};
use ethers::types::Address as EthereumAddress;
use steward_proto::steward::{
    cellar_v2_governance::{Function, SetupAdaptor},
    governance_call::Call,
    CellarV2Governance, GovernanceCall,
};

/// Fees Distributor subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nCalls setupAdaptor() on the target V2 cellar contract at the specified block height.\nFor more information see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol"
)]
pub struct SetupAdaptorCmd {
    #[clap(short = 'a', long)]
    /// Address of the adaptor contract.
    adaptor_address: EthereumAddress,

    /// Target contract for scheduled cork.
    #[clap(short, long)]
    cellar_id: String,

    /// Block height to schedule cork.
    #[clap(short, long)]
    block_height: u64,

    /// Only print JSON output, omitting explanatory text
    #[clap(short, long)]
    quiet: bool,
}

impl Runnable for SetupAdaptorCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            cellars::validate_cellar_id(&self.cellar_id)
                .await
                .unwrap_or_else(|err| {
                    status_err!("invalid cellar ID: {}", err);
                    std::process::exit(1);
                });

            let governance_call = GovernanceCall {
                call: Some(Call::CellarV2(CellarV2Governance {
                    function: Some(Function::SetupAdaptor(SetupAdaptor {
                        adaptor: utils::format_eth_address(self.adaptor_address),
                    })),
                })),
            };

            print_proposal(
                self.block_height,
                self.cellar_id.clone(),
                governance_call,
                self.quiet,
            )
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
