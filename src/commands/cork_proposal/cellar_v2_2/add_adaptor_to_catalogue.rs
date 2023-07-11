use abscissa_core::{clap::Parser, Command, Runnable};

use crate::{
    application::APP,
    cellars::{self, is_evm_address},
    commands::cork_proposal::print_proposal,
    prelude::*,
    proto::{
        cellar_v2_2governance::{AddAdaptorToCatalogue, Function},
        governance_call::Call,
        CellarV22governance, GovernanceCall,
    },
};

#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nCalls addAdaptorToCatalogue() on the target V2.2 cellar contract at the specified block height.\nFor more information see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol"
)]
pub struct AddAdaptorToCatalogueCmd {
    #[clap(short = 'a', long)]
    /// Address of the adaptor contract.
    adaptor_address: String,

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

impl Runnable for AddAdaptorToCatalogueCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            cellars::validate_cellar_id(&self.cellar_id)
                .await
                .unwrap_or_else(|err| {
                    status_err!("invalid cellar ID: {}", err);
                    std::process::exit(1);
                });

            if !is_evm_address(&self.adaptor_address) {
                status_err!("invalid adaptor address: {}", self.adaptor_address);
                std::process::exit(1);
            }

            let governance_call = GovernanceCall {
                call: Some(Call::CellarV22(CellarV22governance {
                    function: Some(Function::AddAdaptorToCatalogue(AddAdaptorToCatalogue {
                        adaptor: self.adaptor_address.clone(),
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
