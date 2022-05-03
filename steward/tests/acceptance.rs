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
    fs,
    io::{self},
};
use steward::config::{KeysConfig, StewardConfig};
use tempdir::TempDir;

/// Executes your application binary via `cargo run`.
///
/// Storing this value as a [`Lazy`] static ensures that all instances of
/// the runner acquire a mutex when executing commands and inspecting
/// exit statuses, serializing what would otherwise be multithreaded
/// invocations as `cargo test` executes tests in parallel by default.

pub const PRIVATE_KEY: &str = "-----BEGIN PRIVATE KEY-----\n\
MIGEAgEAMBAGByqGSM49AgEGBSuBBAAKBG0wawIBAQQg4t6AYvfQgwhpq2YAUpG8\n\
qK43zP8REoo0Ppd9CjN/3rGhRANCAATIY9rZnmtgdkKI5+amFGsorum2Lm8fvHMU\n\
iCY6boIqnpNo1CR+My92ra3DtCw3O27u5m+IClq7wLwM4YlnLjJg\n\
-----END PRIVATE KEY-----";

/// Eth add key test
#[test]
fn eth_keys_add() -> io::Result<()> {
    let mut runner: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());
    let key_dir = TempDir::new("test_key")?;

    let keystore_dir_path = key_dir.path().join("keystore");
    let keystore_dir_string = keystore_dir_path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    fs::create_dir(keystore_dir_path.clone()).expect("could not create keystore dir");

    let key_file_path = keystore_dir_path.join("sha.pem");

    let config = StewardConfig {
        keys: KeysConfig {
            delegate_key: "cellar".to_string(),
        },
        keystore: keystore_dir_string.clone(),
        ..Default::default()
    };

    let config_file_path = key_dir.path().join("config.toml");
    let config_string = toml::to_string(&config).expect("could not write config to TOML string");
    fs::write(config_file_path.clone(), config_string).expect("could not write config file");

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

    Ok(())
}

/// Eth delete key test
#[test]
fn eth_keys_delete() -> io::Result<()> {
    let mut runner: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());
    let key_dir = TempDir::new("test_key")?;

    let keystore_dir_path = key_dir.path().join("keystore");
    let keystore_dir_string = keystore_dir_path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    fs::create_dir(keystore_dir_path.clone()).expect("could not create keystore dir");

    let key_file_path = keystore_dir_path.join("ethkey.pem");
    fs::write(key_file_path.clone(), PRIVATE_KEY).expect("could not write key file");

    let config = StewardConfig {
        keys: KeysConfig {
            delegate_key: "cellar".to_string(),
        },
        keystore: keystore_dir_string.clone(),
        ..Default::default()
    };

    let config_file_path = key_dir.path().join("config.toml");
    let config_string = toml::to_string(&config).expect("could not write config to TOML string");
    fs::write(config_file_path.clone(), config_string).expect("could not write config file");

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

    Ok(())
}

/// Eth rename key test
#[test]
fn eth_keys_rename() -> io::Result<()> {
    let mut runner: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());
    let key_dir = TempDir::new("test_key")?;

    let keystore_dir_path = key_dir.path().join("keystore");
    let keystore_dir_string = keystore_dir_path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    fs::create_dir(keystore_dir_path.clone()).expect("could not create keystore dir");

    let key_file_path = keystore_dir_path.join("ethkey.pem");
    let new_key_file_path = keystore_dir_path.join("newethkey.pem");
    fs::write(key_file_path.clone(), PRIVATE_KEY).expect("could not write key file");

    let config = StewardConfig {
        keys: KeysConfig {
            delegate_key: "cellar".to_string(),
        },
        keystore: keystore_dir_string.clone(),
        ..Default::default()
    };

    let config_file_path = key_dir.path().join("config.toml");
    let config_string = toml::to_string(&config).expect("could not write config to TOML string");
    fs::write(config_file_path.clone(), config_string).expect("could not write config file");

    let cmd = runner
        .args(&[
            "-c",
            config_file_path.to_str().unwrap(),
            "keys",
            "eth",
            "rename",
            "ethkey",
            "newethkey",
        ])
        .capture_stdout()
        .run();
    //check that command executes without error
    cmd.wait().unwrap().expect_success();

    assert!(new_key_file_path.exists());
    assert_eq!(key_file_path.exists(), false);

    Ok(())
}

/// Eth list keys test
#[test]
fn eth_keys_list() -> io::Result<()> {
    let mut runner: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());
    let key_dir = TempDir::new("test_key")?;

    let keystore_dir_path = key_dir.path().join("keystore");
    let keystore_dir_string = keystore_dir_path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    fs::create_dir(keystore_dir_path.clone()).expect("could not create keystore dir");

    let key_file_path = keystore_dir_path.join("ethkey.pem");
    fs::write(key_file_path, PRIVATE_KEY).expect("could not write key file");

    let config = StewardConfig {
        keys: KeysConfig {
            delegate_key: "cellar".to_string(),
        },
        keystore: keystore_dir_string.clone(),
        ..Default::default()
    };

    let config_file_path = key_dir.path().join("config.toml");
    let config_string = toml::to_string(&config).expect("could not write config to TOML string");
    fs::write(config_file_path.clone(), config_string).expect("could not write config file");

    let mut cmd = runner
        .args(&[
            "-c",
            config_file_path.to_str().unwrap(),
            "keys",
            "eth",
            "list",
        ])
        .capture_stdout()
        .run();

    // Check that the list keys list keys in the keystore we created.
    cmd.stdout()
        .expect_line("ethkey\t0xfa06d54153d56cd7c8fe2141c2bc2e1d43a08286");

    Ok(())
}

/// Eth show key test
#[test]
fn eth_keys_show() -> io::Result<()> {
    let mut runner: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());
    let key_dir = TempDir::new("test_key")?;

    let keystore_dir_path = key_dir.path().join("keystore");
    let keystore_dir_string = keystore_dir_path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    fs::create_dir(keystore_dir_path.clone()).expect("could not create keystore dir");

    let key_file_path = keystore_dir_path.join("ethkey.pem");
    fs::write(key_file_path, PRIVATE_KEY).expect("could not write key file");

    let config = StewardConfig {
        keys: KeysConfig {
            delegate_key: "cellar".to_string(),
        },
        keystore: keystore_dir_string.clone(),
        ..Default::default()
    };

    let config_file_path = key_dir.path().join("config.toml");
    let config_string = toml::to_string(&config).expect("could not write config to TOML string");
    fs::write(config_file_path.clone(), config_string).expect("could not write config file");

    let mut cmd = runner
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
    // Check that the show key command shows the key we created.
    cmd.stdout()
        .expect_line("ethkey\t0xfa06d54153d56cd7c8fe2141c2bc2e1d43a08286");

    Ok(())
}

/// Eth import key test
#[test]
fn eth_keys_import() -> io::Result<()> {
    let mut runner: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());
    let key_dir = TempDir::new("test_key")?;

    let keystore_dir_path = key_dir.path().join("keystore");
    let keystore_dir_string = keystore_dir_path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    fs::create_dir(keystore_dir_path.clone()).expect("could not create keystore dir");

    let key_file_path = keystore_dir_path.join("mykey.pem");

    let config = StewardConfig {
        keys: KeysConfig {
            delegate_key: "cellar".to_string(),
        },
        keystore: keystore_dir_string.clone(),
        ..Default::default()
    };

    let config_file_path = key_dir.path().join("config.toml");
    let config_string = toml::to_string(&config).expect("could not write config to TOML string");
    fs::write(config_file_path.clone(), config_string).expect("could not write config file");

    let cmd = runner
        .args(&[
            "-c",
            config_file_path.to_str().unwrap(),
            "keys",
            "eth",
            "import",
            "mykey",
            "movie tumble tape seven tool session relax youth pet situate bone leave ordinary oxygen silly picture thing fortune genuine attend clerk super seven cement",
        ])
        .capture_stdout()
        .run();
    // Check that command executes without error.
    cmd.wait().unwrap().expect_success();
    assert!(key_file_path.exists());

    Ok(())
}

/// Cosmos add key test
#[test]
fn cosmos_keys_add() -> io::Result<()> {
    let mut runner: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());
    let key_dir = TempDir::new("test_key")?;

    let keystore_dir_path = key_dir.path().join("keystore");
    let keystore_dir_string = keystore_dir_path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    fs::create_dir(keystore_dir_path.clone()).expect("could not create keystore dir");

    let key_file_path = keystore_dir_path.join("sha.pem");

    let config = StewardConfig {
        keys: KeysConfig {
            delegate_key: "cellar".to_string(),
        },
        keystore: keystore_dir_string.clone(),
        ..Default::default()
    };

    let config_file_path = key_dir.path().join("config.toml");
    let config_string = toml::to_string(&config).expect("could not write config to TOML string");
    fs::write(config_file_path.clone(), config_string).expect("could not write config file");

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

    Ok(())
}

/// Cosmos list key test
#[test]
fn cosmos_keys_list() -> io::Result<()> {
    let mut runner: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());
    let key_dir = TempDir::new("test_key")?;

    let keystore_dir_path = key_dir.path().join("keystore");
    let keystore_dir_string = keystore_dir_path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    fs::create_dir(keystore_dir_path.clone()).expect("could not create keystore dir");

    let key_file_path = keystore_dir_path.join("cosmoskey.pem");
    fs::write(key_file_path, PRIVATE_KEY).expect("could not write key file");

    let config = StewardConfig {
        keys: KeysConfig {
            delegate_key: "cellar".to_string(),
        },
        keystore: keystore_dir_string.clone(),
        ..Default::default()
    };

    let config_file_path = key_dir.path().join("config.toml");
    let config_string = toml::to_string(&config).expect("could not write config to TOML string");
    fs::write(config_file_path.clone(), config_string).expect("could not write config file");

    let mut cmd = runner
        .args(&[
            "-c",
            config_file_path.to_str().unwrap(),
            "keys",
            "cosmos",
            "list",
        ])
        .capture_stdout()
        .run();
    // Check that the list keys list keys in the keystore we created.
    cmd.stdout()
        .expect_line("cosmoskey\tsomm1wzp8pks7hzavh7r4dmenpszxyzfxyk34xlmcfh");

    Ok(())
}

/// Cosmos show key test
#[test]
fn cosmos_keys_show() -> io::Result<()> {
    let mut runner: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());
    let key_dir = TempDir::new("test_key")?;

    let keystore_dir_path = key_dir.path().join("keystore");
    let keystore_dir_string = keystore_dir_path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    fs::create_dir(keystore_dir_path.clone()).expect("could not create keystore dir");

    let key_file_path = keystore_dir_path.join("cosmoskey.pem");
    fs::write(key_file_path, PRIVATE_KEY).expect("could not write key file");

    let config = StewardConfig {
        keys: KeysConfig {
            delegate_key: "cellar".to_string(),
        },
        keystore: keystore_dir_string.clone(),
        ..Default::default()
    };

    let config_file_path = key_dir.path().join("config.toml");
    let config_string = toml::to_string(&config).expect("could not write config to TOML string");
    fs::write(config_file_path.clone(), config_string).expect("could not write config file");

    let mut cmd = runner
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

    cmd.stdout()
        .expect_line("cosmoskey\tsomm1wzp8pks7hzavh7r4dmenpszxyzfxyk34xlmcfh");

    Ok(())
}

/// Cosmos delete key test
#[test]
fn cosmos_keys_delete() -> io::Result<()> {
    let mut runner: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());
    let key_dir = TempDir::new("test_key")?;

    let keystore_dir_path = key_dir.path().join("keystore");
    let keystore_dir_string = keystore_dir_path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    fs::create_dir(keystore_dir_path.clone()).expect("could not create keystore dir");

    let key_file_path = keystore_dir_path.join("cosmoskey.pem");
    fs::write(key_file_path.clone(), PRIVATE_KEY).expect("could not write key file");

    let config = StewardConfig {
        keys: KeysConfig {
            delegate_key: "cellar".to_string(),
        },
        keystore: keystore_dir_string.clone(),
        ..Default::default()
    };

    let config_file_path = key_dir.path().join("config.toml");
    let config_string = toml::to_string(&config).expect("could not write config to TOML string");
    fs::write(config_file_path.clone(), config_string).expect("could not write config file");

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

    Ok(())
}

/// Cosmos recover key test
#[test]
fn cosmos_keys_recover() -> io::Result<()> {
    let mut runner: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());
    let key_dir = TempDir::new("test_key")?;

    let keystore_dir_path = key_dir.path().join("keystore");
    let keystore_dir_string = keystore_dir_path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    fs::create_dir(keystore_dir_path.clone()).expect("could not create keystore dir");

    let key_file_path = keystore_dir_path.join("cosmkey.pem");

    let config = StewardConfig {
        keys: KeysConfig {
            delegate_key: "cellar".to_string(),
        },
        keystore: keystore_dir_string.clone(),
        ..Default::default()
    };

    let config_file_path = key_dir.path().join("config.toml");
    let config_string = toml::to_string(&config).expect("could not write config to TOML string");
    fs::write(config_file_path.clone(), config_string).expect("could not write config file");

    let cmd = runner
        .args(&[
            "-c",
            config_file_path.to_str().unwrap(),
            "keys",
            "cosmos",
            "recover",
            "cosmkey",
            "merry blush enforce kite tired cheap use learn there oyster possible thank woman web midnight december sentence ski throw father scheme element onion talent"
        ])
        .capture_stdout()
        .run();
    // Check that command executes without error.
    cmd.wait().unwrap().expect_success();
    assert!(key_file_path.exists());

    Ok(())
}

/// Cosmos rename key test
#[test]
fn cosmos_keys_rename() -> io::Result<()> {
    let mut runner: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());
    let key_dir = TempDir::new("test_key")?;

    let keystore_dir_path = key_dir.path().join("keystore");
    let keystore_dir_string = keystore_dir_path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    fs::create_dir(keystore_dir_path.clone()).expect("could not create keystore dir");

    let key_file_path = keystore_dir_path.join("cosmoskey.pem");
    let new_key_file_path = keystore_dir_path.join("newcosmoskey.pem");
    fs::write(key_file_path.clone(), PRIVATE_KEY).expect("could not write key file");

    let config = StewardConfig {
        keys: KeysConfig {
            delegate_key: "cellar".to_string(),
        },
        keystore: keystore_dir_string.clone(),
        ..Default::default()
    };

    let config_file_path = key_dir.path().join("config.toml");
    let config_string = toml::to_string(&config).expect("could not write config to TOML string");
    fs::write(config_file_path.clone(), config_string).expect("could not write config file");

    let cmd = runner
        .args(&[
            "-c",
            config_file_path.to_str().unwrap(),
            "keys",
            "cosmos",
            "rename",
            "cosmoskey",
            "newcosmoskey",
        ])
        .capture_stdout()
        .run();
    //check that command executes without error
    cmd.wait().unwrap().expect_success();

    assert!(new_key_file_path.exists());
    assert_eq!(key_file_path.exists(), false);

    Ok(())
}

/// print_config test
#[test]
fn print_config() {
    let mut runner: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());
    let cmd = runner.args(&["print-config"]).capture_stdout().run();
    //check that command executes without error
    cmd.wait().unwrap().expect_success();
}

/// Test config file order
#[test]
fn test_configfile_order() -> io::Result<()> {
    let mut runner: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());
    let key_dir = TempDir::new("test_key")?;

    let keystore_dir_path = key_dir.path().join("keystore");
    let env_keystore_dir_path = key_dir.path().join("envkeystore");
    let keystore_dir_string = keystore_dir_path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    fs::create_dir(keystore_dir_path.clone()).expect("could not create keystore dir");
    let env_keystore_dir_string = env_keystore_dir_path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    fs::create_dir(env_keystore_dir_path.clone()).expect("could not create keystore dir");

    let config = StewardConfig {
        keys: KeysConfig {
            delegate_key: "cellar".to_string(),
        },
        keystore: keystore_dir_string.clone(),
        ..Default::default()
    };
    let key_file_path = keystore_dir_path.join("sha.pem");

    let env_config = StewardConfig {
        keys: KeysConfig {
            delegate_key: "cellar".to_string(),
        },
        keystore: env_keystore_dir_string.clone(),
        ..Default::default()
    };
    let env_key_file_path = env_keystore_dir_path.join("sha.pem");

    let config_file_path = key_dir.path().join("config.toml");
    let config_string = toml::to_string(&config).expect("could not write config to TOML string");
    fs::write(config_file_path.clone(), config_string).expect("could not write config file");

    let env_config_file_path = key_dir.path().join("config.toml");
    let env_config_string =
        toml::to_string(&env_config).expect("could not write config to TOML string");
    fs::write(env_config_file_path.clone(), env_config_string)
        .expect("could not write config file");

    let config_file_path = key_dir.path().join("config.toml");
    let config_string = toml::to_string(&config).expect("could not write config to TOML string");
    fs::write(config_file_path.clone(), config_string).expect("could not write config file");

    std::env::set_var("STEWARD_CONFIG", env_config_file_path.to_str().unwrap());

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
    assert_eq!(env_key_file_path.exists(), false);

    Ok(())
}

/// Test Env Works Appropriately
#[test]
fn test_env() -> io::Result<()> {
    let mut runner: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());
    let key_dir = TempDir::new("test_key")?;

    let keystore_dir_path = key_dir.path().join("keystore");
    let keystore_dir_string = keystore_dir_path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    fs::create_dir(keystore_dir_path.clone()).expect("could not create keystore dir");

    let key_file_path = keystore_dir_path.join("sha.pem");

    let config = StewardConfig {
        keys: KeysConfig {
            delegate_key: "cellar".to_string(),
        },
        keystore: keystore_dir_string.clone(),
        ..Default::default()
    };

    let config_file_path = key_dir.path().join("config.toml");
    let config_string = toml::to_string(&config).expect("could not write config to TOML string");
    fs::write(config_file_path.clone(), config_string).expect("could not write config file");

    std::env::set_var("STEWARD_CONFIG", config_file_path.to_str().unwrap());

    let cmd = runner
        .args(&["keys", "cosmos", "add", "sha"])
        .capture_stdout()
        .run();
    // Check that command executes without error.
    cmd.wait().unwrap().expect_success();
    assert!(key_file_path.exists());

    Ok(())
}
