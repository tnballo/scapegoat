<br><p align="center"><img src="https://raw.githubusercontent.com/tnballo/scapegoat/master/img/scapegoat.svg" width="333" alt="scapegoat"></p><br>
# scapegoat

[![crates.io](https://img.shields.io/crates/v/scapegoat.svg)](https://crates.io/crates/scapegoat)
[![docs.rs](https://docs.rs/scapegoat/badge.svg)](https://docs.rs/scapegoat/)
[![GitHub Actions](https://github.com/tnballo/scapegoat/workflows/test/badge.svg)](https://github.com/tnballo/scapegoat/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](https://github.com/tnballo/scapegoat/blob/master/LICENSE)
[![Unsafe-Zero-Percent](https://img.shields.io/badge/Unsafety-0%25-brightgreen.svg)](https://github.com/tnballo/scapegoat/blob/master/src/lib.rs#L223)

Ordered set and map data structures via an arena-based [scapegoat tree](https://people.csail.mit.edu/rivest/pubs/GR93.pdf) (memory-efficient, self-balancing binary search tree).

* Safe: `#![forbid(unsafe_code)]`.
* Embedded-friendly: `!#[no_std]` by default.
* Validated via differential fuzzing, against the standard library's `BTreeSet` and `BTreeMap`.

### About

Three APIs:

* Ordered Set API ([`SGSet`](crate::SGSet)) - subset of [`BTreeSet`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeSet.html) nightly.
* Ordered Map API ([`SGMap`](crate::SGMap)) - subset of [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) nightly.
* Binary Tree API ([`SGTree`](crate::SGTree)) - backend for the previous two.

Strives for two properties:

* **Maximal safety:** strong [memory safety](https://tiemoko.com/blog/blue-team-rust/) guarantees, hence `#![forbid(unsafe_code)]`.
    * **Compile-time safety:** no `unsafe` (no raw pointer dereference, etc.).
    * **Debug-time safety:** `debug_assert!` for logical invariants exercised in testing.
    * **Runtime safety:** no interior mutability (e.g. no need for `Rc<RefCell<T>>`'s runtime check).

* **Minimal footprint:** low resource use, hence `!#[no_std]`.
    * **Memory-efficient:** nodes have only child index metadata, node memory is re-used.
    * **Recursion-free:** all operations are iterative, so stack use is fixed and runtime is minimized.
    * **Zero-copy:** rebuild/removal re-point in-place, nodes are never copied or cloned.

Other features:

* **Generic:** map keys and set elements can be any type that implements the `Ord` trait.
* **Arbitrarily mutable:** elements can be insert and removed, map values can be mutated.

### Usage

`SGMap` non-exhaustive, `!#[no_std]` API example (would work identically for `std::collections::BTreeMap`):

```rust
use scapegoat::SGMap;
use smallvec::{smallvec, SmallVec};

const REF_BUF_LEN: usize = 5;

let mut example = SGMap::new();
let mut stack_str = "your friend the";

// Insert "dynamically" (as if heap)
example.insert(3, "the");
example.insert(2, "don't blame");
example.insert(1, "Please");
example.insert(4, "borrow checker");

// Ordered reference iterator
assert!(example
    .iter()
    .map(|(_, v)| *v)
    .collect::<SmallVec<[&str; REF_BUF_LEN]>>()
    .iter()
    .eq(["Please","don't blame","the","borrow checker"].iter()));

// Container indexing
assert_eq!(example[&3], "the");

// Fast (no search) head removal
let please_tuple = example.pop_first().unwrap();
assert_eq!(please_tuple, (1, "Please"));

// By-predicate removal (iterates all entries)
example.retain(|_, v| !v.contains("a"));

// Extension
let iterable: SmallVec<[(isize, &str); REF_BUF_LEN]> =
    smallvec![(1337, "safety!"), (0, "Leverage"), (100, "for")];
example.extend(iterable.into_iter());

// Value mutation
if let Some(three_val) = example.get_mut(&3) {
    *three_val = &mut stack_str;
}

// New message :)
assert!(example
    .into_values()
    .collect::<SmallVec<[&str; REF_BUF_LEN]>>()
    .iter()
    .eq(["Leverage","your friend the","borrow checker","for","safety!"].iter()));
```

Additional [examples here](https://github.com/tnballo/scapegoat/blob/master/examples/README.md).

### Configuring a Stack Storage Limit

The maximum number of stack-stored elements (set) or key-value pairs (map/tree) is determined at compile-time, via the environment variable `SG_MAX_STACK_ELEMS`.
[Valid values](https://docs.rs/smallvec/1.7.0/smallvec/trait.Array.html#foreign-impls) include the range `[0, 32]` and powers of 2 up to `1,048,576`.
For example, to store up to `2048` items on the stack:

```bash
export SG_MAX_STACK_ELEMS=2048
cargo build --release
```

Please note:

* If the `SG_MAX_STACK_ELEMS` environment variable is not set, it will default to `1024`.

* For any system with heap memory: the first `SG_MAX_STACK_ELEMS` elements are stack-allocated and the remainder will be automatically heap-allocated.

* For embedded systems with only stack memory: `SG_MAX_STACK_ELEMS` is a hard maximum - attempting to insert beyond this limit will cause a panic.
    * Use feature `high_assurance` to ensure error handling and avoid panic (see below).

> **Warning:**
> Although stack usage is constant (no recursion), a stack overflow can happen at runtime if `SG_MAX_STACK_ELEMS` (configurable) and/or the stored item type (generic) is too large.
> Note *stack* overflow is distinct from *buffer* overflow (which safe Rust prevents).
> Regardless, you must test to ensure you don't exceed the stack frame(s) size limit of your target platform.
> Rust only supports stack probes on x86/x64, although [creative linking solutions](https://blog.japaric.io/stack-overflow-protection/) have been suggested for other architectures.

For more advanced configuration options, see [the documentation here](https://github.com/tnballo/scapegoat/blob/master/CONFIG.md).

### The `high_assurance` Feature

For embedded use cases prioritizing robustness (or [kernelspace](https://lkml.org/lkml/2021/4/14/1099) code), the `high_assurance` feature can be enabled with the standard `Cargo.toml` declaration:

```rust
[dependencies]
scapegoat = { version = "^1.7", features = ["high_assurance"] }
```

Enabling this feature makes two changes:

1. **Front-end, API Tweak:** `insert` and `append` APIs now return `Result`. `Err` indicates stack storage is already at maximum capacity, so caller must handle. No heap use, no panic potential on insert.

2. **Back-end, Integer Packing:** Because the fixed/max size of the stack arena is known, indexing integers (metadata stored at every node!) can be size-optimized. This memory micro-optimization honors the original design goals of the scapegoat data structure.

That second change is a subtle but interesting one.
Example of packing saving 53% (31 KB) of RAM usage:

```rust
use core::mem::size_of;
use scapegoat::SGMap;

// If you're on a 64-bit system, you can compile-time check the below numbers yourself!
// Just do:
//
// $ cargo test --doc
// $ cargo test --doc --features="high_assurance"
//
// One command per set of `cfg` macros below.
// Internally, this compile-time struct packing is done with the `smallnum` crate:
// https://crates.io/crates/smallnum

// This code assumes `SG_MAX_STACK_ELEMS == 1024` (default)
let temp: SGMap<u64, u64> = SGMap::new();
let other_features_enabled = cfg!(any(feature = "fast_rebalance", feature = "low_mem_insert"));
if temp.capacity() == 1024 && (!other_features_enabled) {

    // Without packing
    #[cfg(target_pointer_width = "64")]
    #[cfg(not(feature = "high_assurance"))] // Disabled
    {
        assert_eq!(size_of::<SGMap<u64, u64>>(), 57_440);
    }

    // With packing
    #[cfg(target_pointer_width = "64")]
    #[cfg(feature = "high_assurance")]  // Enabled
    {
        assert_eq!(size_of::<SGMap<u64, u64>>(), 26_688);
    }
}
```

### Considerations

#### General Goals

This project is an exercise in safe, portable data structure design.
The goal is to offer embedded developers familiar, ergonomic APIs on resource constrained systems that otherwise don't get the luxury of dynamic collections.
Without sacrificing safety.

`scapegoat` is not as fast or mature as the [standard library's `BTreeMap`/`BTreeSet`](http://cglab.ca/~abeinges/blah/rust-btree-case/) (benchmarks via `cargo bench`).
The standard library has been heavily optimized for cache performance.
This library is optimized for low, stack-only memory footprint.
It offers:

* **Best-effort Compatibility:** APIs are a subset of `BTreeMap`'s/`BTreeSet`'s, making it a somewhat "drop-in" replacement for `!#[no_std]` systems. Please [open an issue](https://github.com/tnballo/scapegoat/issues) if an API you need isn't yet supported.

* **Dynamic Validation:** [Coverage-guided differential fuzzing](https://github.com/tnballo/scapegoat/blob/master/fuzz/README.md) is used to demonstrate that this implementation is logically equivalent and equally reliable.

* **Tunable Performance:** A [single floating point value](https://github.com/tnballo/scapegoat/blob/master/CONFIG.md#tuning-the-the-trees-a-factor) optimizes relative performance of `insert`, `get`, and `remove` operation classes. And it can be changed at runtime.

#### Algorithmic Complexity

Space complexity is always `O(n)`.

| Operation | Average Case | Worst Case |
| --- | --- | --- |
| `get` | `O(log n)` | `O(log n)` |
| `insert` | `O(log n)` | Amortized `O(log n)` |
| `remove` | `O(log n)` | Amortized `O(log n)` |

The [`low_mem_insert`](https://github.com/tnballo/scapegoat/blob/master/CONFIG.md#the-low_mem_insert-feature) and [`fast_rebalance`](https://github.com/tnballo/scapegoat/blob/master/CONFIG.md#the-fast_rebalance-feature) features can be used to fine-tune tradeoffs of memory usage and speed.

#### Memory Footprint Demos

* [Code size demo](https://github.com/tnballo/scapegoat/blob/master/misc/min_size/README.md) - `SGMap<usize, usize>` with `insert`, `get`, and `remove` called: **18.8KB** for an x86-64 binary. Caveat: you'll likely want to use more than 3 functions, resulting in more executable code getting included.

* [Stack space demo](https://github.com/tnballo/scapegoat/blob/master/examples/tiny_map.rs) - `SGMap<u8, u8>` with a 256 pair capacity: **2.6KB** storage cost. Caveat: 2-3x more stack space is required for runtime book keeping (e.g. rebalancing).

#### Trusted Dependencies

This library has three dependencies, each of which have no dependencies of their own (e.g. exactly three total dependencies).

* [`smallvec`](https://crates.io/crates/smallvec) - `!#[no_std]` compatible `Vec` alternative. Used in Mozilla's Servo browser engine.
* [`micromath`](https://crates.io/crates/micromath) - `!#[no_std]`, `#![forbid(unsafe_code)]` floating point approximations.
* [`smallnum`](https://crates.io/crates/smallnum) - `!#[no_std]`, `#![forbid(unsafe_code)]` integer packing.

### License and Contributing

Licensed under the [MIT license](https://github.com/tnballo/scapegoat/blob/master/LICENSE).
Contributions are welcome!
