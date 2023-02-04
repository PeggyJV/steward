#![allow(clippy::all)]
use ethers::contract::Abigen;
use std::process;

fn main() {
    // (JSON/Contract name, output file name)
    let contracts = vec![
        ("AaveV2StablecoinCellar", "aave_v2_stablecoin"),
        ("CellarV1", "cellar_v1"),
        ("CellarV2", "cellar_v2"),
        ("UniswapV3Adaptor", "uniswap_v3_adaptor"),
        ("AaveATokenAdaptor", "aave_a_token_adaptor"),
        ("AaveDebtTokenAdaptor", "aave_debt_token_adaptor"),
        ("CompoundCTokenAdaptor", "compound_c_token_adaptor"),
        ("VestingSimpleAdaptor", "vesting_simple_adaptor"),
    ];

    contracts
        .iter()
        .for_each(|n| generate_contract_abi(n.0, n.1))
}

fn generate_contract_abi(name: &str, file_name: &str) {
    let abigen = match Abigen::new(name, format!("../steward_abi/{}.json", name)) {
        Ok(abigen) => abigen,
        Err(e) => {
            println!("Could not open {}.json: {}", name, e);
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
            println!("Could not generate abi from {}.json: {}", name, e);
            process::exit(1);
        }
    };

    match abi.write_to_file(format!("../steward_abi/src/{}.rs", file_name)) {
        Ok(_) => (),
        Err(e) => println!("Error writing {}.rs: {}", file_name, e),
    }
}
