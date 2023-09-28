# node-js-info

[![Latest Version](https://img.shields.io/crates/v/node-js-info.svg)](https://crates.io/crates/node-js-info)
[![Documentation](https://docs.rs/node-js-info/badge.svg)](https://docs.rs/node-js-info)

Asynchronously retrieve Node.js release info by version and platform from the [downloads server](https://nodejs.org/download/release/)

## Installation

```shell
cargo add node-js-info
```

## Examples

```rust
use node_js_info::NodeJSInfo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let info = NodeJSInfo::new("20.6.1").macos().arm64().fetch().await?;
  assert_eq!(info.version, "20.6.1");
  assert_eq!(info.filename, "node-v20.6.1-darwin-arm64.tar.gz");
  assert_eq!(info.sha256, "d8ba8018d45b294429b1a7646ccbeaeb2af3cdf45b5c91dabbd93e2a2035cb46");
  assert_eq!(info.url, "https://nodejs.org/download/release/v20.6.1/node-v20.6.1-darwin-arm64.tar.gz");
  Ok(())
}
```


