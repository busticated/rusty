use crate::error::NodeJsRelInfoError;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use std::env::consts::ARCH;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// The CPU architecture a Node.js distributable targets
///
/// Non-exhaustive: Node.js has added and removed target architectures over
/// time, so new variants may appear in a minor release
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "json", derive(Deserialize, Serialize))]
#[non_exhaustive]
pub enum NodeJsArch {
    /// 64-bit x86 (`x64`)
    #[default]
    #[cfg_attr(feature = "json", serde(rename = "x64"))]
    X64,
    /// 32-bit x86 (`x86`)
    #[cfg_attr(feature = "json", serde(rename = "x86"))]
    X86,
    /// 64-bit ARM (`arm64`)
    #[cfg_attr(feature = "json", serde(rename = "arm64"))]
    Arm64,
    /// 32-bit ARMv6 with hardware floating point (`armv6l`) - shipped up to
    /// Node.js v11
    #[cfg_attr(feature = "json", serde(rename = "armv6l"))]
    Armv6l,
    /// 32-bit ARMv7 with hardware floating point (`armv7l`) - shipped up to
    /// Node.js v23
    #[cfg_attr(feature = "json", serde(rename = "armv7l"))]
    Armv7l,
    /// 64-bit PowerPC, big-endian (`ppc64`)
    #[cfg_attr(feature = "json", serde(rename = "ppc64"))]
    Ppc64,
    /// 64-bit PowerPC, little-endian (`ppc64le`)
    #[cfg_attr(feature = "json", serde(rename = "ppc64le"))]
    Ppc64le,
    /// 64-bit IBM Z (`s390x`)
    #[cfg_attr(feature = "json", serde(rename = "s390x"))]
    S390x,
}

impl NodeJsArch {
    /// Creates a new instance using the default architecture ([`X64`](NodeJsArch::X64))
    ///
    /// # Examples
    ///
    /// ```rust
    /// use node_js_release_info::NodeJsArch;
    /// assert_eq!(NodeJsArch::new(), NodeJsArch::X64);
    /// ```
    pub fn new() -> NodeJsArch {
        NodeJsArch::default()
    }

    /// Determines the architecture of the current environment via
    /// [`std::env::consts::ARCH`]
    ///
    /// # Errors
    ///
    /// Returns [`NodeJsRelInfoError::UnrecognizedArch`] when the current
    /// architecture has no corresponding Node.js distributable
    pub fn from_env() -> Result<NodeJsArch, NodeJsRelInfoError> {
        NodeJsArch::from_str(ARCH)
    }
}

impl Display for NodeJsArch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let arch = match self {
            NodeJsArch::X64 => "x64",
            NodeJsArch::X86 => "x86",
            NodeJsArch::Arm64 => "arm64",
            NodeJsArch::Armv6l => "armv6l",
            NodeJsArch::Armv7l => "armv7l",
            NodeJsArch::Ppc64 => "ppc64",
            NodeJsArch::Ppc64le => "ppc64le",
            NodeJsArch::S390x => "s390x",
        };

        write!(f, "{arch}")
    }
}

impl FromStr for NodeJsArch {
    type Err = NodeJsRelInfoError;

    fn from_str(s: &str) -> Result<NodeJsArch, NodeJsRelInfoError> {
        match s {
            "x64" | "x86_64" => Ok(NodeJsArch::X64),
            "x86" => Ok(NodeJsArch::X86),
            "arm64" | "aarch64" => Ok(NodeJsArch::Arm64),
            "armv6l" => Ok(NodeJsArch::Armv6l),
            "arm" | "armv7l" => Ok(NodeJsArch::Armv7l),
            "ppc64" | "powerpc64" => Ok(NodeJsArch::Ppc64),
            "ppc64le" => Ok(NodeJsArch::Ppc64le),
            "s390x" => Ok(NodeJsArch::S390x),
            _ => Err(NodeJsRelInfoError::UnrecognizedArch(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let arch = NodeJsArch::new();
        assert_eq!(arch, NodeJsArch::X64);
    }

    #[test]
    fn it_initializes_with_defaults() {
        let arch = NodeJsArch::default();
        assert_eq!(arch, NodeJsArch::X64);
    }

    #[test]
    fn it_initializes_from_str() {
        let arch = NodeJsArch::from_str("x64").unwrap();

        assert_eq!(arch, NodeJsArch::X64);

        let arch = NodeJsArch::from_str("x86_64").unwrap();

        assert_eq!(arch, NodeJsArch::X64);

        let arch = NodeJsArch::from_str("x86").unwrap();

        assert_eq!(arch, NodeJsArch::X86);

        let arch = NodeJsArch::from_str("arm64").unwrap();

        assert_eq!(arch, NodeJsArch::Arm64);

        let arch = NodeJsArch::from_str("aarch64").unwrap();

        assert_eq!(arch, NodeJsArch::Arm64);

        let arch = NodeJsArch::from_str("arm").unwrap();

        assert_eq!(arch, NodeJsArch::Armv7l);

        let arch = NodeJsArch::from_str("armv6l").unwrap();

        assert_eq!(arch, NodeJsArch::Armv6l);

        let arch = NodeJsArch::from_str("ppc64").unwrap();

        assert_eq!(arch, NodeJsArch::Ppc64);

        let arch = NodeJsArch::from_str("ppc64le").unwrap();

        assert_eq!(arch, NodeJsArch::Ppc64le);

        let arch = NodeJsArch::from_str("powerpc64").unwrap();

        assert_eq!(arch, NodeJsArch::Ppc64);

        let arch = NodeJsArch::from_str("s390x").unwrap();

        assert_eq!(arch, NodeJsArch::S390x);
    }

    #[test]
    fn it_serializes_to_str() {
        let text = format!("{}", NodeJsArch::X64);

        assert_eq!(text, "x64");

        let text = format!("{}", NodeJsArch::X86);

        assert_eq!(text, "x86");

        let text = format!("{}", NodeJsArch::Arm64);

        assert_eq!(text, "arm64");

        let text = format!("{}", NodeJsArch::Armv7l);

        assert_eq!(text, "armv7l");

        let text = format!("{}", NodeJsArch::Armv6l);

        assert_eq!(text, "armv6l");

        let text = format!("{}", NodeJsArch::Ppc64);

        assert_eq!(text, "ppc64");

        let text = format!("{}", NodeJsArch::Ppc64le);

        assert_eq!(text, "ppc64le");

        let text = format!("{}", NodeJsArch::S390x);

        assert_eq!(text, "s390x");
    }

    #[test]
    fn it_initializes_using_current_environment() {
        NodeJsArch::from_env().unwrap();
    }

    #[test]
    fn it_fails_when_arch_is_unrecognized() {
        let err = NodeJsArch::from_str("NOPE!").unwrap_err();
        assert!(matches!(err, NodeJsRelInfoError::UnrecognizedArch(x) if x == "NOPE!"));
    }

    #[test]
    #[cfg(feature = "json")]
    fn it_serializes_and_deserializes() {
        let arch_json = serde_json::to_string(&NodeJsArch::X64).unwrap();
        let arch: NodeJsArch = serde_json::from_str(&arch_json).unwrap();
        assert_eq!(arch, NodeJsArch::X64);
    }
}
