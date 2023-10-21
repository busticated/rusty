use crate::error::NodeJSRelInfoError;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use std::env::consts::ARCH;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "json", derive(Deserialize, Serialize))]
pub enum NodeJSArch {
    #[cfg_attr(feature = "json", serde(rename = "x64"))]
    X64,
    #[cfg_attr(feature = "json", serde(rename = "x86"))]
    X86,
    #[cfg_attr(feature = "json", serde(rename = "arm64"))]
    ARM64,
    #[cfg_attr(feature = "json", serde(rename = "armv7l"))]
    ARMV7L,
    #[cfg_attr(feature = "json", serde(rename = "ppc64"))]
    PPC64,
    #[cfg_attr(feature = "json", serde(rename = "ppc64le"))]
    PPC64LE,
    #[cfg_attr(feature = "json", serde(rename = "s390x"))]
    S390X,
}

impl Default for NodeJSArch {
    fn default() -> Self {
        NodeJSArch::new()
    }
}

impl NodeJSArch {
    pub fn new() -> NodeJSArch {
        NodeJSArch::X64
    }

    pub fn from_env() -> Result<NodeJSArch, NodeJSRelInfoError> {
        NodeJSArch::from_str(ARCH)
    }
}

impl Display for NodeJSArch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let arch = match self {
            NodeJSArch::X64 => "x64",
            NodeJSArch::X86 => "x86",
            NodeJSArch::ARM64 => "arm64",
            NodeJSArch::ARMV7L => "armv7l",
            NodeJSArch::PPC64 => "ppc64",
            NodeJSArch::PPC64LE => "ppc64le",
            NodeJSArch::S390X => "s390x",
        };

        write!(f, "{}", arch)
    }
}

impl FromStr for NodeJSArch {
    type Err = NodeJSRelInfoError;

    fn from_str(s: &str) -> Result<NodeJSArch, NodeJSRelInfoError> {
        match s {
            "x64" | "x86_64" => Ok(NodeJSArch::X64),
            "x86" => Ok(NodeJSArch::X86),
            "arm64" | "aarch64" => Ok(NodeJSArch::ARM64),
            "arm" | "armv7l" => Ok(NodeJSArch::ARMV7L),
            "ppc64" | "powerpc64" => Ok(NodeJSArch::PPC64),
            "ppc64le" => Ok(NodeJSArch::PPC64LE),
            "s390x" => Ok(NodeJSArch::S390X),
            _ => Err(NodeJSRelInfoError::UnrecognizedArch(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let arch = NodeJSArch::new();
        assert_eq!(arch, NodeJSArch::X64);
    }

    #[test]
    fn it_initializes_with_defaults() {
        let arch = NodeJSArch::default();
        assert_eq!(arch, NodeJSArch::X64);
    }

    #[test]
    fn it_initializes_from_str() {
        let arch = NodeJSArch::from_str("x64").unwrap();

        assert_eq!(arch, NodeJSArch::X64);

        let arch = NodeJSArch::from_str("x86_64").unwrap();

        assert_eq!(arch, NodeJSArch::X64);

        let arch = NodeJSArch::from_str("x86").unwrap();

        assert_eq!(arch, NodeJSArch::X86);

        let arch = NodeJSArch::from_str("arm64").unwrap();

        assert_eq!(arch, NodeJSArch::ARM64);

        let arch = NodeJSArch::from_str("aarch64").unwrap();

        assert_eq!(arch, NodeJSArch::ARM64);

        let arch = NodeJSArch::from_str("arm").unwrap();

        assert_eq!(arch, NodeJSArch::ARMV7L);

        let arch = NodeJSArch::from_str("ppc64").unwrap();

        assert_eq!(arch, NodeJSArch::PPC64);

        let arch = NodeJSArch::from_str("ppc64le").unwrap();

        assert_eq!(arch, NodeJSArch::PPC64LE);

        let arch = NodeJSArch::from_str("powerpc64").unwrap();

        assert_eq!(arch, NodeJSArch::PPC64);

        let arch = NodeJSArch::from_str("s390x").unwrap();

        assert_eq!(arch, NodeJSArch::S390X);
    }

    #[test]
    fn it_serializes_to_str() {
        let text = format!("{}", NodeJSArch::X64);

        assert_eq!(text, "x64");

        let text = format!("{}", NodeJSArch::X86);

        assert_eq!(text, "x86");

        let text = format!("{}", NodeJSArch::ARM64);

        assert_eq!(text, "arm64");

        let text = format!("{}", NodeJSArch::ARMV7L);

        assert_eq!(text, "armv7l");

        let text = format!("{}", NodeJSArch::PPC64);

        assert_eq!(text, "ppc64");

        let text = format!("{}", NodeJSArch::PPC64LE);

        assert_eq!(text, "ppc64le");

        let text = format!("{}", NodeJSArch::S390X);

        assert_eq!(text, "s390x");
    }

    #[test]
    fn it_initializes_using_current_environment() {
        NodeJSArch::from_env().unwrap();
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: UnrecognizedArch(\"NOPE!\")"
    )]
    fn it_fails_when_arch_is_unrecognized() {
        NodeJSArch::from_str("NOPE!").unwrap();
    }

    #[test]
    fn it_serializes_and_deserializes() {
        let arch_json = serde_json::to_string(&NodeJSArch::X64).unwrap();
        let arch: NodeJSArch = serde_json::from_str(&arch_json).unwrap();
        assert_eq!(arch, NodeJSArch::X64);
    }
}
