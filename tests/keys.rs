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
fn add_keys() {
    let mut runner = RUNNER.clone();
    runner
        .args(&[
            "-- -c tests/meconfig.toml",
            "keys",
            "add",
            "test1",
        ])
        .capture_stdout()
        .status()
        .expect_success();
}

#[test]
fn delete_keys() {
    let mut runner = RUNNER.clone();
    runner
        .args(&[
            "-- -c tests/meconfig.toml",
            "keys",
            "delete",
            "test1",
        ])
        .capture_stdout()
        .status()
        .expect_success();
}

#[test]
fn import_keys() {
    let mut runner = RUNNER.clone();
    runner
        .args(&[
            "-- -c tests/meconfig.toml",
            "keys",
            "import",
            "test2",
        ])
        .capture_stdout()
        .status()
        .expect_success();
}

#[test]
fn list_keys() {
    let mut runner = RUNNER.clone();
    runner
        .args(&[
            "-- -c tests/meconfig.toml",
            "keys",
            "list",
        ])
        .capture_stdout()
        .status()
        .expect_success();
}

#[test]
fn show_keys() {
    let mut runner = RUNNER.clone();
    runner
        .args(&[
            "-- -c tests/meconfig.toml",
            "keys",
            "show",
            "test2",
        ])
        .capture_stdout()
        .status()
        .expect_success();
}