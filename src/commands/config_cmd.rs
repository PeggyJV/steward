use abscissa_core::{clap::Parser, Command, Runnable};

#[derive(Command, Debug, Default, Parser)]
pub struct ConfigCmd {}

impl Runnable for ConfigCmd {
    fn run(&self) {
        let config = crate::config::StewardConfig::default();
        print!("{}", toml::to_string(&config).unwrap());
    }
}
