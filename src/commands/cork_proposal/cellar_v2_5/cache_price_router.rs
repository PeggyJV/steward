use abscissa_core::{clap::Parser, Command, Runnable};

use crate::{
    application::APP,
    cellars::{self, is_evm_address},
    commands::cork_proposal::print_proposal,
    prelude::*,
    proto::{
        cellar_v2_5governance::{CachePriceRouter, Function},
        governance_call::Call,
        CellarV25governance, GovernanceCall,
    },
};

#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nCalls cachePriceRouter() on the target V2.5 cellar contract at the specified block height.\nFor more information see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol"
)]
pub struct CachePriceRouterCmd {
    #[clap(short, long)]
    /// Whether to check total assets
    check_total_assets: bool,

    /// Allowable range
    #[clap(short, long)]
    allowable_range: u32,

    /// Expected price router address
    #[clap(short, long)]
    router_address: String,

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

impl Runnable for CachePriceRouterCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            cellars::validate_cellar_id(self.chain_id, &self.cellar_id)
                .await
                .unwrap_or_else(|err| {
                    status_err!("invalid cellar ID: {}", err);
                    std::process::exit(1);
                });

            if !is_evm_address(&self.router_address) {
                status_err!("invalid router address: {}", self.router_address);
                std::process::exit(1);
            }

            let governance_call = GovernanceCall {
                call: Some(Call::CellarV25(CellarV25governance {
                    function: Some(Function::CachePriceRouter(CachePriceRouter {
                        check_total_assets: self.check_total_assets,
                        allowable_range: self.allowable_range,
                        expected_price_router: self.router_address.clone(),
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
