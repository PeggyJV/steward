#![allow(clippy::all)]
use ethers::contract::Abigen;
use std::process;

use std::path::Path;
use std::{
    fs::{self, create_dir_all, remove_dir_all},
    path::PathBuf,
};
use walkdir::WalkDir;

/// A temporary directory for proto building
const TMP_PATH: &str = "/tmp/steward/";
/// the output directory
const OUT_PATH: &str = "src/gen/proto/";

fn main() {
    generate_contract_abis();
    generate_protos();
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
        let abigen = match Abigen::new(name, format!("abi/{}.json", name)) {
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

        match abi.write_to_file(format!("src/gen/abi/{}.rs", file_name)) {
            Ok(_) => (),
            Err(e) => println!("Error writing {}.rs: {}", file_name, e),
        }
    })
}

fn generate_protos() {
    let out_dir = Path::new(&OUT_PATH);
    let tmp_dir = Path::new(&TMP_PATH);
    let root = env!("CARGO_MANIFEST_DIR");
    let root: PathBuf = root.parse().unwrap();
    let mut steward_proto_dir = root;
    steward_proto_dir.push("proto/");
    let steward_proto_dir = [steward_proto_dir];

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    for proto_path in &steward_proto_dir {
        protos.append(
            &mut WalkDir::new(proto_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().is_some()
                        && e.path().extension().unwrap() == "proto"
                })
                .map(|e| e.into_path())
                .collect(),
        );
    }

    // create directories for temporary build dirs
    fs::create_dir_all(tmp_dir)
        .unwrap_or_else(|_| panic!("Failed to create {:?}", tmp_dir.to_str()));

    // Compile all proto files
    let mut config = prost_build::Config::default();
    config.out_dir(tmp_dir);
    config.type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]");
    config.compile_protos(&protos, &steward_proto_dir).unwrap();

    // Compile all proto client for GRPC services
    println!("Compiling proto clients for GRPC services...");
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .file_descriptor_set_path(tmp_dir.join("descriptor.bin"))
        .format(true)
        .out_dir(tmp_dir)
        .compile_with_config(config, &protos, &steward_proto_dir)
        .unwrap();

    copy_generated_files(tmp_dir, out_dir);
    println!("Done!");
}

fn copy_generated_files(from_dir: &Path, to_dir: &Path) {
    println!("Copying generated files into '{}'...", to_dir.display());

    // Remove old compiled files
    remove_dir_all(&to_dir).unwrap_or_default();
    create_dir_all(&to_dir).unwrap();

    let mut filenames = Vec::new();

    // Copy new compiled files (prost does not use folder structures)
    let errors = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();
            filenames.push(filename.clone());
            let contents = fs::read(e.path())?;
            fs::write(format!("{}/{}", to_dir.display(), &filename), contents)
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            eprintln!("[error] Error while copying compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }
}