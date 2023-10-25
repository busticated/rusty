use node_js_release_info::*;

const VERSION: &str = "20.7.0";

#[test]
fn it_provides_expected_resources() {
    let info = NodeJSRelInfo::new(VERSION);
    let os = NodeJSOS::Linux;
    let arch = NodeJSArch::X64;
    let ext = NodeJSPkgExt::Targz;
    assert_eq!(info.version, VERSION);
    assert_eq!(info.os, os);
    assert_eq!(info.arch, arch);
    assert_eq!(info.ext, ext);
}

#[tokio::test]
async fn it_fetches_node_js_release_info_for_a_given_configuration() {
    let mut info = NodeJSRelInfo::new(VERSION);
    let result = info.macos().x64().tar_gz().fetch().await.unwrap();
    assert_eq!(
        result.url,
        "https://nodejs.org/download/release/v20.7.0/node-v20.7.0-darwin-x64.tar.gz"
    );
    assert_eq!(
        result.sha256,
        "ceeba829f44e7573949f2ce2ad5def27f1d6daa55f2860bea82964851fae01bc"
    );
}

#[tokio::test]
async fn it_fetches_node_js_release_info_for_all_supported_configurations() {
    let info = NodeJSRelInfo::new(VERSION);
    let result = info.fetch_all().await.unwrap();
    assert_eq!(result.len(), 24);
    assert_eq!(
        result[4].url,
        "https://nodejs.org/download/release/v20.7.0/node-v20.7.0-darwin-x64.tar.gz"
    );
    assert_eq!(
        result[4].sha256,
        "ceeba829f44e7573949f2ce2ad5def27f1d6daa55f2860bea82964851fae01bc"
    );
}
