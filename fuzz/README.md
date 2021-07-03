# Coverage-guided Fuzzing Strategy

All targets use [`libfuzzer` via `cargo-fuzz`](https://rust-fuzz.github.io/book/introduction.html).
This repo's [`run_fuzz.sh`](../run_fuzz.sh) is a convenience wrapper that automatically starts multi-process fuzzing campaigns.
Usage:

```
./run_fuzz.sh <target_name>
```

### Internal Arena

The arena (internal-use only) relies on a non-standard `Vec` implemented via a dependency, so the goal of fuzzing is to verify our use of the safety and reliability of this 3rd-party implementation.
For structure-aware fuzzing:

```
cargo fuzz run sg_arena
```

### `SGSet` APIs

The goal of fuzzing is to ensure "lock step" with `std::collections::BTreeSet`.
For structure-aware, differential fuzzing:

```
cargo fuzz run sg_set
```

### `SGMap` APIs

The goal of fuzzing is to ensure "lock step" with `std::collections::BTreeMap`.
For structure-aware, differential fuzzing:

```
cargo fuzz run sg_map
```

### `SGTree` APIs

Both the `SGSet` and `SGMap` APIs are build atop `SGSet`, so their respective targets already provide `SGTree` coverage.