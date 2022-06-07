use crate::{
    application::APP,
    cellars::{self},
    config,
    prelude::*,
    somm_send,
    utils::{CHAIN_PREFIX, MESSAGE_TIMEOUT},
};
use abscissa_core::{clap::Parser, Command, Runnable};
use deep_space::{Coin, Contact};
use ethers::abi::AbiEncode;
use ethers::types::*;
use somm_proto::cork::Cork;
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
        let config = APP.config();

        abscissa_tokio::run_with_actix(&APP, async {
            // Encoded call for transfer_ownership
            let call = TransferOwnershipCall {
                new_owner: self.new_owner,
            };

            let encoded_call = AaveV2StablecoinCellarCalls::TransferOwnership(call).encode();

            // Validate cellar id
            cellars::validate_cellar_id(self.contract.as_str()).unwrap_or_else(|err| {
                status_err!("Can't validate contract address format: {}", err);
                std::process::exit(1);
            });

            let cork = Cork {
                encoded_contract_call: encoded_call,
                target_contract_address: self.contract.clone(),
            };

            // Establish grpc connections
            debug!("establishing grpc connection");
            let contact = Contact::new(&config.cosmos.grpc, MESSAGE_TIMEOUT, CHAIN_PREFIX).unwrap();

            // Get cosmos fees
            debug!("getting cosmos fee");
            let cosmos_gas_price = config.cosmos.gas_price.as_tuple();

            let fee = Coin {
                amount: (cosmos_gas_price.0 as u64).into(),
                denom: cosmos_gas_price.1,
            };

            // send scheduled cork
            somm_send::schedule_cork(
                &contact,
                cork,
                config::DELEGATE_ADDRESS.to_string(),
                &config::DELEGATE_KEY,
                fee,
                self.height,
            )
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
