## Examples

The examples don't have output, but you can run them to check their asserts (features optional, e.g. append `--features="low_mem_insert"`):

```
cargo run --example <name>
```

Alphabetical enumeration:

* [`search_by_slice`](./search_by_slice.rs) - search set of sized types by an unsized type.
* [`static_strs`](./static_strs.rs) - build sentences with a mutable `SgMap<isize, &str, 5>` (from main [`README.md`](../README.md)).
* [`tiny_map`](./tiny_map.rs) - `SgMap<u8, u8, 128>` using 1.3 KB of stack space.
* [`try_insert`](./try_insert.rs) - fallible insertion semantics.

Have another `#![no_std]` example?
Consider [contributing](https://github.com/tnballo/scapegoat/pulls) it!