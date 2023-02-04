#!/usr/bin/env bash

set -e

# Quick and dirty script to build the API docs.

PWD=$(pwd)

mkdir -p docs/api/adaptors/uniswap docs/api/adaptors/aave docs/api/adaptors/compound

docker run --platform linux/amd64 --rm -v $PWD/docs/api:/out -v $PWD/proto:/protos pseudomuto/protoc-gen-doc --doc_opt=markdown,steward.md steward.proto

docker run --platform linux/amd64 --rm -v $PWD/docs/api:/out -v $PWD/proto:/protos pseudomuto/protoc-gen-doc --doc_opt=markdown,aave_v2_stablecoin.md aave_v2_stablecoin.proto

docker run --platform linux/amd64 --rm -v $PWD/docs/api:/out -v $PWD/proto:/protos pseudomuto/protoc-gen-doc --doc_opt=markdown,cellar_v1.md cellar_v1.proto

docker run --platform linux/amd64 --rm -v $PWD/docs/api:/out -v $PWD/proto:/protos pseudomuto/protoc-gen-doc --doc_opt=markdown,cellar_v2.md cellar_v2.proto

docker run --platform linux/amd64 --rm -v $PWD/docs/api:/out -v $PWD/proto:/protos pseudomuto/protoc-gen-doc --doc_opt=markdown,base.md adaptors/base.proto

docker run --platform linux/amd64 --rm -v $PWD/docs/api:/out -v $PWD/proto:/protos pseudomuto/protoc-gen-doc --doc_opt=markdown,common.md common.proto

docker run --platform linux/amd64 --rm -v $PWD/docs/api:/out -v $PWD/proto:/protos pseudomuto/protoc-gen-doc --doc_opt=markdown,vesting_simple.md adaptors/vesting_simple.proto

docker run --platform linux/amd64 --rm -v $PWD/docs/api:/out -v $PWD/proto:/protos pseudomuto/protoc-gen-doc --doc_opt=markdown,uniswap_v3.md adaptors/uniswap/uniswap_v3.proto

docker run --platform linux/amd64 --rm -v $PWD/docs/api:/out -v $PWD/proto:/protos pseudomuto/protoc-gen-doc --doc_opt=markdown,a_token.md adaptors/aave/a_token.proto

docker run --platform linux/amd64 --rm -v $PWD/docs/api:/out -v $PWD/proto:/protos pseudomuto/protoc-gen-doc --doc_opt=markdown,debt_token.md adaptors/aave/debt_token.proto

docker run --platform linux/amd64 --rm -v $PWD/docs/api:/out -v $PWD/proto:/protos pseudomuto/protoc-gen-doc --doc_opt=markdown,c_token.md adaptors/compound/c_token.proto

mv docs/api/base.md docs/api/adaptors/
mv docs/api/a_token.md docs/api/adaptors/aave/
mv docs/api/debt_token.md docs/api/adaptors/aave/
mv docs/api/c_token.md docs/api/adaptors/compound/
mv docs/api/uniswap_v3.md docs/api/adaptors/uniswap/
