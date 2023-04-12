#!/usr/bin/env bash

set -e

# Quick and dirty script to build the API docs.

PWD=$(pwd)

mkdir -p docs/api/adaptors/uniswap docs/api/adaptors/aave docs/api/adaptors/compound

docker run --platform linux/amd64 --rm -v $PWD/docs/api:/out -v $PWD/proto:/protos pseudomuto/protoc-gen-doc --doc_opt=markdown,steward.md steward.proto

docker run --platform linux/amd64 --rm -v $PWD/docs/api:/out -v $PWD/proto:/protos pseudomuto/protoc-gen-doc --doc_opt=markdown,cellar_v2.md cellar_v2.proto

docker run --platform linux/amd64 --rm -v $PWD/docs/api:/out -v $PWD/proto:/protos pseudomuto/protoc-gen-doc --doc_opt=markdown,debt_token.md adaptors/aave/debt_token.proto

docker run --platform linux/amd64 --rm -v $PWD/docs/api:/out -v $PWD/proto:/protos pseudomuto/protoc-gen-doc --doc_opt=markdown,aave_v2_collateral.md adaptors/aave/aave_v2_enable_asset_as_collateral_adaptor.proto

mv docs/api/debt_token.md docs/api/adaptors/aave/
mv docs/api/aave_v2_collateral.md docs/api/adaptors/aave/
