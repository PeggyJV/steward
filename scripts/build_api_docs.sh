#!/usr/bin/env bash

set -e

echo
echo Generating proto API docs. Output will be in docs/api
PWD=$(pwd)
docker run --platform linux/amd64 --rm -v $PWD/docs/api:/out -v $PWD/proto/steward/v3:/protos pseudomuto/protoc-gen-doc --doc_opt=markdown,steward_api_doc.md
echo Done!
echo
