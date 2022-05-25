use super::show::ShowKeyCmd;
use crate::application::APP;
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use k256::{pkcs8::ToPrivateKey, SecretKey};
use signatory::FsKeyStore;
use std::path;

#[derive(Command, Debug, Default, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Import an external Eth key.\n This command will recover a Eth key, storing it in the keystore. \n It takes a keyname and bip39-mnemonic."
)]
pub struct ImportEthKeyCmd {
    /// Eth keyname.
    pub name: String,

    /// Overwrite key with the same name in the keystore when set to true. Takes a Boolean.
    #[clap(short, long)]
    pub overwrite: bool,

    /// bip39-mnemonic optional. When absent you'll be prompted to enter it.
    pub key: Option<String>,
}

// Entry point for `keys eth import [name] (bip39-mnemonic)`
// - [name] required; keyname
// - (bip39-mnemonic) optional; when absent the user will be prompted to enter it
impl Runnable for ImportEthKeyCmd {
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

        let key = match self.key.clone() {
            Some(private_key) => private_key,
            None => rpassword::read_password_from_tty(Some("> Enter your private-key:\n"))
                .expect("Could not read private-key"),
        };

        let key = key
            .parse::<clarity::PrivateKey>()
            .expect("Could not parse private-key");

        let key = SecretKey::from_bytes(key.to_bytes()).expect("Could not convert private-key");

        let key = key
            .to_pkcs8_der()
            .expect("Could not PKCS8 encod private key");

        keystore.store(&name, &key).expect("Could not store key");

        let name = name.to_string();
        let show_cmd = ShowKeyCmd { name };
        show_cmd.run();
    }
}
