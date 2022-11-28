use crate::{application::APP, cellars, cork, prelude::*};
use abscissa_core::{clap::Parser, Command, Runnable};
use deep_space::Address;
use ethers::abi::AbiEncode;
use steward_abi::aave_v2_stablecoin::*;

/// Fees Distributor subcommand
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Set fees distributor Cosmos address of target cellar when chain reaches specified height"
)]
pub struct FeesDistributorCmd {
    #[clap(short = 'n', long)]
    /// Fee distributor's address
    new_fees_distributor: Address,

    /// Target contract for scheduled cork.
    #[clap(short = 'c', long)]
    contract: String,

    /// Block height to schedule cork.
    #[clap(short = 'b', long)]
    height: u64,
}

impl Runnable for FeesDistributorCmd {
    fn run(&self) {
        abscissa_tokio::run_with_actix(&APP, async {
            let mut address = self.new_fees_distributor.as_bytes().to_vec();

            while address.len() < 32 {
                address.insert(0, 0u8);
            }

            let mut address_slice: [u8; 32] = Default::default();
            address_slice.copy_from_slice(&address[..]);

            let call = SetFeesDistributorCall {
                new_fees_distributor: address_slice,
            };

            let encoded_call = AaveV2StablecoinCellarCalls::SetFeesDistributor(call).encode();

            cellars::validate_cellar_id(self.contract.as_str()).unwrap_or_else(|err| {
                status_err!("Can't validate contract address format: {}", err);
                std::process::exit(1);
            });

            cork::schedule_cork(&self.contract, encoded_call, self.height)
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
