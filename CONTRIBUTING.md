# Contributing Guidelines

Thank you for your interest in contributing!
Whether it's a bug report, new feature, performance optimization, correction, or additional documentation, your feedback and contributions are valuable.

Please read through this document before submitting pull requests to ensure an effective response to your contribution.

## Background Resources

* The APIs this project strives to be compatible with: [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html), [`BTreeSet`](https://doc.rust-lang.org/stable/std/collections/btree_set/index.html).
    * List of map structs [here](https://doc.rust-lang.org/std/collections/btree_map/index.html), set structs [here](https://doc.rust-lang.org/stable/std/collections/btree_set/index.html).

* ["Safe && Portable Data Structure Design"](https://tiemoko.com/slides/SafeAndPortableDataStructureDesign_CodeAndSupply_Dec2021.pdf) ([video](https://youtu.be/1UtklNrB8XA?t=1615)).
    * 10 minute (lightning talk) overview of this library's design.

* ["Beyond the Borrow Checker: Differential Fuzzing"](https://tiemoko.com/blog/diff-fuzz/).
    * Blog post explaining how and why this project uses fuzz testing.

Questions are welcome, feel free to [open an issue](https://github.com/tnballo/scapegoat/issues) or use the [discussion board](https://github.com/tnballo/scapegoat/discussions).

## Development Environment

This project offers a [Dockerfile](https://github.com/tnballo/scapegoat/blob/master/Dockerfile) for your convenience, but it's use is entirely optional.

## Contributing via Pull Requests

Contributions via pull requests are much appreciated. Before submitting a pull request, please ensure that:

1. You are working against the latest source on the `master` branch.
2. You check existing open, and recently merged, pull requests to make sure someone else hasn't addressed the problem already.
3. You open an issue to discuss any significant work - we would hate for your time to be wasted.

To submit a pull request, please:

1. Fork the repository.
2. Modify the source; please focus on the specific change you are contributing.
3. Ensure local tests pass (`cargo test`), including any you may have added.
4. Ensure you've run `cargo fmt` (default settings).
5. Ensure you've run `cargo clippy` and addressed all warnings emitted.
6. Commit to your fork using clear commit messages.
7. Submit the pull request.
8. Pay attention to any automated CI failures reported in the pull request, and stay involved in the conversation.

GitHub provides additional document on [forking a repository](https://help.github.com/articles/fork-a-repo/) and
[creating a pull request](https://help.github.com/articles/creating-a-pull-request/).

## Finding Contributions to Work On

Looking at [currently open issues](https://github.com/tnballo/scapegoat/issues) is a great way to find something to work on.
However, you're free to suggest your own features and ideas - we'd love to discuss them!

## Licensing

See the [LICENSE](hhttps://github.com/tnballo/scapegoat/blob/master/LICENSE) file for this project's licensing.
All contributions to this project will, by default, use this license.