#!/usr/bin/env bash

set -e

PWD=$(pwd)
docker run --platform linux/amd64 --rm -v $PWD/docs/api:/out -v $PWD/proto:/protos pseudomuto/protoc-gen-doc --doc_opt=markdown,steward_api_doc.md
