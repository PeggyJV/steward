use crate::config::CellarRebalancerConfig;
use crate::{application::APP, prelude::*};
use abscissa_core::{Command, Clap, Runnable};

/// Print default configurations
#[derive(Command, Debug, Default, Clap)]
pub struct PrintConfigCmd {
    #[clap(short, long)]
    show_default: bool,
}

impl Runnable for PrintConfigCmd {
    fn run(&self) {
        let config = if self.show_default {
            CellarRebalancerConfig::default()
        } else {
            let config = APP.config();
            CellarRebalancerConfig {
                cellars: config.cellars.to_owned(),
                ethereum: config.ethereum.to_owned(),
                keys: config.keys.to_owned(),
                mongo: config.mongo.to_owned(),
                keystore: config.keystore.to_owned(),
                gravity: config.gravity.to_owned(),
                cosmos: config.cosmos.to_owned(),
                metrics: config.metrics.to_owned(),
            }
        };

        print!("{}", toml::to_string(&config).unwrap());
    }
}