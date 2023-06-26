use super::show::ShowKeyCmd;
use crate::application::APP;
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use std::path::Path;

#[derive(Command, Debug, Default, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nList all Eth keys in keystore.\n This command lists all Eth keys and their addresses from the keystore."
)]
pub struct ListKeyCmd {}

// Entry point for `keys list`
impl Runnable for ListKeyCmd {
    fn run(&self) {
        let config = APP.config();
        let keystore = Path::new(&config.keystore);

        for entry in keystore.read_dir().expect("Could not read keystore") {
            let path = entry.unwrap().path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "pem" {
                        let name = path.file_stem().unwrap();
                        let name = name.to_str().unwrap();
                        let name = name.to_string();
                        let show_cmd = ShowKeyCmd { name };
                        show_cmd.run();
                    }
                }
            }
        }
    }
}
