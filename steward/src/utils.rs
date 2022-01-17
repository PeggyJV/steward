use ethers::types::Address as EthAddress;
use std::time::Duration;

pub const TIMEOUT: Duration = Duration::from_secs(60);

pub fn format_eth_address(address: EthAddress) -> String {
    format!("0x{}", bytes_to_hex_str(address.as_bytes()))
}

pub fn bytes_to_hex_str(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:0>2x?}", b))
        .fold(String::new(), |acc, x| acc + &x)
}
