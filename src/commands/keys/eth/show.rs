use std::{fs, path};

use abscissa_core::{clap::Parser, Application, Command, Runnable};
use ethers::signers::{LocalWallet, Signer};
use k256::pkcs8::FromPrivateKey;
use k256::SecretKey;

use crate::application::APP;

#[derive(Command, Debug, Default, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nShow details of an Eth key in the keystore.\n This command shows details of an Eth key in the keystore, it takes the name of the key."
)]
pub struct ShowKeyCmd {
    /// Eth keyname
    pub name: String,
}

// Entry point for `contract monitor keys show [name]`
impl Runnable for ShowKeyCmd {
    fn run(&self) {
        let config = APP.config();
        let key_file_name = format!("{}.pem", self.name);
        let key_path = path::Path::new(&config.keystore).join(&key_file_name);
        let pem = fs::read_to_string(&key_path).unwrap();
        let secret_key = SecretKey::from_pkcs8_pem(&pem).expect("Could not parse PEM");
        let wallet: LocalWallet =
            LocalWallet::from_bytes(&secret_key.to_bytes()).expect("Could not create wallet");
        let address = wallet.address();

        println!("{}\t{:#x}", self.name, address);
    }
}
