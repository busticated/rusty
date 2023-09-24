# Rusty

[![Actions Status](https://github.com/busticated/rusty/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/busticated/rusty/actions) [![Rust Version Support](https://img.shields.io/badge/rust%20version-%3E%3D1.72.1-orange)](https://releases.rs/) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/busticated/rusty/blob/master/LICENSE)

A `cargo` workspace ([docs](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)) monorepo ([info](https://en.wikipedia.org/wiki/Monorepo)) hosting a collection of Rust utility crates.

[Installation](#installation) | [Crates](#crates) | [Docs](#docs--resources)


## Installation

1. Install Rust using `rustup` ([instructions](https://www.rust-lang.org/tools/install))
2. Clone this repository: `$ git clone git@github.com:busticated/rusty.git && cd ./rusty`
3. Setup local dev environment: `$ cargo xtask setup`
4. View available commands: `$ cargo xtask help`
5. Run the tests `$ cargo xtask test`
6. Start Hacking!


## Crates

<!-- crate-list-start -->
* [node-js-info](crates/node-js-info)
	* Asynchronously retrieve Node.js release info by version and platform
<!-- crate-list-end -->


## Docs & Resources

* [Rust](https://www.rust-lang.org)
* [Cargo](https://github.com/rust-lang/cargo)
* [Tokio](https://tokio.rs)
* [XTask](https://github.com/matklad/cargo-xtask)
* [Mockito](https://github.com/lipanski/mockito)

