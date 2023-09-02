#!/usr/bin/env bash

set -euo pipefail

script_dir=$(readlink -f $(dirname ${BASH_SOURCE[0]}))

. $script_dir/../lib.sh

new_version="$1"

function replace {
    with_log sed --in-place --file - "$1"
}

# This regex hackery will work if the `version=` field in the Cargo.toml
# begins at the line start and we consistently use workspace inheritance
# for local path dependencies.
replace Cargo.toml <<EOF
    s/^\(version\s*=\s*\)".*"/\1"$new_version"/;
    s/^\(marker_.*\s*=\s*{\s*path\s*=.*version\s*=\s*\)".*"\(.*\)/\1"$new_version"\2/;
EOF

# For now, we assume all components have the same version
replace cargo-marker/src/backend/driver.rs <<EOF
    s/\(version: "\).*\(".*\)/\1$new_version\2/;
    s/\(api_version: "\).*\(".*\)/\1$new_version\2/;
EOF

# If this is not a dev version update all mentions of `maker_*` crate dependencies
# in docs and messages.
if [[ "$new_version" != *-dev ]]
then
    # Exclude files
    excludes=(
        CHANGELOG.md
    )

    # Include files
    includes=(
        cargo-marker/src/error.rs
    )

    files=($(find . -type f -name "*.md"))

    for exclude in "${excludes[@]}"; do
        for i in "${!files[@]}"; do
            if [[ "${files[i]}" == "./$exclude" ]]; then
                unset 'files[i]'
            fi
        done
    done

    files+=("${includes[@]}")

    for file in "${files[@]}"
    do
        replace "$file" <<-EOF
            s/\(marker_\w*\s*=\s*['"]\)[^'"]*\(['"].*\)/\1$new_version\2/;
EOF
    done
fi

with_log cargo update --workspace
