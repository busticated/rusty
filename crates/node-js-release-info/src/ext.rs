use crate::error::NodeJsRelInfoError;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// The file extension of a Node.js distributable
///
/// Non-exhaustive: Node.js has added and removed package formats over time,
/// so new variants may appear in a minor release
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "json", derive(Deserialize, Serialize))]
#[non_exhaustive]
pub enum NodeJsPkgExt {
    /// gzip-compressed tarball (`tar.gz`)
    #[default]
    #[cfg_attr(feature = "json", serde(rename = "tar.gz"))]
    Targz,
    /// xz-compressed tarball (`tar.xz`)
    #[cfg_attr(feature = "json", serde(rename = "tar.xz"))]
    Tarxz,
    /// zip archive (`zip`) - Windows only
    #[cfg_attr(feature = "json", serde(rename = "zip"))]
    Zip,
    /// Windows installer package (`msi`)
    #[cfg_attr(feature = "json", serde(rename = "msi"))]
    Msi,
    /// 7-Zip archive (`7z`) - Windows only
    #[cfg_attr(feature = "json", serde(rename = "7z"))]
    S7z, // can't start w/ a number (X_x)
}

impl NodeJsPkgExt {
    /// Creates a new instance using the default extension ([`Targz`](NodeJsPkgExt::Targz))
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsPkgExt;
    /// assert_eq!(NodeJsPkgExt::new(), NodeJsPkgExt::Targz);
    /// ```
    pub fn new() -> NodeJsPkgExt {
        NodeJsPkgExt::default()
    }
}
impl Display for NodeJsPkgExt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let arch = match self {
            NodeJsPkgExt::Targz => "tar.gz",
            NodeJsPkgExt::Tarxz => "tar.xz",
            NodeJsPkgExt::Zip => "zip",
            NodeJsPkgExt::Msi => "msi",
            NodeJsPkgExt::S7z => "7z",
        };

        write!(f, "{arch}")
    }
}

impl FromStr for NodeJsPkgExt {
    type Err = NodeJsRelInfoError;

    fn from_str(s: &str) -> Result<NodeJsPkgExt, NodeJsRelInfoError> {
        match s {
            "tar.gz" => Ok(NodeJsPkgExt::Targz),
            "tar.xz" => Ok(NodeJsPkgExt::Tarxz),
            "zip" => Ok(NodeJsPkgExt::Zip),
            "msi" => Ok(NodeJsPkgExt::Msi),
            "7z" => Ok(NodeJsPkgExt::S7z),
            _ => Err(NodeJsRelInfoError::UnrecognizedExt(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let ext = NodeJsPkgExt::new();
        assert_eq!(ext, NodeJsPkgExt::Targz);
    }

    #[test]
    fn it_initializes_with_defaults() {
        let ext = NodeJsPkgExt::default();
        assert_eq!(ext, NodeJsPkgExt::Targz);
    }

    #[test]
    fn it_initializes_from_str() {
        let ext = NodeJsPkgExt::from_str("tar.gz").unwrap();

        assert_eq!(ext, NodeJsPkgExt::Targz);

        let ext = NodeJsPkgExt::from_str("tar.xz").unwrap();

        assert_eq!(ext, NodeJsPkgExt::Tarxz);

        let ext = NodeJsPkgExt::from_str("zip").unwrap();

        assert_eq!(ext, NodeJsPkgExt::Zip);

        let ext = NodeJsPkgExt::from_str("msi").unwrap();

        assert_eq!(ext, NodeJsPkgExt::Msi);

        let ext = NodeJsPkgExt::from_str("7z").unwrap();

        assert_eq!(ext, NodeJsPkgExt::S7z);
    }

    #[test]
    fn it_serializes_to_str() {
        let text = format!("{}", NodeJsPkgExt::Targz);

        assert_eq!(text, "tar.gz");

        let text = format!("{}", NodeJsPkgExt::Tarxz);

        assert_eq!(text, "tar.xz");

        let text = format!("{}", NodeJsPkgExt::Zip);

        assert_eq!(text, "zip");

        let text = format!("{}", NodeJsPkgExt::Msi);

        assert_eq!(text, "msi");

        let text = format!("{}", NodeJsPkgExt::S7z);

        assert_eq!(text, "7z");
    }

    #[test]
    fn it_fails_when_ext_is_unrecognized() {
        let err = NodeJsPkgExt::from_str("NOPE!").unwrap_err();
        assert!(matches!(err, NodeJsRelInfoError::UnrecognizedExt(x) if x == "NOPE!"));
    }

    #[test]
    #[cfg(feature = "json")]
    fn it_serializes_and_deserializes() {
        let ext_json = serde_json::to_string(&NodeJsPkgExt::Tarxz).unwrap();
        let ext: NodeJsPkgExt = serde_json::from_str(&ext_json).unwrap();
        assert_eq!(ext, NodeJsPkgExt::Tarxz);
    }
}
