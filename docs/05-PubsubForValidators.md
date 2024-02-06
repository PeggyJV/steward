# The x/pubsub Module for Validators

Before Sommelier v7, the Steward Registry was used to record the CA and server endpoint of all steward processes. This database was then used by Strategy Providers (SPs) to send requests to each steward instance. 

The Steward Registry has been deprecated in favor of a (mostly) generalized pubslisher/subscriber registry Cosmos SDK module, [x/pubsub](https://github.com/peggyjv/sommelier/tree/main/x/pubsub).

For more information on x/pubsub itself, please refer to the [protocol documentation]()

## Registering Steward as a Subscriber

A `Subscriber` is an entity representing the identity of your steward instance. It contains your Sommelier address (either the validator or delegate address), server Certificate Authority, and the Steward server endpoint to which Strategy Providers (publishers) will send requests. 
 
### Prerequisites

- The latest `sommelier` binary release
- Key, CA, and self-signed certificate generated using the steps in [the previous doc](./04-GeneratingCertificates.md).
- Your validator or delegate Sommelier address registered on the gravity module
- A domain and port that will or already points to your steward serverwith
- Firewall configured to allow requests to the former

### Steps

A subscriber can be created using the following CLI command

```bash
sommelier tx pubsub <subscriber_address> [<ca_cert_file>] [<push_url>]
```

*The signer must match the address argument*. If you are using your delegate address, sign the transaction with your delegate key, etc.

Note that even though the CA file and push URL args are "optional" in the command, they are *required* in order to facilitate Cellar calls from strategists.

To update an existing subscriber, such as to replace an expired CA cert, simply run the same command for the same `subscriber_address`. 

If a subscriber needs to be altogether replaced, such as if a change in keys occurs, the old subscriber can be removed using the counterpart to the previous command:

```bash
sommelier tx pubsub <subscriber_address>
```

Once again, the signer of the transaction must match the `subscriber_address` argument.

## Overriding Default Subscriptions

For convenience, default subscriptions exist in x/pubsub that map Cellar IDs to their agreed upon "default" publisher. Steward will load these subscriptions into its Publisher Trust Cache automatically once configured. It is possible that Sommelier participants may choose to delegate managing authorty to a different publisher for a particular Cellar. This can be accomplished by creating a `SubscriberIntent`, whereby a validator registers their intent to receive requests for the target Cellar from an alternative publisher. Steward will override any default subscriptions in its cache with the respective subscriber intents reigstered by the owning validator.

To create a subscriber intent use the following command:

```bash
sommelier tx pubsub add-subscriber-intent <subscription_id> <subscriber_address> <publisher_domain> 
```

Where subscriber ID is the concatenated Chain ID and checksum contract address of the Cellar. See the protocol docs for more details on the different entities in x/pubsub.


## Blocking a Publisher

In certain cases such as if a publisher's keys are compromised or the publisher has become malicious it is desirable to "block" requests from the publisher's domain. Simply add the publisher's registered domain to Steward's [publisher domain block list](https://github.com/PeggyJV/steward/blob/collin/v4-docs/docs/01-Configuration.md#publisher_domain_block_list) 
