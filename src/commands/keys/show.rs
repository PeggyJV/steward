use crate::application::APP;
use abscissa_core::{Application, Command, Clap, Runnable};
use ethers::prelude::*;
use signatory::FsKeyStore;
use std::path;

#[derive(Command, Debug, Default, Clap)]
pub struct ShowKeyCmd {
    #[clap()]
    pub args: Vec<String>,
}

// Entry point for `contract monitor keys show [name]`
impl Runnable for ShowKeyCmd {
    fn run(&self) {
        let config = APP.config();
        let keystore = path::Path::new(&config.keys.keystore);
        let keystore = FsKeyStore::create_or_open(keystore).expect("Could not open keystore");

        let name = self.args.get(0).expect("name is required");
        let name = name.parse().expect("Could not parse name");

        let key = keystore.load(&name).expect("Could not load key");

        let key = key
            .to_pem()
            .parse::<k256::elliptic_curve::SecretKey<k256::Secp256k1>>()
            .expect("Could not parse key");

        let wallet: LocalWallet = Wallet::from(key);

        let address = wallet.address();

        println!("{}\t{:#x}", name, address);
    }
}