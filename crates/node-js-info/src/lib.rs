#![doc = include_str!("../README.md")]

mod os;
mod arch;
mod ext;
mod url;

use std::string::ToString;
use std::error::Error;
use semver::Version;
use strum::ParseError;
pub use crate::os::NodeJSOS;
pub use crate::arch::NodeJSArch;
use crate::ext::NodeJSPkgExt;
use crate::url::NodeJSURLFormatter;

type DynError = Box<dyn Error>;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeJSInfo {
    /// The operating system for the Node.js distributable you are targeting
    pub os: NodeJSOS,
    /// The CPU architecture for the Node.js distributable you are targeting
    pub arch: NodeJSArch,
    /// The version of Node.js you are targeting as a [semver](https://semver.org) string
    pub version: String,
    /// The filename of the Node.js distributable (populated after fetching)
    pub filename: String,
    /// The hash for the Node.js distributable (populated after fetching)
    pub sha256: String,
    /// The fully qualified url for the Node.js distributable (populated after fetching)
    pub url: String,
    ext: NodeJSPkgExt,
    url_fmt: NodeJSURLFormatter,
}

impl NodeJSInfo {
    /// Creates a new instance using default settings
    ///
    /// # Arguments
    ///
    /// * `semver` - The Node.js version you are targeting (`String` / `&str`)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_info::NodeJSInfo;
    /// let info = NodeJSInfo::new("20.6.1");
    /// ```
    pub fn new<T: AsRef<str>>(semver: T) -> Self {
        NodeJSInfo {
            version: semver.as_ref().to_owned(),
            ..Default::default()
        }
    }

    /// Creates a new instance mirroring current environment based on `std::env::consts::OS` and `std::env::consts::ARCH`
    ///
    /// # Arguments
    ///
    /// * `semver` - The Node.js version you are targeting (`String` / `&str`)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_info::NodeJSInfo;
    /// let info = NodeJSInfo::from_env("20.6.1");
    /// ```
    // TODO (busticated): reexport ParseError? or introduce customer error and convert?
    pub fn from_env<T: AsRef<str>>(semver: T) -> Result<NodeJSInfo, ParseError> {
        let mut info = NodeJSInfo::new(semver);
        info.os = NodeJSOS::from_env().unwrap();
        info.arch = NodeJSArch::from_env().unwrap();
        info.ext = match info.os {
            NodeJSOS::Windows => NodeJSPkgExt::Zip,
            _ => NodeJSPkgExt::Targz,
        };
        Ok(info)
    }

    /// Sets instance `os` field to `darwin`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_info::NodeJSInfo;
    /// let info = NodeJSInfo::new("20.6.1").macos();
    /// ```
    pub fn macos(&mut self) -> &mut Self {
        self.os = NodeJSOS::Darwin;
        self
    }

    /// Sets instance `os` field to `linux`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_info::NodeJSInfo;
    /// let info = NodeJSInfo::new("20.6.1").linux();
    /// ```
    pub fn linux(&mut self) -> &mut Self {
        self.os = NodeJSOS::Linux;
        self
    }

    /// Sets instance `os` field to `windows`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_info::NodeJSInfo;
    /// let info = NodeJSInfo::new("20.6.1").windows();
    /// ```
    pub fn windows(&mut self) -> &mut Self {
        self.os = NodeJSOS::Windows;
        self
    }

    /// Sets instance `arch` field to `x64`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_info::NodeJSInfo;
    /// let info = NodeJSInfo::new("20.6.1").x64();
    /// ```
    pub fn x64(&mut self) -> &mut Self {
        self.arch = NodeJSArch::X64;
        self
    }

    /// Sets instance `arch` field to `x86`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_info::NodeJSInfo;
    /// let info = NodeJSInfo::new("20.6.1").x86();
    /// ```
    pub fn x86(&mut self) -> &mut Self {
        self.arch = NodeJSArch::X86;
        self
    }

    /// Sets instance `arch` field to `arm64`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_info::NodeJSInfo;
    /// let info = NodeJSInfo::new("20.6.1").arm64();
    /// ```
    pub fn arm64(&mut self) -> &mut Self {
        self.arch = NodeJSArch::ARM64;
        self
    }

    /// Sets instance `arch` field to `armv7l`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_info::NodeJSInfo;
    /// let info = NodeJSInfo::new("20.6.1").armv7l();
    /// ```
    pub fn armv7l(&mut self) -> &mut Self {
        self.arch = NodeJSArch::ARMV7L;
        self
    }

    /// Sets instance `arch` field to `ppc64le`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_info::NodeJSInfo;
    /// let info = NodeJSInfo::new("20.6.1").ppc64le();
    /// ```
    pub fn ppc64le(&mut self) -> &mut Self {
        self.arch = NodeJSArch::PPC64LE;
        self
    }

    /// Sets instance `ext` field to `tar.gz`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_info::NodeJSInfo;
    /// let info = NodeJSInfo::new("20.6.1").tar_gz();
    /// ```
    pub fn tar_gz(&mut self) -> &mut Self {
        self.ext = NodeJSPkgExt::Targz;
        self
    }

    /// Sets instance `ext` field to `tar.xz`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_info::NodeJSInfo;
    /// let info = NodeJSInfo::new("20.6.1").tar_xz();
    /// ```
    pub fn tar_xz(&mut self) -> &mut Self {
        self.ext = NodeJSPkgExt::Tarxz;
        self
    }

    /// Sets instance `ext` field to `zip`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_info::NodeJSInfo;
    /// let info = NodeJSInfo::new("20.6.1").zip();
    /// ```
    pub fn zip(&mut self) -> &mut Self {
        self.ext = NodeJSPkgExt::Zip;
        self
    }

    /// Sets instance `ext` field to `msi`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_info::NodeJSInfo;
    /// let info = NodeJSInfo::new("20.6.1").msi();
    /// ```
    pub fn msi(&mut self) -> &mut Self {
        self.ext = NodeJSPkgExt::Msi;
        self
    }

    /// Creates owned data from reference for convenience when chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_info::NodeJSInfo;
    /// let info = NodeJSInfo::new("20.6.1").windows().x64().zip().to_owned();
    /// ```
    pub fn to_owned(&self) -> Self {
        self.clone()
    }

    /// Creates JSON String from instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_info::NodeJSInfo;
    /// let info = NodeJSInfo::new("20.6.1");
    /// assert_eq!(info.to_json_string(), "{\"version\":\"20.6.1\",\"os\":\"linux\",\"arch\":\"x64\",\"filename\":\"node-v20.6.1-linux-x64.tar.gz\",\"sha256\":\"\",\"url\":\"\"}");
    /// ```
    // TODO (busticated): should probably just use serde
    pub fn to_json_string(&self) -> String {
        let entries = vec![
            format!("\"version\":\"{}\"", self.version),
            format!("\"os\":\"{}\"", self.os),
            format!("\"arch\":\"{}\"", self.arch),
            format!("\"filename\":\"{}\"", self.filename()),
            format!("\"sha256\":\"{}\"", self.sha256),
            format!("\"url\":\"{}\"", self.url),
        ];

        format!("{{{}}}", entries.join(","))
    }

    /// Fetches Node.js metadata from the [releases download server](https://nodejs.org/download/release/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_info::NodeJSInfo;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///   let info = NodeJSInfo::new("20.6.1").macos().arm64().fetch().await?;
    ///   assert_eq!(info.version, "20.6.1");
    ///   assert_eq!(info.filename, "node-v20.6.1-darwin-arm64.tar.gz");
    ///   assert_eq!(info.sha256, "d8ba8018d45b294429b1a7646ccbeaeb2af3cdf45b5c91dabbd93e2a2035cb46");
    ///   assert_eq!(info.url, "https://nodejs.org/download/release/v20.6.1/node-v20.6.1-darwin-arm64.tar.gz");
    ///   Ok(())
    /// }
    /// ```
    pub async fn fetch(&mut self) -> Result<Self, DynError> {
        self.version = match Version::parse(self.version.as_str()) {
            Err(e) => return Err(Box::new(e)),
            Ok(v) => v.to_string(),
        };

        let info_url = self.url_fmt.info(&self.version);
        let res = match reqwest::get(info_url.as_str()).await {
            Err(e) => return Err(Box::new(e)),
            Ok(r) => r,
        };

        // TODO (busticated): handle 5xx errors
        if res.status().as_u16() >= 400 {
            return Err(format!("Unrecognized version! Received: {}", self.version))?
        }

        let body = match res.text().await {
            Err(e) => return Err(Box::new(e)),
            Ok(b) => b,
        };

        let filename = self.filename();
        let info = body.lines().find(|&line| {
            line.contains(filename.as_str())
        });

        let mut specs = match info {
            None => return Err(format!("Unrecognized configuration! Using: {}", filename))?,
            Some(s) => s.split_whitespace(),
        };

        self.filename = filename;
        self.sha256 = specs.nth(0).unwrap().to_string();
        self.url = self.url_fmt.pkg(&self.version, &self.filename);
        Ok(self.to_owned())
    }

    fn filename(&self) -> String {
        let arch = self.arch.to_string();
        let ext = self.ext.to_string();

        if self.ext == NodeJSPkgExt::Msi {
            return format!("node-v{}-{}.{}", self.version, arch, ext);
        }

        format!("node-v{}-{}-{}.{}", self.version, self.os, arch, ext)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[cfg(test)]
mod tests {
    use mockito::{Server, Mock};
    use super::*;

    #[test]
    fn it_initializes(){
        let info = NodeJSInfo::new("1.0.0");
        assert_eq!(info.os, NodeJSOS::Linux);
        assert_eq!(info.arch, NodeJSArch::X64);
        assert_eq!(info.ext, NodeJSPkgExt::Targz);
        assert_eq!(info.version, "1.0.0".to_string());
        assert_eq!(info.filename, "".to_string());
        assert_eq!(info.sha256, "".to_string());
        assert_eq!(info.url, "".to_string());
    }

    #[test]
    fn it_initializes_with_defaults() {
        let info = NodeJSInfo::default();
        assert_eq!(info.os, NodeJSOS::Linux);
        assert_eq!(info.arch, NodeJSArch::X64);
        assert_eq!(info.ext, NodeJSPkgExt::Targz);
        assert_eq!(info.version, "".to_string());
        assert_eq!(info.filename, "".to_string());
        assert_eq!(info.sha256, "".to_string());
        assert_eq!(info.url, "".to_string());
    }

    #[test]
    #[cfg_attr(not(target_os = "macos"), ignore)]
    fn it_initializes_using_current_environment_on_macos() {
        let info = NodeJSInfo::from_env("1.0.0").unwrap();
        assert_eq!(info.ext, NodeJSPkgExt::Targz);
    }

    #[test]
    #[cfg_attr(not(target_os = "linux"), ignore)]
    fn it_initializes_using_current_environment_on_linux() {
        let info = NodeJSInfo::from_env("1.0.0").unwrap();
        assert_eq!(info.ext, NodeJSPkgExt::Targz);
    }

    #[test]
    #[cfg_attr(not(target_os = "windows"), ignore)]
    fn it_initializes_using_current_environment_on_windows() {
        let info = NodeJSInfo::from_env("1.0.0").unwrap();
        assert_eq!(info.ext, NodeJSPkgExt::Zip);
    }

    #[test]
    fn it_sets_os() {
        let mut info = NodeJSInfo::new("1.0.0");

        assert_eq!(info.os, NodeJSOS::Linux);

        info.windows();

        assert_eq!(info.os, NodeJSOS::Windows);

        info.macos();

        assert_eq!(info.os, NodeJSOS::Darwin);

        info.linux();

        assert_eq!(info.os, NodeJSOS::Linux);
    }

    #[test]
    fn it_sets_arch() {
        let mut info = NodeJSInfo::new("1.0.0");

        info.x86();

        assert_eq!(info.arch, NodeJSArch::X86);

        info.x64();

        assert_eq!(info.arch, NodeJSArch::X64);

        info.arm64();

        assert_eq!(info.arch, NodeJSArch::ARM64);

        info.armv7l();

        assert_eq!(info.arch, NodeJSArch::ARMV7L);

        info.ppc64le();

        assert_eq!(info.arch, NodeJSArch::PPC64LE);
    }

    #[test]
    fn it_sets_ext() {
        let mut info = NodeJSInfo::new("1.0.0");

        info.zip();

        assert_eq!(info.ext, NodeJSPkgExt::Zip);

        info.tar_gz();

        assert_eq!(info.ext, NodeJSPkgExt::Targz);

        info.tar_xz();

        assert_eq!(info.ext, NodeJSPkgExt::Tarxz);

        info.msi();

        assert_eq!(info.ext, NodeJSPkgExt::Msi);
    }

    #[test]
    fn it_gets_owned_copy() {
        let mut info1 = NodeJSInfo::new("1.0.0");
        let info2 = info1.to_owned();

        assert_eq!(info1, info2);

        info1.windows();

        assert_ne!(info1, info2);
    }

    #[test]
    fn it_gets_json_string() {
        let mut info = NodeJSInfo::new("1.0.0").macos().x64().zip().to_owned();
        info.sha256 = "fake-sha256".into();
        info.url = "https://example.com/fake-url".into();
        let json = info.to_json_string();
        let result: Vec<&str> = json.split(',').collect();

        assert_eq!(result, vec![
            "{\"version\":\"1.0.0\"",
            "\"os\":\"darwin\"",
            "\"arch\":\"x64\"",
            "\"filename\":\"node-v1.0.0-darwin-x64.zip\"",
            "\"sha256\":\"fake-sha256\"",
            "\"url\":\"https://example.com/fake-url\"}"
        ]);

        info.windows().arm64().msi();
        let json = info.to_json_string();
        let result: Vec<&str> = json.split(',').collect();

        assert_eq!(result, vec![
            "{\"version\":\"1.0.0\"",
            "\"os\":\"win\"",
            "\"arch\":\"arm64\"",
            "\"filename\":\"node-v1.0.0-arm64.msi\"",
            "\"sha256\":\"fake-sha256\"",
            "\"url\":\"https://example.com/fake-url\"}"
        ]);
    }

    #[test]
    fn it_formats_filename() {
        let info = NodeJSInfo::new("1.0.0").macos().x64().zip().to_owned();

        assert_eq!(info.filename(), "node-v1.0.0-darwin-x64.zip");

        let info = NodeJSInfo::new("1.0.0").windows().x64().msi().to_owned();

        assert_eq!(info.filename(), "node-v1.0.0-x64.msi");
    }

    #[tokio::test]
    #[should_panic(expected = "unexpected character 'N' while parsing major version number")]
    async fn it_fails_to_fetch_info_when_version_is_invalid() {
        let mut info = NodeJSInfo::new("NOPE!");
        info.fetch().await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Unrecognized version! Received: 1.0.0")]
    async fn it_fails_to_fetch_info_when_version_is_unrecognized() {
        let version = "1.0.0";
        let mut info = NodeJSInfo::new(version);
        let mut server = Server::new_async().await;
        let mock = setup_server_mock(version, &mut info, &mut server)
            .with_body(get_fake_info())
            .with_status(404)
            .create_async()
            .await;

        info.fetch().await.unwrap();
        mock.assert_async().await;
    }

    #[tokio::test]
    #[should_panic(expected = "Unrecognized configuration! Using: node-v20.6.1-linux-x64.zip")]
    async fn it_fails_to_fetch_info_when_configuration_is_unrecognized() {
        let version = "20.6.1";
        let mut server = Server::new_async().await;
        let mut info = NodeJSInfo::new(version).linux().zip().to_owned();
        let mock = setup_server_mock(version, &mut info, &mut server)
            .with_body(get_fake_info())
            .create_async()
            .await;

        info.fetch().await.unwrap();
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn it_fetches_node_js_info() {
        let version = "20.6.1";
        let mut info = NodeJSInfo::new(version);
        let mut server = Server::new_async().await;
        let mock = setup_server_mock(version, &mut info, &mut server)
            .with_body(get_fake_info())
            .create_async()
            .await;

        info.fetch().await.unwrap();
        mock.assert_async().await;

        assert_eq!(info.filename, "node-v20.6.1-linux-x64.tar.gz");
        assert_eq!(info.url, format!("{}{}", server.url(), "/download/release/v20.6.1/node-v20.6.1-linux-x64.tar.gz"));
        assert_eq!(info.sha256, "26dd13a6f7253f0ab9bcab561353985a297d927840771d905566735b792868da");
    }

    #[tokio::test]
    async fn it_fetches_node_js_info_when_ext_is_msi() {
        let version = "20.6.1";
        let mut info = NodeJSInfo::new(version).arm64().msi().to_owned();
        let mut server = Server::new_async().await;
        let mock = setup_server_mock(version, &mut info, &mut server)
            .with_body(get_fake_info())
            .create_async()
            .await;

        info.fetch().await.unwrap();
        mock.assert_async().await;

        assert_eq!(info.filename, "node-v20.6.1-arm64.msi");
        assert_eq!(info.url, format!("{}{}", server.url(), "/download/release/v20.6.1/node-v20.6.1-arm64.msi"));
        assert_eq!(info.sha256, "9471bd6dc491e09c31b0f831f5953284b8a6842ed4ccb98f5c62d13e6086c471");
    }

    fn setup_server_mock(version: &str, info: &mut NodeJSInfo, server: &mut Server) -> Mock {
        info.url_fmt.host = server.host_with_port();
        info.url_fmt.protocol = "http:".to_string();
        server.mock("GET", info.url_fmt.info_pathname(version).as_str())
    }

    fn get_fake_info() -> &'static str {
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
}
