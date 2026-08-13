#![doc = include_str!("../README.md")]

mod arch;
mod error;
mod ext;
mod os;
mod specs;
mod url;

pub use crate::arch::NodeJsArch;
pub use crate::error::NodeJsRelInfoError;
pub use crate::ext::NodeJsPkgExt;
pub use crate::os::NodeJsOs;
use crate::url::NodeJsUrlFormatter;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use std::string::ToString;

/// Metadata describing a single Node.js distributable
///
/// Build one with [`new`](NodeJsRelInfo::new) or
/// [`from_env`](NodeJsRelInfo::from_env), narrow it with the builder methods
/// (e.g. [`macos`](NodeJsRelInfo::macos), [`arm64`](NodeJsRelInfo::arm64)),
/// then call [`fetch`](NodeJsRelInfo::fetch) to populate `filename`, `sha256`
/// and `url` from the downloads server
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "json", derive(Deserialize, Serialize))]
pub struct NodeJsRelInfo {
    /// The operating system for the Node.js distributable you are targeting
    pub os: NodeJsOs,
    /// The CPU architecture for the Node.js distributable you are targeting
    pub arch: NodeJsArch,
    /// The file extension for the Node.js distributable you are targeting
    pub ext: NodeJsPkgExt,
    /// The version of Node.js you are targeting as a [semver](https://semver.org) string
    pub version: String,
    /// The filename of the Node.js distributable (populated after fetching)
    pub filename: String,
    /// The hash for the Node.js distributable (populated after fetching)
    pub sha256: String,
    /// The fully qualified url for the Node.js distributable (populated after fetching)
    pub url: String,
    #[cfg_attr(feature = "json", serde(skip))]
    url_fmt: NodeJsUrlFormatter,
}

impl NodeJsRelInfo {
    /// Creates a new instance using default settings
    ///
    /// # Arguments
    ///
    /// * `semver` - The Node.js version you are targeting (`String` / `&str`)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::new("24.19.0");
    /// ```
    pub fn new<T: AsRef<str>>(semver: T) -> Self {
        NodeJsRelInfo {
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
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::from_env("24.19.0");
    /// ```
    pub fn from_env<T: AsRef<str>>(semver: T) -> Result<NodeJsRelInfo, NodeJsRelInfoError> {
        let mut info = NodeJsRelInfo::new(semver);
        info.os = NodeJsOs::from_env()?;
        info.arch = NodeJsArch::from_env()?;
        info.ext = match info.os {
            NodeJsOs::Windows => NodeJsPkgExt::Zip,
            _ => NodeJsPkgExt::Targz,
        };
        Ok(info)
    }

    /// Sets instance `os` field to `darwin`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::new("24.19.0").macos();
    /// ```
    pub fn macos(mut self) -> Self {
        self.os = NodeJsOs::Darwin;
        self
    }

    /// Sets instance `os` field to `linux`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::new("24.19.0").linux();
    /// ```
    pub fn linux(mut self) -> Self {
        self.os = NodeJsOs::Linux;
        self
    }

    /// Sets instance `os` field to `windows`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::new("24.19.0").windows();
    /// ```
    pub fn windows(mut self) -> Self {
        self.os = NodeJsOs::Windows;
        self
    }

    /// Sets instance `os` field to `aix`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::new("24.19.0").aix();
    /// ```
    pub fn aix(mut self) -> Self {
        self.os = NodeJsOs::Aix;
        self
    }

    /// Sets instance `arch` field to `x64`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::new("24.19.0").x64();
    /// ```
    pub fn x64(mut self) -> Self {
        self.arch = NodeJsArch::X64;
        self
    }

    /// Sets instance `arch` field to `x86`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::new("24.19.0").x86();
    /// ```
    pub fn x86(mut self) -> Self {
        self.arch = NodeJsArch::X86;
        self
    }

    /// Sets instance `arch` field to `arm64`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::new("24.19.0").arm64();
    /// ```
    pub fn arm64(mut self) -> Self {
        self.arch = NodeJsArch::Arm64;
        self
    }

    /// Sets instance `arch` field to `armv7l`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::new("24.19.0").armv7l();
    /// ```
    pub fn armv7l(mut self) -> Self {
        self.arch = NodeJsArch::Armv7l;
        self
    }

    /// Sets instance `arch` field to `ppc64`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::new("24.19.0").ppc64();
    /// ```
    pub fn ppc64(mut self) -> Self {
        self.arch = NodeJsArch::Ppc64;
        self
    }

    /// Sets instance `arch` field to `ppc64le`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::new("24.19.0").ppc64le();
    /// ```
    pub fn ppc64le(mut self) -> Self {
        self.arch = NodeJsArch::Ppc64le;
        self
    }

    /// Sets instance `arch` field to `s390x`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::new("24.19.0").s390x();
    /// ```
    pub fn s390x(mut self) -> Self {
        self.arch = NodeJsArch::S390x;
        self
    }

    /// Sets instance `ext` field to `tar.gz`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::new("24.19.0").tar_gz();
    /// ```
    pub fn tar_gz(mut self) -> Self {
        self.ext = NodeJsPkgExt::Targz;
        self
    }

    /// Sets instance `ext` field to `tar.xz`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::new("24.19.0").tar_xz();
    /// ```
    pub fn tar_xz(mut self) -> Self {
        self.ext = NodeJsPkgExt::Tarxz;
        self
    }

    /// Sets instance `ext` field to `zip`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::new("24.19.0").zip();
    /// ```
    pub fn zip(mut self) -> Self {
        self.ext = NodeJsPkgExt::Zip;
        self
    }

    /// Sets instance `ext` field to `7z`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::new("24.19.0").s7z();
    /// ```
    pub fn s7z(mut self) -> Self {
        self.ext = NodeJsPkgExt::S7z;
        self
    }

    /// Sets instance `ext` field to `msi`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsRelInfo;
    /// let info = NodeJsRelInfo::new("24.19.0").msi();
    /// ```
    pub fn msi(mut self) -> Self {
        self.ext = NodeJsPkgExt::Msi;
        self
    }

    /// Creates owned data from reference for convenience when chaining
    ///
    /// Fetches Node.js metadata for specified configuration from the
    /// [releases download server](https://nodejs.org/download/release/)
    ///
    /// # Errors
    ///
    /// Returns [`InvalidVersion`](NodeJsRelInfoError::InvalidVersion) when
    /// `version` is not valid semver,
    /// [`UnrecognizedVersion`](NodeJsRelInfoError::UnrecognizedVersion) when
    /// the release does not exist,
    /// [`UnrecognizedConfiguration`](NodeJsRelInfoError::UnrecognizedConfiguration)
    /// when the release exists but ships no such os/arch/ext combination, and
    /// [`HttpError`](NodeJsRelInfoError::HttpError) when the request fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::{NodeJsRelInfo, NodeJsRelInfoError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), NodeJsRelInfoError> {
    ///   let info = NodeJsRelInfo::new("24.19.0").macos().arm64().fetch().await?;
    ///   assert_eq!(info.version, "24.19.0");
    ///   assert_eq!(info.filename, "node-v24.19.0-darwin-arm64.tar.gz");
    ///   assert_eq!(info.sha256, "8294b7aa9b03997481c06babf1e8b270c859358f27da57a11509afe537ac381d");
    ///   assert_eq!(info.url, "https://nodejs.org/download/release/v24.19.0/node-v24.19.0-darwin-arm64.tar.gz");
    ///   Ok(())
    /// }
    /// ```
    pub async fn fetch(mut self) -> Result<Self, NodeJsRelInfoError> {
        let version = specs::validate_version(self.version.as_str())?;
        let specs = specs::fetch(&version, &self.url_fmt).await?;
        let filename = self.filename();
        let info = specs.lines().find(|&line| line.contains(filename.as_str()));

        let Some(line) = info else {
            return Err(NodeJsRelInfoError::UnrecognizedConfiguration(filename));
        };

        let Some(sha256) = line.split_whitespace().next() else {
            return Err(NodeJsRelInfoError::UnrecognizedConfiguration(filename));
        };

        self.filename = filename;
        self.sha256 = sha256.to_string();
        self.url = self.url_fmt.pkg(&self.version, &self.filename);
        Ok(self)
    }

    /// Fetches Node.js metadata for all supported configurations from the
    /// [releases download server](https://nodejs.org/download/release/)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::{NodeJsRelInfo, NodeJsRelInfoError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), NodeJsRelInfoError> {
    ///   let info = NodeJsRelInfo::new("24.19.0");
    ///   let all = info.fetch_all().await?;
    ///   assert_eq!(all.len(), 19);
    ///   assert_eq!(all[2].version, "24.19.0");
    ///   assert_eq!(all[2].filename, "node-v24.19.0-darwin-arm64.tar.gz");
    ///   assert_eq!(all[2].sha256, "8294b7aa9b03997481c06babf1e8b270c859358f27da57a11509afe537ac381d");
    ///   assert_eq!(all[2].url, "https://nodejs.org/download/release/v24.19.0/node-v24.19.0-darwin-arm64.tar.gz");
    ///   Ok(())
    /// }
    /// ```
    pub async fn fetch_all(&self) -> Result<Vec<NodeJsRelInfo>, NodeJsRelInfoError> {
        let version = specs::validate_version(self.version.as_str())?;
        let specs = specs::fetch(&version, &self.url_fmt).await?;
        let specs = match specs::parse(&version, specs) {
            Some(s) => s,
            None => {
                return Err(NodeJsRelInfoError::UnrecognizedVersion(version.clone()));
            }
        };

        let mut all: Vec<NodeJsRelInfo> = vec![];
        for (os, arch, ext, sha256, filename) in specs.into_iter() {
            let version = version.clone();
            let mut info = NodeJsRelInfo {
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

        if self.ext == NodeJsPkgExt::Msi {
            return format!("node-v{}-{}.{}", self.version, arch, ext);
        }

        format!("node-v{}-{}-{}.{}", self.version, self.os, arch, ext)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    fn is_thread_safe<T: Sized + Send + Sync + Unpin>() {}

    #[test]
    fn it_initializes() {
        let info = NodeJsRelInfo::new("1.0.0");
        assert_eq!(info.os, NodeJsOs::Linux);
        assert_eq!(info.arch, NodeJsArch::X64);
        assert_eq!(info.ext, NodeJsPkgExt::Targz);
        assert_eq!(info.version, "1.0.0".to_string());
        assert_eq!(info.filename, "".to_string());
        assert_eq!(info.sha256, "".to_string());
        assert_eq!(info.url, "".to_string());
        is_thread_safe::<NodeJsRelInfo>();
    }

    #[test]
    fn it_initializes_with_defaults() {
        let info = NodeJsRelInfo::default();
        assert_eq!(info.os, NodeJsOs::Linux);
        assert_eq!(info.arch, NodeJsArch::X64);
        assert_eq!(info.ext, NodeJsPkgExt::Targz);
        assert_eq!(info.version, "".to_string());
        assert_eq!(info.filename, "".to_string());
        assert_eq!(info.sha256, "".to_string());
        assert_eq!(info.url, "".to_string());
    }

    #[test]
    #[cfg_attr(not(target_os = "macos"), ignore)]
    fn it_initializes_using_current_environment_on_macos() {
        let info = NodeJsRelInfo::from_env("1.0.0").unwrap();
        assert_eq!(info.ext, NodeJsPkgExt::Targz);
    }

    #[test]
    #[cfg_attr(not(target_os = "linux"), ignore)]
    fn it_initializes_using_current_environment_on_linux() {
        let info = NodeJsRelInfo::from_env("1.0.0").unwrap();
        assert_eq!(info.ext, NodeJsPkgExt::Targz);
    }

    #[test]
    #[cfg_attr(not(target_os = "windows"), ignore)]
    fn it_initializes_using_current_environment_on_windows() {
        let info = NodeJsRelInfo::from_env("1.0.0").unwrap();
        assert_eq!(info.ext, NodeJsPkgExt::Zip);
    }

    #[test]
    fn it_sets_os() {
        let info = NodeJsRelInfo::new("1.0.0");

        assert_eq!(info.os, NodeJsOs::Linux);
        assert_eq!(info.clone().windows().os, NodeJsOs::Windows);
        assert_eq!(info.clone().macos().os, NodeJsOs::Darwin);
        assert_eq!(info.clone().linux().os, NodeJsOs::Linux);
        assert_eq!(info.clone().aix().os, NodeJsOs::Aix);
    }

    #[test]
    fn it_sets_arch() {
        let info = NodeJsRelInfo::new("1.0.0");

        assert_eq!(info.clone().x86().arch, NodeJsArch::X86);
        assert_eq!(info.clone().x64().arch, NodeJsArch::X64);
        assert_eq!(info.clone().arm64().arch, NodeJsArch::Arm64);
        assert_eq!(info.clone().armv7l().arch, NodeJsArch::Armv7l);
        assert_eq!(info.clone().ppc64().arch, NodeJsArch::Ppc64);
        assert_eq!(info.clone().ppc64le().arch, NodeJsArch::Ppc64le);
        assert_eq!(info.clone().s390x().arch, NodeJsArch::S390x);
    }

    #[test]
    fn it_sets_ext() {
        let info = NodeJsRelInfo::new("1.0.0");

        assert_eq!(info.clone().zip().ext, NodeJsPkgExt::Zip);
        assert_eq!(info.clone().tar_gz().ext, NodeJsPkgExt::Targz);
        assert_eq!(info.clone().tar_xz().ext, NodeJsPkgExt::Tarxz);
        assert_eq!(info.clone().msi().ext, NodeJsPkgExt::Msi);
        assert_eq!(info.clone().s7z().ext, NodeJsPkgExt::S7z);
    }

    #[test]
    fn it_clones() {
        let info1 = NodeJsRelInfo::new("1.0.0");
        let info2 = info1.clone();

        assert_eq!(info1, info2);
        // builders consume, so the clone is unaffected by further chaining
        assert_ne!(info1.windows(), info2);
    }

    #[test]
    fn it_formats_filename() {
        let info = NodeJsRelInfo::new("1.0.0").macos().x64().zip();

        assert_eq!(info.filename(), "node-v1.0.0-darwin-x64.zip");

        let info = NodeJsRelInfo::new("1.0.0").windows().x64().msi();

        assert_eq!(info.filename(), "node-v1.0.0-x64.msi");
    }

    #[test]
    #[cfg(feature = "json")]
    fn it_serializes_and_deserializes() {
        let version = "20.6.1".to_string();
        let filename = "node-v20.6.1-darwin-arm64.tar.gz".to_string();
        let sha256 = "d8ba8018d45b294429b1a7646ccbeaeb2af3cdf45b5c91dabbd93e2a2035cb46".to_string();
        let url = "https://nodejs.org/download/release/v20.6.1/node-v20.6.1-darwin-arm64.tar.gz"
            .to_string();
        let info_orig = NodeJsRelInfo {
            os: NodeJsOs::Darwin,
            arch: NodeJsArch::Arm64,
            ext: NodeJsPkgExt::Targz,
            version: version.clone(),
            filename: filename.clone(),
            sha256: sha256.clone(),
            url: url.clone(),
            ..Default::default()
        };
        let info_json = serde_json::to_string(&info_orig).unwrap();
        let info: NodeJsRelInfo = serde_json::from_str(&info_json).unwrap();
        assert_eq!(info.os, NodeJsOs::Darwin);
        assert_eq!(info.arch, NodeJsArch::Arm64);
        assert_eq!(info.ext, NodeJsPkgExt::Targz);
        assert_eq!(info.version, "20.6.1".to_string());
        assert_eq!(
            info.filename,
            "node-v20.6.1-darwin-arm64.tar.gz".to_string()
        );
        assert_eq!(
            info.sha256,
            "d8ba8018d45b294429b1a7646ccbeaeb2af3cdf45b5c91dabbd93e2a2035cb46".to_string()
        );
        assert_eq!(
            info.url,
            "https://nodejs.org/download/release/v20.6.1/node-v20.6.1-darwin-arm64.tar.gz"
                .to_string()
        );
    }

    #[tokio::test]
    async fn it_fails_to_fetch_info_when_version_is_invalid() {
        let info = NodeJsRelInfo::new("NOPE!");
        let err = info.fetch().await.unwrap_err();

        assert!(matches!(err, NodeJsRelInfoError::InvalidVersion(x) if x == "NOPE!"));
    }

    #[tokio::test]
    async fn it_fails_to_fetch_info_when_version_is_unrecognized() {
        let mut info = NodeJsRelInfo::new("1.0.0");
        let mut server = Server::new_async().await;
        let mock = specs::setup_server_mock(&info.version, &mut info.url_fmt, &mut server)
            .with_body(specs::get_fake_specs())
            .with_status(404)
            .create_async()
            .await;

        let err = info.fetch().await.unwrap_err();
        mock.assert_async().await;

        assert!(matches!(err, NodeJsRelInfoError::UnrecognizedVersion(x) if x == "1.0.0"));
    }

    #[tokio::test]
    async fn it_fails_to_fetch_info_when_configuration_is_unrecognized() {
        let mut server = Server::new_async().await;
        let mut info = NodeJsRelInfo::new("20.6.1").linux().zip();
        let mock = specs::setup_server_mock(&info.version, &mut info.url_fmt, &mut server)
            .with_body(specs::get_fake_specs())
            .create_async()
            .await;

        let err = info.fetch().await.unwrap_err();
        mock.assert_async().await;

        assert!(
            matches!(err, NodeJsRelInfoError::UnrecognizedConfiguration(x) if x == "node-v20.6.1-linux-x64.zip")
        );
    }

    #[tokio::test]
    async fn it_fetches_node_js_release_info() {
        let mut info = NodeJsRelInfo::new("20.6.1");
        let mut server = Server::new_async().await;
        let mock = specs::setup_server_mock(&info.version, &mut info.url_fmt, &mut server)
            .with_body(specs::get_fake_specs())
            .create_async()
            .await;

        let info = info.fetch().await.unwrap();
        mock.assert_async().await;

        assert_eq!(info.filename, "node-v20.6.1-linux-x64.tar.gz");
        assert_eq!(
            info.url,
            format!(
                "{}{}",
                server.url(),
                "/download/release/v20.6.1/node-v20.6.1-linux-x64.tar.gz"
            )
        );
        assert_eq!(
            info.sha256,
            "26dd13a6f7253f0ab9bcab561353985a297d927840771d905566735b792868da"
        );
    }

    #[tokio::test]
    async fn it_fetches_node_js_release_info_when_ext_is_msi() {
        let mut info = NodeJsRelInfo::new("20.6.1").arm64().msi();
        let mut server = Server::new_async().await;
        let mock = specs::setup_server_mock(&info.version, &mut info.url_fmt, &mut server)
            .with_body(specs::get_fake_specs())
            .create_async()
            .await;

        let info = info.fetch().await.unwrap();
        mock.assert_async().await;

        assert_eq!(info.filename, "node-v20.6.1-arm64.msi");
        assert_eq!(
            info.url,
            format!(
                "{}{}",
                server.url(),
                "/download/release/v20.6.1/node-v20.6.1-arm64.msi"
            )
        );
        assert_eq!(
            info.sha256,
            "9471bd6dc491e09c31b0f831f5953284b8a6842ed4ccb98f5c62d13e6086c471"
        );
    }

    #[tokio::test]
    async fn it_fetches_all_supported_node_js_configurations() {
        let mut info = NodeJsRelInfo::new("20.6.1");
        let mut server = Server::new_async().await;
        let mock = specs::setup_server_mock(&info.version, &mut info.url_fmt, &mut server)
            .with_body(specs::get_fake_specs())
            .create_async()
            .await;

        let all = info.fetch_all().await.unwrap();
        mock.assert_async().await;

        assert_eq!(all.len(), 24);
        assert_eq!(all[2].version, "20.6.1");
        assert_eq!(all[2].os, NodeJsOs::Darwin);
        assert_eq!(all[2].arch, NodeJsArch::Arm64);
        assert_eq!(all[2].ext, NodeJsPkgExt::Targz);
        assert_eq!(all[2].filename, "node-v20.6.1-darwin-arm64.tar.gz");
        assert_eq!(
            all[2].sha256,
            "d8ba8018d45b294429b1a7646ccbeaeb2af3cdf45b5c91dabbd93e2a2035cb46"
        );
        assert_eq!(
            all[2].url,
            "https://nodejs.org/download/release/v20.6.1/node-v20.6.1-darwin-arm64.tar.gz"
        );
    }

    #[tokio::test]
    async fn it_fails_to_fetch_all_supported_node_js_configurations_when_version_is_unrecognized() {
        let mut info = NodeJsRelInfo::new("1.0.0");
        let mut server = Server::new_async().await;
        let mock = specs::setup_server_mock(&info.version, &mut info.url_fmt, &mut server)
            .with_body(String::from(""))
            .create_async()
            .await;

        let err = info.fetch_all().await.unwrap_err();
        mock.assert_async().await;

        assert!(matches!(err, NodeJsRelInfoError::UnrecognizedVersion(x) if x == "1.0.0"));
    }
}
