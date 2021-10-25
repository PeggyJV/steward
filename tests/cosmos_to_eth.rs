#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]

use abscissa_core::testing::prelude::*;
use cellar_rebalancer::config::CellarRebalancerConfig;
use once_cell::sync::Lazy;

pub static RUNNER: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());


#[test]
fn cosmos_to_eth() {
    let mut runner = RUNNER.clone();
    runner
        .args(&[
            "-- -c meconfig.toml",
            "cosmos-to-eth",
        ])
        .capture_stdout()
        .status()
        .expect_success();
}