#[allow(unused_imports)]
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, Display, EnumString, PartialEq)]
pub enum NodeJSPkgExt {
    #[strum(serialize = "tar.gz")]
    Targz,

    #[strum(serialize = "tar.xz")]
    Tarxz,

    #[strum(serialize = "zip")]
    Zip,
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
        let ext = NodeJSPkgExt::from_str("tar.xz").unwrap();
        assert_eq!(ext, NodeJSPkgExt::Tarxz);
    }
}
