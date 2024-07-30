#!/bin/bash

set -e

# this is a little fragile because it assumes the [package] section will be at the
# top of the Cargo.toml
BINARY_VERSION_STRING=$(cat Cargo.toml | grep "^\(version = \"\)\([0-9]\+\)\.\([0-9]\+\)\.\([0-9]\+\)\(?:-\([0-9A-Za-z-]\+\(?:\.[0-9A-Za-z-]\+\)*\)\)\?\(?:\+[0-9A-Za-z-]\+\)\?" | head -1)
# above output is 'version = "x.x.x"' so we trim the leading and trailing characters
BINARY_VERSION=${BINARY_VERSION_STRING:11:-1}
# trim the leading 'v' from the tag
RELEASE_VERSION=${GITHUB_REF_NAME: 1}

echo "Release version: $RELEASE_VERSION"
echo "Binary version: $BINARY_VERSION"

if [ "$RELEASE_VERSION" != "$BINARY_VERSION" ]; then
    >&2 echo "Version mismatch. Has the binary version been bumped?"
    exit 1
fi
