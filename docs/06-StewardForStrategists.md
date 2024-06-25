# Steward for Strategists

## Installation

If you plan to use steward for development and testing you will need to install it first. You can follow the steps outlined [in the README](../README.md).

## Encode Mode

Steward can be started in encode mode, which runs a gRPC server that accepts call data params and returns the final hex-encoded contract call data. This is meant intended as a way to test parameter construction with the protos.

Strategists can run steward locally and send requests without needing TLS authentication. To Start the server simply run:

```bash
steward encode start
```

Encode mode is only available in steward 4.2.3 and later.

### Sending Requests 

The request type for the encoding server is a subset of the fields of a regular `ScheduleRequest` used in production requiring only a cellar ID and call data parameters:

```protobuf
# steward/proto/steward/v4/steward.proto

message EncodeRequest {
    string cellar_id = 1;
    // The data from which the desired contract function will be encoded
    oneof call_data {
        AaveV2Stablecoin aave_v2_stablecoin = 2;
        CellarV1 cellar_v1 = 3;
        CellarV2 cellar_v2 = 4;
        CellarV2_2 cellar_v2_2 = 5;
        CellarV2_5 cellar_v2_5 = 6;
    }
}

message EncodeResponse {
    // The encoded contract call
    string encoded_call = 1;
}
```

The server can be called with any gRPC client, such as `grpcurl`:

```bash
grpcurl \
    --plaintext \
    -d '{"cellar_id": "0x0000000000000000000000000000000000000000", "cellar_v2_5": {"function_call": {"lift_shutdown": {}}}}' \
    localhost:5734 \
    steward.v4.EncodingService/Encode

# Output:
# {
#   "encodedCall": "5e2c576e"
# }
#
```
