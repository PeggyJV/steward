# Validators Instructions for Setting Up Steward

Steward is a side-car process that connects the Sommelier chain to the outside world. It provides an API for Strategy Providers to rebalance Cellars through the Sommelier validator set based on off-chain calculations. Additionally, it subsumes the functionality of [gorc](https://github.com/PeggyJV/gravity-bridge/tree/main/orchestrator/gorc) for managing the [Gravity bridge Orchestrator](https://github.com/PeggyJV/gravity-bridge/tree/main/orchestrator), and provides developer tools and features for developing and testing Cellars.

Steward runs on every Validator in the Sommelier Validator set. It runs a server to which Strategy Providers (SPs) send requests whenever they determine that the market has changed enough to warrant action. The request payload contains everything needed to make a *cork*: a signed combination of a cellar address and an ABI encoded contract call. When Steward receives a submission from the SP, it validates the target cellar address, builds a cork, signs it with the delegate key, and submits it to the [Cork module](https://github.com/peggyjv/sommelier/tree/main/x/cork) on chain.

> :bulb:  Steward authorizes function calls both by only supporting certain Cellar functions in its API, and by validating the target cellar before forwarding a call.

## Quickstart

To start Steward, simply run

```bash
steward -c <config_toml_path> start
```

> :warning: If this is production and your node is running, don't forget to make sure the Orchestrator is also running! If you don't, you can be jailed and your submitted corks will not be processed.
See the [Orchestrator Quickstart](./docs/03-TheOrchestrator.md#quickstart) section of the docs.

## Running Steward as a server

### TLS Certificates

Before SPs can establish a connection with your `steward` server, you will need to generate a CA, a TLS certificate signed by this CA, and you'll need to the SP's client CA. These certificates must be TLS 1.2 compliant. The paths to these values and the key used to create your signed server certificate must be configured in the `[server]` table of your config file. Starting out, Sommelier will be running the only SP, and Steward will use our client CA cert by default. This CA cert can be found in the `tls/` directory of this repo. There is no need to set the `client_ca_cert_path` configuration field at this time.

### Cosmos Delegate Key

Steward signs corks with the same key that the Orchestrator uses to sign Ethereum -> Cosmos transactions. Therefore, if you have already generated a delegate key for the Orchestrator in the past, you do not need to do it again for Steward. Simply [set the path to this key](./01-Configuration.md#delegatekey) in your Steward config.

If you are a new validator, we recommend following the [Sommelier installation steps](https://github.com/PeggyJV/sommelier#installation) to generate all the keys you need and set them as the delegate keys for your validator. It is ok to use [gorc](https://github.com/PeggyJV/gravity-bridge/tree/main/orchestrator/gorc) for key generation as that document recommends, however you can also generate them using Steward. Examples of the commands used to do this can be found in the [Orchestrator Setup section](./03-TheOrchestrator.md#setup).

The Cosmos delegate key generated (called "orchestrator" in the Sommelier docs) should be set as the [delegate_key field](./01-Configuration.md#delegatekey) under the `[server]` table of your configuration file.

> :warning: Remember to set this key as a delegate key in your validator node. The steps to do this are covered in the Sommelier documentaion linked above.

### Other config

Other important configuration fields like your Sommelier node's endpoint are also required so that Steward knows where to send corks. Here is an [example configuration](./01-Configuration.md#complete-example-configtoml) you can use to get your config file started. Most of the fields have sensible defaults; you can use the [configuration reference](./01-Configuration.md#reference) to determine which fields you don't need to explicity set if you wish.

Once your keys, certs, and config file are ready, and your Sommelier node is running, refer to the [Quickstart section](#quickstart) above to start Steward!
