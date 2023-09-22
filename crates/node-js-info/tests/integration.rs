use node_js_info::*;

#[test]
fn it_provides_expected_resources() {
    let info = NodeJSInfo::default();
    let os = NodeJSOS::default();
    let arch = NodeJSArch::default();
    let ext = NodeJSPkgExt::default();
    let url_fmt = NodeJSURLFormatter::default();
    assert_eq!(info.os, os);
    assert_eq!(info.arch, arch);
    assert_eq!(info.ext, ext);
    assert_eq!(info.url_fmt, url_fmt);
}

#[tokio::test]
async fn it_works() {
    let mut info = NodeJSInfo::new("20.7.0");
    let result = info.macos().x64().tar_gz().fetch().await.unwrap();
    assert_eq!(result.url, format!("https://nodejs.org/download/release/v20.7.0/node-v20.7.0-darwin-x64.tar.gz"));
    assert_eq!(result.sha256, format!("ceeba829f44e7573949f2ce2ad5def27f1d6daa55f2860bea82964851fae01bc"));
}
