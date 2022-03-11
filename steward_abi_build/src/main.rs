use ethers::contract::Abigen;
use std::process;

fn main() {
    // Aave
    let abigen = match Abigen::new("Aave", "../steward_abi/aave_lending_abi.json") {
        Ok(abigen) => abigen,
        Err(e) => {
            println!("Could not open aave_lending_abi.json: {}", e);
            process::exit(1);
        }
    };

    let abi = match abigen
        .add_event_derive("serde::Deserialize")
        .add_event_derive("serde::Serialize")
        .generate()
    {
        Ok(abi) => abi,
        Err(e) => {
            println!("Could not generate abi from aave_lending_abi.json: {}", e);
            process::exit(1);
        }
    };

    match abi.write_to_file("../steward_abi/src/aave.rs") {
        Ok(_) => (),
        Err(e) => println!("Error writing aave.rs: {}", e),
    }

    // Erc20
    let abigen = match Abigen::new("Erc20", "../steward_abi/erc20_abi.json") {
        Ok(abigen) => abigen,
        Err(e) => {
            println!("Could not open erc20_abi.json: {}", e);
            process::exit(1);
        }
    };

    let abi = match abigen
        .add_event_derive("serde::Deserialize")
        .add_event_derive("serde::Serialize")
        .generate()
    {
        Ok(abi) => abi,
        Err(e) => {
            println!("Could not generate abi from erc20.json: {}", e);
            process::exit(1);
        }
    };

    match abi.write_to_file("../steward_abi/src/erc20.rs") {
        Ok(_) => (),
        Err(e) => println!("Error writing erc20.rs: {}", e),
    }

    // Weth
    let abigen = match Abigen::new("Weth", "../steward_abi/weth_abi.json") {
        Ok(abigen) => abigen,
        Err(e) => {
            println!("Could not open weth_abi.json: {}", e);
            process::exit(1);
        }
    };

    let abi = match abigen
        .add_event_derive("serde::Deserialize")
        .add_event_derive("serde::Serialize")
        .generate()
    {
        Ok(abi) => abi,
        Err(e) => {
            println!("Could not generate abi from weth_abi.json: {}", e);
            process::exit(1);
        }
    };

    match abi.write_to_file("../steward_abi/src/weth.rs") {
        Ok(_) => (),
        Err(e) => println!("Error writing weth.rs: {}", e),
    }

    // Cellar Aave
    let abigen = match Abigen::new("AaveCellar", "../steward_abi/cellar_aave_abi.json") {
        Ok(abigen) => abigen,
        Err(e) => {
            println!("Could not open cellar_aave_abi.json: {}", e);
            process::exit(1);
        }
    };

    let abi = match abigen
        .add_event_derive("serde::Deserialize")
        .add_event_derive("serde::Serialize")
        .generate()
    {
        Ok(abi) => abi,
        Err(e) => {
            println!("Could not generate abi from cellar_aave_abi.json: {}", e);
            process::exit(1);
        }
    };

    match abi.write_to_file("../steward_abi/src/cellar_aave.rs") {
        Ok(_) => (),
        Err(e) => println!("Error writing cellar_aave.rs: {}", e),
    }
}
