# Strategists Proposals

It is required of Strategists to create a Publisher entity on the Sommelier chain, which maps their cryptographic identity to the domain from which they will send requests to the chain to manage Cellars. A Publisher contains the strategist's Certificate Authority, which must sign the certificate used to establish a TLS connection with every Steward instances. It also contains the fully qualified domain name where the requests will originate from, and their SOMM address.

Each instance of Steward run by the validators will automatically load in Publisher data and create a trust cache. When a Strategist sends a request, Steward compares the certificate and signature contained in the request to it's trust cache to authenticate the strategist before processing the request.

Additionally, any new Cellars that a strategist wishes to manage must be approved by governance. These are submitted with an `AddManagedCellarIdsProposal` and `AddAxelarManagedCellarIdsProposal` respectively.

## Registering as a Publisher

To register as a Publisher, a Strategist must submit an AddPublisher governance proposal. A partially completed template of the proposal JSON can be generated using the steward CLI:

```bash
steward pubsub add-publisher --ca-path <path to CA file> --proof-url <url> --address <publisher somm address>
```

### CA Path

This is a path on the local file system to the publisher's certificate authority. This should be generated along with a self-signed certificate using the steps outligned in the [Generating Certificates doc](./06-GeneratingCertificates.md).

### Proof URL

The proof URL is a way to verify that the party submitting the proposal is the owner of the domain. It should be the full path to the hosted CA cert file in the following form: 

`https://<publisher domain>/<publisher somm address>/ca.crt`

This allows proposal voters to verify the domain, address, and CA all match the values submitted in the proposal.

### Address

The SOMM address of the publisher. It should match the address of the signer that actually submits the proposal transaction.

## Adding Managed Cellars

The Steward CLI has commands that will print partially filled proposal JSON templates.

For Cellars that reside on Ethereum, use the following command:

```bash
# USAGE 
steward cork-proposal add-managed-cellar-ids --publisher-domain <domain> [cellar ids...]

# EXAMPLE
steward cork-proposal add-managed-cellar-ids --publisher-domain domain.example.com 0x0000000000000000000000000000000000000000 0x1111111111111111111111111111111111111111

# OUTPUT
# {
#     "title": "",
#     "description": "",
#     "cellar_ids": [
#         "0x0000000000000000000000000000000000000000","0x1111111111111111111111111111111111111111"
#     ],
#     "publisher_domain": "domain.example.com",
#     "deposit": "5000000000usomm"
# }
```

Notice that the title and description are left empty. These must be filled out before submitting the proposal transaction.

Similiarly, for Cellars that reside on any other EVM chain, use the following command:

```bash
# USAGE
steward cork-proposal add-axelar-managed-cellar-ids --chain-id <id> --publisher-domain <domain> [cellar ids...]

# EXAMPLE
steward cork-proposal add-axelar-managed-cellar-ids --chain-id 10 --publisher-domain domain.example.com 0x0000000000000000000000000000000000000000 0x1111111111111111111111111111111111111111

# OUTPUT
# { 
#     "title": "",
#     "description": "",
#     "chain_id": 10,
#     "cellar_ids": [
#         "0x0000000000000000000000000000000000000000","0x1111111111111111111111111111111111111111"
#     ],
#     "publisher_domain": "domain.example.com",
#     "deposit": "5000000000usomm"
# }
```

One again, make sure to fill out he title and description. 
