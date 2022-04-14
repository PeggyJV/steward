# Steward

Steward is an application intended for developers and validators in the Sommelier network.

It can run as a server in the Sommelier protocol or in test mode to directly interact with Ethereum contracts.

It integrates the full functionality of [Gorc](https://github.com/PeggyJV/gravity-bridge/tree/main/orchestrator/gorc) for operating the [Gravity bridge](https://github.com/PeggyJV/gravity-bridge), connecting Ethereum and Cosmos chains.

Steward is built with the [Abscissa](https://github.com/iqlusioninc/abscissa) app micro-framework.

Please also see the [Steward Docs](docs/).

## Steward use case

Steward wears several hats:

###  1. Gravity Bridge Operator

Steward is responsible for running the Relayer, which handles relaying Cosmos transactions to Ethereum, and the Orchestrator, which enables the co-processing of Ethereum transactions on Sommelier. Steward runs the Orchestrator so that Sommelier can manage Cellars on Ethereum. Which brings us to Steward's second responsibility.

### 2. Middleman between the Sommelier chain and Strategy Providers

Steward provides a gRPC server to accept recommendations from Strategy Providers. When Strategy Providers submit a Cellar function call to Steward, it validates, encodes, signs, and forwards the call to the [Cork module](https://github.com/PeggyJV/sommelier/tree/main/x/cork) on the Sommelier chain.

### 3. Cellar Development Tool

Steward provides a suite of utility functions for interacting with Sommelier, the Gravity bridge, and Cellar contracts directly.

## Getting Started for Validators

Please refer to the [Steward for Validators](docs/02-StewardForValidators.md) document.

## Getting Started for Strategists

If you want to test and PR changes to Steward to support your Cellar, or you just want to include Steward in your model testing against an existing Cellar, you can use Steward's test mode. This mode will run a gRPC server just like in production, but without the Sommelier chain and gravity bridge. Instead, Steward will send function calls directly to the target Cellar contract.

The developer workflow for this mode is still being refined and documentation is forthcoming.

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

If you have any questions, you can ask the community in our [Telegram](https://t.me/getsomm) and [Discord](https://discord.com/invite/ZcAYgSBxvY) group.
