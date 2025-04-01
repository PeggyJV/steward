use crate::{application::APP, prelude::*};
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use ethers::signers::Signer;
use gravity_bridge::gravity_proto::gravity as proto;
use rustls::crypto::CryptoProvider;
use std::time::Duration;

/// Sign delegate keys
#[derive(Command, Debug, Default, Parser)]
pub struct SignDelegateKeysCmd {
    /// Ethereum keyname
    ethereum_key: String,

    /// Validator address
    val_address: String,

    /// nonce
    nonce: Option<u64>,
}

impl Runnable for SignDelegateKeysCmd {
    fn run(&self) {
        let config = APP.config();
        rustls::crypto::ring::default_provider().install_default();
        abscissa_tokio::run_with_actix(&APP, async {
            let signer = config
                .load_ethers_signer(self.ethereum_key.clone())
                .await
                .expect("Failed to load signer");

            println!("signer: {:#x}", signer.address());

            let clarity_signer = config.load_clarity_key(self.ethereum_key.clone());
            let local_wallet = config.load_ethers_wallet(self.ethereum_key.clone());
            // let address = self.val_address.parse().expect("Could not parse address");

            // let nonce: u64 = match self.nonce {
            //     Some(nonce) => nonce,
            //     None => {
            //         let timeout = Duration::from_secs(10);
            //         let contact = deep_space::Contact::new(
            //             &config.cosmos.grpc,
            //             timeout,
            //             &config.cosmos.prefix,
            //         )
            //         .expect("Could not create contact");

            //         let account_info = contact.get_account_info(address).await;
            //         let account_info = account_info.expect("Did not receive account info");
            //         account_info.sequence
            //     }
            // };

            let nonce = 1;

            println!("nonce: {}", nonce);

            let msg = proto::DelegateKeysSignMsg {
                validator_address: self.val_address.clone(),
                nonce,
            };

            let size = prost::Message::encoded_len(&msg);
            let mut buf = bytes::BytesMut::with_capacity(size);
            prost::Message::encode(&msg, &mut buf).expect("Failed to encode DelegateKeysSignMsg!");


            let signature = signer
                .sign_message(&buf)
                .await
                .expect("Failed to sign message");

            let local_signature = local_wallet.sign_message(&buf).await.expect("Failed to sign message");
            let clarity_signature = clarity_signer.sign_ethereum_msg(&buf);

            println!("SignerType (ethers) signature: {}", signature);
            println!("LocalWallet (ethers) signature: {}", local_signature);
            println!("clarity::PrivateKey signature: {}", clarity_signature);
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
