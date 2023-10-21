use crate::arch::NodeJSArch;
use crate::error::NodeJSRelInfoError;
use crate::url::NodeJSURLFormatter;
use crate::ext::NodeJSPkgExt;
use crate::os::NodeJSOS;
use semver::Version;
use std::str::FromStr;

pub fn validate_version<T: AsRef<str>>(semver: T) -> Result<String, NodeJSRelInfoError> {
    match Version::parse(semver.as_ref()) {
        Err(_) => return Err(NodeJSRelInfoError::InvalidVersion(semver.as_ref().to_owned())),
        Ok(v) => Ok(v.to_string()),
    }
}

pub async fn fetch(version: &String, url_fmt: &NodeJSURLFormatter) -> Result<String, NodeJSRelInfoError> {
    let info_url = url_fmt.info(version);
    let res = match reqwest::get(info_url.as_str()).await {
        Err(e) => return Err(NodeJSRelInfoError::HttpError(e)),
        Ok(r) => r,
    };

    // TODO (busticated): handle 5xx errors
    if res.status().as_u16() >= 400 {
        return Err(NodeJSRelInfoError::UnrecognizedVersion(version.clone()));
    }

    match res.text().await {
        Err(e) => Err(NodeJSRelInfoError::HttpError(e)),
        Ok(b) => Ok(b),
    }
}

pub type ParsedSpecs = Vec<(NodeJSOS, NodeJSArch, NodeJSPkgExt, String, String)>;

pub fn parse(version: &String, specs: String) -> Option<ParsedSpecs> {
    let mut all: ParsedSpecs = vec![];
    for line in specs.lines() {
        let (sha256, filename) = match line.trim().split_once(' ') {
            Some((s, f)) => (s.trim(), f.trim()),
            None => ("", ""),
        };

        if sha256.is_empty() || filename.is_empty() {
            continue;
        }

        if !filename.starts_with(format!("node-v{}", &version).as_str()) {
            continue;
        }

        let parts: Vec<&str> = filename.split('-').collect();
        let last = parts.last().unwrap(); // b/c it'll never be empty
        let is_msi = last.ends_with(".msi");

        if parts.len() < 4 && !is_msi {
            continue;
        }

        let os = if is_msi {
            "win"
        } else {
            parts[2]
        };

        let os = match NodeJSOS::from_str(os) {
            Ok(os) => os,
            Err(_) => {
                continue;
            }
        };

        let (arch, ext) = match last.split_once('.') {
            Some((a, e)) => (a.trim(), e.trim()),
            None => {
                continue;
            }
        };

        let arch = match NodeJSArch::from_str(arch) {
            Ok(a) => a,
            Err(_) => {
                continue;
            }
        };

        let ext = match NodeJSPkgExt::from_str(ext) {
            Ok(ext) => ext,
            Err(_) => {
                continue;
            }
        };

        let filename = filename.to_string();
        let sha256 = sha256.to_string();
        all.push((os, arch, ext, sha256, filename));
    }

    if all.is_empty() {
        return None;
    }

    Some(all)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validates_a_version_string() {
        let version = validate_version("20.6.1").unwrap();

        assert_eq!(version, String::from("20.6.1"));

        let error = validate_version("NOPE").unwrap_err();

        assert_eq!(format!("{error}"), "Error: Invalid Version! Received: 'NOPE'");

        let error = validate_version("").unwrap_err();

        assert_eq!(format!("{error}"), "Error: Invalid Version! Received: ''");
    }

    #[test]
    fn it_parses_node_js_specs() {
        let version = String::from("20.6.1");
        let specs_raw = get_fake_specs().to_string();
        let specs = parse(&version, specs_raw).unwrap();
        assert_eq!(specs.len(), 24);
        let (os, arch, ext, sha256, filename) = &specs[2];
        assert_eq!(*os, NodeJSOS::Darwin);
        assert_eq!(*arch, NodeJSArch::ARM64);
        assert_eq!(*ext, NodeJSPkgExt::Targz);
        assert_eq!(filename, "node-v20.6.1-darwin-arm64.tar.gz");
        assert_eq!(sha256, "d8ba8018d45b294429b1a7646ccbeaeb2af3cdf45b5c91dabbd93e2a2035cb46");
    }

    #[test]
    fn it_handles_empty_data_when_parsing_node_js_specs() {
        let version = String::from("20.6.1");
        let specs_raw = ["NOPE"];
        assert!(parse(&version, specs_raw.join("\n").to_string()).is_none());
    }

    #[test]
    fn it_ignores_invalid_data_when_parsing_node_js_specs() {
        let version = String::from("20.6.1");
        let specs_raw = ["NOPE", "FAKESHA node-v20.6.1-darwin-arm64.tar.gz"];
        let specs = parse(&version, specs_raw.join("\n").to_string()).unwrap();
        assert_is_darwin_arm64_targz_specs(specs);
    }

    #[test]
    fn it_ignores_unknown_file_when_parsing_node_js_specs() {
        let version = String::from("20.6.1");
        let specs_raw = ["FAKESHA win_x86/node.lib", "FAKESHA node-v20.6.1-darwin-arm64.tar.gz"];
        let specs = parse(&version, specs_raw.join("\n").to_string()).unwrap();
        assert_is_darwin_arm64_targz_specs(specs);
    }

    #[test]
    fn it_ignores_unknown_filename_when_parsing_node_js_specs() {
        let version = String::from("20.6.1");
        let specs_raw = ["FAKESHA NOPE-v20.6.1-darwin-arm64.tar.gz", "FAKESHA node-v20.6.1-darwin-arm64.tar.gz"];
        let specs = parse(&version, specs_raw.join("\n").to_string()).unwrap();
        assert_is_darwin_arm64_targz_specs(specs);
    }

    #[test]
    fn it_ignores_malformed_filename_when_parsing_node_js_specs() {
        let version = String::from("20.6.1");
        let specs_raw = ["FAKESHA node-v20.6.1-NOPE-", "FAKESHA node-v20.6.1-darwin-arm64.tar.gz"];
        let specs = parse(&version, specs_raw.join("\n").to_string()).unwrap();
        assert_is_darwin_arm64_targz_specs(specs);
    }

    #[test]
    fn it_ignores_unknown_os_when_parsing_node_js_specs() {
        let version = String::from("20.6.1");
        let specs_raw = ["FAKESHA node-v20.6.1-NOPE-arm64.tar.gz", "FAKESHA node-v20.6.1-darwin-arm64.tar.gz"];
        let specs = parse(&version, specs_raw.join("\n").to_string()).unwrap();
        assert_is_darwin_arm64_targz_specs(specs);
    }

    #[test]
    fn it_ignores_unknown_arch_when_parsing_node_js_specs() {
        let version = String::from("20.6.1");
        let specs_raw = ["FAKESHA node-v20.6.1-darwin-NOPE.tar.gz", "FAKESHA node-v20.6.1-darwin-arm64.tar.gz"];
        let specs = parse(&version, specs_raw.join("\n").to_string()).unwrap();
        assert_is_darwin_arm64_targz_specs(specs);
    }

    #[test]
    fn it_ignores_unknown_ext_when_parsing_node_js_specs() {
        let version = String::from("20.6.1");
        let specs_raw = ["FAKESHA node-v20.6.1-darwin-arm64.NOPE", "FAKESHA node-v20.6.1-darwin-arm64.tar.gz"];
        let specs = parse(&version, specs_raw.join("\n").to_string()).unwrap();
        assert_is_darwin_arm64_targz_specs(specs);
    }

    #[test]
    fn it_ignores_missing_ext_when_parsing_node_js_specs() {
        let version = String::from("20.6.1");
        let specs_raw = ["FAKESHA node-v20.6.1-darwin-arm64", "FAKESHA node-v20.6.1-darwin-arm64.tar.gz"];
        let specs = parse(&version, specs_raw.join("\n").to_string()).unwrap();
        assert_is_darwin_arm64_targz_specs(specs);
    }

    #[tokio::test]
    async fn it_fetches_node_js_specs() {
        let version = String::from("20.6.1");
        let mut url_fmt = NodeJSURLFormatter::new();
        let mut server = Server::new_async().await;
        let mock = setup_server_mock(&version, &mut url_fmt, &mut server)
            .with_body(get_fake_specs())
            .create_async()
            .await;

        let specs = fetch(&version, &url_fmt).await.unwrap();
        mock.assert_async().await;
        assert_eq!(specs, get_fake_specs());
    }

    #[tokio::test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: UnrecognizedVersion(\"1.0.0\")")]
    async fn it_fails_to_fetch_node_js_specs_when_version_is_unrecognized() {
        let version = String::from("1.0.0");
        let mut url_fmt = NodeJSURLFormatter::new();
        let mut server = Server::new_async().await;
        let mock = setup_server_mock(&version, &mut url_fmt, &mut server)
            .with_body(get_fake_specs())
            .with_status(404)
            .create_async()
            .await;

        fetch(&version, &url_fmt).await.unwrap();
        mock.assert_async().await;
    }
}

#[cfg(test)]
use mockito::{Server, Mock};

#[cfg(test)]
fn assert_is_darwin_arm64_targz_specs(specs: ParsedSpecs) {
    assert_eq!(specs.len(), 1);
    let (os, arch, ext, sha256, filename) = &specs[0];
    assert_eq!(*os, NodeJSOS::Darwin);
    assert_eq!(*arch, NodeJSArch::ARM64);
    assert_eq!(*ext, NodeJSPkgExt::Targz);
    assert_eq!(filename, "node-v20.6.1-darwin-arm64.tar.gz");
    assert_eq!(sha256, "FAKESHA");
}

#[cfg(test)]
pub fn setup_server_mock(version: &str, url_fmt: &mut NodeJSURLFormatter, server: &mut Server) -> Mock {
    url_fmt.host = server.host_with_port();
    url_fmt.protocol = "http:".to_string();
    server.mock("GET", url_fmt.info_pathname(version).as_str())
}

#[cfg(test)]
pub fn get_fake_specs() -> &'static str {
    "ea52b4feaf917e08cd2c729c1186585fcacef07c261a01310c91333b9e41d93c  node-v20.6.1-aix-ppc64.tar.gz
    9471bd6dc491e09c31b0f831f5953284b8a6842ed4ccb98f5c62d13e6086c471  node-v20.6.1-arm64.msi
    d8ba8018d45b294429b1a7646ccbeaeb2af3cdf45b5c91dabbd93e2a2035cb46  node-v20.6.1-darwin-arm64.tar.gz
    9c61b0d60fce962244d5e54549dc912e28b3c5f5e23149bfd15f66f8f7269129  node-v20.6.1-darwin-arm64.tar.xz
    365ec544c6596f194afff9a613554abfc68d4a2274181b7651386d9a11cf5862  node-v20.6.1-darwin-x64.tar.gz
    9b10c16670781e3a5af722656d28f264cdd8ebb3140f62692b33813100391349  node-v20.6.1-darwin-x64.tar.xz
    d8271461ced2887f65af413949caee19db3e80d22bbefdaf01252ca998570052  node-v20.6.1-headers.tar.gz
    60963e3ee60b6739e97e0c7b8ffb25848a82649c0c277af728400c570fd9db6d  node-v20.6.1-headers.tar.xz
    d38fe2e41e3fe8ae81b517b4cf49521f500e181e54f4c3d05e2b2d691a57b2ca  node-v20.6.1-linux-arm64.tar.gz
    6823720796b287465bb4aa8e7611143322ffd6cbdb9c6e3b149576f6d87953bf  node-v20.6.1-linux-arm64.tar.xz
    459510281ea51cf5d89fc666e36fbba80793ae4b90c3a7f89dd6666c65c825b3  node-v20.6.1-linux-armv7l.tar.gz
    9dbd4fd7f804a28de91ffb8792df6e89bbb4f934fccd013624b3dabf8bf809ac  node-v20.6.1-linux-armv7l.tar.xz
    ca00f1aa8b2535fa167258cf5f2cfce4b79d83c442dd5e46f5e17d6a5749ec0f  node-v20.6.1-linux-ppc64le.tar.gz
    27884935b025b6676e4b8737f334673ee825947d0baef61aa0326374597aeb05  node-v20.6.1-linux-ppc64le.tar.xz
    4a3f29cfc8a7ed1e9e44fcacb78e2fbaa3ce01be1efc4971a42710ad1e9e45d1  node-v20.6.1-linux-s390x.tar.gz
    3968d629989b6de16b8872b6d7ee6e6cdf1204def99c43412a6ee28203ed0022  node-v20.6.1-linux-s390x.tar.xz
    26dd13a6f7253f0ab9bcab561353985a297d927840771d905566735b792868da  node-v20.6.1-linux-x64.tar.gz
    591f9f274104f266a8cf085d2c7d5d2848ba73b98ae323d501db2d4c4b7026e5  node-v20.6.1-linux-x64.tar.xz
    d9acf82d9576dd0350c8e66b55f6fc2750fa9f4aa23d6453ffc58e32af995894  node-v20.6.1.pkg
    0053c09a01b1b355bca5af82927cae376124c13d74fa53567f08f4cfb085e6aa  node-v20.6.1.tar.gz
    3aec5e728daa38800c343b129221d3488064a2529a39bb5467bc55be226c6a2b  node-v20.6.1.tar.xz
    337549faf397deb0da3bccd4e27db45a619d89de4ea12830d16d9dfaded8e92c  node-v20.6.1-win-arm64.7z
    0e62045bfc9d7c38360bd7da152c75ed82087242d5e4b401fa23a439588d36f6  node-v20.6.1-win-arm64.zip
    c6cfe7824770a266a30bee8c33f485d0e89b94254c682250a239d83adfb7ce77  node-v20.6.1-win-x64.7z
    88371914f1f75d594bb367570e163cf5ecebeb514fd54cc765093819ebb0ed48  node-v20.6.1-win-x64.zip
    87d631b294a25386400d0f44d227330da62a1326e2a4fbb98bda3d7c431257f1  node-v20.6.1-win-x86.7z
    578cff623601aa8878a035f06edbf69190338ee3b345e7a096e804cb80c4ce24  node-v20.6.1-win-x86.zip
    5c2616da46728dd1326645c7db114e78ad87138a258c0724a035269258c23509  node-v20.6.1-x64.msi
    cb83586af83182187e760b7e01aa7c7b2bacb521d60ceefed3ac6fc62c222449  node-v20.6.1-x86.msi
    7cc3240fd7ce7926eef1cbbad33b033f7c5d97b3f3e527d65ff1e2c3f7638a11  win-arm64/node.exe
    deb027ded744371657811cfe52e774881ea928d36779924af84aa9a7a31104d2  win-arm64/node.lib
    dcb6b4bc6f2a78bf0f759853b59e94ddbe9ad6b9f32d24fdcf590d74c6350bc2  win-arm64/node_pdb.7z
    bdcd574e99646ec4a03bb13b3661c957f5a7ca837f5c33827075c4262d449689  win-arm64/node_pdb.zip
    5b824f3a375cca06dfd7dc70fa341a6ef8bb0b2e912358d8602a0c7ad273b9a4  win-x64/node.exe
    d275cfc4d637d2feaf4c39e1a5f5cd84f5b474fa713c15013e940c329feed13b  win-x64/node.lib
    fea6c0fcff45739a6e5af9843ec45455c97ff8677167bd649fd48cbef59ca52d  win-x64/node_pdb.7z
    bc13f5e63c1510cd41f82dc20725f40bbfa378252e09a00a8531cddabbf1b106  win-x64/node_pdb.zip
    837db0d8fb7fa194ebe23cd34ac7bedc02d1132de67cf4f147d694574be5cc4e  win-x86/node.exe
    a0738dec64427ae73eeb1d036081652c1c0223a679a63e0459c2af667f284f58  win-x86/node.lib
    516ac820f05eb8478be541ac12386c3b5b5c07624f73934bcf0b11a3fcdb1c95  win-x86/node_pdb.7z
    9b68f3e1f1717a2f6a090e1679f8cc627566ed064c657c35eddd0dba9484e310  win-x86/node_pdb.zip"
}
