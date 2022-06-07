use crate::{application::APP, prelude::*, utils::SubmitCork};
use abscissa_core::{clap::Parser, Command, Runnable};
use ethers::abi::AbiEncode;
use ethers::types::*;
use steward_abi::aave_v2_stablecoin::*;

/// Transfer Ownership subcommand
#[derive(Command, Debug, Parser)]

pub struct TransferOwnershipCmd {
    /// Address of new owner
    #[clap(short = 'n', long)]
    new_owner: H160,

    /// Target contract for scheduled cork.
    #[clap(short = 'c', long)]
    contract: String,

    /// Block height to schedule cork.
    #[clap(short = 'b', long)]
    height: u64,
}

impl Runnable for TransferOwnershipCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            // Encoded call for transfer_ownership
            let call = TransferOwnershipCall {
                new_owner: self.new_owner,
            };

            let encoded_call = AaveV2StablecoinCellarCalls::TransferOwnership(call).encode();
            let submit = SubmitCork {
                contract: self.contract.clone(),
                height: self.height,
                encoded_call,
            };

            submit.submit_cork().await.unwrap_or_else(|err| {
                status_err!("executor exited with error: {}", err);
                std::process::exit(1);
            })
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
