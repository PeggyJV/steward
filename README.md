# Steward

Steward is an application intended for developers and validators on Sommelier network.

It can run as a voter in the Cosmos Sommelier protocol or in test mode to directly interact with Ethereum contracts as a single signer.

It integrates the full functionality of [Gorc](https://github.com/PeggyJV/gravity-bridge/tree/main/orchestrator/gorc) for operating as an orchestrator and relayer of [Gravity bridge](https://github.com/PeggyJV/gravity-bridge) messages between the Ethereum and Cosmos chains.

Steward is built with the [Abscissa](https://github.com/iqlusioninc/abscissa) app micro-framework.

## Steward use case

Steward is responsible for running the Orchestrator, which handles relaying Cosmos transactions to Ethereum, and co-processing Ethereum transactions on Sommelier. Steward runs the Orchestrator so that Sommelier can manage [Cellars](steward/src/cellars) on Ethereum.

Steward provides a gRPC server to accept recommendations from data providers. Data Provision involves both calculating strategic rebalance recommendations based on market data and relaying that recommendation to the Sommelier Validators via the exposed Steward endpoints. To provide data to Steward, an encrypted and authenticated gRPC connection must be established. The client certificate authority used by the initial Data Provider is included in `tls/`. This is the only client root of trust accepted by default in Steward right now as we are only accepting client certs from one Data Provider, [Peggy JV](https://peggy.cool/).

Strategies determine where to invest funds and how to rebalance them in reaction to market events. When developers want to make their own strategy to run on Sommelier, they need two things:

1. A [Cellar contract](docs/Cellarsetup_instructions) containing a `rebalance` function.
2. [Data Providers](docs/data_providers)

Steward also provides a suite of utility functions for interacting with Sommelier and the Gravity bridge. For instance, the last section shows series of subcommands that allows you to interact with the Sommelier chain via Steward.

## Getting started with the testing mode

Steward has two modes; the single signer mode(testing mode) and the allocation mode. The Sommelier chain will run the allocation module, while the single signer mode can be bootstraped. The section gives an overview on how to bootstrap the testing mode.

### Testing mode

These instructions assume that the Cellar has been deployed to the target ethereum blockchain.

#### Setup Configuration File

The first step is to setup your configuration file.

To generate a configuation file template, run the command below in your terminal:

```
cargo run --bin steward print-config
```

Next, create a `toml` file in the root of the application, replacing the default keys in the template displayed in your terminal with your configuration. Make sure to confirm that the token info in your configuration file matches the deployed Cellar contract.

You can create keys or import keys. To create keys, run the command below. Replace `eth` with `cosmos` if you want to create a `cosmos` key instead of an `eth` key.

```
cargo run --bin steward -- -c [your_config_file_name.toml] keys eth add [key_name]
```

If you already have a key, you can import it with the command below:

```
cargo run --bin steward -- -c [your_config_file_name.toml] keys eth import [key_name]
```

Now, navigate to the keystore location in your local environment, i.e `keys.keystore`. Confirm that the key was created successfully in the location you specified in your config file.

#### Authorize Erc20 Token to interact with Cellar contract.

Run the `allow-erc-2-0` command as shown below, to allow Erc20 token to interact with Cellar contract.

```
cargo run --bin steward -- -c [your_config_file_name.toml] allow-erc-2-0 --cellar-address=[the_cellar_address] address [the_erc20_address] --amount [amount]

// The command above, should look like this:
cargo run --bin steward -- -c your_config_file_name.toml allow-erc-2-0 --cellar-address=0x08c0a00000000000000000000000000000000000 address 0x08c0a00000000000000000000000000000000000 --amount [amount]
```

#### Fund Cellars

Before rebalancing Cellars, it has to be funded. follow the command below, to fund Cellars.

```
cargo run --bin steward -- -c [your_config_file_name.toml] fund-cellar --cellar-id[cellar_id] --amount-0[amount] --amount-1[amount]
```

#### Rebalance Cellars

To start automatic rebalancing with the Cellars rebalancer, run the `single-signer` command in your terminal. Note that we need the standard environment variable `CELLAR_DRY_RUN` to be false in order to decide rebalance in the test mode. Therefore, to start the application run the command below:

```
CELLAR_DRY_RUN=false cargo run --bin steward -- -c [your_config_file_name.toml] single-signer
```

### Steward Subcommands

Below is a list of the Steward's subcommands:

| Subcommand        | Description                                                |
| ----------------- | ---------------------------------------------------------- |
| help              | Help command to get usage information                      |
| transfer          | Command to transfer ETH                                    |
| version           | Display version information                                |
| predictions       | Display lastest prediction in the application              |
| keys              | Key management commands for the rebalancer                 |
| print-config      | Command for printing default configurations                |
| fund-cellar       | Command to fund the Cellars                                |
| cosmos-to-eth     | This command sends Cosmos to the Eth chain                 |
| deploy            | Provides tools for contract deployment                     |
| eth-to-cosmos     | Command to Send Ethereum to Cosmos                         |
| orchestrator      | The orchestrator management commands                       |
| query             | Command to query state on either ethereum or cosmos chains |
| sign-delegate-key | This command is to sign delegate keys                      |
| tx                | Create transactions on either ethereum or cosmos chains    |
| allow-erc-2-0     | Allow Erc20 Token to interact with cellar contract         |

If you have any questions, you can ask the community in our [Telegram](https://t.me/getsomm) and [Discord](https://discord.com/invite/ZcAYgSBxvY) group. Also, you can take a look at the [documentation](docs/validators_instructions) for Validators instructions.
