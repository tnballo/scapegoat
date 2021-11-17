## Minimum Executable Size Comparison: `SGMap` vs `BTreeMap`

This directory contains an imprecise but repeatable experiment: how small is `SGMap` relative to it's API-compatible counterpart, `BTreeMap`?
In terms of executable code bytes stored in the `.text` section.

[`min_size_no_std`](./min_size_no_std/src/main.rs) is a `scapegoat::SGMap` test binary that calls only the most basic functions of the data structure: `insert`, `get`, and `remove`.
It doesn't need a global allocator, since `SGMap` uses a stack arena.

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

[`min_size_std`](./min_size_std/src/main.rs) is an equivalent for `std::collections::BTreeMap`.
It does need a global allocator, since `BTreeMap` uses the heap.

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

Your output for the above command should be something like:

```
/usr/lib/x86_64-linux-musl/libc.a
```

Add this path to your `~/.cargo/config` like so:

```
[target.x86_64-unknown-linux-musl]
rustflags = ["-L/usr/lib/x86_64-linux-musl"]
```

If you don't make this modification to your `~/.cargo/config` file, you may get this error when attempting to build `min_size_std`:

```
error: could not find native static library `c`, perhaps an -L flag is missing?
```

Now we're ready to build some static binaries, fully from source!

### Results for `scapegoat::SGMap`

Determine executable byte count:

```
cd min_size_no_std
cargo size --release --target x86_64-unknown-linux-musl
```

Sample output from an x86-64 machine (note your milage may vary, depending on your host architecture and compiler version):

```
  text	   data	    bss	    dec	    hex	filename
  17022	   3616	    728	  21366	   5376	min_size_no_std
```

**This demonstrates a `.text` section under 20KB in size is possible!**
Realistically, you'll probably use functions of the library beyond `insert`, `get`, and `remove` and thus increase code size.
But the purpose of this demo is to show that we can have a working, BST-backed map in under 20KB of 64-bit code.

To check sources of bloat:

```
cargo bloat --release --target x86_64-unknown-linux-musl --crates --split-std
```

Sample output (oddly the reported `.text` size of 14KB is smaller than `cargo size`'s 17KB):

```
 File  .text    Size Crate
10.3%  49.7%  7.3KiB core
 5.5%  26.7%  3.9KiB [Unknown]
 2.7%  13.3%  2.0KiB smallvec
 1.0%   4.9%    740B libc
 0.5%   2.5%    381B scapegoat
 0.1%   0.4%     57B compiler_builtins
 0.0%   0.2%     30B alloc
 0.0%   0.1%     18B min_size_no_std
20.7% 100.0% 14.7KiB .text section size, the file size is 71.1KiB

Note: numbers above are a result of guesswork. They are not 100% correct and never will be.
```

As the output indicates, this data isn't precise.
`[Unknown]` probably includes `scapegoat` code, `381B` is suspiciously small.
But we're definitely in that **20KB ballpark**.

### Results for `std::collections::BTreeMap`

We first build a target in which the standard library is compiled from source, using the [`build-std` feature](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std), instead of linking in the pre-compiled `std`.
This ensures the size optimized profile (`opt-level = "z"`, `codegen-units = 1`, etc) is applied to `BTreeSet` and all other `std` code we include.

`cargo size` doesn't seem to accept the unstable `build-std` feature (e.g. `-Z build-std=...`).
So we're going to run the native `size` command it wraps directly instead.

To determine executable byte count:

```
cd ../min_size_std
cargo +nightly build --release --target x86_64-unknown-linux-musl -Z build-std=core,std,alloc,proc_macro,panic_abort
size ./target/x86_64-unknown-linux-musl/release/min_size_std
```

Sample output from an x86-64 machine (note your milage may vary, depending on your host architecture and compiler version):

```
   text	   data	    bss	    dec	    hex	filename
 274669	  13548	   7121	 295338	  481aa	./target/x86_64-unknown-linux-musl/release/min_size_std
```

**We're just over 270KB, 15x the amount of executable code**.
Unfortunately, much of that is machinery to support `RUST_BACKTRACE=1` (DWARF parser, symbol demangling) - not data structure logic.
So it's not exactly an apples-to-apples comparison.
Stripping the binary doesn't help.
But some of that machinery is `musl`'s global allocator, which `BTreeMap` needs in order to work.

Just for completeness, let's say we expect an allocator to be dynamically linked and don't want to count it's code toward our total.
We'd change the target to `x86_64-unknown-linux-gnu`:

```
cargo +nightly build --release --target x86_64-unknown-linux-gnu -Z build-std=core,std,alloc,proc_macro,panic_abort
size ./target/x86_64-unknown-linux-gnu/release/min_size_std
```

That gets us down to 190KB. Still about 10x.

```
 text	   data	    bss	    dec	    hex	filename
 190553	  11232	    473	 202258	  31612	./target/x86_64-unknown-linux-gnu/release/min_size_std
```

Let's try to tease things apart and check sources of bloat.

```
cargo bloat --release --crates --split-std --target x86_64-unknown-linux-musl
```

Like `cargo size`, `cargo bloat` doesn't take the `-Z build-std=..` flag.
So bear in mind these `bloat` results are for the pre-compiled (e.g. not size optimized) standard library and a dynamically linked binary (e.g. not including an allocator).
Sample output:

```
File  .text     Size Crate
 3.8%  22.3%  57.5KiB [Unknown]
 3.3%  19.0%  48.9KiB std
 2.6%  15.4%  39.7KiB addr2line
 2.4%  13.7%  35.4KiB core
 1.3%   7.8%  20.0KiB rustc_demangle
 1.1%   6.2%  15.9KiB libc
 1.0%   5.7%  14.7KiB gimli
 0.7%   4.2%  10.9KiB miniz_oxide
 0.6%   3.4%   8.7KiB alloc
 0.1%   0.8%   2.2KiB min_size_std
 0.0%   0.2%     547B object
 0.0%   0.0%      57B compiler_builtins
 0.0%   0.0%      17B panic_abort
17.2% 100.0% 257.8KiB .text section size, the file size is 1.5MiB

Note: numbers above are a result of guesswork. They are not 100% correct and never will be.
```

We're definitely still in the **200KB ballpark**.
Symbol demangling alone (`rustc_demangle`) requires as more executable code than was present in `scapegoat::SGMap`.
`std` alone is 50KB, which likely includes the data structure logic among other things.
Plus we need another 8KB for `alloc`, even not including a statically linked global allocator.

### Conclusion

The numbers here shouldn't be taken as gospel, and, due to backtrace support, this isn't an apples-to-apples comparison.
But at the end of the day, `scapegoat::SGMap` can have a really tiny code footprint!
Roughly 10-15x smaller than `std::collections::BTreeMap`.

If you have ideas for improving the accuracy of this size comparison, issues/PRs are welcome!
Thanks everyone that made suggestions on [this reddit thread](https://www.reddit.com/r/rust/comments/qu3k38/1012x_smaller_executable_footprint_than/).