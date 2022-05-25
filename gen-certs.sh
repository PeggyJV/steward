#!/bin/bash

set -e

# These commands are correct for generating certificates using openssl 1.1.1l
# Check your version by running `openssl version` and if it's LibreSSL, install
# the proper version of openssl using a tool like homebrew

VERSION=$(openssl version)

if [[ "$VERSION" == *"Libre"* ]]; then
	echo "This script is not compatible with LibreSSL."
	exit
fi

while getopts o: flag
do
    case "${flag}" in
        o) output_location=${OPTARG};;
    esac
done

if [ -z "$output_location" ]; then
	output_location=.
fi

temp=$output_location/temp
mkdir -p $temp

# Create the private keys

openssl ecparam -name secp384r1 -noout -out $temp/server_ca_key_non-pkcs8.pem -genkey
openssl ecparam -name secp384r1 -noout -out $temp/server_key_non-pkcs8.pem -genkey

# Create PKCS8 versions of the private keys, which the Rust libraries expect

openssl pkcs8 -in $temp/server_ca_key_non-pkcs8.pem -out $output_location/server_ca_key_pkcs8.pem -topk8 -nocrypt
openssl pkcs8 -in $temp/server_key_non-pkcs8.pem -out $output_location/server_key_pkcs8.pem -topk8 -nocrypt

# Create CA

echo
echo "You're going to be asked to fill in fields for the server CA."
echo "Apart from identifying info (CN, OU), these values don't really matter, just go with the defaults."
echo "Common Name should be your domain name."
echo
read -p "Press enter to continue."
echo
echo "====================="
echo "Server CA Certificate"
echo "====================="
echo

openssl req -x509 -new -key $output_location/server_ca_key_pkcs8.pem -out $output_location/server_ca.crt -sha384 -days 730

# Create CSR

echo
echo "You're going to be asked to fill in fields for the server certificate."
echo "Apart from identifying info (CN, OU), these values don't really matter, just go with the defaults."
echo "Common Name should be your domain name."
echo "Please leave password blank."
echo
read -p "Press enter to continue."
echo
echo "=================="
echo "Server Certificate"
echo "=================="
echo

openssl req -new -sha384 -key $output_location/server_key_pkcs8.pem -out $temp/server.csr

# Collect Subject Alt Name info and create extensions file
read -p "Domain name where Steward will be hosted (please do not use an IP for this value): " domain_name

# Sign the server certificate with the CA
# the v3.ext file makes sure we include the subjectAltName extensions needed

ext=$temp/v3.ext

echo subjectKeyIdentifier=hash > $ext
echo authorityKeyIdentifier=keyid,issuer >> $ext
echo basicConstraints=CA:FALSE >> $ext
echo subjectAltName=DNS:$domain_name >> $ext

openssl x509 \
	-req \
	-in $temp/server.csr \
	-CA $output_location/server_ca.crt \
	-CAkey $output_location/server_ca_key_pkcs8.pem \
	-CAcreateserial \
	-out $output_location/server.crt \
	-sha384 \
	-extfile $temp/v3.ext \
	-days 730

# Clean up the certificate requests

rm -r $temp || true
