use crate::error::NodeJSRelInfoError;
use std::env::consts::ARCH;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum NodeJSArch {
    X64,
    X86,
    ARM64,
    ARMV7L,
    PPC64LE,
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
            NodeJSArch::PPC64LE => "ppc64le",
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
            "arm" => Ok(NodeJSArch::ARMV7L),
            "ppc64le" | "powerpc64" => Ok(NodeJSArch::PPC64LE),
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

        let arch = NodeJSArch::from_str("ppc64le").unwrap();

        assert_eq!(arch, NodeJSArch::PPC64LE);

        let arch = NodeJSArch::from_str("powerpc64").unwrap();

        assert_eq!(arch, NodeJSArch::PPC64LE);
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
}
