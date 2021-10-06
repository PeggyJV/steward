# Cellar Rebalancer

The Cellar Rebalancer is an appliaction intended for developers and validators on Sommelier network.

It can run in single signer mode or as a voter in the Cosmos sommelier protocol.

It integrates the full functionality of gorc for operating as an orchestator and relayer of gravity bridge messages between Ethereum and Cosmos chains.

## Getting Started

The rebalancer has two modes; the single signer mode(testing mode) and the cosmos voting mode, also known as the chain signer mode. The gravity bridge will run the chain signer mode, while the single signer mode can be bootstraped. The section gives an overview on how to bootstrap the testing mode.

### Testing mode

These instructions assume that the cellar has been deployed to the target ethereum blockchain.

#### Setup Configuration File
The first step is to setup your configuration file.

To generate a configuation file template, run the command below in your terminal:

```
cargo run -- print-config
```

Next, create a `toml` file in the root of the application, replacing the default keys in the template displayed in your terminal with your configuration. Make sure to confirm that the token info in your configuration file matches the deployed cellar contract.

You can create keys or import keys. To create keys, run the command below:

```
cargo run -- -c [your_config_file_name.toml] keys add [key_name]
```

Now, navigate to the keystore location in your local environment, i.e `keys.keystore`. Confirm that the key was created successfully in the location you specified in your config file.

#### Authorize Erc20 Token to interact with Cellar contract.

Run the `allow-erc-2-0` command as shown below, to allow Erc20 token to interact with Cellar contract.

```
cargo run -- -c [your_config_file_name.toml] allow-erc-2-0 --cellar-address=[the_cellar_address] address [the_erc20_address] --amount [amount]

// The command above, should look like this:
cargo run -- -c your_config_file_name.toml allow-erc-2-0 --cellar-address=0x08c0a00000000000000000000000000000000000 address 0x08c0a00000000000000000000000000000000000 --amount [amount]
```

#### Fund Cellars
Before rebalancing Cellars, it has to be funded. follow the command below, to fund Cellars.

```
cargo run -- -c [your_config_file_name.toml] fund-cellar --cellar-id[cellar_id] --amount-0[amount] --amount-1[amount]
```

#### Rebalance Cellars

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

