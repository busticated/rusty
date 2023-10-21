#![doc = include_str!("../README.md")]

mod os;
mod arch;
mod error;
mod ext;
mod specs;
mod url;

use std::string::ToString;
#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};
pub use crate::os::NodeJSOS;
pub use crate::arch::NodeJSArch;
pub use crate::error::NodeJSRelInfoError;
pub use crate::ext::NodeJSPkgExt;
use crate::url::NodeJSURLFormatter;

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "json", derive(Deserialize, Serialize))]
pub struct NodeJSRelInfo {
    /// The operating system for the Node.js distributable you are targeting
    pub os: NodeJSOS,
    /// The CPU architecture for the Node.js distributable you are targeting
    pub arch: NodeJSArch,
    /// The file extension for the Node.js distributable you are targeting
    pub ext: NodeJSPkgExt,
    /// The version of Node.js you are targeting as a [semver](https://semver.org) string
    pub version: String,
    /// The filename of the Node.js distributable (populated after fetching)
    pub filename: String,
    /// The hash for the Node.js distributable (populated after fetching)
    pub sha256: String,
    /// The fully qualified url for the Node.js distributable (populated after fetching)
    pub url: String,
    #[cfg_attr(feature = "json", serde(skip))]
    url_fmt: NodeJSURLFormatter,
}

impl NodeJSRelInfo {
    /// Creates a new instance using default settings
    ///
    /// # Arguments
    ///
    /// * `semver` - The Node.js version you are targeting (`String` / `&str`)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::new("20.6.1");
    /// ```
    pub fn new<T: AsRef<str>>(semver: T) -> Self {
        NodeJSRelInfo {
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
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::from_env("20.6.1");
    /// ```
    pub fn from_env<T: AsRef<str>>(semver: T) -> Result<NodeJSRelInfo, NodeJSRelInfoError> {
        let mut info = NodeJSRelInfo::new(semver);
        info.os = NodeJSOS::from_env()?;
        info.arch = NodeJSArch::from_env()?;
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
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::new("20.6.1").macos();
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
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::new("20.6.1").linux();
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
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::new("20.6.1").windows();
    /// ```
    pub fn windows(&mut self) -> &mut Self {
        self.os = NodeJSOS::Windows;
        self
    }

    /// Sets instance `os` field to `aix`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::new("20.6.1").aix();
    /// ```
    pub fn aix(&mut self) -> &mut Self {
        self.os = NodeJSOS::AIX;
        self
    }

    /// Sets instance `arch` field to `x64`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::new("20.6.1").x64();
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
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::new("20.6.1").x86();
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
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::new("20.6.1").arm64();
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
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::new("20.6.1").armv7l();
    /// ```
    pub fn armv7l(&mut self) -> &mut Self {
        self.arch = NodeJSArch::ARMV7L;
        self
    }

    /// Sets instance `arch` field to `ppc64`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::new("20.6.1").ppc64();
    /// ```
    pub fn ppc64(&mut self) -> &mut Self {
        self.arch = NodeJSArch::PPC64;
        self
    }

    /// Sets instance `arch` field to `ppc64le`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::new("20.6.1").ppc64le();
    /// ```
    pub fn ppc64le(&mut self) -> &mut Self {
        self.arch = NodeJSArch::PPC64LE;
        self
    }

    /// Sets instance `arch` field to `s390x`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::new("20.6.1").s390x();
    /// ```
    pub fn s390x(&mut self) -> &mut Self {
        self.arch = NodeJSArch::S390X;
        self
    }

    /// Sets instance `ext` field to `tar.gz`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::new("20.6.1").tar_gz();
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
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::new("20.6.1").tar_xz();
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
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::new("20.6.1").zip();
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
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::new("20.6.1").msi();
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
    /// use node_js_release_info::NodeJSRelInfo;
    /// let info = NodeJSRelInfo::new("20.6.1").windows().x64().zip().to_owned();
    /// ```
    pub fn to_owned(&self) -> Self {
        self.clone()
    }

    /// Fetches Node.js metadata for specified configuration from the
    /// [releases download server](https://nodejs.org/download/release/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::{NodeJSRelInfo, NodeJSRelInfoError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), NodeJSRelInfoError> {
    ///   let info = NodeJSRelInfo::new("20.6.1").macos().arm64().fetch().await?;
    ///   assert_eq!(info.version, "20.6.1");
    ///   assert_eq!(info.filename, "node-v20.6.1-darwin-arm64.tar.gz");
    ///   assert_eq!(info.sha256, "d8ba8018d45b294429b1a7646ccbeaeb2af3cdf45b5c91dabbd93e2a2035cb46");
    ///   assert_eq!(info.url, "https://nodejs.org/download/release/v20.6.1/node-v20.6.1-darwin-arm64.tar.gz");
    ///   Ok(())
    /// }
    /// ```
    pub async fn fetch(&mut self) -> Result<Self, NodeJSRelInfoError> {
        let version = specs::validate_version(self.version.as_str())?;
        let specs = specs::fetch(&version, &self.url_fmt).await?;
        let filename = self.filename();
        let info = specs.lines().find(|&line| {
            line.contains(filename.as_str())
        });

        let mut specs = match info {
            None => return Err(NodeJSRelInfoError::UnrecognizedConfiguration(filename))?,
            Some(s) => s.split_whitespace(),
        };

        self.filename = filename;
        self.sha256 = specs.nth(0).unwrap().to_string();
        self.url = self.url_fmt.pkg(&self.version, &self.filename);
        Ok(self.to_owned())
    }

    /// Fetches Node.js metadata for all supported configurations from the
    /// [releases download server](https://nodejs.org/download/release/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::{NodeJSRelInfo, NodeJSRelInfoError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), NodeJSRelInfoError> {
    ///   let info = NodeJSRelInfo::new("20.6.1");
    ///   let all = info.fetch_all().await?;
    ///   assert_eq!(all.len(), 21);
    ///   assert_eq!(all[2].version, "20.6.1");
    ///   assert_eq!(all[2].filename, "node-v20.6.1-darwin-arm64.tar.gz");
    ///   assert_eq!(all[2].sha256, "d8ba8018d45b294429b1a7646ccbeaeb2af3cdf45b5c91dabbd93e2a2035cb46");
    ///   assert_eq!(all[2].url, "https://nodejs.org/download/release/v20.6.1/node-v20.6.1-darwin-arm64.tar.gz");
    ///   Ok(())
    /// }
    /// ```
    pub async fn fetch_all(&self) -> Result<Vec<NodeJSRelInfo>, NodeJSRelInfoError> {
        let version = specs::validate_version(self.version.as_str())?;
        let specs = specs::fetch(&version, &self.url_fmt).await?;
        let specs = match specs::parse(&version, specs) {
            Some(s) => s,
            None => {
                return Err(NodeJSRelInfoError::UnrecognizedVersion(version.clone()));
            }
        };

        let mut all: Vec<NodeJSRelInfo> = vec![];
        for (os, arch, ext, sha256, filename) in specs.into_iter() {
            let version = version.clone();
            let mut info = NodeJSRelInfo {
                os,
                arch,
                version,
                ext,
                filename,
                sha256,
                ..Default::default()
            };

            info.url = info.url_fmt.pkg(&info.version, &info.filename);
            all.push(info);
        }

        Ok(all)
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
    use mockito::Server;
    use super::*;

    fn is_thread_safe<T: Sized + Send + Sync + Unpin>() {}

    #[test]
    fn it_initializes(){
        let info = NodeJSRelInfo::new("1.0.0");
        assert_eq!(info.os, NodeJSOS::Linux);
        assert_eq!(info.arch, NodeJSArch::X64);
        assert_eq!(info.ext, NodeJSPkgExt::Targz);
        assert_eq!(info.version, "1.0.0".to_string());
        assert_eq!(info.filename, "".to_string());
        assert_eq!(info.sha256, "".to_string());
        assert_eq!(info.url, "".to_string());
        is_thread_safe::<NodeJSRelInfo>();
    }

    #[test]
    fn it_initializes_with_defaults() {
        let info = NodeJSRelInfo::default();
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
        let info = NodeJSRelInfo::from_env("1.0.0").unwrap();
        assert_eq!(info.ext, NodeJSPkgExt::Targz);
    }

    #[test]
    #[cfg_attr(not(target_os = "linux"), ignore)]
    fn it_initializes_using_current_environment_on_linux() {
        let info = NodeJSRelInfo::from_env("1.0.0").unwrap();
        assert_eq!(info.ext, NodeJSPkgExt::Targz);
    }

    #[test]
    #[cfg_attr(not(target_os = "windows"), ignore)]
    fn it_initializes_using_current_environment_on_windows() {
        let info = NodeJSRelInfo::from_env("1.0.0").unwrap();
        assert_eq!(info.ext, NodeJSPkgExt::Zip);
    }

    #[test]
    fn it_sets_os() {
        let mut info = NodeJSRelInfo::new("1.0.0");

        assert_eq!(info.os, NodeJSOS::Linux);

        info.windows();

        assert_eq!(info.os, NodeJSOS::Windows);

        info.macos();

        assert_eq!(info.os, NodeJSOS::Darwin);

        info.linux();

        assert_eq!(info.os, NodeJSOS::Linux);

        info.aix();

        assert_eq!(info.os, NodeJSOS::AIX);
    }

    #[test]
    fn it_sets_arch() {
        let mut info = NodeJSRelInfo::new("1.0.0");

        info.x86();

        assert_eq!(info.arch, NodeJSArch::X86);

        info.x64();

        assert_eq!(info.arch, NodeJSArch::X64);

        info.arm64();

        assert_eq!(info.arch, NodeJSArch::ARM64);

        info.armv7l();

        assert_eq!(info.arch, NodeJSArch::ARMV7L);

        info.ppc64();

        assert_eq!(info.arch, NodeJSArch::PPC64);

        info.ppc64le();

        assert_eq!(info.arch, NodeJSArch::PPC64LE);

        info.s390x();

        assert_eq!(info.arch, NodeJSArch::S390X);
    }

    #[test]
    fn it_sets_ext() {
        let mut info = NodeJSRelInfo::new("1.0.0");

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
        let mut info1 = NodeJSRelInfo::new("1.0.0");
        let info2 = info1.to_owned();

        assert_eq!(info1, info2);

        info1.windows();

        assert_ne!(info1, info2);
    }

    #[test]
    fn it_formats_filename() {
        let info = NodeJSRelInfo::new("1.0.0").macos().x64().zip().to_owned();

        assert_eq!(info.filename(), "node-v1.0.0-darwin-x64.zip");

        let info = NodeJSRelInfo::new("1.0.0").windows().x64().msi().to_owned();

        assert_eq!(info.filename(), "node-v1.0.0-x64.msi");
    }

    #[test]
    fn it_serializes_and_deserializes() {
        let version = "20.6.1".to_string();
        let filename = "node-v20.6.1-darwin-arm64.tar.gz".to_string();
        let sha256 = "d8ba8018d45b294429b1a7646ccbeaeb2af3cdf45b5c91dabbd93e2a2035cb46".to_string();
        let url = "https://nodejs.org/download/release/v20.6.1/node-v20.6.1-darwin-arm64.tar.gz".to_string();
        let info_orig = NodeJSRelInfo {
            os: NodeJSOS::Darwin,
            arch: NodeJSArch::ARM64,
            ext: NodeJSPkgExt::Targz,
            version: version.clone(),
            filename: filename.clone(),
            sha256: sha256.clone(),
            url: url.clone(),
            ..Default::default()
        };
        let info_json = serde_json::to_string(&info_orig).unwrap();
        let info: NodeJSRelInfo = serde_json::from_str(&info_json).unwrap();
        assert_eq!(info.os, NodeJSOS::Darwin);
        assert_eq!(info.arch, NodeJSArch::ARM64);
        assert_eq!(info.ext, NodeJSPkgExt::Targz);
        assert_eq!(info.version, "20.6.1".to_string());
        assert_eq!(info.filename, "node-v20.6.1-darwin-arm64.tar.gz".to_string());
        assert_eq!(info.sha256, "d8ba8018d45b294429b1a7646ccbeaeb2af3cdf45b5c91dabbd93e2a2035cb46".to_string());
        assert_eq!(info.url, "https://nodejs.org/download/release/v20.6.1/node-v20.6.1-darwin-arm64.tar.gz".to_string());
    }

    #[tokio::test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: InvalidVersion(\"NOPE!\")")]
    async fn it_fails_to_fetch_info_when_version_is_invalid() {
        let mut info = NodeJSRelInfo::new("NOPE!");
        info.fetch().await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: UnrecognizedVersion(\"1.0.0\")")]
    async fn it_fails_to_fetch_info_when_version_is_unrecognized() {
        let mut info = NodeJSRelInfo::new("1.0.0");
        let mut server = Server::new_async().await;
        let mock = specs::setup_server_mock(&info.version, &mut info.url_fmt, &mut server)
            .with_body(specs::get_fake_specs())
            .with_status(404)
            .create_async()
            .await;

        info.fetch().await.unwrap();
        mock.assert_async().await;
    }

    #[tokio::test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: UnrecognizedConfiguration(\"node-v20.6.1-linux-x64.zip\")")]
    async fn it_fails_to_fetch_info_when_configuration_is_unrecognized() {
        let mut server = Server::new_async().await;
        let mut info = NodeJSRelInfo::new("20.6.1").linux().zip().to_owned();
        let mock = specs::setup_server_mock(&info.version, &mut info.url_fmt, &mut server)
            .with_body(specs::get_fake_specs())
            .create_async()
            .await;

        info.fetch().await.unwrap();
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn it_fetches_node_js_release_info() {
        let mut info = NodeJSRelInfo::new("20.6.1");
        let mut server = Server::new_async().await;
        let mock = specs::setup_server_mock(&info.version, &mut info.url_fmt, &mut server)
            .with_body(specs::get_fake_specs())
            .create_async()
            .await;

        info.fetch().await.unwrap();
        mock.assert_async().await;

        assert_eq!(info.filename, "node-v20.6.1-linux-x64.tar.gz");
        assert_eq!(info.url, format!("{}{}", server.url(), "/download/release/v20.6.1/node-v20.6.1-linux-x64.tar.gz"));
        assert_eq!(info.sha256, "26dd13a6f7253f0ab9bcab561353985a297d927840771d905566735b792868da");
    }

    #[tokio::test]
    async fn it_fetches_node_js_release_info_when_ext_is_msi() {
        let mut info = NodeJSRelInfo::new("20.6.1").arm64().msi().to_owned();
        let mut server = Server::new_async().await;
        let mock = specs::setup_server_mock(&info.version, &mut info.url_fmt, &mut server)
            .with_body(specs::get_fake_specs())
            .create_async()
            .await;

        info.fetch().await.unwrap();
        mock.assert_async().await;

        assert_eq!(info.filename, "node-v20.6.1-arm64.msi");
        assert_eq!(info.url, format!("{}{}", server.url(), "/download/release/v20.6.1/node-v20.6.1-arm64.msi"));
        assert_eq!(info.sha256, "9471bd6dc491e09c31b0f831f5953284b8a6842ed4ccb98f5c62d13e6086c471");
    }

    #[tokio::test]
    async fn it_fetches_all_supported_node_js_configurations() {
        let mut info = NodeJSRelInfo::new("20.6.1");
        let mut server = Server::new_async().await;
        let mock = specs::setup_server_mock(&info.version, &mut info.url_fmt, &mut server)
            .with_body(specs::get_fake_specs())
            .create_async()
            .await;

        let all = info.fetch_all().await.unwrap();
        mock.assert_async().await;

        assert_eq!(all.len(), 21);
        assert_eq!(all[2].version, "20.6.1");
        assert_eq!(all[2].os, NodeJSOS::Darwin);
        assert_eq!(all[2].arch, NodeJSArch::ARM64);
        assert_eq!(all[2].ext, NodeJSPkgExt::Targz);
        assert_eq!(all[2].filename, "node-v20.6.1-darwin-arm64.tar.gz");
        assert_eq!(all[2].sha256, "d8ba8018d45b294429b1a7646ccbeaeb2af3cdf45b5c91dabbd93e2a2035cb46");
        assert_eq!(all[2].url, "https://nodejs.org/download/release/v20.6.1/node-v20.6.1-darwin-arm64.tar.gz");
    }

    #[tokio::test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: UnrecognizedVersion(\"1.0.0\")")]
    async fn it_fails_to_fetch_all_supported_node_js_configurations_when_version_is_unrecognized() {
        let mut info = NodeJSRelInfo::new("1.0.0");
        let mut server = Server::new_async().await;
        let mock = specs::setup_server_mock(&info.version, &mut info.url_fmt, &mut server)
            .with_body(String::from(""))
            .create_async()
            .await;

        info.fetch_all().await.unwrap();
        mock.assert_async().await;
    }
}
