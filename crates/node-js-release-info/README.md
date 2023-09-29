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

```rust
use node_js_release_info::{NodeJSRelInfo, NodeJSRelInfoError};

#[tokio::main]
async fn main() -> Result<(), NodeJSRelInfoError> {
  let info = NodeJSRelInfo::new("20.6.1").macos().arm64().fetch().await?;
  assert_eq!(info.version, "20.6.1");
  assert_eq!(info.filename, "node-v20.6.1-darwin-arm64.tar.gz");
  assert_eq!(info.sha256, "d8ba8018d45b294429b1a7646ccbeaeb2af3cdf45b5c91dabbd93e2a2035cb46");
  assert_eq!(info.url, "https://nodejs.org/download/release/v20.6.1/node-v20.6.1-darwin-arm64.tar.gz");
  Ok(())
}
```

