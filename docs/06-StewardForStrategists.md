# Steward for Strategists

## Installation

If you plan to use steward for development and testing you will need to install it first. You can follow the steps outlined [in the README](../README.md).

## Testing and Development

Steward can be run locally for testing and development purposes. It has a simulate mode that comes with two useful features:

1. Encode only, which simply takes a `ScheduleRequest` and returns the ABI encoded call data. 
2. Transaction simulation, which uses Tenderly to simulate the actual transaction execution against a contract. This allows the strategist confirm they get the expected state changes from their call data parameters.

To use the simulation features you will need to have a Tenderly account, project, and API key. You can sign up for a free account at [Tenderly](https://tenderly.co/). You will also need to install the `steward` binary. Installation steps can be found [here](https://github.com/PeggyJV/steward?tab=readme-ov-file#installation). You may need to build from source if you are planning to use it on your local machine. Windows has not been tested.

To start the simulate server with Tenderly integration run the following:

```bash
steward encode start
```

To get call data encoding only, add the `--encode-only` flag:

```bash
steward encode start --encode-only
```

By default the server will run on port 5734.

### Sending Requests 

The request type for the simulate server is a wrapper around a normal`ScheduleRequest` used in production:

```bash
/*
 * Represents a simulated function call to a particular Cellar
 */
 message SimulateRequest {
    ScheduleRequest request = 1;
    /// Whether to simply encode and return the contract call data, skipping the Tenderly simulation
    bool encode_only = 2;
}

message SimulateResponse {
    /// The encoded contract call
    string encoded_call = 1;
    /// The response body from the Tenderly simulation
    string response_body = 2;
}
```

The server can be called with any gRPC client, such as `grpcurl`:

```bash
# Get the ABI endcoded call data for a simple contract call
grpcurl \
    --plaintext \
    -d '{"request": {"cellar_id": "0x0000000000000000000000000000000000000000", "cellar_v2_5": {"function_call": {"lift_shutdown": {}}}}, "encode_only": true}' \
    localhost:5734 \
    steward.v4.SimulateContractCallService/Simulate

# Output:
# {
#   "encodedCall": "5e2c576e"
# }
#
```

## Production

In production, all requests to Steward instances must be authenticated via TLS and authorized for each Cellar. This ensures that only authorized strategists can manage Cellars, and only the Cellars they approved to submit recommendations for.

Though there are no-code options progress, it is currently required for strategists to write code for building and submitting Steward requests. The Steward API is defined in a number of protobuf files which can be used to generate client code in the Strategist's language of choice.

Endpoints for each Steward instance can be queried from the chain using the Sommelier protos via the `pubsub.v1.Query/QuerySubscribers` method.

A package for constructing requests using the Builder Pattern is provided for the Go language and can be found [here](https://github.com/peggyjv/steward/main/go/builder).

### Prerequisites

Below is a checklist of items a strategist must complete before being able to successfully send recommendation to Sommelier for a Cellar. 


- Generate a self-signed certificate and CA pair. Steps to generate these cryptographic materials can be found in the [Generating Certificates doc](./04-GeneratingCertificates.md).
- Register on-chain as a Publisher via governance with the endpoint from which you will send requests, and the CA you generated. The steps to submit an AddPublisher proposal can be found [here]()
- Deploy a Cellar to the desired EVM chain.
- Change the owner of the Cellar to the bridge contract for the respective chain. The correct owner addresses for each chain can be found [here]().
- Acquire authorization to manage the Cellar via governance. The steps to submit an AddManagedCellarIDs proposal can be found [here]().
- Build a TLS-enabled gRPC client with the key and self-signed certificate signed by the CA of your Publisher registration.


### Examples

Go code examples can be found in the [builder package](https://github.com/peggyjv/steward/main/go/builder/examples). If you are using a different language, please request examples from the Sommelier team and we will work to assist you. 
