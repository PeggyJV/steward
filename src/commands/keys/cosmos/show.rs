use crate::application::APP;
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use deep_space::PrivateKey;

#[derive(Command, Debug, Default, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nShow details of a Cosmos key in the keystore.\n This command shows details of a Cosmos key in the keystore, it takes the name of the key."
)]
pub struct ShowCosmosKeyCmd {
    /// Cosmos key name
    pub name: String,
}

// Entry point for `gorc keys cosmos show [name]`
impl Runnable for ShowCosmosKeyCmd {
    fn run(&self) {
        let config = APP.config();
        let name = self.name.clone();
        let key = config.load_deep_space_key(name.clone());

        let address = key
            .to_address(config.cosmos.prefix.trim())
            .expect("Could not generate public key");

        println!("{}\t{}", name, address)
    }
}
