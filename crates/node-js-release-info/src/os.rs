use crate::error::NodeJsRelInfoError;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use std::env::consts::OS;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// The operating system a Node.js distributable targets
///
/// Non-exhaustive: Node.js has added and removed target platforms over time,
/// so new variants may appear in a minor release
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "json", derive(Deserialize, Serialize))]
#[non_exhaustive]
pub enum NodeJsOs {
    /// Linux (`linux`)
    #[default]
    #[cfg_attr(feature = "json", serde(rename = "linux"))]
    Linux,
    /// macOS (`darwin`)
    #[cfg_attr(feature = "json", serde(rename = "darwin"))]
    Darwin,
    /// Windows (`win`)
    #[cfg_attr(feature = "json", serde(rename = "win"))]
    Windows,
    /// IBM AIX (`aix`)
    #[cfg_attr(feature = "json", serde(rename = "aix"))]
    Aix,
    /// illumos / Solaris (`sunos`) - shipped up to Node.js v14
    #[cfg_attr(feature = "json", serde(rename = "sunos"))]
    SunOs,
}

impl NodeJsOs {
    /// Creates a new instance using the default OS ([`Linux`](NodeJsOs::Linux))
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsOs;
    /// assert_eq!(NodeJsOs::new(), NodeJsOs::Linux);
    /// ```
    pub fn new() -> NodeJsOs {
        NodeJsOs::default()
    }

    /// Determines the OS of the current environment via
    /// [`std::env::consts::OS`]
    ///
    /// # Errors
    ///
    /// Returns [`NodeJsRelInfoError::UnrecognizedOs`] when the current OS has
    /// no corresponding Node.js distributable
    pub fn from_env() -> Result<NodeJsOs, NodeJsRelInfoError> {
        NodeJsOs::from_str(OS)
    }
}

impl Display for NodeJsOs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let os = match self {
            NodeJsOs::Linux => "linux",
            NodeJsOs::Darwin => "darwin",
            NodeJsOs::Windows => "win",
            NodeJsOs::Aix => "aix",
            NodeJsOs::SunOs => "sunos",
        };

        write!(f, "{os}")
    }
}

impl FromStr for NodeJsOs {
    type Err = NodeJsRelInfoError;

    fn from_str(s: &str) -> Result<NodeJsOs, NodeJsRelInfoError> {
        match s {
            "linux" => Ok(NodeJsOs::Linux),
            "darwin" | "macos" => Ok(NodeJsOs::Darwin),
            "windows" | "win" => Ok(NodeJsOs::Windows),
            "sunos" | "solaris" | "illumos" => Ok(NodeJsOs::SunOs),
            "aix" => Ok(NodeJsOs::Aix),
            _ => Err(NodeJsRelInfoError::UnrecognizedOs(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let os = NodeJsOs::new();
        assert_eq!(os, NodeJsOs::Linux);
    }

    #[test]
    fn it_initializes_with_defaults() {
        let os = NodeJsOs::default();
        assert_eq!(os, NodeJsOs::Linux);
    }

    #[test]
    fn it_initializes_from_str() {
        let os = NodeJsOs::from_str("linux").unwrap();

        assert_eq!(os, NodeJsOs::Linux);

        let os = NodeJsOs::from_str("darwin").unwrap();

        assert_eq!(os, NodeJsOs::Darwin);

        let os = NodeJsOs::from_str("macos").unwrap();

        assert_eq!(os, NodeJsOs::Darwin);

        let os = NodeJsOs::from_str("windows").unwrap();

        assert_eq!(os, NodeJsOs::Windows);

        let os = NodeJsOs::from_str("win").unwrap();

        assert_eq!(os, NodeJsOs::Windows);

        let os = NodeJsOs::from_str("aix").unwrap();

        assert_eq!(os, NodeJsOs::Aix);

        let os = NodeJsOs::from_str("sunos").unwrap();

        assert_eq!(os, NodeJsOs::SunOs);

        let os = NodeJsOs::from_str("solaris").unwrap();

        assert_eq!(os, NodeJsOs::SunOs);
    }

    #[test]
    fn it_serializes_to_str() {
        let text = format!("{}", NodeJsOs::Linux);

        assert_eq!(text, "linux");

        let text = format!("{}", NodeJsOs::Darwin);

        assert_eq!(text, "darwin");

        let text = format!("{}", NodeJsOs::Windows);

        assert_eq!(text, "win");

        let text = format!("{}", NodeJsOs::Aix);

        assert_eq!(text, "aix");

        let text = format!("{}", NodeJsOs::SunOs);

        assert_eq!(text, "sunos");
    }

    #[test]
    fn it_initializes_using_current_environment() {
        NodeJsOs::from_env().unwrap();
    }

    #[test]
    fn it_fails_when_os_cannot_be_determined_from_str() {
        let err = NodeJsOs::from_str("NOPE!").unwrap_err();
        assert!(matches!(err, NodeJsRelInfoError::UnrecognizedOs(x) if x == "NOPE!"));
    }

    #[test]
    #[cfg(feature = "json")]
    fn it_serializes_and_deserializes() {
        let os_json = serde_json::to_string(&NodeJsOs::Darwin).unwrap();
        let os: NodeJsOs = serde_json::from_str(&os_json).unwrap();
        assert_eq!(os, NodeJsOs::Darwin);
    }
}
