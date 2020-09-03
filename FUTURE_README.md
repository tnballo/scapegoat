# scapegoat

Ordered set and map data structures via an arena-based [scapegoat tree](https://people.csail.mit.edu/rivest/pubs/GR93.pdf) (memory-efficient, self-balancing binary search tree).

### About

Three APIs:

* Binary Tree API (`SGTree`)
* Ordered Set API (`SGSet`)
* Ordered Map API (`SGMap`)

Strives for two properties:

* **Maximal safety:** strong [memory safety](https://tiemoko.com/blog/blue-team-rust/) guarantees.
    * **Compile-time safety:** no `unsafe` (no raw pointer dereference, etc.).
    * **Debug-time safety:** `debug_assert!` for logical invariants exercised in testing.
    * **Runtime safety:** no interior mutability (e.g. no need for `Rc<RefCell<T>>`'s runtime check).

* **Minimal footprint:** small binary (no dependencies outside of the standard library) with low resource use.
    * **Memory-efficient:** nodes have only child index metadata, node memory is re-used.
    * **Recursion-free:** all operations are iterative, so stack use and runtime are both minimized.
    * **Zero-copy:** rebuild/removal re-point in-place, nodes are never copied or cloned.

Other features:

* **Generic:** map keys and set elements can be any type that implements the `Ord` trait.
* **Arbitrarily Mutable:** elements can be insert and removed, map values can be mutated.

### Usage

`SGMap` non-exhaustive API example:

TODO: test this example as Docstring!

```rust
use scapegoat::SGMap;

let example = SGMap::new();

example.insert(3, "the");
example.insert(2, "don't blame");
example.insert(1, "Please");
example.insert(4, "borrow checker");

assert(example.iter(), vec!["Please","don't blame","the","borrow checker"]);

please = example.remove_min();
assert_eq!(please, (1, "Please"));

example.insert(5, "! :P");
let mut dont_blame = example.get_mut(2);
dont_blame[0] = "D";

assert(example.iter(), vec!["Don't blame","the","borrow checker", "! :P"]);
```
