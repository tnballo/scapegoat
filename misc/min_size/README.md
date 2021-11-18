## Minimum Executable Size Comparison: `SGMap` vs `BTreeMap`

This directory contains a repeatable experiment: how small can we possibly make binaries that use `SGMap` and it's API-compatible counterpart, `BTreeMap`?
In terms of executable code bytes stored in the `.text` section.

[`min_size_no_std`](./min_size_no_std/src/main.rs) is a `scapegoat::SGMap` test binary that calls only the most basic functions of the data structure: `insert`, `get`, and `remove`.
It doesn't need a global allocator, since `SGMap` uses a stack arena.

```rust
#![no_std]
#![no_main]
use scapegoat::SGMap;

#[no_mangle]
pub fn main(_argc: i32, _argv: *const *const u8) -> isize {
    let mut map: SGMap<usize, usize> = SGMap::new();
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

Install [`cargo-binutils`](https://github.com/rust-embedded/cargo-binutils), [`cargo-bloat`](https://github.com/RazrFalcon/cargo-bloat), [`musl` libc](https://musl.libc.org/) static binary target, and the [standard library](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std) source. Switch to the `nightly` toolchain:

```
cargo install cargo-binutils
cargo install cargo-bloat
rustup component add llvm-tools-preview
rustup target add x86_64-unknown-linux-musl
rustup component add rust-src --toolchain nightly
rustup default nightly
```

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

### Results for `scapegoat::SGMap`

Determine executable byte count:

```
cd min_size_no_std
cargo size --release
```

Sample output from an x86-64 machine (note your milage may vary, depending on your host architecture and compiler version):

```
  text	   data	    bss	    dec	    hex	filename
  17014	   3592	    728	  21334	   5356	min_size_no_std
```

**This demonstrates a `.text` section under 20KB in size is possible!**
Realistically, you'll probably use functions of the library beyond `insert`, `get`, and `remove` and thus increase code size.
But the purpose of this demo is to show that we can have a working, BST-backed map in under 20KB of 64-bit code.

To check sources of bloat:

```
cargo bloat --release -n 10
```

Sample output (oddly the reported `.text` size of 14.7KB is smaller than `cargo size`'s 17.0KB):

```
 File  .text    Size     Crate Name
 4.1%  19.6%  2.9KiB [Unknown] main
 3.4%  16.6%  2.4KiB       std core::slice::sort::recurse
 0.9%   4.3%    651B       std core::fmt::Formatter::pad_integral
 0.9%   4.1%    625B       std <core::fmt::builders::PadAdapter as core::fmt::Write>::write_str
 0.8%   3.8%    568B       std core::fmt::write
 0.6%   2.7%    410B  smallvec smallvec::SmallVec<A>::push
 0.6%   2.7%    403B  smallvec smallvec::SmallVec<A>::push
 0.5%   2.6%    393B  smallvec smallvec::SmallVec<A>::push
 0.5%   2.6%    391B [Unknown] static_init_tls
 0.5%   2.6%    386B  smallvec smallvec::SmallVec<A>::push
 7.5%  36.2%  5.3KiB           And 73 smaller methods. Use -n N to show more.
20.7% 100.0% 14.7KiB           .text section size, the file size is 71.1KiB
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
cargo size --release
```

Sample output from an x86-64 machine (note your milage may vary, depending on your host architecture and compiler version):

```
  text	   data	    bss	    dec	    hex	filename
  17892	    812	   2368	  21072	   5250	min_size_std
```

**The size-optimized `BTreeMap` also fits in under 20KB!**
Including the memory allocator!
That's a testament to how small `musl` libc's allocator is, even if it's not the most performant.

To check sources of bloat:

```
cargo bloat --release -n 10
```

Sample output (oddly the reported `.text` size of 16.4KB is smaller than `cargo size`'s 17.8KB):

```
 File  .text    Size     Crate Name
 5.0%  12.0%  2.0KiB [Unknown] main
 3.5%   8.5%  1.4KiB      libc malloc
 3.0%   7.3%  1.2KiB      libc __bin_chunk
 2.3%   5.7%    956B     alloc alloc::collections::btree::remove::<impl alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<alloc::collections::btree...
 1.7%   4.1%    694B [Unknown] alloc_fwd
 1.7%   4.0%    678B [Unknown] alloc_rev
 1.3%   3.2%    536B      libc __init_libc
 1.3%   3.2%    532B      core core::fmt::Formatter::pad_integral
 1.3%   3.1%    523B     alloc alloc::collections::btree::node::BalancingContext<K,V>::merge_tracking_child_edge
 1.2%   2.8%    474B [Unknown] static_init_tls
17.8%  43.2%  7.1KiB           And 75 smaller methods. Use -n N to show more.
41.2% 100.0% 16.4KiB           .text section size, the file size is 39.8KiB
```

### Conclusion

Both `scapegoat::SGMap` and `std::collections::BTreeMap` can produce working dynamic collections in binaries under 20KB.
Much to my surprise, both produce 17KB binaries despite the latter including `musl` libc's memory allocator.

* Thanks to everyone that made suggestions on [this reddit thread](https://www.reddit.com/r/rust/comments/qu3k38/1012x_smaller_executable_footprint_than/).

* [min-sized-rust](https://github.com/johnthagen/min-sized-rust) is an awesome resource, it's techniques took the `BTreeMap` binary down from an initial 270KB to the current 17KB.

If you have ideas for improving the accuracy of this size comparison, issues/PRs are welcome!