# Orchestrator

The Orchestrator is a sidecar application to a Cosmos chain responsible for *orchestrating* transactions between Cosmos and Ethereum chains. In the Cosmos -> Ethereum direction, validators running the orchestrator with the Relayer enabled will query the [Gravity Module](https://github.com/peggyjv/gravity-bridge/tree/main/module/x/gravity) for pending transactions, and then send them to the gravity contract. In the Ethereum -> Cosmos direction, the Orchestrator acts as an oracle for the Cosmos chain to inform it of incoming transactions. This allows the coordination of appropriate locking/unlocking/minting/burning of tokens on either side of the bridge. The Orchestrator ensures that Gravity module state on the Cosmos chain is synchronized with the state of the gravity contract.

## Quickstart

The Orchestrator and Relayer can be started using the following command:

```bash
steward -c <config toml path> orchestrator start --ethereum-key <eth_key_name> --cosmos-key <cosmos key name>
```

If you do not wish to participate in Relaying Cosmos -> Ethereum transactions, you can start the orchestrator without the relayer by adding the `--orchestrator-only` flag:

```bash
steward -c <config toml path> orchestrator start --ethereum-key <eth_key_name> --cosmos-key <cosmos key name> --orchestrator-only
```

## Setup

Before you'll be able to run the start command successfully however, you'll need keys, a configuration file, and a running Cosmos chain. Let's walk through these one at a time.

### Create or import an Ethereum key

Example minimum required config:

```toml
[ethereum]
key_derivation_path = "m/44'/60'/0'/0/0"

[keys]
keystore = /my/keystore/path
```

To add or import an Ethereum key to your keystore, run either of the following commands respectively:

```bash
steward -c <config toml path> keys eth add <key_name>

steward -c <config toml path> keys eth import <key_name>
```

To confirm it works, check your keystore directory for a file with the key name you provided.

### Create or import a Cosmos key

Example minimum required config:

```toml
[cosmos]
key_derivation_path = "m/44'/118'/0'/0/0"

[keys]
keystore = /my/keystore/path
```

To add a new Cosmos key or recover one from a mnemonic, run either of the following commands respectively:

```bash
steward -c <config toml path> keys cosmos add <key_name>]

# Omitting the mnemonic argument will provide a prompt for its entry when the
# command is run.
steward -c <config toml path> keys cosmos recover <key_name> [mnemonic]
```

To confirm it works, check your keystore directory for a file with the key name you provided.

### Configuration

The following configuration fields are required to run the Orchestrator. Please refer to the [Configuration Referance](./configuration#L20) for explanations of each fields' purpose.

```toml
[cosmos]
gas_adjustment = 1.0
grpc = "http://localhost:9090"
msg_batch_size = 5
prefix = "somm"

[cosmos.gas_price]
amount = 0.0
demom = "usomm"

[ethereum]
blocks_to_search = 5000
gas_price_multiplier = 1.2
rpc = "http://localhost:8545"

[gravity]
contract = "0x00000000000000000000000000000000000"
fees_denom = "usomm"
```

You should be ready to go. You'll find the start command in the Quickstart section at the top of this document.

