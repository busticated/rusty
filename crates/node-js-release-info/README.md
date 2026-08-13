# node-js-release-info

[![Latest Version](https://img.shields.io/crates/v/node-js-release-info.svg)](https://crates.io/crates/node-js-release-info)
[![Documentation](https://docs.rs/node-js-release-info/badge.svg)](https://docs.rs/node-js-release-info)
[![CI Status](https://github.com/busticated/rusty/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/busticated/rusty/actions)

Asynchronously retrieve Node.js release info by version and platform from the [downloads server](https://nodejs.org/download/release/)

## Installation

```shell
cargo add node-js-release-info
```

## Examples

This example uses [Tokio](https://tokio.rs), be sure to install it with:

```shell
cargo add tokio --features full
```

```rust
use node_js_release_info::{NodeJsRelInfo, NodeJsRelInfoError};

#[tokio::main]
async fn main() -> Result<(), NodeJsRelInfoError> {
  // get a specific configuration
  let info = NodeJsRelInfo::new("24.19.0").macos().arm64().fetch().await?;
  assert_eq!(info.version, "24.19.0");
  assert_eq!(info.filename, "node-v24.19.0-darwin-arm64.tar.gz");
  assert_eq!(info.sha256, "8294b7aa9b03997481c06babf1e8b270c859358f27da57a11509afe537ac381d");
  assert_eq!(info.url, "https://nodejs.org/download/release/v24.19.0/node-v24.19.0-darwin-arm64.tar.gz");

  // get all supported configurations
  let all = info.fetch_all().await?;
  assert_eq!(all.len(), 19);
  assert_eq!(all[2], info);
  println!("{:?}", all);
  Ok(())
}
```

## Features

Full `json` serialization + deserialization is available via the `json` feature.

```shell
cargo add node-js-release-info --features json
```

```rust,ignore
use node_js_release_info::NodeJsRelInfo;

#[tokio::main]
async fn main() {
  let info = NodeJsRelInfo::new("24.19.0").macos().arm64();
  let json = serde_json::to_string(&info).unwrap();
  let info_deserialized = serde_json::from_str(&json).unwrap();
  assert_eq!(info, info_deserialized);
}
```

