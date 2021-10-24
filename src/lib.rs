/*!
Ordered set and map data structures via an arena-based [scapegoat tree](https://people.csail.mit.edu/rivest/pubs/GR93.pdf) (memory-efficient, self-balancing binary search tree).

This library is `#![no_std]` compatible by default, strictly `#![forbid(unsafe_code)]`, and validated using differential fuzzing.

### About

Three APIs:

* Ordered Set API ([`SGSet`](crate::SGSet))
* Ordered Map API ([`SGMap`](crate::SGMap))
* Binary Tree API ([`SGTree`](crate::SGTree))

Strives for two properties:

* **Maximal safety:** strong [memory safety](https://tiemoko.com/blog/blue-team-rust/) guarantees.
    * **Compile-time safety:** no `unsafe` (no raw pointer dereference, etc.).
    * **Debug-time safety:** `debug_assert!` for logical invariants exercised in testing.
    * **Runtime safety:** no interior mutability (e.g. no need for `Rc<RefCell<T>>`'s runtime check).

* **Minimal footprint:** small binary with low resource use.
    * **Memory-efficient:** nodes have only child index metadata, node memory is re-used.
    * **Recursion-free:** all operations are iterative, so stack use and runtime are both minimized.
    * **Zero-copy:** rebuild/removal re-point in-place, nodes are never copied or cloned.

Other features:

* **Generic:** map keys and set elements can be any type that implements the `Ord` trait.
* **Arbitrarily mutable:** elements can be insert and removed, map values can be mutated.

### Usage

`SGMap` non-exhaustive API example (would work identically for `std::collections::BTreeMap`):

```rust
use scapegoat::SGMap;

let mut example = SGMap::new();

example.insert(3, String::from("the"));
example.insert(2, String::from("don't blame"));
example.insert(1, String::from("Please"));
example.insert(4, String::from("borrow checker"));

assert_eq!(
    example.iter().map(|(_, v)| v).collect::<Vec<&String>>(),
    vec!["Please","don't blame","the","borrow checker"]
);

assert_eq!(example[&3], "the");

let please_tuple = example.pop_first().unwrap();
assert_eq!(please_tuple, (1, String::from("Please")));

example.insert(5, String::from("! :P"));

let dont_blame = example.get_mut(&2).unwrap();
dont_blame.remove(0);
dont_blame.insert(0, 'D');

assert_eq!(
    example.into_iter().map(|(_, v)| v).collect::<Vec<String>>(),
    vec!["Don't blame","the","borrow checker","! :P"]
);
```

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

* For any system with dynamic (heap) memory: the first `SG_MAX_STACK_ELEMS` elements are stack-allocated and the remainder will be automatically heap-allocated.

* For embedded systems without dynamic memory: `SG_MAX_STACK_ELEMS` is a hard maximum - attempting to insert beyond this limit will cause a panic.
    * Use feature `high_assurance` to ensure error handling and avoid panic (see below).

> **Warning:**
> Although stack usage is constant (no recursion), a stack overflow can happen at runtime if `SG_MAX_STACK_ELEMS` (configurable) and/or the stored item type (generic) is too large.
> You must test to ensure you don't exceed the stack frame size limit of your target platform.
> Rust only supports stack probes on x86/x64, although [creative linking solutions](https://blog.japaric.io/stack-overflow-protection/) have been suggested for other architectures.

For more advanced configuration options, see [the documentation here](https://github.com/tnballo/scapegoat/blob/master/CONFIG.md).

### The `high_assurance` Feature

For embedded use cases prioritizing robustness (or [kernelspace](https://lkml.org/lkml/2021/4/14/1099) code), enabling the `high_assurance` feature makes two changes:

1. **Front-end, API Tweak:** `insert` and `append` APIs now return `Result`. `Err` indicates stack storage is already at maximum capacity, so caller must handle. No heap use, no panic potential on insert.

2. **Back-end, Integer Packing:** Because the fixed/max size of the stack arena is known, indexing integers (metadata stored at every node!) can be size-optimized. This memory micro-optimization honors the original design goals of the scapegoat data structure.

That second change is a subtle but interesting one.
Example of packing saving 53% (31 KB) of RAM usage:

```rust
use scapegoat::SGMap;
use core::mem::size_of;

// If you're on a 64-bit system, you can compile-time check the below numbers yourself!
// Just do:
//
// $ cargo test --doc
// $ cargo test --doc --features="high_assurance"
//
// One command per set of `cfg` macros below.
// Internally, this compile time struct packing is done with the `smallnum` crate:
// https://crates.io/crates/smallnum

// This code assumes `SG_MAX_STACK_ELEMS == 1024` (default)
let temp: SGMap<u64, u64> = SGMap::new();
if temp.capacity() == 1024 {

    // Without packing
    #[cfg(target_pointer_width = "64")]
    #[cfg(not(feature = "high_assurance"))]
    {
        assert_eq!(size_of::<SGMap<u64, u64>>(), 57_432);
    }

    // With packing
    #[cfg(target_pointer_width = "64")]
    #[cfg(feature = "high_assurance")]
    {
        assert_eq!(size_of::<SGMap<u64, u64>>(), 26_680);
    }
}
```

### Trusted Dependencies

This library has three dependencies, each of which have no dependencies of their own (e.g. exactly three total dependencies).

* [`smallvec`](https://crates.io/crates/smallvec) - `!#[no_std]` compatible `Vec` alternative. Used in Mozilla's Servo browser engine.
* [`micromath`](https://crates.io/crates/micromath) - `!#[no_std]`, `#![forbid(unsafe_code)]` floating point approximations.
* [`smallnum`](https://crates.io/crates/smallnum) - `!#[no_std]`, `#![forbid(unsafe_code)]` integer packing.

### Considerations

This project is an exercise in safe, portable data structure design.
It's not as mature, fast, or memory efficient as the [standard library's `BTreeMap`/`BTreeSet`](http://cglab.ca/~abeinges/blah/rust-btree-case/) (benchmarks via `cargo bench`).
It does, however, offer:

* **Best-effort Compatibility:** APIs are a subset of `BTreeMap`'s/`BTreeSet`'s, making it a somewhat "drop-in" replacement for `!#[no_std]` systems. Please [open an issue](https://github.com/tnballo/scapegoat/issues) if an API you need isn't yet supported!

* **Dynamic Validation:** [Coverage-guided differential fuzzing](https://github.com/tnballo/scapegoat/blob/master/fuzz/README.md) is used to demonstrate that this implementation is logically equivalent and equally reliable.

*/

#![forbid(unsafe_code)]
#![cfg_attr(not(any(test, fuzzing)), no_std)]
#![cfg_attr(not(any(test, fuzzing)), deny(missing_docs))]

include!(concat!(env!("OUT_DIR"), "/consts.rs"));

// Only expose arena internals for fuzzing harness
#[cfg(fuzzing)]
pub use crate::tree::{Node, NodeArena, NodeGetHelper, NodeRebuildHelper};

mod tree;
pub use crate::tree::SGTree;

#[cfg(feature = "high_assurance")]
pub use crate::tree::SGErr;

mod map;
pub use crate::map::SGMap;

mod set;
pub use crate::set::SGSet;
