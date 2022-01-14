## Minimum Executable Size Comparison: `SgMap` vs `BTreeMap`

This directory contains a repeatable experiment: how small can we possibly make binaries that use `SgMap` and it's API-compatible counterpart, `BTreeMap`?
In terms of executable code bytes stored in the `.text` section.

[`min_size_no_std`](./min_size_no_std/src/main.rs) is a `scapegoat::SgMap` test binary that calls only the most basic functions of the data structure: `insert`, `get`, and `remove`.
It doesn't need a global allocator, since `SgMap` uses a stack arena.

```rust
#![no_std]
#![no_main]
use scapegoat::SgMap;

#[no_mangle]
pub fn main(_argc: i32, _argv: *const *const u8) -> isize {
    let mut map: SgMap<usize, usize, 1024> = SgMap::new();
    map.insert(1, 2);
    assert_eq!(map.get(&1), Some(&2));
    assert_eq!(map.remove(&1), Some(2));
    return 0;
}

// Boiler plate for a free-standing binary omitted...
```

[`min_size_std`](./min_size_std/src/main.rs) is an equivalent for `std::collections::BTreeMap`.
It does need a global allocator, since `BTreeMap` uses the heap.

```rust
#![no_main]
use std::collections::BTreeMap;

#[no_mangle]
pub fn main(_argc: i32, _argv: *const *const u8) -> isize {
    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    map.insert(1, 2);
    assert_eq!(map.get(&1), Some(&2));
    assert_eq!(map.remove(&1), Some(2));
    return 0;
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

Both will be built for a static binary target, `x86_64-unknown-linux-musl`.
This ensures we're comparing free-standing binaries that include everything they need to function.
There's no dynamically linked code that we're forgetting to count.

Neither is stripped, since we want to run `cargo bloat`.
And we're concerned with code size, not overall executable size (e.g. not counting `.symtab` and DWARF info, just `.text` bytes).

### Setup

Install [`cargo-binutils`](https://github.com/rust-embedded/cargo-binutils) and [`cargo-bloat`](https://github.com/RazrFalcon/cargo-bloat):

```
cargo install cargo-binutils
cargo install cargo-bloat
```

This writeup uses `cargo-size` version `0.3.4` and `cargo-bloat` version `0.11.0` (version numbers gathered with `cargo size --version` and `cargo bloat --version`).
These versions are the latest available at the time of this writing (1/14/22 update).

The [`rust-toolchain.toml`](./min_size_no_std/rust-toolchain.toml) files will take care of the rest (e.g. grabbing the [`musl` libc](https://musl.libc.org/) static binary target, the [standard library](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std) source, and using `nightly-2021-12-04` to replicate numbers).

To use the `build-std` feature (e.g. build the standard library from source) and `x86_64-unknown-linux-musl` (e.g. build static binary) simultaneously, we have to do a little extra work, per [this incredibly helpful comment](https://github.com/japaric/xargo/issues/133#issuecomment-681194097).

Install `musl-tools` (Ubuntu command, your distro may vary):

```
sudo apt install -y musl-tools
```

Musl's `libc.a` isn't in the link path by default, so find it with:

```
dpkg -L 'musl-dev' | grep 'libc\.a'
```

Does your output for the above command match the below?

```
/usr/lib/x86_64-linux-musl/libc.a
```

If yes, you're good to go.
If not, you'll need to update the path in `./min_size_std/.cargo/config.toml`, under the `rustflags` key:

```
[target.x86_64-unknown-linux-musl]
rustflags = ["-L/usr/lib/x86_64-linux-musl"]
```

If you don't make this modification, you may get this error when attempting to build `min_size_std`:

```
error: could not find native static library `c`, perhaps an -L flag is missing?
```

Now we're ready to build some static binaries, fully from source!

### Results for `scapegoat::SgMap`

Determine executable byte count.
Note `-A` flag for GNU `binutils` `size` to see the actual size of the `.text` section.
Without this flag the Berkley format (default) counts [read-only data](https://sourceware.org/binutils/docs/binutils/size.html) in the `text` column.

```
cd min_size_no_std
cargo size --release -- -A
```

Sample output from an x86-64 machine (note your milage may vary, depending on your host architecture):

```
min_size_no_std  :
section                 size    addr
// Prior omitted...
.text                  14231  0x1040
// Remainder omitted...
```

**This demonstrates a `.text` section under 20KB in size is possible!**
Realistically, you'll probably use functions of the library beyond `insert`, `get`, and `remove` and thus increase code size.
But the purpose of this demo is to show that we can have a working, BST-backed map in under 14.2KB of 64-bit code.

To check sources of bloat:

```
cargo bloat --release -n 10
```

Sample output (oddly the reported `.text` size of 13.9KB is smaller than `cargo size`'s 14.2KB):

```
File  .text    Size     Crate Name
 3.1%  17.7%  2.5KiB       std core::slice::sort::recurse
 2.5%  14.0%  1.9KiB [Unknown] main
 1.6%   9.1%  1.3KiB scapegoat scapegoat::tree::tree::SgTree<K,V,_>::rebuild
 0.8%   4.6%    651B       std core::fmt::Formatter::pad_integral
 0.8%   4.4%    625B       std <core::fmt::builders::PadAdapter as core::fmt::Write>::write_str
 0.7%   4.0%    568B       std core::fmt::write
 0.5%   2.7%    391B [Unknown] static_init_tls
 0.5%   2.7%    385B [Unknown] __init_libc
 0.4%   2.4%    346B [Unknown] _start_c
 0.4%   2.4%    335B       std core::fmt::builders::DebugTuple::field
 6.0%  34.1%  4.7KiB           And 69 smaller methods. Use -n N to show more.
17.6% 100.0% 13.9KiB           .text section size, the file size is 78.8KiB
```

Unclear why we need `core::fmt::write`, but regardless we're definitely in that **20KB ballpark**.

### Results for `std::collections::BTreeMap`

We need to build a target in which the standard library is compiled from source, using the [`build-std` feature](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std), instead of linking in the pre-compiled `std`.
This ensures the size optimized profile (`opt-level = "z"`, `codegen-units = 1`, etc) is applied to `BTreeSet` and all other `std` code we include.
Towards this end, `./min_size_std/.cargo/config.toml` contains the following settings:

```
[unstable]
build-std=["core","std","alloc","proc_macro","panic_abort"]
build-std-features=["panic_immediate_abort"]
```

To determine executable byte count:

```
cd ../min_size_std
cargo size --release -- -A
```

Sample output from an x86-64 machine (note your milage may vary, depending on your host architecture):

```
min_size_std  :
section                 size    addr
// Prior omitted...
.text                  16809  0x1040
// Remainder omitted...
```

**The size-optimized `BTreeMap` also fits in under 20KB!**
Including the memory allocator!
That's a testament to how small `musl` libc's allocator is, even if it's not the most performant.

To check sources of bloat:

```
cargo bloat --release -n 10
```

Sample output (oddly the reported `.text` size of 16.4KB is smaller than `cargo size`'s 16.8KB):

```
File  .text    Size     Crate Name
 4.7%  12.0%  2.0KiB [Unknown] main
 3.3%   8.5%  1.4KiB [Unknown] malloc
 2.8%   7.3%  1.2KiB [Unknown] __bin_chunk
 2.2%   5.7%    956B     alloc alloc::collections::btree::remove::<impl alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<alloc::collections::btree...
 1.6%   4.1%    694B [Unknown] alloc_fwd
 1.6%   4.0%    678B [Unknown] alloc_rev
 1.2%   3.2%    536B [Unknown] __init_libc
 1.2%   3.2%    532B      core core::fmt::Formatter::pad_integral
 1.2%   3.1%    523B     alloc alloc::collections::btree::node::BalancingContext<K,V>::merge_tracking_child_edge
 1.1%   2.8%    474B [Unknown] static_init_tls
16.8%  43.2%  7.1KiB           And 75 smaller methods. Use -n N to show more.
38.9% 100.0% 16.4KiB           .text section size, the file size is 42.2KiB
```

### Conclusion

Both `scapegoat::SgMap` and `std::collections::BTreeMap` can produce working dynamic collections in binaries under 20KB.
Perhaps surprisingly, both produce 14-17KB binaries despite the latter including `musl` libc's memory allocator.

* Thanks to everyone that made suggestions on [this reddit thread](https://www.reddit.com/r/rust/comments/qu3k38/1012x_smaller_executable_footprint_than/).

* [min-sized-rust](https://github.com/johnthagen/min-sized-rust) is an awesome resource, it's techniques took the `BTreeMap` binary down from an initial 270KB to the current 17KB.

If you have ideas for improving the accuracy of this size comparison, issues/PRs are welcome!