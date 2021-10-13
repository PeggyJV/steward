use super::show::ShowKeyCmd;
use crate::application::APP;
use abscissa_core::{Application, Command, Clap, Runnable};
use std::path::Path;

#[derive(Command, Debug, Default, Clap)]
pub struct ListKeyCmd {}

// Entry point for `keys list`
impl Runnable for ListKeyCmd {
    fn run(&self) {
        let config = APP.config();
        let keystore = Path::new(&config.keys.keystore);

        for entry in keystore.read_dir().expect("Could not read keystore") {
            let path = entry.unwrap().path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "pem" {
                        let name = path.file_stem().unwrap();
                        let name = name.to_str().unwrap();
                        let args = vec![name.to_string()];
                        let show_cmd = ShowKeyCmd { args };
                        show_cmd.run();
                    }
                }
            }
        }
    }
}