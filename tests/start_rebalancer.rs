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
fn start_rebalancer() {
    let mut runner = RUNNER.clone();
    runner
        .args(&[
            "-- -c meconfig.toml",
            "start",
        ])
        .capture_stdout()
        .status()
        .expect_success();
}