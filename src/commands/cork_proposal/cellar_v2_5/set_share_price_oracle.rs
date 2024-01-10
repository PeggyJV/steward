use abscissa_core::{clap::Parser, Command, Runnable};

use crate::{
    application::APP,
    cellars::{self, is_evm_address},
    commands::cork_proposal::{get_proposal_json, print_proposal},
    prelude::*,
    proto::{
        cellar_v2_5governance::{Function, SetSharePriceOracle},
        governance_call::Call,
        CellarV25governance, GovernanceCall,
    },
    utils::string_to_u256,
};

#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nCalls setSharePriceOracle() on the target V2.5 cellar contract at the specified block height.\nFor more information see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol"
)]
pub struct SetSharePriceOracleCmd {
    #[clap(short, long)]
    /// ID of the oracle registry
    registry_id: String,

    #[clap(short, long)]
    /// Address of the oracle
    share_price_oracle: String,

    /// Target contract for scheduled cork.
    #[clap(short, long)]
    cellar_id: String,

    #[clap(long)]
    chain_id: u64,

    /// Block height to schedule cork.
    #[clap(short, long)]
    block_height: u64,

    /// Only print JSON output, omitting explanatory text
    #[clap(short, long)]
    quiet: bool,
}

impl Runnable for SetSharePriceOracleCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            cellars::validate_cellar_id(self.chain_id, &self.cellar_id)
                .await
                .unwrap_or_else(|err| {
                    status_err!("invalid cellar ID: {}", err);
                    std::process::exit(1);
                });

            if let Err(err) = string_to_u256(self.registry_id.clone()) {
                status_err!("invalid registry ID: {}", err);
                std::process::exit(1);
            }

            if !is_evm_address(&self.share_price_oracle) {
                status_err!("invalid oracle address: {}", self.share_price_oracle);
                std::process::exit(1);
            }

            let governance_call = GovernanceCall {
                call: Some(Call::CellarV25(CellarV25governance {
                    function: Some(Function::SetSharePriceOracle(SetSharePriceOracle {
                        registry_id: self.registry_id.clone(),
                        share_price_oracle: self.share_price_oracle.clone(),
                    })),
                })),
            };

            let proposal_json = get_proposal_json(
                self.block_height,
                self.cellar_id.clone(),
                governance_call,
                self.chain_id,
            );

            print_proposal(proposal_json, self.quiet)
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
