# Generating Certificates

This document will walk you through one way to generate a CA and self-signed certificate for use with Steward.  

Two-way trust must be established between Steward and the Strategy Provider client for each request. To accomplish this, both the Server and Client will need TLS certificates and the CA certificate of the other party (Steward needs the client's CA, and the client needs Steward's CA). To make generating Steward's certificates easier, we've provided a convenience script called `gen-script.sh` which can be found [here](https://github.com/peggyjv/steward/gen-script.sh). The script takes one optional flag for the output location of the certificates and their signing keys. 

## Steps

0. Ensure you have OpenSSL installed.

This script requires OpenSSL to be installed; it will not work with LibreSSL.

Note that LibreSSL still uses the binary name `openssl`. You'll know if you have the wrong binary if `openssl version` says "LibreSSL".

1. Run the following command from the root of this repository

```
./gen-certs.sh -o some/output/location/
```

If you don't set the output path flag, certificates and keys will be created in the current working directory.

2. Fill out the Server CA information

After running this command you run through a series of prompts. The first will be for information pertaining to the certificate authority like geographical location, email, etc. These fields can be safely left blank. 

3. Fill out the Server Certificate information

A similar set of prompts will ask for the same information for the Server (Steward) certificate which will be signed by the CA. These fields can also be left blank, though you may want to set the Common Name to your domain name.

4. Provide your IP and/or domain name

The final two prompts will ask for the IP and/or domain name where Steward will be publically hosted. At least one of these MUST be set for Steward to start up with the certificate.

5. (Optional) View your signed certificate

You can view your certificate plaintext with the following command:

```
openssl x509 -text -in <output path>/server.crt
```

Valid TLS certificates will have "Version 3" near the top of the certificate plaintext. You should also see your IP or domain name as a Subject Alternative Name.  

6. Test Steward with your certificate

You can use the test client certificates included in this repo to simulate a client connection to Steward. You'll need to create a TOML config file with the `[server]` section containing the paths to your generated artifacts. See the configuration reference for more info. Your file will need to look something like this:

```toml
[server]
client_ca_cert_path = "integration_tests/tls/client/test_client_ca.crt"
server_cert_path = "<output path>/server.crt"
server_key_path = "<output path>/server_key_pkcs8.pem"
```

You'll then need to start Steward and then send a gRPC request to it.

To start steward run

```bash
steward -c <path to your config file> start
```

If you see something like the following log messages, so far so good:

```bash
INFO steward::commands::cosmos_mode: Starting application
INFO steward::commands::cosmos_mode: listening on 0.0.0.0:5734
```

Then you'll need to send a gRPC request to the endpoint (in the case above, 0.0.0.0:5734).

You can use your client of preference. Here is an example using `grpcurl` with the test client certs from this repo:

```bash
grpcurl -cert integration_tests/tls/client/test_client.crt \
	-key integration_tests/tls/client/test_client_key_pkcs8.pem \
	-cacert <output path>/server_ca.crt \
	-d '' \
	localhost:5734 \
	steward.v1.ContractCall/Submit
```

When you send this request, if you get an error to establishing a connection, there is probably something wrong with your configuartion or your certificates. If you get an error internal to the business logic of Steward, you know it's working. 

An error like this one means the connection was established successfully:

```bash
# steward log
# this is steward failing to connect to the Sommelier chain, which it would not be attempting
# if your client connection wasn't working.
ERROR steward::cork: cork query client connection failed: transport error: error trying to connect: tcp connect error: Connection refused (os error 61)

# client log
ERROR:
  Code: Internal
  Message: failed to query chain to validate cellar id
``` 

7. Provide your CA to the Strategy Provider

Add your information to the [Steward Registry](https://github.com/peggyjv/steward-registry) by following the steps outlined in the README there.
 
