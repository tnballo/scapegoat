## Minimum Executable Size Comparison: `SGMap` vs `BTreeMap`

This directory contains an imprecise but repeatable experiment: how small is `SGMap` relative to it's API-compatible counterpart, `BTreeMap`?
In terms of executable code bytes stored in the `.text` section.

[`min_size_no_std`](./min_size_no_std/src/main.rs) is a `scapegoat::SGMap` test binary that calls only the most basic functions of the data structure: `insert`, `get`, and `remove`.

```rust
use scapegoat::SGMap;

fn main() {
    let mut map: SGMap<usize, usize> = SGMap::new();
    map.insert(1, 2);
    assert_eq!(map.get(&1), Some(&2));
    assert_eq!(map.remove(&1), Some(2));
}

// Boiler plate for a free-standing binary omitted...
```

[`min_size_std`](./min_size_std/src/main.rs) is an equivalent for `std::collections::BTreeMap`:

```rust
use std::collections::BTreeMap;

fn main() {
    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    map.insert(1, 2);
    assert_eq!(map.get(&1), Some(&2));
    assert_eq!(map.remove(&1), Some(2));
}
```

Both use the same size-optimized release profile in their respective `Cargo.toml` files:

```
[profile.release]
panic = "abort"
opt-level = "z"
lto = true
codegen-units = 1
```

Neither is stripped, since we want to run `cargo bloat`.
And we're concerned with code size, not overall executable size (e.g. not counting `.symtab` and DWARF info, just `.text` bytes).

### Setup

Install [`cargo-binutils`](https://github.com/rust-embedded/cargo-binutils) and [`cargo-bloat`](https://github.com/RazrFalcon/cargo-bloat), switch to the `nightly` toolchain:

```
cargo install cargo-binutils
cargo install cargo-bloat
rustup component add llvm-tools-preview
rustup default nightly
```

### Results for `scapegoat::SGMap`

Determine executable byte count:

```
cd min_size_no_std
cargo size --release
```

Sample output from an x86-64 machine (note your milage may vary, depending on your host architecture and compiler version):

```
  text	   data	    bss	    dec	    hex	filename
  18825	    776	      8	  19609	   4c99	min_size
```

**This demonstrates a `.text` section under 20KB in size is possible!**
Realistically, you'll probably use functions of the library beyond `insert`, `get`, and `remove` and thus increase code size.
But the purpose of this demo is to show that we can have a working, BST-backed map in under 20KB of 64-bit code.

To check sources of bloat:

```
cargo bloat --release --crates --split-std
```

Sample output (oddly the reported `.text` size of 13KB is smaller than `cargo size`'s 18KB):

```
 File  .text    Size Crate
10.3%  54.9%  7.2KiB core
 4.4%  23.3%  3.1KiB [Unknown]
 2.8%  14.9%  2.0KiB smallvec
 0.5%   2.8%    381B scapegoat
 0.1%   0.4%     57B compiler_builtins
 0.0%   0.2%     30B alloc
 0.0%   0.1%     18B min_size_no_std
18.7% 100.0% 13.1KiB .text section size, the file size is 70.4KiB

Note: numbers above are a result of guesswork. They are not 100% correct and never will be.
```

As the output indicates, this data isn't precise.
`[Unknown]` probably includes `scapegoat` code, `381B` is suspiciously small.
But we're definitely in that **20KB ballpark**.

### Results for `std::collections::BTreeMap`

Determine executable byte count:

```
cd ../min_size_std
cargo size --release
```

Sample output from an x86-64 machine (note your milage may vary, depending on your host architecture and compiler version):

```
  text	   data	    bss	    dec	    hex	filename
 240844	   9456	    449	 250749	  3d37d	min_size_std
```

We're just over 240KB, **12x the amount of executable code**.
Unfortunately, much of that is machinery to support `RUST_BACKTRACE=1` (DWARF parser, symbol demangling) - not data structure logic.
So it's not exactly an apples-to-apples comparison.
Stripping the binary doesn't help.

Let's try to tease things apart and check sources of bloat:

```
cargo bloat --release --crates --split-std
```

Sample output:

```
File  .text     Size Crate
 3.5%  26.5%  49.6KiB std
 2.8%  21.0%  39.4KiB addr2line
 2.5%  18.9%  35.4KiB core
 1.4%  10.7%  20.0KiB rustc_demangle
 1.0%   7.8%  14.7KiB gimli
 0.8%   5.8%  10.9KiB miniz_oxide
 0.6%   4.7%   8.7KiB alloc
 0.2%   1.8%   3.4KiB [Unknown]
 0.2%   1.2%   2.2KiB min_size_std
 0.0%   0.3%     547B object
 0.0%   0.0%      57B compiler_builtins
 0.0%   0.0%      17B panic_abort
13.3% 100.0% 187.5KiB .text section size, the file size is 1.4MiB

Note: numbers above are a result of guesswork. They are not 100% correct and never will be.
```

We're definitely now in the **200KB ballpark**.
Symbol demangling alone (`rustc_demangle`) requires as more executable code than was present in `scapegoat::SGMap`.
`std` alone is 50KB, which likely includes the data structure logic among other things.
Plus we need another 8KB for `alloc`, since `BTreeMap` uses dynamic memory.

### Conclusion

The numbers here shouldn't be taken as gospel, and, due to backtrace support, this isn't an apples-to-apples comparison.
But at the end of the day, `scapegoat::SGMap` can have a really tiny code footprint!
Roughly 10-12x smaller than `std::collections::BTreeMap`.

If you have ideas for improving the accuracy of this size comparison, issues/PRs are welcome!