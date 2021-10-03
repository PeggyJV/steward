# Cellar Rebalancer

The Cellar Rebalancer is an appliaction intended for developers and validators on Sommelier network.

It can run in single signer mode or as a voter in the Cosmos sommelier protocol.

It integrates the full functionality of gorc for operating as an orchestator and relayer of gravity bridge messages between Ethereum and Cosmos chains.

## Getting Started

Below is a list of the Cellar rebalancer's subcommands:

| Subcommand        | Description                                                 |
| ----------------- | ----------------------------------------------------------- |
| help              | Help command to get usage information                       |
| start             | Command to start the rebalancer                             |
| transfer          | Command to transfer ETH                                     |
| version           | Display version information                                 |
| predictions       | Display lastest prediction in the application               |
| keys              | Key management commands for the rebalancer                  |
| print-config      | Command for printing default configurations                 |
| fund-cellar       | Command to fund the Cellars                                 |
| cosmos-to-eth     | This command sends Cosmos to the Eth chain                  |
| deploy            | Provides tools for contract deployment                      |
| eth-to-cosmos     | Command to Send Ethereum to Cosmos                          |
| orchestrator      | The orchestrator management commands                        |
| query             | Command to query state on either ethereum or cosmos chains  |
| sign-delegate-key | This command is to sign delegate keys                       |
| tx                | Create transactions on either ethereum or cosmos chains     |
| allow-erc-2-0     | Allow Erc20 Token to interact with cellar contract          |

### Testing mode

1. These instructions assume that the cellar has been deployed to the target ethereum blockchain.

1. The first step is to setup your configuration file.

You can generate an example configuation file from

`cargo run -- print-config`