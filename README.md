# Cellar Rebalancer

The Cellar Rebalancer is an appliaction intended for developers and validators on Sommelier network.

It can run in single signer mode or as a voter in the Cosmos sommelier protocol.

It integrates the full functionality of gorc for operating as an orchestator and relayer of gravity bridge messages between Ethereum and Cosmos chains.

## Getting Started

### Testing mode

1. These instructions assume that the cellar has been deployed to the target ethereum blockchain.

1. The first step is to setup your configuration file.

You can generate an example configuation file from

`cargo run -- print-config`