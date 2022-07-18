use super::show::ShowKeyCmd;
use crate::application::APP;
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use k256::pkcs8::ToPrivateKey;
use signatory::FsKeyStore;
use std::path;

#[derive(Command, Debug, Default, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Import an external Eth key via private key.\n This command will recover a Eth key, storing it in the keystore. \n It takes a keyname and private key."
)]

pub struct ImportFromKeyCmd {
    /// Eth keyname.
    pub name: String,

    /// Overwrite key with the same name in the keystore when set to true. Takes a Boolean.
    #[clap(short, long)]
    pub overwrite: bool,

    /// private-key optional. When absent you'll be prompted to enter it.
    pub key: Option<String>,
}

// Entry point for `keys eth import [name] (private-key)`
// - [name] required; keyname
// - (private-key) optional; when absent the user will be prompted to enter it
impl Runnable for ImportFromKeyCmd {
    fn run(&self) {
        let config = APP.config();
        let keystore = path::Path::new(&config.keystore);
        let keystore = FsKeyStore::create_or_open(keystore).expect("Could not open keystore");

        let name = self.name.parse().expect("Could not parse name");
        if let Ok(_info) = keystore.info(&name) {
            if !self.overwrite {
                eprintln!("Key already exists, exiting.");
                return;
            }
        }

        let private_key = match self.key.clone() {
            Some(private_key) => private_key,
            None => rpassword::read_password_from_tty(Some("> Enter your private-key:\n"))
                .expect("Could not read private-key"),
        };

        let key: ethers::types::H256 = private_key.parse().expect("Could not parse private-key");

        let key = k256::SecretKey::from_bytes(key).expect("Could not make private key");
        let key = key
            .to_pkcs8_der()
            .expect("Could not PKCS8 encode private key");

        keystore.store(&name, &key).expect("Could not store key");

        let name = name.to_string();
        let show_cmd = ShowKeyCmd { name };
        show_cmd.run();
    }
}
