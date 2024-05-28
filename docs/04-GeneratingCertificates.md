# Generating Certificates

This document will walk you through one way to generate a CA and self-signed certificate for use with Steward.

Two-way trust must be established between Steward and the Strategy Provider client for each request. To accomplish this, both the Server and Client will need TLS certificates and the CA certificate of the other party (Steward needs the client's CA, and the client needs Steward's CA). To make generating Steward's certificates easier, we've provided a convenience script called `gen_certs.sh` which can be found [here](../scripts/gen_certs.sh). The script takes one optional flag for the output location of the certificates and their signing keys. The client CA will be set by default to Seven Seas' CA for now.

## Steps

0. Ensure you have OpenSSL installed.

This script requires OpenSSL to be installed; it will not work with LibreSSL.

Note that LibreSSL still uses the binary name `openssl`. You'll know if you have the wrong binary if `openssl version` says "LibreSSL". This script was tested against version `OpenSSL 1.1.1n`.

1. Run the following command from the root of this repository

```
scripts/gen_certs.sh -o some/output/location/
```

If you don't set the output path flag, certificates and keys will be created in the current working directory.

2. Fill out the Server CA information

After running this command you run through a series of prompts. The first will be for information pertaining to the certificate authority like geographical location, email, etc. Set your CN to your domain name, and optionally set the OU for identification purposes. The rest of the fields can be safely left blank.

3. Fill out the Server Certificate information

A similar set of prompts will ask for the same information for the Server (Steward) certificate which will be signed by the CA. As before, set your CN to your domain name, and optionally set the OU for identification purposes. Please leave the password blank. The rest of the fields can also be safely left blank.

4. Provide your domain name

The final two prompts will ask for the domain name where Steward will be publically hosted. At least one of these MUST be set for Steward to start up with the certificate. Please do not use an IP address.

5. (Optional) View your signed certificate

You can view your certificate plaintext with the following command:

```
openssl x509 -text -in <output_path>/server.crt
```

Valid TLS certificates will have "Version 3" near the top of the certificate plaintext. You should also see your domain name as a Subject Alternative Name.

6. Provide your CA on-chain by creating or updating your Subscriber

The x/pusbub module on the Sommelier chain acts as a Steward registry, referring to each registration as a Subscriber.

Subscribers can be created or updated with the `AddSubscriber` message. You can either use the `sommelier` CLI or the `steward` CLI.

A. To add a subscriber with the `sommelier` CLI, follow the steps outlined [in this doc](./05-PubsubForValidators.md)

B. To use the `steward` CLI, you can run the following command:

```bash
# use the same config.toml file you use in production
steward --config <config.toml> pubsub add-subscriber --ca-path <path to CA file> --push-url <your steward endpoint>
```

As a reminder, your "push URL" is the combination of the fully qualified domain name of your production Steward instance and the port configured for your Steward server. For example `steward.example.com:5734`.

