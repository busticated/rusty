use std::env::consts::OS;
use std::str::FromStr;
use strum::ParseError;
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, Display, EnumString, PartialEq)]
pub enum NodeJSOS {
    #[strum(serialize = "linux")]
    Linux,

    #[strum(serialize = "darwin")]
    Darwin,

    #[strum(serialize = "win")]
    Windows,
}

impl Default for NodeJSOS {
    fn default() -> Self {
        NodeJSOS::new()
    }
}

impl NodeJSOS {
    pub fn new() -> NodeJSOS {
        NodeJSOS::Linux
    }

    pub fn like<N: AsRef<str>>(name: N) -> Result<NodeJSOS, ParseError> {
        let n = name.as_ref();
        match n {
            "macos" => Ok(NodeJSOS::Darwin),
            "windows" => Ok(NodeJSOS::Windows),
            _ => NodeJSOS::from_str(n),
        }
    }

    pub fn from_env() -> Result<NodeJSOS, ParseError> {
        NodeJSOS::like(OS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let os = NodeJSOS::new();
        assert_eq!(os, NodeJSOS::Linux);
    }

    #[test]
    fn it_initializes_with_defaults() {
        let os = NodeJSOS::default();
        assert_eq!(os, NodeJSOS::Linux);
    }

    #[test]
    fn it_initializes_from_str() {
        let os = NodeJSOS::from_str("linux").unwrap();
        assert_eq!(os, NodeJSOS::Linux);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: VariantNotFound")]
    fn it_fails_when_os_cannot_be_determined_from_str() {
        NodeJSOS::from_str("NOPE!").unwrap();
    }

    #[test]
    fn it_initializes_with_os_like() {
        let os = NodeJSOS::like("macos").unwrap();

        assert_eq!(os, NodeJSOS::Darwin);

        let os = NodeJSOS::like("linux").unwrap();

        assert_eq!(os, NodeJSOS::Linux);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: VariantNotFound")]
    fn it_fails_when_os_is_unrecognized() {
        NodeJSOS::like("NOPE!").unwrap();
    }

    #[test]
    fn it_initializes_using_current_environment() {
        NodeJSOS::from_env().unwrap();
    }
}
