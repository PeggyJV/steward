#!/usr/bin/env bash

set -e

REPO_ROOT=$(git rev-parse --show-toplevel)
GO_OUT=$REPO_ROOT/steward_proto_go
PROTO_PATH=$REPO_ROOT/proto

echo
echo Building Go bindings. Output will be in ./steward_proto_go/steward_proto
protoc --proto_path=$PROTO_PATH \
	--go_out=$GO_OUT/ \
	--go-grpc_out=$GO_OUT/ \
    $PROTO_PATH/*.proto \
	$PROTO_PATH/adaptors/*.proto \
	$PROTO_PATH/adaptors/*/*.proto

echo Done!
