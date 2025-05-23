use crate::application::APP;
use abscissa_core::{clap::Parser, status_err, Application, Command, Runnable};
use deep_space::address::Address as CosmosAddress;
use ethers::middleware::SignerMiddleware;
use ethers::providers::Middleware;
use ethers::signers::Signer;
use ethers::types::{Address as EthAddress, U256};
use gravity_bridge::gravity::ethereum::erc20_utils::get_erc20_balance;
use gravity_bridge::gravity::ethereum::send_to_cosmos::send_to_cosmos;
use gravity_bridge::gravity::utils::connection_prep::{check_for_eth, create_rpc_connections};
use gravity_bridge::gravity::utils::ethereum::downcast_to_u64;
use std::sync::Arc;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(60);

/// Send Ethereum to Cosmos
#[derive(Command, Debug, Default, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nSend Eth token to Cosmos chain.\n This command sends Eth token to the Cosmos chain via the Gravity bridge. \n It takes the tx amount, Eth key name, Gravity contract address, Cosmos token destination, number of times \n and the ERC20 token contract address."
)]
pub struct EthToCosmosCmd {
    #[clap(short = 'E', long)]
    erc20_address: String,

    #[clap(short, long)]
    init_amount: String,

    #[clap(short, long)]
    ethereum_key: String,

    #[clap(short, long)]
    cosmos_dest: String,

    /// The number of times transactions should repeat itself, default is 1.
    #[clap(short, long, default_value = "1")]
    times: String,

    #[clap(short = 'C', long)]
    gravity_address: String,
}

impl Runnable for EthToCosmosCmd {
    fn run(&self) {
        let config = APP.config();
        let erc20_address: EthAddress = self
            .erc20_address
            .parse()
            .expect("Invalid ERC20 contract address!");

        let gravity_address: EthAddress = self
            .gravity_address
            .parse()
            .expect("Invalid contract address!");

        let cosmos_prefix = config.cosmos.prefix.trim();
        let eth_rpc = config.ethereum.rpc.trim();

        abscissa_tokio::run_with_actix(&APP, async {
            let connections = create_rpc_connections(
                cosmos_prefix.to_string(),
                None,
                Some(eth_rpc.to_string()),
                TIMEOUT,
            )
            .await;

            let ethers_signer = config
                .load_ethers_signer(self.ethereum_key.clone())
                .await
                .expect("Could not load Ethereum signer");
            let provider = connections.eth_provider.clone().unwrap();
            let chain_id = provider
                .get_chainid()
                .await
                .expect("Could not retrieve chain ID");
            let chain_id =
                downcast_to_u64(chain_id).expect("Chain ID overflowed when downcasting to u64");
            let eth_client =
                SignerMiddleware::new(provider, ethers_signer.clone().with_chain_id(chain_id));
            let eth_client = Arc::new(eth_client);
            let cosmos_dest: CosmosAddress = self.cosmos_dest.parse().unwrap();
            let ethereum_address = eth_client.address();
            check_for_eth(ethereum_address, eth_client.clone()).await;
            let amount =
                U256::from_dec_str(&self.init_amount).expect("cannot parse amount as U256");

            let erc20_balance =
                get_erc20_balance(erc20_address, ethereum_address, eth_client.clone())
                    .await
                    .expect("Failed to get balance, check ERC20 contract address");

            let times = self.times.clone();
            let times_usize = times.parse::<usize>().expect("cannot parse times as usize");
            let times_u256 = U256::from_dec_str(&times).expect("cannot parse times as U256");

            if erc20_balance == 0u8.into() {
                panic!(
                    "You have zero {} tokens, please double check your sender and erc20 addresses!",
                    gravity_address
                );
            } else if amount * times_u256 > erc20_balance {
                panic!(
                    "Insufficient balance {} > {}",
                    amount * times_u256,
                    erc20_balance
                );
            }

            for _ in 0..times_usize {
                println!(
                    "Sending {} / {} to Cosmos from {} to {}",
                    self.init_amount.parse::<f64>().unwrap(),
                    erc20_address,
                    ethereum_address,
                    cosmos_dest
                );
                // we send some erc20 tokens to the gravity contract to register a deposit
                let res = send_to_cosmos(
                    erc20_address,
                    gravity_address,
                    amount,
                    cosmos_dest,
                    Some(TIMEOUT),
                    eth_client.clone(),
                )
                .await;
                match res {
                    Ok(tx_id) => println!("Send to Cosmos txid: {}", tx_id),
                    Err(e) => println!("Failed to send tokens! {:?}", e),
                }
            }
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
        });
    }
}
