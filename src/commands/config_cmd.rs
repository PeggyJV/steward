use abscissa_core::{Command, Options, Runnable};

#[derive(Command, Debug, Default, Options)]
pub struct ConfigCmd {}

impl Runnable for ConfigCmd {
    fn run(&self) {
        let config = crate::config::CellarRebalancerConfig::default();
        print!("{}", toml::to_string(&config).unwrap());
    }
}
