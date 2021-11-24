use ethers::contract::Abigen;
use std::process;

fn main() {
    // Aave    
    let abigen = match Abigen::new("Aave", "../rebalancer_abi/aave_lending_abi.json") {
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

    match abi.write_to_file("../rebalancer_abi/src/aave.rs") {
        Ok(_) => (),
        Err(e) => println!("Error writing aave.rs: {}", e),
    }

    // Erc20

    let abigen = match Abigen::new("Erc20", "../rebalancer_abi/erc20_abi.json") {
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

    match abi.write_to_file("../rebalancer_abi/src/erc20.rs") {
        Ok(_) => (),
        Err(e) => println!("Error writing erc20.rs: {}", e),
    }

    // Uniswap Router
    let abigen = match Abigen::new("Erc20", "../rebalancer_abi/uniswap_router_abi.json") {
        Ok(abigen) => abigen,
        Err(e) => {
            println!("Could not open uniswap_router_abi.json: {}", e);
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
            println!("Could not generate abi from uniswap_router_abi.json: {}", e);
            process::exit(1);
        }
    };

    match abi.write_to_file("../rebalancer_abi/src/uniswap_router.rs") {
        Ok(_) => (),
        Err(e) => println!("Error writing uniswap_router.rs: {}", e),
    }

    // Uniswapv3 Pool
    let abigen = match Abigen::new("Erc20", "../rebalancer_abi/uniswapv3pool_abi.json") {
        Ok(abigen) => abigen,
        Err(e) => {
            println!("Could not open uniswapv3pool_abi.json: {}", e);
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
            println!("Could not generate abi from uniswapv3pool_abi.json: {}", e);
            process::exit(1);
        }
    };

    match abi.write_to_file("../rebalancer_abi/src/uniswapv3pool.rs") {
        Ok(_) => (),
        Err(e) => println!("Error writing uniswapv3pool.rs: {}", e),
    }
    // Weth
}

