# Cellar Rebalancer

The Cellar Rebalancer is an appliaction intended for developers and validators on Sommelier network.

It can run in single signer mode or as a voter in the Cosmos sommelier protocol.

It integrates the full functionality of gorc for operating as an orchestator and relayer of gravity bridge messages between Ethereum and Cosmos chains.

## Getting Started

### Testing mode

1. These instructions assume that the cellar has been deployed to the target ethereum blockchain.

1. The first step is to setup your configuration file.

To generate a configuation file template, run the command below in your terminal:

```
cargo run -- print-config
```

Next, create a `toml` file in the root of the application, replacing the default keys in the template displayed in your terminal with your configuration. 

Run the `print-config` command again, to ensure that the configurations are a great fit.

Now, run the command below to allow `erc20` to interact with cellar contract.

### Cellar Rebalancer Subcommands

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
