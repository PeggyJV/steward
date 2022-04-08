# Validators Instructions for Setting Up Steward

Steward is an application intended for developers and validators on the Sommelier network.

It can run as a voter in the Cosmos Sommelier protocol or in test mode to directly interact with Ethereum contracts as a single signer.

It integrates the full functionality of gorc for operating as an orchestrator and relayer of [Gravity bridge](https://github.com/PeggyJV/gravity-bridge/) messages between the Ethereum and Cosmos chains.

## Background

Every automated investing system has a feedback loop where changes in the market must be observed and investments rebalanced in response.

Steward closes the loop in the Sommelier chain. For instance, it allows Validators to vote for rebalancing Cellar funds in response to a Data Provider’s recommendation based on market changes.

Additionally, Steward houses the Orchestrator, which is responsible for interactions with both chains via the Gravity Bridge.

### **Cellars**

Cellars are smart contracts that are designed to manage funds based on a particular investment strategy.

### **Data Providers**

Data Providers (DPs) are users or organizations who provide recommended rebalance arguments based on price or other market data to the Sommelier validators in exchange for a fee. There can be multiple Data Providers per strategy, and Validators are responsible for deciding which one they pay in exchange for data.

### **Sommelier Chain**

The Sommelier chain is a Cosmos SDK application chain. It is the core of Sommelier that makes decisions about whether to act on DP recommendations and executes Cellar functions. Transactions and function calls are only settled on-chain when the Validators reach consensus. In order to fulfill a strategy, Validators need a way to receive recommendation data from the Data Providers, reach consensus on whether to act on that data, and then call functions on the Cellars with the provided data as parameters. This is where Steward comes in.

### **Steward**

Steward runs on every Validator in the Sommelier Validator set. It runs a server to which Data Providers send recommendations as soon as market data becomes available. This payload contains identifying information about the Cellar which Steward is meant to act on, and the recommended arguments for its rebalance function parameters. When Steward receives a new recommendation, it submits a vote to rebalance the appropriate Cellar based on that data. If Validators reach a quorum, the process of rebalancing begins.

Since Sommelier is a Cosmos chain, in order to provide strategies on Ethereum it needs a way to bridge assets and logic calls. Steward is responsible for running the Orchestrator, which handles relaying Cosmos transactions to Ethereum, and co-processing Ethereum transactions on Sommelier. Steward runs the Orchestrator so that Sommelier can manage Cellars on Ethereum.

## Setting Up Steward

In this section, let’s explore setting up steward for validators. First, ensure the sommelier chain is running. Next, create a `toml` file in the root of the application which will hold your configuration. To get a template for your configuration file, run the command below:

```bash
steward print-config
```

Replace the default keys in the template displayed in your terminal with your configuration. Now you have the configurations set up, let’s go through stewards commands.

## Start Cosmos Mode

To start the Allocation signer mode, ensure that the server section in your config file is set properly as shown below. The `address`, `client_ca_cert_path` and `port` are optional fields.

```toml
keystore = "/tmp/keystore"

[server]
address = "0.0.0.0"           // This is an optional feild.
client_ca_cert_path = ""       // optional, defaults to Peggy JV client cert
port = 5734                  // optional, default is 5734
server_cert_path = ""
server_key_path = ""

[gravity]
contract = "0x0000000000000000000000000000000000000000"
fees_denom = "usomm"

[ethereum]
key_derivation_path = "m/44'/60'/0'/0/0"
rpc = "http://localhost:8545"
gas_price_multiplier = 1.0
blocks_to_search = 5000

[cosmos]
key_derivation_path = "m/44'/118'/0'/0/0"
grpc = "http://localhost:9090"
prefix = "cosmos"
msg_batch_size = 5
gas_adjustment = 1.0

[cosmos.gas_price]
amount = 0.001
denom = "usomm"

[metrics]
listen_addr = "127.0.0.1:3000"

[[cellars]]
pair_id = "0x0"
name = ""
token_id = "0x0"
cellar_address = "0x0000000000000000000000000000000000000000"
pool_address = "0x0000000000000000000000000000000000000000"
weight_factor = 100
max_gas_price_gwei = 100

[cellars.token_0]
decimals = 18
symbol = "NA"
address = "0x0000000000000000000000000000000000000000"

[cellars.token_1]
decimals = 18
symbol = "NA"
address = "0x0000000000000000000000000000000000000000"

[cellars.duration]
secs = 60
nanos = 0

[keys]
keystore = "/tmp/keystore"
rebalancer_key = ""
```

All cellar configurations would only be required in the single signer testing mode.
Next, run the cosmos signer command to start the cosmos mode:

```bash
steward -c [your_config_file_name.toml] cosmos-signer
```

## Start Orchestrator

Steward allows you to start the Orchestrator with or without the Relayer. First, you’ll need an Ethereum key and a Cosmos key. Run the command below to create your keys if you don’t have one. Replace `eth` with `cosmos` if you want to create a `cosmos` key instead of an `eth` key.

```bash
steward -c [your_config_file_name.toml] keys eth add [key_name]

steward -c [your_config_file_name.toml] keys eth import [key_name]
```

To start the Orchestrator with the Relayer, run the command below:

```bash
steward -c [your_config_file_name.toml] orchestrator start cosmos_key=[key_name] ethereum_key=[key_name] orchestrator_only=false
```

You can start the Orchestrator only by running the command below:

```bash
steward -c [your_config_file_name.toml] orchestrator start cosmos_key=[key_name] ethereum_key=[key_name] orchestrator_only=true
```
