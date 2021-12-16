/*!
Ordered set and map data structures via an arena-based [scapegoat tree](https://people.csail.mit.edu/rivest/pubs/GR93.pdf) (memory-efficient, self-balancing binary search tree).

* Embedded-friendly: `!#[no_std]` by default.
* Safe: `#![forbid(unsafe_code)]`, including all dependencies.
* Validated via differential fuzzing, against the standard library's `BTreeSet` and `BTreeMap`.

### About

Two APIs:

* Ordered Set API ([`SgSet`](crate::SgSet)) - subset of [`BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html) nightly.
* Ordered Map API ([`SgMap`](crate::SgMap)) - subset of [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) nightly.

Strives for three properties:

* **Maximal safety:** strong [memory safety](https://tiemoko.com/blog/blue-team-rust/) guarantees, hence `#![forbid(unsafe_code)]`.
    * **Compile-time safety:** no `unsafe` (no raw pointer dereference, etc.).
    * **Debug-time safety:** `debug_assert!` for logical invariants exercised in testing.
    * **Runtime safety:** no interior mutability (e.g. no need for `Rc<RefCell<T>>`'s runtime check).

* **Minimal footprint:** low resource use, hence `!#[no_std]`.
    * **Memory-efficient:** nodes have only child index metadata, node memory is re-used.
    * **Recursion-free:** all operations are iterative, so stack use is fixed and runtime is minimized.
    * **Zero-copy:** rebuild/removal re-point in-place, nodes are never copied or cloned.

* **Fallibility**: for embedded use cases prioritizing robustness (or [kernelspace](https://lkml.org/lkml/2021/4/14/1099) code).
    * A `try_*` variant of each fallible API (e.g. `insert`, `append`, `extend`, etc.) is available.
    * **Out-Of-Memory (OOM)** `panic!` becomes avoidable: `try_*` variants return [`Result<_, SgError>`](crate::SgError).
    * Heap fragmentation doesn't impact **Worst Case Execution Time (WCET)**, this library doesn't use the heap.

Other features:

* **Generic:** map keys and set elements can be any type that implements traits [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html) and [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html).
* **Arbitrarily mutable:** elements can be inserted and removed, map values can be mutated. Safely.

### Usage

`SgMap` non-exhaustive, `!#[no_std]` API example (would work almost identically for `std::collections::BTreeMap`):

```rust
use scapegoat::SgMap;
use tinyvec::{array_vec, ArrayVec};

// This const is an argument to each generic constructor below.
// So we'll use *only the bare minimum* memory for 5 elements.
// - Stack RAM usage can be precisely controlled: per map instance (constructor call-site).
// - To save executable RAM/ROM (monomorphization!), stick to a global capacity like this.
const CAPACITY: usize = 5;

let mut example = SgMap::<_, _, CAPACITY>::new(); // BTreeMap::new()
let mut stack_str = "your friend the";

// Insert "dynamically" (as if heap)
example.insert(3, "the");
example.insert(2, "don't blame");
example.insert(1, "Please");

// Fallible insert variant
assert!(example.try_insert(4, "borrow checker").is_ok());

// Ordered reference iterator
assert!(example
    .iter()
    .map(|(_, v)| *v)
    .collect::<ArrayVec<[&str; CAPACITY]>>()
    .iter()
    .eq(["Please","don't blame","the","borrow checker"].iter()));

// Container indexing
assert_eq!(example[&3], "the");

// Head removal
let please_tuple = example.pop_first().unwrap();
assert_eq!(please_tuple, (1, "Please"));

// By-predicate removal
example.retain(|_, v| !v.contains("a"));

// Extension
let iterable = array_vec![
    [(isize, &str); CAPACITY] =>
    (1337, "safety!"), (0, "Leverage"), (100, "for")
];
example.extend(iterable.into_iter());

// Value mutation
if let Some(three_val) = example.get_mut(&3) {
    *three_val = &mut stack_str;
}

// New message :)
assert!(example
    .into_values()
    .collect::<ArrayVec<[&str; CAPACITY]>>()
    .iter()
    .eq(["Leverage","your friend the","borrow checker","for","safety!"].iter()));
```

Additional [examples here](https://github.com/tnballo/scapegoat/blob/master/examples/README.md).

### Stack Capacity: Important Context

Per the above, const generic type parameters decide collection capacity.
And thus also stack usage.
That usage is fixed:

```rust
use core::mem::size_of_val;
use scapegoat::SgMap;

let small_map: SgMap<u64, u64, 100> = SgMap::new(); // 100 item capacity
let big_map: SgMap<u64, u64, 2_048> = SgMap::new(); // 2,048 item capacity

#[cfg(target_pointer_width = "64")]
#[cfg(not(feature = "low_mem_insert"))]
#[cfg(not(feature = "fast_rebalance"))]
{
    assert_eq!(size_of_val(&small_map), 2_680); // 2.7 KB
    assert_eq!(size_of_val(&big_map), 53_328);  // 53.3 KB
}
```

The maximum supported capacity is `65_535` (e.g. `0xffff` or [`u16::MAX`](https://doc.rust-lang.org/std/primitive.u16.html#associatedconstant.MAX)) items.

> **WARNING:**
> Although stack usage is constant (no recursion), a stack overflow can happen at runtime if `N` (const generic capacity) and/or the stored item type (generic) is too large.
> Note *stack* overflow is distinct from *buffer* overflow (which safe Rust prevents).
> Regardless, you must test to ensure you don't exceed the stack frame(s) size limit of your target platform.
> Rust only supports stack probes on x86/x64, although [creative linking solutions](https://blog.japaric.io/stack-overflow-protection/) have been suggested for other architectures.

For more advanced configuration options, see [the documentation here](https://github.com/tnballo/scapegoat/blob/master/CONFIG.md).

### Trusted Dependencies

This library has three dependencies, each of which have no dependencies of their own (e.g. exactly three total dependencies).

* [`tinyvec`](https://crates.io/crates/tinyvec) - `!#[no_std]`, `#![forbid(unsafe_code)]` alternative to `Vec`.
* [`micromath`](https://crates.io/crates/micromath) - `!#[no_std]`, `#![forbid(unsafe_code)]` floating point approximations.
* [`smallnum`](https://crates.io/crates/smallnum) - `!#[no_std]`, `#![forbid(unsafe_code)]` integer abstraction.

Because this library and all dependencies are `#![forbid(unsafe_code)]`, no 3rd-party `unsafe` code is introduced into your project.

### Additional Considerations

**General Goals**

This project is an exercise in safe, portable data structure design.
The goal is to offer embedded developers familiar, ergonomic APIs on resource constrained systems that otherwise don't get the luxury of dynamic collections.
Without sacrificing safety.

`scapegoat` is not as fast or mature as the [standard library's `BTreeMap`/`BTreeSet`](http://cglab.ca/~abeinges/blah/rust-btree-case/) (benchmarks via `cargo bench`).
The standard library has been heavily optimized for cache performance.
This library is optimized for low, stack-only memory footprint.
It offers:

* **Best-effort Compatibility:** APIs are mostly a subset of `BTreeMap`'s/`BTreeSet`'s, making it a mostly "drop-in" replacement for `!#[no_std]` systems. Please [open an issue](https://github.com/tnballo/scapegoat/issues) if an API you need isn't yet supported.

* **Dynamic Validation:** [Coverage-guided differential fuzzing](https://github.com/tnballo/scapegoat/blob/master/fuzz/README.md) is used to demonstrate that this implementation is logically equivalent and equally reliable.

* **Tunable Performance:** A [single floating point value](https://github.com/tnballo/scapegoat/blob/master/CONFIG.md#tuning-the-the-trees-a-factor) optimizes relative performance of `insert`, `get`, and `remove` operation classes. And it can be changed at runtime.

**Algorithmic Complexity**

Space complexity is always `O(n)`. Time complexity:

| Operation | Average Case | Worst Case |
| --- | --- | --- |
| `get` | `O(log n)` | `O(log n)` |
| `insert` | `O(log n)` | Amortized `O(log n)` |
| `remove` | `O(log n)` | Amortized `O(log n)` |
| `first` | `O(1)` | `O(1)` |
| `last` | `O(1)` | `O(1)` |

The [`low_mem_insert`](https://github.com/tnballo/scapegoat/blob/master/CONFIG.md#the-low_mem_insert-feature) and [`fast_rebalance`](https://github.com/tnballo/scapegoat/blob/master/CONFIG.md#the-fast_rebalance-feature) features can be used to fine-tune tradeoffs of memory usage and speed.

**Memory Footprint Demos**

* [Code size demo](https://github.com/tnballo/scapegoat/blob/master/misc/min_size/README.md) - `SgMap<usize, usize, 1024>` with `insert`, `get`, and `remove` called: **16.0KB** for an x86-64 binary. Caveat: you'll likely want to use more than 3 functions, resulting in more executable code getting included.

* [Stack space demo](https://github.com/tnballo/scapegoat/blob/master/examples/tiny_map.rs) - `SgMap<u8, u8, 128>`: **1.3KB** storage cost. Caveat: more stack space is required for runtime book keeping (e.g. rebalancing).

### License and Contributing

Licensed under the [MIT license](https://github.com/tnballo/scapegoat/blob/master/LICENSE).
Contributions are welcome!
*/

// Test temp
//#![feature(variant_count)]

// Production
#![forbid(unsafe_code)]
#![cfg_attr(not(any(test, fuzzing)), no_std)]
#![cfg_attr(not(any(test, fuzzing)), deny(missing_docs))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/tnballo/scapegoat/master/img/scapegoat.svg"
)]

// Only expose arena internals for fuzzing harness
#[cfg(fuzzing)]
pub use crate::tree::{Arena, Node, NodeGetHelper, NodeRebuildHelper};

mod tree;
pub use crate::tree::SgError;

mod map;
pub use crate::map::SgMap;

/// [`SgMap`][crate::map::SgMap]'s iterator return types.
pub mod map_types;

mod set;
pub use crate::set::SgSet;

/// [`SgSet`][crate::set::SgSet]'s iterator return types.
pub mod set_types;
