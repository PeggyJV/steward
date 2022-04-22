# Orchestrator

The Orchestrator is a sidecar application to a Cosmos chain responsible for *orchestrating* transactions between Cosmos and Ethereum chains. In the Cosmos -> Ethereum direction, validators running the orchestrator with the Relayer enabled will query the [Gravity Module](https://github.com/peggyjv/gravity-bridge/tree/main/module/x/gravity) for pending transactions, and then send them to the gravity contract. In the Ethereum -> Cosmos direction, the Orchestrator acts as an oracle for the Cosmos chain to inform it of incoming transactions. This allows the coordination of appropriate locking/unlocking/minting/burning of tokens on either side of the bridge. The Orchestrator ensures that Gravity module state on the Cosmos chain is synchronized with the state of the gravity contract. Currently we are only asking validators to run the Orchestrator without the Relayer (Ethereum -> Cosmos direction only).

## Quickstart

The Orchestrator can be started using the following command:

```bash
steward -c <config_toml_path> orchestrator start --orchestrator-only --ethereum-key <eth_key_name> --cosmos-key <cosmos_key_name>
```

The `--orchestrator-only` flag prevents the Relayer thread from running, and this is the way in which we encourage validators to run the Orchestrator for now. You will burn gas on failed transactions if you run the Relayer in its current state.

## Setup

Before you'll be able to run the start command successfully, you'll need keys, a configuration file, and a running Cosmos chain.

### Key creation config

Before you attempt to generate keys, be sure your keystore and key derivation paths are configured:

```toml
keystore = "/some/path/keystore"

[cosmos]
key_derivation_path = "m/44'/118'/0'/0/0"

[ethereum]
key_derivation_path = "m/44'/60'/0'/0/0"
```

### Create or import an Ethereum key

To add or import an Ethereum key to your keystore, run either of the following commands respectively:

```bash
steward -c <config toml path> keys eth add <key_name>

steward -c <config toml path> keys eth import <key_name>
```

To confirm it works, check your keystore directory for a file with the key name you provided.

### Create or import a Cosmos key

To add a new Cosmos key or recover one from a mnemonic, run either of the following commands respectively:

```bash
steward -c <config toml path> keys cosmos add <key_name>

# Omitting the mnemonic argument will provide a prompt for its entry when the
# command is run.
steward -c <config toml path> keys cosmos recover <key_name> [mnemonic]
```

To confirm it works, check your keystore directory for a file with the key name you provided.

### Configuration

Please refer to this [example configuration](./01-Configuration.md#complete-example-configtoml) and the [configuration reference](./01-Configuration.md#reference).
