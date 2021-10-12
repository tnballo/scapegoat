This doc tackles advanced configuration options, it assumed you've read the main [README.md](https://github.com/tnballo/scapegoat/blob/master/README.md).

### Tuning the the tree's `a` factor

Original scapegoat tree paper's alpha, `a`, can be chosen in the range `0.5 <= a < 1.0`.
`a` tunes how "aggressively" the data structure self-balances.

* As `a` approaches `0.5`, the library will rebalance more often. Ths means slower insertions, but faster lookups and deletions.
	* An `a` equal to `0.5` means a tree that always maintains a perfect balance (e.g."complete" binary tree, at all times).

* As `a` approaches `1.0`, the library will rebalance less. This means quicker insertions, but slower lookups and deletions.
	* An `a` nearly equal to `1.0` means a tree that never rebalances.

We choose 2/3, i.e. `a = 0.666...`, by default.
This default was not empirically chosen, it's just the one used in the [Open Data Structures textbook implementation](https://opendatastructures.org/ods-java/8_Scapegoat_Trees.html) of a scapegoat tree.
But that implementation is quite different from this library (one major difference being that it uses recursion), so an `a` of 2/3 may not be optimal for the majority of workloads (testing needed!).

Just like with stack arena size, `a` can be compile-timed configured by exporting environment variables before build.
The alpha denominator is the floating point string assigned to env var `SG_ALPHA_NUMERATOR`.
The alpha denominator is the floating point string assigned to env var `SG_ALPHA_DENOMINATOR`.

For example, manually setting the default 2/3 would be:

```
export SG_ALPHA_NUMERATOR=2.0
export SG_ALPHA_DENOMINATOR=3.0
cargo build --release
```


### The `alt_impl` feature

By default, this library uses the algorithms proposed in the original paper ([Galperin and Rivest, 1993](https://people.csail.mit.edu/rivest/pubs/GR93.pdf)).
The `alt_impl` feature enables optimizations proposed in the subsequent PhD thesis ([Galperin, 1996](https://dspace.mit.edu/handle/1721.1/10639)).

> **Warning:** This feature is currently a work in progress, it's not finished or guaranteed to be an improvement (e.g. the implementation may be incorrect).
>
> The feature-gate means we can compare the two modes before potentially setting a new default in a future version.
> Beyond that point the non-default is only worth supporting if it's measurably superior for some usecase.

The main optimization is eliminating recursion.
This library already does that, but likely in a way inferior to the "official" algorithm (implemented prior to find/reading the thesis). Please see thesis pages 95 and 97 for the algorithm's pseudo code (needs translation to Rust!).

>