# Configuration

This document is meant to be an exhaustive reference of all TOML configuration fields available in Steward. Some of these fields are also used by the Orchestrator.

## Default configuration

You can see all available configuration fields with their default values by running

```bash
steward print-config
```

To generate a default config file, simply redirect stdout to a file:

```bash
# You must update these values to your own configuration
steward print-config > config.toml
```

## Reference

### Top-level table

#### `keystore`

Type: string

Path to the location where `steward keys` will manage keys on-disk

```
keystore = ""
```

### `[cosmos]` table

Configuration related interactions with the Cosmos chain in question

#### `gas_adjustment`

Type: float

Multiplied by estimated gas fee per transaction. It is recommended to set this to a value above 1.0 to avoid gas estimation issues.

```
[cosmos]
gas_adjustment = 1.1
```

#### `grpc`

Type: string

Your validator node's gRPC endpoint

```
[cosmos]
grpc = "http://localhost:9090"
```

#### `key_derivation_path`

Type: string

The path used to derive accounts from your private keys. For Steward to work this must match whatever derivation path was used for the delegate and operator accounts you registered in the validator set on-chain.

```
[cosmos]
key_derivation_path = "m/44'/118'/0'/0/0"
```

#### `msg_batch_size`

Type: integer

Used by the Orchestrator. The max number of Msgs to send to Sommelier at once.

```
[cosmos]
msg_batch_size = 5
```

#### `prefix`

Type: string

The bech32 prefix to use for representing account addresses

```
[cosmos]
prefix = "somm"
```

### `[cosmos.gas_price]` table

For setting transaction fees

#### `amount`

Type: float

The amount of `denom` you are willing to pay per gas for transactions.

> :warning: This should be kept set to 0.0 for now

```
[cosmos.gas_price]
amount = 0.0
```

#### `denom`

Type: string

The denomination of the gas fee `amount`. Sommelier handles fees in `usomm`.

```
[cosmos.gas_price]
denom = "usomm"
```

### `[ethereum]` table

Config related to interaction with Ethereum

#### `blocks_to_search`

Type: integer

How many Ethereum blocks back the Orchestrator should search for the last event nonce relayed to Cosmos.

```
[ethereum]
blocks_to_search = 5000
```

#### `gas_price_multiplier`

Type: float

Multiplied by estimated gas fee per transaction. If your Ethereum transactions are failing due to insufficient gas, try increasing this value.

```
[ethereum]
gas_price_multiplier = 1.0
```

#### `gas_multiplier`

Type: float

Multiplied by estimated gas limit per transaction. If your Ethereum transactions are failing due to insufficient gas, try increasing this value.

```
[ethereum]
gas_multiplier = 1.0
```

#### `key_derivation_path`

Type: string

The path used to derive Etherum addresses from your private keys.

```
[ethereum]
key_derivation_path = "m/44'/60'/0'/0/0"
```

#### `rpc`

Type: string

The node endpoint to be used for Ethereum interactions

```
[ethereum]
rpc = "http://localhost:8545"
```

### `[gravity]` table

Configuration related to the gravity bridge

#### `contract`

Type: string

The address of the gravity contract on Ethereum

```
[gravity]
contract = "0x0000000000000000000000000000000000000000"
```

#### `fees_denom`

Type: string

The denomination in which fees will be paid to relayers. This value must be a denom supported by the configured chain (whatever chain the node at `cosmos.grpc` is validating on).

```
[gravity]
fees_denom = "usomm"
```

### `[keys]` table

Keys config

#### `delegate_key`

Type: string

The name of the key in `keystore` used for delegate signing by both Steward and the Orchestrator. The key name is the name of the key file *without the file extension*.

```
[keys]
delegate_key = ""
```

### `[cork]` table

Config related to steward's interaction with the x/cork and x/axelarcork modules.

#### `cache_refresh_period`

Type: integer

Steward's approved cellar cache refresh period in seconds.

```
[cork]
cache_refresh_period = 3600
```

#### `proposal_poll_period`

Type: integer

How frequently (in seconds) steward should query the chain for approved scheduled cork proposals that must be submitted to the chain.

```
[cork]
proposal_poll_period = 300
```

#### `max_scheduling_retries`

Type: integer

How many times steward should try to submit a scheduled cork proposal that errored during processing. 

```
[cork]
max_scheduling_retries = 3
```

### `[pubsub]` table

#### `cache_refresh_period`

Type: integer

How often steward's publisher trust data cache should be refreshed

```
[pubsub]
cache_refresh_period = 3600
```

#### `publisher_domain_block_list`

Type: string array

Publisher domains that steward should reject calls from. Primarily used in emergency situations.

```
[pubsub]
publisher_domain_block_list = ["compromised-publisher.example.com"]
```

### `[metrics]` table

Config related to the Orchestrator metrics server

#### `listen_addr`

Type: string

The server endpoint for monitoring Orchestrator metrics

```
[metrics]
listen_addr = "127.0.0.1:3000"
```

### `[server]` table

Config related to the Steward server

#### `address`

Type: string

The IP address at which the Steward server will run

```
[server]
address = "0.0.0.0"
```

#### `port`

Type: integer

The port on which the Steward server will be hosted

```
[server]
port = 5734
```

#### `server_cert_path`

Type: string

The path to your signed Steward server certificate

```
[server]
server_cert_path = ""
```

#### `server_key_path`

Type: string

The path to the PKCS8-formatted key used to generate the server certificate

```
[server]
server_key_path = ""
```

### `[simulate]` table

#### `use_tls`

Type: boolean

Whether to require TLS for calls in simulate mode

```
[simulate]
use_tls = false
```

#### `network_id`

Type: string

The EVM network ID of the target Ethereum network

```
[simulate]
network_id = "1"
```

#### `tenderly_access_key`

Type: string

Tenderly access key for to use the API for running simulations

```
[simulate]
tenderly_access_key = ""
```

#### `tenderly_project_name`

Type: string

Tenderly project name

```
[simulate]
tenderly_project_name = "my_project"
```

#### `tenderly_username`

Type: string

Tenderly username

```
[simulate]
tenderly_username = "myuser"
```

#### `gravity_address`

Type: string

Address of the gravity contract on the target network

```
[simulate]
gravity_address = ""
```

#### `server_cert_path`

Type: string

Path to the server certificate to use if `use_tls` is `true`

```
[simulate]
server_cert_path = ""
```

#### `server_key_path`

Type: string

Path to the server key used to create the server cert if `use_tls` is `true`

```
[simulate]
server_cert_path = ""
```

#### `client_ca_cert_path`

Type: string

Path to the client certificate authority for use if `use_tls` is `true`

```
[simulate]
client_ca_cert_path = ""
```

## Complete Example config.toml

This example will not work as is, you'll need to supply your own values.

```toml
keystore = "/some/path/keystore"

[cosmos]
gas_adjustment = 1.1
grpc = "https://127.0.0.1:9090"
key_derivation_path = "m/44'/118'/0'/0/0"
prefix = "somm"
msg_batch_size = 5

[cosmos.gas_price]
# Please keep amount set to 0.0 at this time
amount = 0.0
denom = "usomm"

[ethereum]
blocks_to_search = 5000
gas_price_multiplier = 1.0
gas_multiplier = 1.1
key_derivation_path = "m/44'/60'/0'/0/0"
rpc = "http://localhost:8545"

[gravity]
contract = "0x0000000000000000000000000000000000000000"
fees_denom = "usomm"

[keys]
delegate_key = "mykey"

[cork]
cache_refresh_period = 60
proposal_poll_period = 300
max_scheduling_retries = 3

[pubsub]
cache_refresh_period = 3600
publisher_domain_block_list = [] 

[metrics]
listen_addr = "127.0.0.1:3000"

# Please leave the client_ca_cert_path field unset for now
[server]
address = "0.0.0.0"
port = 5734
server_cert_path = "/server/cert/path"
server_key_path = "/server/key/path"

# Meant for use by strategists for development and testing
[simulate]
use_tls = false
network_id = "1"
tenderly_access_key = ""
tenderly_project_name = ""
tenderly_username = ""
gravity_address = ""
server_cert_path = ""
server_key_path = ""
client_ca_cert_path = ""
```
