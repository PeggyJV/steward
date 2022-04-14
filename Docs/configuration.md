# Configuration

This document is meant to be an exhaustive reference of all TOML configuration fields available in Steward. Some of these fields are also used by the Orchestrator.

You can see all available configuration fields with their default values by running

```bash
steward print-config
```

To generate a default config file, simply redirect stdout to a file:

```bash
# You must update these values to your own configuration
steward print-config > config.toml
```

## Reference by Table

### `[cosmos]` table

Configuration related interactions with the Cosmos chain in question

#### `gas_adjustment`

Type: float

Multiplied by estimated gas fee per transaction. Currently Sommelier fee requirements are 0.0 so this can be left as default.

```
[cosmos]
gas_adjustment = 1.0
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

#### `keystore`

Type: string

Path to the location where `steward keys` will manage keys on-disk

```
[keys]
keystore = "/tmp/keystore"
```


### [metrics] table

Config related to the Orchestrator metrics server

#### `listen_addr`

Type: string

The server endpoint for monitoring Orchestrator metrics

```
[metrics]
listen_addr = "127.0.0.1:3000"
```

### [server] table

Config related to the Steward server

#### `address`

Type: string

The IP address at which the Steward server will run

```
[server]
address = "0.0.0.0"
```

#### `client_ca_cert_path`

Type: string

The path to the trusted CA used to sign the Strategy Provider's client certificate

```
[server]
client_ca_cert_path = ""
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

## Complete Example config.toml

This example will not work as is, you'll need to supply your own values.

```toml
keystore = "/tmp/keystore"

[cosmos]
gas_adjustment = 1.0
grpc = "https://127.0.0.1:9090"
key_derivation_path = "m/44'/118'/0'/0/0"
prefix = "somm"
msg_batch_size = 5

[cosmos.gas_price]
amount = 0.0
denom = "usomm"

[ethereum]
blocks_to_search = 5000
gas_price_multiplier = 1.0
key_derivation_path = "m/44'/60'/0'/0/0"
rpc = "http://localhost:8545"

[gravity]
contract = "0x0000000000000000000000000000000000000000"
fees_denom = "stake"

[keys]
delegate_key = ""

[metrics]
listen_addr = "127.0.0.1:3000"

[server]
address = "0.0.0.0"
client_ca_cert_path = ""
port = 5734
server_cert_path = ""
server_key_path = ""
```
