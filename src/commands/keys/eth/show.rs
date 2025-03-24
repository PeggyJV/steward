use crate::application::APP;
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use ethers::prelude::*;
use sha3::digest::generic_array::GenericArray;
use signatory::FsKeyStore;
use std::path;

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
        let keystore = path::Path::new(&config.keystore);
        let keystore = FsKeyStore::create_or_open(keystore).expect("Could not open keystore");
        let name = self.name.parse().expect("Could not parse name");
        let key = keystore.load(&name).expect("Could not load key");
        let pem = key.to_pem();
        let key_bytes: &[u8] = pem.as_bytes();
        let key_bytes = GenericArray::from_slice(key_bytes);
        let ethers_secret = ethers::core::k256::SecretKey::from_bytes(&key_bytes).unwrap();
        let signing_key = ethers::core::k256::ecdsa::SigningKey::from(ethers_secret);
        let wallet = LocalWallet::from(signing_key);

        let address = wallet.address();

        println!("{}\t{:#x}", name, address);
    }
}
