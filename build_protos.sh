#!/usr/bin/env bash

set -ex

REPO_ROOT=$(git rev-parse --show-toplevel)

echo Building Rust bindings. Output will be in ./steward_proto_rust/prost
cargo run --package steward_proto_rust_build --release

GO_OUT=$REPO_ROOT/steward_proto_go
PROTO_PATH=$REPO_ROOT/proto

echo
echo Building Go bindings. Output will be in ./steward_proto_go/steward_proto
protoc --proto_path=$PROTO_PATH \
	--go_out=$GO_OUT/ \
	--go-grpc_out=$GO_OUT/ \
        $PROTO_PATH/steward.proto \
	$PROTO_PATH/aave_v2_stablecoin.proto

echo Done.
