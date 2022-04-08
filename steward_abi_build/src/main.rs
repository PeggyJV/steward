use ethers::contract::Abigen;
use std::process;

fn main() {
    // Aave
    let abigen = match Abigen::new(
        "AaveV2StablecoinCellar",
        "../steward_abi/cellar_aave_v2_stablecoin_abi.json",
    ) {
        Ok(abigen) => abigen,
        Err(e) => {
            println!("Could not open cellar_aave_v2_stablecoin_abi.json: {}", e);
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
            println!(
                "Could not generate abi from cellar_aave_v2_stablecoin_abi.json: {}",
                e
            );
            process::exit(1);
        }
    };

    match abi.write_to_file("../steward_abi/src/aave_v2_stablecoin.rs") {
        Ok(_) => (),
        Err(e) => println!("Error writing aave_v2_stablecoin.rs: {}", e),
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

    // Cellar Uniswap
    let abigen = match Abigen::new("UniswapV3Cellar", "../steward_abi/cellar_uniswap.json") {
        Ok(abigen) => abigen,
        Err(e) => {
            println!("Could not open cellar_uniswap.json: {}", e);
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
            println!("Could not generate abi from cellar_uniswap.json: {}", e);
            process::exit(1);
        }
    };

    match abi.write_to_file("../steward_abi/src/cellar_uniswap.rs") {
        Ok(_) => (),
        Err(e) => println!("Error writing cellar_uniswap.rs: {}", e),
    }

    // Cellar Uniswap
    let abigen = match Abigen::new(
        "UniswapV3Cellar",
        "../steward_abi/cellar_uniswap_limit_usdc_eth.json",
    ) {
        Ok(abigen) => abigen,
        Err(e) => {
            println!("Could not open cellar_uniswap_limit_usdc_eth.json: {}", e);
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
            println!(
                "Could not generate abi from cellar_uniswap_limit_usdc_eth.json: {}",
                e
            );
            process::exit(1);
        }
    };

    match abi.write_to_file("../steward_abi/src/cellar_uniswap_limit_usdc_eth.rs") {
        Ok(_) => (),
        Err(e) => println!("Error writing cellar_uniswap_limit_usdc_eth.rs: {}", e),
    }
}
