//! Acceptance test: runs the application as a subprocess and asserts its
//! output for given argument combinations matches what is expected.
//!
//! Modify and/or delete these as you see fit to test the specific needs of
//! your application.
//!
//! For more information, see:
//! <https://docs.rs/abscissa_core/latest/abscissa_core/testing/index.html>

// Tip: Deny warnings with `RUSTFLAGS="-D warnings"` environment variable in CI

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]

use abscissa_core::testing::prelude::*;
use once_cell::sync::Lazy;
use std::{
    fs::File,
    io::{self, Write},
};
use steward::config::{KeysConfig, StewardConfig};
use tempdir::TempDir;

/// Executes your application binary via `cargo run`.
///
/// Storing this value as a [`Lazy`] static ensures that all instances of
/// the runner acquire a mutex when executing commands and inspecting
/// exit statuses, serializing what would otherwise be multithreaded
/// invocations as `cargo test` executes tests in parallel by default.
pub static RUNNER: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());

pub const PRIVATE_KEY: &str = "-----BEGIN PRIVATE KEY-----\n\
MIGEAgEAMBAGByqGSM49AgEGBSuBBAAKBG0wawIBAQQg4t6AYvfQgwhpq2YAUpG8\n\
qK43zP8REoo0Ppd9CjN/3rGhRANCAATIY9rZnmtgdkKI5+amFGsorum2Lm8fvHMU\n\
iCY6boIqnpNo1CR+My92ra3DtCw3O27u5m+IClq7wLwM4YlnLjJg\n\
-----END PRIVATE KEY-----";

/// Use command-line argument value
#[test]
fn eth_keys_add() -> io::Result<()> {
    let key_dir = TempDir::new("test_key")?;
    let key_file_path = key_dir.path().join("sha.pem");
    let config_file_path = key_dir.path().join("config.toml");
    let mut f = File::create(config_file_path.clone())?;
    let configu = StewardConfig {
        keys: KeysConfig {
            keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
            rebalancer_key: "cellar".to_string(),
        },
        keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
        ..Default::default()
    };
    f.write_all(toml::to_string(&configu).unwrap().as_bytes())?;
    let mut runner = RUNNER.clone();
    let cmd = runner
        .args(&[
            "-c",
            config_file_path.to_str().unwrap(),
            "keys",
            "eth",
            "add",
            "sha",
        ])
        .capture_stdout()
        .run();
    // Check that command executes without error.
    cmd.wait().unwrap().expect_success();
    assert!(key_file_path.exists());
    f.sync_all()?;

    Ok(())
}

#[test]
fn eth_keys_delete() -> io::Result<()> {
    let key_dir = TempDir::new("test_key")?;
    let key_file_path = key_dir.path().join("ethkey.pem");
    let mut f = File::create(key_file_path.clone())?;
    f.write_all(PRIVATE_KEY.as_bytes())?;
    let config_file_path = key_dir.path().join("config.toml");
    let mut f = File::create(config_file_path.clone())?;
    let configu = StewardConfig {
        keys: KeysConfig {
            keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
            rebalancer_key: "cellar".to_string(),
        },
        keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
        ..Default::default()
    };
    f.write_all(toml::to_string(&configu).unwrap().as_bytes())?;

    let mut runner = RUNNER.clone();
    let cmd = runner
        .args(&[
            "-c",
            config_file_path.to_str().unwrap(),
            "keys",
            "eth",
            "delete",
            "ethkey",
        ])
        .capture_stdout()
        .run();
    //check that command executes without error
    cmd.wait().unwrap().expect_success();

    assert_eq!(key_file_path.exists(), false);
    f.sync_all()?;

    Ok(())
}

/// Use command-line argument value for eth keys list
#[test]
fn eth_keys_list() -> io::Result<()> {
    let key_dir = TempDir::new("test_key")?;
    let key_file_path = key_dir.path().join("ethkey");
    let mut f = File::create(key_file_path.clone())?;
    f.write_all(PRIVATE_KEY.as_bytes())?;
    let config_file_path = key_dir.path().join("config.toml");
    let mut f = File::create(config_file_path.clone())?;
    let configu = StewardConfig {
        keys: KeysConfig {
            keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
            rebalancer_key: "cellar".to_string(),
        },
        keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
        ..Default::default()
    };
    f.write_all(toml::to_string(&configu).unwrap().as_bytes())?;
    let mut runner = RUNNER.clone();
    let cmd = runner
        .args(&[
            "-c",
            config_file_path.to_str().unwrap(),
            "keys",
            "eth",
            "list",
        ])
        .capture_stdout()
        .run();

    cmd.wait().unwrap().expect_success();

    f.sync_all()?;
    Ok(())
}
#[test]
fn eth_keys_show() -> io::Result<()> {
    let key_dir = TempDir::new("test_key")?;
    let key_file_path = key_dir.path().join("ethkey.pem");
    let mut f = File::create(key_file_path.clone())?;
    f.write_all(PRIVATE_KEY.as_bytes())?;
    let config_file_path = key_dir.path().join("config.toml");
    let mut f = File::create(config_file_path.clone())?;
    let configu = StewardConfig {
        keys: KeysConfig {
            keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
            rebalancer_key: "cellar".to_string(),
        },
        keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
        ..Default::default()
    };
    f.write_all(toml::to_string(&configu).unwrap().as_bytes())?;
    let mut runner = RUNNER.clone();
    let cmd = runner
        .args(&[
            "-c",
            config_file_path.to_str().unwrap(),
            "keys",
            "eth",
            "show",
            "ethkey",
        ])
        .capture_stdout()
        .run();
    //check that command executes without error
    cmd.wait().unwrap().expect_success();

    f.sync_all()?;

    Ok(())
}
///use command-line argument value
#[test]
fn cosmos_keys_add() -> io::Result<()> {
    let key_dir = TempDir::new("test_key")?;
    let key_file_path = key_dir.path().join("sha.pem");
    let config_file_path = key_dir.path().join("config.toml");
    let mut f = File::create(config_file_path.clone())?;
    let configu = StewardConfig {
        keys: KeysConfig {
            keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
            rebalancer_key: "cellar".to_string(),
        },
        keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
        ..Default::default()
    };
    f.write_all(toml::to_string(&configu).unwrap().as_bytes())?;
    let mut runner = RUNNER.clone();
    let cmd = runner
        .args(&[
            "-c",
            config_file_path.to_str().unwrap(),
            "keys",
            "cosmos",
            "add",
            "sha",
        ])
        .capture_stdout()
        .run();
    // Check that command executes without error.
    cmd.wait().unwrap().expect_success();
    assert!(key_file_path.exists());

    f.sync_all()?;

    Ok(())
}
///use command-line argument value for cosmos keys list
#[test]
fn cosmos_keys_list() -> io::Result<()> {
    let key_dir = TempDir::new("test_key")?;
    let key_file_path = key_dir.path().join("cosmoskey");
    let mut f = File::create(key_file_path.clone())?;
    f.write_all(PRIVATE_KEY.as_bytes())?;
    let config_file_path = key_dir.path().join("config.toml");
    let mut f = File::create(config_file_path.clone())?;
    let configu = StewardConfig {
        keys: KeysConfig {
            keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
            rebalancer_key: "cellar".to_string(),
        },
        keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
        ..Default::default()
    };
    f.write_all(toml::to_string(&configu).unwrap().as_bytes())?;
    let mut runner = RUNNER.clone();
    let cmd = runner
        .args(&[
            "-c",
            config_file_path.to_str().unwrap(),
            "keys",
            "cosmos",
            "list",
        ])
        .capture_stdout()
        .run();

    //check that command executes without error
    cmd.wait().unwrap().expect_success();

    f.sync_all()?;
    Ok(())
}

#[test]
fn cosmos_keys_show() -> io::Result<()> {
    let key_dir = TempDir::new("test_key")?;
    let key_file_path = key_dir.path().join("cosmoskey.pem");
    let mut f = File::create(key_file_path.clone())?;
    f.write_all(PRIVATE_KEY.as_bytes())?;
    let config_file_path = key_dir.path().join("config.toml");
    let mut f = File::create(config_file_path.clone())?;
    let configu = StewardConfig {
        keys: KeysConfig {
            keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
            rebalancer_key: "cellar".to_string(),
        },
        keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
        ..Default::default()
    };
    f.write_all(toml::to_string(&configu).unwrap().as_bytes())?;
    let mut runner = RUNNER.clone();
    let cmd = runner
        .args(&[
            "-c",
            config_file_path.to_str().unwrap(),
            "keys",
            "cosmos",
            "show",
            "cosmoskey",
        ])
        .capture_stdout()
        .run();
    //check that command executes without error
    cmd.wait().unwrap().expect_success();

    f.sync_all()?;

    Ok(())
}
///use command-line argument value for cosmos keys delete
#[test]
fn cosmos_keys_delete() -> io::Result<()> {
    let key_dir = TempDir::new("test_key")?;
    let key_file_path = key_dir.path().join("cosmoskey.pem");
    let mut f = File::create(key_file_path.clone())?;
    f.write_all(PRIVATE_KEY.as_bytes())?;
    let config_file_path = key_dir.path().join("config.toml");
    let mut f = File::create(config_file_path.clone())?;
    let configu = StewardConfig {
        keys: KeysConfig {
            keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
            rebalancer_key: "cellar".to_string(),
        },
        keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
        ..Default::default()
    };
    f.write_all(toml::to_string(&configu).unwrap().as_bytes())?;

    let mut runner = RUNNER.clone();
    let cmd = runner
        .args(&[
            "-c",
            config_file_path.to_str().unwrap(),
            "keys",
            "cosmos",
            "delete",
            "cosmoskey",
        ])
        .capture_stdout()
        .run();
    //check that command executes without error
    cmd.wait().unwrap().expect_success();

    assert_eq!(key_file_path.exists(), false);

    f.sync_all()?;

    Ok(())
}

#[test]
#[ignore]
fn deploy_erc20() -> io::Result<()> {
    let key_dir = TempDir::new("test_key")?;
    let key_file_path = key_dir.path().join("cosmoskey.pem");
    let mut f = File::create(key_file_path.clone())?;
    f.write_all(PRIVATE_KEY.as_bytes())?;
    let config_file_path = key_dir.path().join("config.toml");
    let mut f = File::create(config_file_path.clone())?;
    let configu = StewardConfig {
        keys: KeysConfig {
            keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
            rebalancer_key: "cellar".to_string(),
        },
        keystore: key_dir.path().as_os_str().to_str().unwrap().to_string(), // fix this before pushing
        ..Default::default()
    };
    f.write_all(toml::to_string(&configu).unwrap().as_bytes())?;

    let mut runner = RUNNER.clone();
    let cmd = runner
        .args(&[
            "-c",
            config_file_path.to_str().unwrap(),
            "deploy",
            "erc20",
            "--denom",
            "USDT",
            "--ethereum-key",
            "cosmoskey",
        ])
        .capture_stdout()
        .run();
    //check that command executes without error
    cmd.wait().unwrap().expect_success();

    f.sync_all()?;

    Ok(())
}

#[test]
fn print_config() {
    let mut runner = RUNNER.clone();
    let cmd = runner.args(&["print-config"]).capture_stdout().run();
    //check that command executes without error
    cmd.wait().unwrap().expect_success();
}
