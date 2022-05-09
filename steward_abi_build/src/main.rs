use ethers::contract::Abigen;
use std::process;

fn main() {
    // Aave
    let aave_contract = "AaveV2StablecoinCellar";
    let abigen = match Abigen::new(
        aave_contract,
        format!("../steward_abi/{}.json", aave_contract),
    ) {
        Ok(abigen) => abigen,
        Err(e) => {
            println!("Could not open {}.json: {}", aave_contract, e);
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
            println!("Could not generate abi from {}.json: {}", aave_contract, e);
            process::exit(1);
        }
    };

    match abi.write_to_file("../steward_abi/src/aave_v2_stablecoin.rs") {
        Ok(_) => (),
        Err(e) => println!("Error writing aave_v2_stablecoin.rs: {}", e),
    }
}
