use crate::error::NodeJSInfoError;
use std::env::consts::OS;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum NodeJSOS {
    Linux,
    Darwin,
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

    pub fn from_env() -> Result<NodeJSOS, NodeJSInfoError> {
        NodeJSOS::from_str(OS)
    }
}

impl Display for NodeJSOS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let os = match self {
            NodeJSOS::Linux => "linux",
            NodeJSOS::Darwin => "darwin",
            NodeJSOS::Windows => "win",
        };

        write!(f, "{}", os)
    }
}

impl FromStr for NodeJSOS {
    type Err = NodeJSInfoError;

    fn from_str(s: &str) -> Result<NodeJSOS, NodeJSInfoError> {
        match s {
            "linux" => Ok(NodeJSOS::Linux),
            "darwin" | "macos" => Ok(NodeJSOS::Darwin),
            "windows" | "win" => Ok(NodeJSOS::Windows),
            _ => Err(NodeJSInfoError::UnrecognizedOs(s.to_string())),
        }
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

        let os = NodeJSOS::from_str("darwin").unwrap();

        assert_eq!(os, NodeJSOS::Darwin);

        let os = NodeJSOS::from_str("macos").unwrap();

        assert_eq!(os, NodeJSOS::Darwin);

        let os = NodeJSOS::from_str("windows").unwrap();

        assert_eq!(os, NodeJSOS::Windows);

        let os = NodeJSOS::from_str("win").unwrap();

        assert_eq!(os, NodeJSOS::Windows);
    }

    #[test]
    fn it_initializes_using_current_environment() {
        NodeJSOS::from_env().unwrap();
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: UnrecognizedOs(\"NOPE!\")"
    )]
    fn it_fails_when_os_cannot_be_determined_from_str() {
        NodeJSOS::from_str("NOPE!").unwrap();
    }
}
