#!/usr/bin/env bash

set -e

REPO_ROOT=$(git rev-parse --show-toplevel)

# Build Rust bindings
cargo run --package steward_proto_build --release

# Build Go bindings
GO_OUT=$REPO_ROOT/steward_proto/go

protoc --proto_path=$REPO_ROOT/steward_proto/proto \
	-I=$GO_OUT \
	--go_out=$GO_OUT/ \
	--go-grpc_out=$GO_OUT/ \
       	steward.proto
