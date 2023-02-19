#![allow(clippy::all)]
use ethers::contract::Abigen;
use std::process;

fn main() {
    generate_contract_abis();
}

fn generate_contract_abis() {
    // (JSON/Contract name, output file name)
    let contracts = vec![
        ("AaveV2StablecoinCellar", "aave_v2_stablecoin"),
        ("CellarV1", "cellar_v1"),
        ("CellarV2", "cellar_v2"),
        ("UniswapV3Adaptor", "uniswap_v3"),
        ("AaveATokenAdaptor", "aave_a_token"),
        ("AaveDebtTokenAdaptor", "aave_debt_token"),
        ("CompoundCTokenAdaptor", "compound_c_token"),
        ("VestingSimpleAdaptor", "vesting_simple"),
    ];

    contracts.iter().for_each(|n| {
        let name = n.0;
        let file_name = n.1;
        let abigen = match Abigen::new(name, format!("../abi/{}.json", name)) {
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

        match abi.write_to_file(format!("./src/gen/abi/{}.rs", file_name)) {
            Ok(_) => (),
            Err(e) => println!("Error writing {}.rs: {}", file_name, e),
        }
    })
}
