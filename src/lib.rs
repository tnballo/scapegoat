//!# scapegoat
//!
//!Ordered set and map data structures via an arena-based [scapegoat tree](https://people.csail.mit.edu/rivest/pubs/GR93.pdf) (memory-efficient, self-balancing binary search tree).
//!
//!### About
//!
//!Three APIs:
//!
//!* Ordered Set API ([`SGSet`](https://docs.rs/scapegoat/1.0.0/scapegoat/set/struct.SGSet.html))
//!* Ordered Map API ([`SGMap`](https://docs.rs/scapegoat/1.0.0/scapegoat/map/struct.SGMap.html))
//!* Binary Tree API ([`SGTree`](https://docs.rs/scapegoat/1.0.0/scapegoat/struct.SGTree.html))
//!
//!Strives for two properties:
//!
//!* **Maximal safety:** strong [memory safety](https://tiemoko.com/blog/blue-team-rust/) guarantees.
//!    * **Compile-time safety:** no `unsafe` (no raw pointer dereference, etc.).
//!    * **Debug-time safety:** `debug_assert!` for logical invariants exercised in testing.
//!    * **Runtime safety:** no interior mutability (e.g. no need for `Rc<RefCell<T>>`'s runtime check).
//!
//!* **Minimal footprint:** small binary (no dependencies outside of the standard library) with low resource use.
//!    * **Memory-efficient:** nodes have only child index metadata, node memory is re-used.
//!    * **Recursion-free:** all operations are iterative, so stack use and runtime are both minimized.
//!    * **Zero-copy:** rebuild/removal re-point in-place, nodes are never copied or cloned.
//!
//!Other features:
//!
//!* **Generic:** map keys and set elements can be any type that implements the `Ord` trait.
//!* **Arbitrarily mutable:** elements can be insert and removed, map values can be mutated.
//!
//!### Usage
//!
//!`SGMap` non-exhaustive API example (would work identically for `std::collections::BTreeMap`):
//!
//!```rust
//!use scapegoat::SGMap;
//!
//!let mut example = SGMap::new();
//!
//!example.insert(3, String::from("the"));
//!example.insert(2, String::from("don't blame"));
//!example.insert(1, String::from("Please"));
//!example.insert(4, String::from("borrow checker"));
//!
//!assert_eq!(
//!    (&example).into_iter().map(|(_, v)| v).collect::<Vec<&String>>(),
//!    vec!["Please","don't blame","the","borrow checker"]
//!);
//!
//!assert_eq!(example[&3], "the");
//!
//!let please_tuple = example.pop_first().unwrap();
//!assert_eq!(please_tuple, (1, String::from("Please")));
//!
//!example.insert(5, String::from("! :P"));
//!
//!let dont_blame = example.get_mut(&2).unwrap();
//!dont_blame.remove(0);
//!dont_blame.insert(0, 'D');
//!
//!assert_eq!(
//!    example.into_iter().map(|(_, v)| v).collect::<Vec<String>>(),
//!    vec!["Don't blame","the","borrow checker","! :P"]
//!);
//!```
//!
//!### Note
//!
//!This project is an exercise in safe datastructure design. It's not as mature, fast, or memory efficient as the [standard library's `BTreeMap`/`BTreeSet`](http://cglab.ca/~abeinges/blah/rust-btree-case/).

mod tree;
pub use crate::tree::SGTree;

mod map;
pub use crate::map::SGMap;

mod set;
pub use crate::set::SGSet;
