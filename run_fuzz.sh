#! /bin/bash

TARGET="$1"
FLAGS_ARR=(
    --release
    --debug-assertions
    --jobs $(nproc)
    -s address
)

LIBFUZZER_OPTS_ARR=(
    -max_len=65536 # Default is 4096 if no corpus, 16x for more interesting API call sequences
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
cargo fuzz run "$TARGET" "${FLAGS_ARR[@]}" -- "${LIBFUZZER_OPTS_ARR[@]}"
