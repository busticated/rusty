# Rusty

[![CI Status](https://github.com/busticated/rusty/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/busticated/rusty/actions) [![Rust Version Support](https://img.shields.io/badge/rust%20version-%3E%3D1.72.1-orange)](https://releases.rs/) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/busticated/rusty/blob/master/LICENSE)

A `cargo` workspace ([docs](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)) monorepo ([info](https://en.wikipedia.org/wiki/Monorepo)) hosting a collection of Rust utility crates.

[Installation](#installation) | [Crates](#crates) | [Development](#development) | [Docs](#docs--resources)


## Installation

1. Install Rust using `rustup` ([instructions](https://www.rust-lang.org/tools/install))
2. Clone this repository: `$ git clone git@github.com:busticated/rusty.git && cd ./rusty`
3. Setup local dev environment: `$ cargo xtask setup`
4. View available commands: `$ cargo xtask help`
5. Run the tests `$ cargo xtask test`
6. Start Hacking!


## Crates

<!-- crate-list-start -->
* [detect-newline-style](crates/detect-newline-style)
	* Determine a string's preferred newline character
* [node-js-release-info](crates/node-js-release-info)
	* Asynchronously retrieve Node.js release info by version and platform from the [downloads server](https://nodejs.org/download/release/)
<!-- crate-list-end -->

## Development

This repository contains a series of `rust` crates managed together as a `cargo` workspace ([docs](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)) with [XTask](https://github.com/matklad/cargo-xtask). All essential commands are available via `cargo xtask <script name>` - e.g. `cargo xtask todo`. To view the available commands, run: `cargo xtask help`


<details id="develop-add-crate">
<summary><b>How to add a new crate</b></summary>
<p>

To add a _new_ crate to the workspace, run `cargo xtask crate:add` and follow the prompts. Upon completion, your new crate will be available within `./crates/<your crate>`

</p>
</details>

<details id="develop-run-tests">
<summary><b>How to run tests</b></summary>
<p>

To run _all_ tests for _all_ crates:

```
cargo test
```

To run _unit_ tests for _all_ crates:

```
cargo test --lib --workspace
```

To run _unit_ tests for _just your_ crate:

```
cargo test --lib --package <your crate's name>
```

To run _integration_ tests for _all_ crates:

```
cargo test --test integration --workspace
```

To run _integration_ tests for _just your_ crate:

```
cargo test --test integration --package <your crate's name>
```

Run `cargo xtask help` to see any other test-related commands that are available.

</p>
</details>

<details id="develop-run-coverage">
<summary><b>How to see code coverage stats</b></summary>
<p>

To see code coverage stats for _all_ crates:

```
cargo xtask coverage
```

Run `cargo xtask help` to see any other coverage-related commands that are available.

</p>
</details>

<details id="develop-build-docs">
<summary><b>How to create docs</b></summary>
<p>

Public interfaces must be documented using inline annotations ([docs](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)).

Once you've added your inline documentation, run:

```
cargo xtask doc --open
```

Run `cargo xtask help` to see any other docs-related commands that are available.

</p>
</details>

<details id="develop-todo">
<summary><b>How to view and add TODO source code comments</b></summary>
<p>

To see what TODOs exist across crates, run:

```
cargo xtask todo
```

When adding a TODO comment to your source code, format it like:

```rust
// TODO (<name>): <message>
```

e.g.

```rust
// TODO (busticated): this is my example todo comment
```

</p>
</details>


## Docs & Resources

* [Rust](https://www.rust-lang.org)
* [Cargo](https://github.com/rust-lang/cargo)
* [Tokio](https://tokio.rs)
* [XTask](https://github.com/matklad/cargo-xtask)
* [Duct](https://github.com/oconnor663/duct.rs)
* [TOML](https://github.com/toml-rs/toml)
* [Inquire](https://github.com/mikaelmello/inquire)
* [Mockito](https://github.com/lipanski/mockito)

