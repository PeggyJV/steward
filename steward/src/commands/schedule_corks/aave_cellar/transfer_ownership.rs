use crate::{application::APP, prelude::*, utils::submit_schedule_cork};
use abscissa_core::{clap::Parser, Command, Runnable};
use ethers::abi::AbiEncode;
use ethers::types::*;
use somm_proto::cork::Cork;
use steward_abi::aave_v2_stablecoin::*;

/// Transfer Ownership subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Transfer ownership to new owner.\n This command schedules transfer of ownership of a Cellar to a new owner. This is a validator only command and can only be run by validators. It Schedules transfer ownership based on the height specified by the validators. Therefore, it'll execute the function when the chain reaches that height. This command also takes the new owner flag."
)]

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

            let cork = Cork {
                encoded_contract_call: encoded_call,
                target_contract_address: self.contract.clone(),
            };

            submit_schedule_cork(cork, self.height)
                .await
                .unwrap_or_else(|err| {
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
