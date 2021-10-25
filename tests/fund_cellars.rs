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
fn fund_cellar() {
    let mut runner = RUNNER.clone();
    runner
        .args(&[
            "tests/meconfig.toml",
            "fund-cellar",
            "--cellar-id 0",
            "--amount-0 10",
            "--amount-1 10",
        ])
        .capture_stdout()
        .status()
        .expect_success();
}