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


## Migrations

<details id="migrate-1x-to-2x">
<summary><b>1.x -> 2.x</b></summary>
<p>

**Type and variant names now follow [RFC 430](https://rust-lang.github.io/rfcs/0430-finalizing-naming-conventions.html)**

| before | after |
| --- | --- |
| `NodeJSRelInfo` | `NodeJsRelInfo` |
| `NodeJSRelInfoError` | `NodeJsRelInfoError` |
| `NodeJSOS` | `NodeJsOs` |
| `NodeJSArch` | `NodeJsArch` |
| `NodeJSPkgExt` | `NodeJsPkgExt` |
| `NodeJSOS::AIX` | `NodeJsOs::Aix` |
| `NodeJSArch::ARM64` | `NodeJsArch::Arm64` |
| `NodeJSArch::ARMV7L` | `NodeJsArch::Armv7l` |
| `NodeJSArch::PPC64` | `NodeJsArch::Ppc64` |
| `NodeJSArch::PPC64LE` | `NodeJsArch::Ppc64le` |
| `NodeJSArch::S390X` | `NodeJsArch::S390x` |

**Builder methods consume `self`**

They now return an owned value directly, so the trailing `to_owned()` is no longer needed - and `NodeJsRelInfo::to_owned()` has been removed. Use `.clone()` if you want a copy.

```rust,ignore
// before
let info = NodeJSRelInfo::new("24.19.0").macos().arm64().to_owned();
// after
let info = NodeJsRelInfo::new("24.19.0").macos().arm64();
```

Calling a builder as a statement no longer works, since it takes `self`:

```rust,ignore
// before
let mut info = NodeJSRelInfo::new("24.19.0");
info.macos();
// after
let info = NodeJsRelInfo::new("24.19.0").macos();
```

**`fetch()` consumes `self`**

It previously mutated in place *and* returned a clone. It now takes `self` and returns the populated value:

```rust,ignore
// before
let mut info = NodeJSRelInfo::new("24.19.0");
info.fetch().await?;          // `info` updated in place
// after
let info = NodeJsRelInfo::new("24.19.0").fetch().await?;
```

**Enums are `#[non_exhaustive]`**

`NodeJsOs`, `NodeJsArch`, `NodeJsPkgExt` and `NodeJsRelInfoError` may gain variants in a minor release, so a `match` over them needs a `_` arm. Node.js adds and removes target platforms over time, and this keeps that from being a breaking change.

**Error messages changed**

The `Error: ` prefix is gone and messages are lowercased, per [API guideline C-GOOD-ERR](https://rust-lang.github.io/api-guidelines/interoperability.html#error-types-are-meaningful-and-well-behaved). Previously they composed into chains as `Error: Error: ...`.

```text
before:  Error: Invalid Version! Received: 'x'
after:   invalid version - received: 'x'
```

`NodeJsRelInfoError` also implements `Error::source()` now, exposing the underlying `reqwest::Error` behind `HttpError`.

**New variants**

`NodeJsOs::SunOs` (`sunos`) and `NodeJsArch::Armv6l` (`armv6l`) were missing. Both appear in older Node.js releases, so fetching those versions previously failed with `UnrecognizedOs` / `UnrecognizedArch` on valid artifacts.

Note that Node.js v24 *dropped* `linux-armv7l` and all 32-bit Windows builds, so `fetch_all` returns 19 configurations for v24 where v20 returned 24.

</p>
</details>
