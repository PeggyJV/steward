//! `cosmos subcommands` subcommand

use crate::{application::APP, prelude::*, utils::*};
use abscissa_core::{clap::Parser, Command, Runnable};
use clarity::Uint256;
use deep_space::{coin::Coin, private_key::PrivateKey as CosmosPrivateKey};
use ethers::types::Address as EthAddress;
use gravity_bridge::cosmos_gravity::send::send_to_eth;
use gravity_bridge::gravity_proto::gravity::DenomToErc20Request;
use gravity_bridge::gravity_utils::connection_prep::{check_for_fee_denom, create_rpc_connections};
use regex::Regex;
use std::process::exit;

/// Create Tx on Cosmos chain
#[derive(Command, Debug, Parser)]
pub enum Cosmos {
    SendToEth(SendToEth),
    Send(Send),
}

impl Runnable for Cosmos {
    /// Start the application.
    fn run(&self) {
        // Your code goes here
    }
}

/// Send tx from Cosmos to Ethereum chain
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Send transactions from Cosmos to Ethereum chain.\n This command sends tx from the Cosmos chain to the Eth chain.\n It takes a free vector of strings as required flags for Cosmos key, Eth key and amount."
)]
pub struct SendToEth {
    /// Cosmos key name
    cosmos_key: String,

    /// Eth Address
    eth_addr: String,

    /// ERC20 coin amount
    erc20_amount: String,

    #[clap(short, long)]
    help: bool,
}

fn parse_denom(s: &str) -> (String, String) {
    let re_dec_amt = r#"[[:digit:]]+(?:\.[[:digit:]]+)?|\.[[:digit:]]+"#;
    let re_dnm_string = r#"[a-zA-Z][a-zA-Z0-9/]{2,127}"#;
    let decimal_regex = Regex::new(re_dec_amt).expect("Invalid Decimal Regex");
    let denom_regex = Regex::new(re_dnm_string).expect("Invalid Denom Regex");
    let amount = decimal_regex
        .find(s)
        .expect("Could not find amount in coin string");
    let denom = denom_regex
        .find(s)
        .expect("Could not find denom in coin string");
    (amount.as_str().to_string(), denom.as_str().to_string())
}

fn get_cosmos_key(_key_name: &str) -> CosmosPrivateKey {
    unimplemented!()
}

impl Runnable for SendToEth {
    fn run(&self) {
        let from_cosmos_key = self.cosmos_key.clone();
        let to_eth_addr = self.eth_addr.clone(); //TODO parse this to an Eth Address
        let erc_20_coin = self.erc20_amount.clone(); // 1231234uatom
        let (amount, denom) = parse_denom(&erc_20_coin);

        let amount: Uint256 = amount.parse().expect("Could not parse amount");

        let cosmos_key = get_cosmos_key(&from_cosmos_key);

        // TODO(bolten): I guess this command doesn't work yet? I hope no one is trying to
        // call it
        let cosmos_address = cosmos_key.to_address("//TODO add to config file").unwrap();

        println!("Sending from Cosmos address {}", cosmos_address);
        let config = APP.config();
        let cosmos_prefix = config.cosmos.prefix.clone();
        let cosmso_grpc = config.cosmos.grpc.clone();

        abscissa_tokio::run_with_actix(&APP, async {
            let connections =
                create_rpc_connections(cosmos_prefix, Some(cosmso_grpc), None, TIMEOUT).await;
            let contact = connections.contact.unwrap();
            let mut grpc = connections.grpc.unwrap();

            let res = grpc
                .denom_to_erc20(DenomToErc20Request{
                    denom: denom.clone(),
                })
                .await;
                match res {
                    Ok(val) => println!(
                        "Asset {} has ERC20 representation {}",
                        denom,
                        val.into_inner().erc20
                    ),
                    Err(_e) => {
                        println!(
                            "Asset {} has no ERC20 representation, you may need to deploy an ERC20 for it!",
                            denom.clone()
                        );
                        exit(1);
                    } }
                    let amount = Coin {
                        amount: amount.clone(),
                        denom: denom.clone(),
                    };
                    let bridge_fee = Coin {
                        denom: denom.clone(),
                        amount: 1u64.into(),
                    };
                    let eth_dest: EthAddress = to_eth_addr.parse().unwrap();
                    check_for_fee_denom(&denom, cosmos_address, &contact).await;

                    let balances = contact
            .get_balances(cosmos_address)
            .await
            .expect("Failed to get balances!");
        let mut found = None;
        for coin in balances.iter() {
            if coin.denom == denom {
                found = Some(coin);
            }
        }
        println!("Cosmos balances {:?}", balances);

        if found.is_none() {
            panic!("You don't have any {} tokens!", denom);}
            println!(
                "Locking {:?} / {} into the batch pool",
                amount,
                denom
            );
            let res = send_to_eth(
                cosmos_key,
                eth_dest,
                amount.clone(),
                bridge_fee.clone(),
                config.cosmos.gas_price.as_tuple(),
                &contact,
                1.0
            )
            .await;
            match res {
                Ok(tx_id) => println!("Send to Eth txid {}", tx_id.txhash),
                Err(e) => println!("Failed to send tokens! {:?}", e),
            }
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            exit(1);
        });
    }
}

/// Send tx from across the Cosmos chain
#[derive(Command, Debug, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Send transactions from across the Cosmos chain.\n This command sends tx from across the Cosmos chain.\n It takes the sender's keyname, reciever's address and amount to be sent."
)]
pub struct Send {
    /// Sender's key name
    key_name: String,

    /// Reciever's address
    to_addr: String,

    /// amount
    amount: String,

    #[clap(short, long)]
    help: bool,
}

impl Runnable for Send {
    /// Start the application.
    fn run(&self) {
        let _from_key = self.key_name.clone();
        let _to_addr = self.to_addr.clone();
        let _coin_amount = self.amount.clone();

        abscissa_tokio::run_with_actix(&APP, async { unimplemented!() }).unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            exit(1);
        });
    }
}
