use crate::error::NodeJSRelInfoError;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum NodeJSPkgExt {
    Targz,
    Tarxz,
    Zip,
    Msi,
}

impl Default for NodeJSPkgExt {
    fn default() -> Self {
        NodeJSPkgExt::new()
    }
}

impl NodeJSPkgExt {
    pub fn new() -> NodeJSPkgExt {
        NodeJSPkgExt::Targz
    }
}
impl Display for NodeJSPkgExt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let arch = match self {
            NodeJSPkgExt::Targz => "tar.gz",
            NodeJSPkgExt::Tarxz => "tar.xz",
            NodeJSPkgExt::Zip => "zip",
            NodeJSPkgExt::Msi => "msi",
        };

        write!(f, "{}", arch)
    }
}

impl FromStr for NodeJSPkgExt {
    type Err = NodeJSRelInfoError;

    fn from_str(s: &str) -> Result<NodeJSPkgExt, NodeJSRelInfoError> {
        match s {
            "tar.gz" => Ok(NodeJSPkgExt::Targz),
            "tar.xz" => Ok(NodeJSPkgExt::Tarxz),
            "zip" => Ok(NodeJSPkgExt::Zip),
            "msi" => Ok(NodeJSPkgExt::Msi),
            _ => Err(NodeJSRelInfoError::UnrecognizedExt(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let ext = NodeJSPkgExt::new();
        assert_eq!(ext, NodeJSPkgExt::Targz);
    }

    #[test]
    fn it_initializes_with_defaults() {
        let ext = NodeJSPkgExt::default();
        assert_eq!(ext, NodeJSPkgExt::Targz);
    }

    #[test]
    fn it_initializes_from_str() {
        let ext = NodeJSPkgExt::from_str("tar.gz").unwrap();

        assert_eq!(ext, NodeJSPkgExt::Targz);

        let ext = NodeJSPkgExt::from_str("tar.xz").unwrap();

        assert_eq!(ext, NodeJSPkgExt::Tarxz);

        let ext = NodeJSPkgExt::from_str("zip").unwrap();

        assert_eq!(ext, NodeJSPkgExt::Zip);

        let ext = NodeJSPkgExt::from_str("msi").unwrap();

        assert_eq!(ext, NodeJSPkgExt::Msi);
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: UnrecognizedExt(\"NOPE!\")"
    )]
    fn it_fails_when_arch_is_unrecognized() {
        NodeJSPkgExt::from_str("NOPE!").unwrap();
    }
}
