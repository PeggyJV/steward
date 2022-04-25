# Steward

Steward is a key piece of the Sommelier system. It is a side-car process that connects the Sommelier chain to the outside world. It provides an API for Strategy Providers to update Cellars through the Sommelier validator set based on off-chain calculations. Additionally, it subsumes the functionality of gorc for managing the [Gravity bridge Orchestrator](https://github.com/PeggyJV/gravity-bridge/tree/main/orchestrator), and provides developer tools and features for developing and testing Cellars.

It can run as a voter in the Cosmos Sommelier protocol or in test mode to directly interact with Ethereum contracts as a single signer.

It integrates the full functionality of [Gorc](https://github.com/PeggyJV/gravity-bridge/tree/main/orchestrator/gorc) for operating as an orchestrator and relayer of [Gravity bridge](https://github.com/PeggyJV/gravity-bridge) messages between the Ethereum and Cosmos chains.

Steward is built with the [Abscissa](https://github.com/iqlusioninc/abscissa) app micro-framework.

## Steward use cases

###  1. Gravity Bridge Operator

Steward is responsible for running the Relayer, which handles relaying Cosmos transactions to Ethereum, and the Orchestrator, which enables the co-processing of Ethereum transactions on Sommelier. Steward runs the Orchestrator so that Sommelier can manage Cellars on Ethereum.

### 2. Middleman between the Sommelier chain and Strategy Providers

Steward runs on every Validator in the Sommelier Validator set. It runs a server to which Strategy Providers (SPs) send requests whenever they determine that the market has changed enough to warrant an update to the Cellar position. The request payload contains everything needed to make a *cork*: a signed combination of a cellar address and an ABI encoded contract call. When Steward receives a submission from the SP, it validates the target cellar address, builds a cork, signs it with the delegate key, and submits it to the [Cork module](https://github.com/PeggyJV/sommelier/tree/main/x/cork) on chain, where it will be bridged to the target Cellar contract if a consensus of validators submit the same cork.

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
| cosmos-to-eth     | This command sends Cosmos to the Eth chain                 |
| deploy            | Provides tools for contract deployment                     |
| eth-to-cosmos     | Command to Send Ethereum to Cosmos                         |
| help              | Help command to get usage information                      |
| keys              | Key management commands for the rebalancer                 |
| orchestrator      | The orchestrator management commands                       |
| print-config      | Command for printing default configurations                |
| sign-delegate-key | This command is to sign delegate keys                      |
| version           | Display version information                                |


If you have any questions, you can ask the community in our [Telegram](https://t.me/getsomm) and [Discord](https://discord.com/invite/ZcAYgSBxvY) group.
