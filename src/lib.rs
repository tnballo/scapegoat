/*!
Ordered set and map data structures via an arena-based [scapegoat tree](https://people.csail.mit.edu/rivest/pubs/GR93.pdf) (memory-efficient, self-balancing binary search tree).

This library is `#![no_std]` compatible by default, strictly `#![forbid(unsafe_code)]`, and verified using differential fuzzing.

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
[Valid values](https://docs.rs/smallvec/1.6.1/smallvec/trait.Array.html#implementors) are in the range `[0, 32]` and powers of 2 up to `1,048,576`.
For example, to store up to `2048` items on the stack:

```bash
export SG_MAX_STACK_ELEMS=2048
cargo build --release
```

Please note:

* If the `SG_MAX_STACK_ELEMS` environment variable is not set, it will default to `1024`.
* For embedded systems without dynamic (heap) memory: `SG_MAX_STACK_ELEMS` is a hard maximum - attempting to insert beyond this limit will cause a panic.
* For any system with dynamic memory: the first `SG_MAX_STACK_ELEMS` elements are stack-allocated and the remainder will be automatically heap-allocated (no panic).

### Trusted Dependencies

This library has two dependencies, each of which have no dependencies of their own (e.g. exactly two total dependencies).
Both dependencies were carefully chosen.

* [`smallvec`](https://crates.io/crates/smallvec) - `!#[no_std]` compatible `Vec` alternative. Used in Mozilla's Servo browser engine.
* [`micromath`](https://crates.io/crates/micromath) - `!#[no_std]`, `#![forbid(unsafe_code)]` floating point approximations.

### Considerations

This project is an exercise in safe data structure design.
It's not as mature, fast, or memory efficient as the [standard library's `BTreeMap`/`BTreeSet`](http://cglab.ca/~abeinges/blah/rust-btree-case/).
It does, however, offer:

* **Best-effort Compatibility:** APIs are a subset of `BTreeMap`'s/`BTreeSet`'s, making it a somewhat "drop-in" replacement for `!#[no_std]` systems. Please [open an issue](https://github.com/tnballo/scapegoat/issues) if an API you need isn't yet supported!

* **Dynamic Verification:** [Coverage-guided differential fuzzing](https://github.com/tnballo/scapegoat/blob/master/fuzz/README.md) is used to verify that this implementation is logically equivalent and equally reliable.

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

mod map;
pub use crate::map::SGMap;

mod set;
pub use crate::set::SGSet;
