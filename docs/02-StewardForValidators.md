# Validators Instructions for Setting Up Steward

Steward is an application intended for developers and validators on the Sommelier network.

It can run as a server for forwarding cellar contract calls from Strategy Providers through the Sommelier chain, or it can run in test mode to directly interact with Ethereum contracts as a single signer.

It integrates the full functionality of gorc for operating as an orchestrator and relayer of [Gravity bridge](https://github.com/PeggyJV/gravity-bridge/) messages between the Ethereum and Cosmos chains.

Stewards works in conjunction with the Orchestrator, so both processes must be running to fully participate in Cellar management.

## Quickstart

To start Steward, simply run

```bash
steward -c [path to your config toml] start
```

## Setting Up Steward

In this section, let’s explore setting up steward for validators. There are two ways that validators will use Steward:

1. Running Steward as a gRPC server for relaying Strategy Provider contract calls through the chain. If a validator is not running this server, it cannot participate in Cellar management.
2. Manually scheduling contract calls when the validator set needs to coordinate administrative function calls on a cellar. An example of this would be shutting down a cellar.

### Running Steward as a server

Steward runs on every Validator in the Sommelier Validator set. It runs a server to which Strategy Providers (SPs) send requests whenever they determine that the market has changed enough to warrant action. The request payload contains everything needed to make a *cork*: a signed combination of a cellar address and an ABI encoded contract call. When Steward receives a submission from the SP, it validates the target cellar address, builds a cork, signs it with the delegate key, and submits it to the Cork module on chain.

> :bulb:  Steward gatekeeps function calls both by only supporting certain Cellar functions in its API, and by validating the target cellar before forwarding a call.  

Before SPs can establish a connection with your `steward` server, you will need to generate a self-signed CA certificate, a certificate signed by this CA, and you'll need to the SP's client CA. These certificates must be TLS 1.2 compliant. The paths to these values and the key used to create your signed server certificate must be set in the `[server]` table of your config file.

Here is an [example configuration](./01-Configuration.md#complete-example-configtoml) you can use to get your config file started. Most of the fields have sensible defaults; you can use the [configuration reference](./01-Configuration.md#reference) to determine which fields you don't need to explicity set if you wish.

Once your certs and config file are ready, refer to the [Quickstart section](#quickstart) above to start Steward!
