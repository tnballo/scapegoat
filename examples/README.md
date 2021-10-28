## Examples

The examples don't have output, but you can run them to check their asserts (features optional, e.g. append `--features="high_assurance"`):

```
cargo run --example <name>
```

Alphabetical enumeration:

* [`ha_insert`](./ha_insert.rs) - insertion semantics, with `high_assurance` feature enabled/disabled.
* [`search_by_slice`](./search_by_slice.rs) - search set of sized types by an unsized type.
* [`static_strs`](./static_strs.rs) - build sentences with a mutable `SGMap<isize, &str>` (from main [`README.md`](../README.md)).
* [`tiny_map`](./tiny_map.rs) - 3KB `SGMap<u8, u8>` with index packing saving 75% (9KB) of stack usage.

Have another `!#[no_std]` example?
Consider [contributing](https://github.com/tnballo/scapegoat/pulls) it!