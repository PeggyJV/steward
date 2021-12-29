use abscissa_core::{Clap, Command, Runnable};

#[derive(Command, Debug, Default, Clap)]
pub struct ConfigCmd {}

impl Runnable for ConfigCmd {
    fn run(&self) {
        let config = crate::config::CellarRebalancerConfig::default();
        print!("{}", toml::to_string(&config).unwrap());
    }
}
