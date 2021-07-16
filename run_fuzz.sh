#! /bin/bash

TARGET="$1"
FLAGS_ARR=(
    --release
    --debug-assertions
    --jobs $(nproc)
    -s address
)

cd "$(dirname "$0")"
mapfile -t TARGET_ARR < <(cargo fuzz list)

if [[ -z "$TARGET" ]] || [[ ! ${TARGET_ARR[*]} =~ "${TARGET}" ]]; then
    echo -e "Usage: $0 <target>\n"
    echo -e "Valid targets are:\n"
    printf '%s\n' "${TARGET_ARR[@]}"
    exit 1
else
    echo -e "Fuzzing $TARGET"
fi

set -x
cargo fuzz run "$TARGET" "${FLAGS_ARR[@]}"