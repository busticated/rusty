//! Integration tests exercising the crate's public API as a consumer would
//!
//! NOTE: these hit the live Node.js downloads server

use node_js_release_info::*;

const VERSION: &str = "24.19.0";
const DARWIN_X64_URL: &str =
    "https://nodejs.org/download/release/v24.19.0/node-v24.19.0-darwin-x64.tar.gz";
const DARWIN_X64_SHA256: &str = "d1b5e999db158c62fe8f7267a4476b035d8bd93b1a605bac24a3f0dd166e3316";

#[test]
fn it_provides_expected_resources() {
    let info = NodeJsRelInfo::new(VERSION);
    let os = NodeJsOs::Linux;
    let arch = NodeJsArch::X64;
    let ext = NodeJsPkgExt::Targz;
    assert_eq!(info.version, VERSION);
    assert_eq!(info.os, os);
    assert_eq!(info.arch, arch);
    assert_eq!(info.ext, ext);
}

#[tokio::test]
async fn it_fetches_node_js_release_info_for_a_given_configuration() {
    let result = NodeJsRelInfo::new(VERSION)
        .macos()
        .x64()
        .tar_gz()
        .fetch()
        .await
        .unwrap();

    assert_eq!(result.url, DARWIN_X64_URL);
    assert_eq!(result.sha256, DARWIN_X64_SHA256);
}

#[tokio::test]
async fn it_fetches_node_js_release_info_for_all_supported_configurations() {
    let info = NodeJsRelInfo::new(VERSION);
    let result = info.fetch_all().await.unwrap();

    // NOTE: v24 dropped `linux-armv7l` and all three 32-bit Windows builds,
    // taking the recognized configuration count from 24 (as of v20) down to 19
    assert_eq!(result.len(), 19);
    assert_eq!(result[4].url, DARWIN_X64_URL);
    assert_eq!(result[4].sha256, DARWIN_X64_SHA256);
}
