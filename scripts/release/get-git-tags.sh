#!/usr/bin/env bash

set -euo pipefail

version="$1"

IFS='.' read -r major minor patch <<< "$version"

# We always put the full version tag even if this is a pre-release
echo -n "v$version"

# For suffixless stable release versions we also want to set the
# sliding `v{major}` and `v{major}.{minor}` tags so that users could
# depend on `rust-marker/marker@v0.3` or `rust-marker/marker@v1`
# version of the Github Action, or use a more flexible URL for
# the installation scripts like the following.
# https://raw.githubusercontent.com/rust-marker/marker/v0.3/scripts/release/install.sh
if [[ "$version" != *-* ]]; then
    echo -n " v$major v$major.$minor"
fi
