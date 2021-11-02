## Minimum Executable Size Check

[`min_size`](./src/main.rs) is a dummy binary for checking how small the `scapegoat` library can be, in terms of executable code bytes stored in the `.text` section.
It has a size-optimized release profile (`opt-level = "z"`, `codegen-units = 1`, etc) and calls only the most basic functions of the data structure: `insert`, `get`, and `remove`.

After [installing `cargo-binutils`](https://github.com/rust-embedded/cargo-binutils) and switching to the nightly toolchain, check size with:

```
cargo size --release
```

Sample output from an x86-64 machine (note your milage may vary, depending on your host architecture and compiler version):

```
  text	   data	    bss	    dec	    hex	filename
  18601	    776	      8	  19385	   4bb9	min_size
```

**This demonstrates a `.text` section under 20KB in size is possible!**
Realistically, you'll probably use functions of the library beyond `insert`, `get`, and `remove` and thus increase code size.
But the purpose of this demo is to show that we can have a working, BST-backed map in under 20KB of 64-bit code.