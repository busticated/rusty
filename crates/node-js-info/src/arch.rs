use std::env::consts::ARCH;
use std::str::FromStr;
use strum::ParseError;
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, Display, EnumString, PartialEq)]
#[strum(serialize_all = "lowercase")]
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

    pub fn like<N: AsRef<str>>(name: N) -> Result<NodeJSArch, ParseError> {
        let n = name.as_ref();
        match n {
            "x86_64" => Ok(NodeJSArch::X64),
            "aarch64" => Ok(NodeJSArch::ARM64),
            "arm" => Ok(NodeJSArch::ARMV7L),
            "powerpc64" => Ok(NodeJSArch::PPC64LE),
            _ => NodeJSArch::from_str(n),
        }
    }

    pub fn from_env() -> Result<NodeJSArch, ParseError> {
        NodeJSArch::like(ARCH)
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
        let arch = NodeJSArch::from_str("arm64").unwrap();
        assert_eq!(arch, NodeJSArch::ARM64);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: VariantNotFound")]
    fn it_fails_when_arch_cannot_be_determined_from_str() {
        NodeJSArch::from_str("NOPE!").unwrap();
    }

    #[test]
    fn it_initializes_with_arch_like() {
        let arch = NodeJSArch::like("x86_64").unwrap();

        assert_eq!(arch, NodeJSArch::X64);

        let arch = NodeJSArch::like("aarch64").unwrap();

        assert_eq!(arch, NodeJSArch::ARM64);

        let arch = NodeJSArch::like("arm").unwrap();

        assert_eq!(arch, NodeJSArch::ARMV7L);

        let arch = NodeJSArch::like("powerpc64").unwrap();

        assert_eq!(arch, NodeJSArch::PPC64LE);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: VariantNotFound")]
    fn it_fails_when_arch_is_unrecognized() {
        NodeJSArch::like("NOPE!").unwrap();
    }

    #[test]
    fn it_initializes_using_current_environment() {
        NodeJSArch::from_env().unwrap();
    }
}
