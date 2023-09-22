#[allow(unused_imports)]
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, Display, EnumString, PartialEq)]
pub enum NodeJSPkgExt {
    #[strum(serialize = "tar.gz")]
    TARGZ,

    #[strum(serialize = "tar.xz")]
    TARXZ,

    #[strum(serialize = "zip")]
    ZIP,
}

impl Default for NodeJSPkgExt {
    fn default() -> Self {
        NodeJSPkgExt::new()
    }
}

impl NodeJSPkgExt {
    pub fn new() -> NodeJSPkgExt {
        NodeJSPkgExt::TARGZ
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let ext = NodeJSPkgExt::new();
        assert_eq!(ext, NodeJSPkgExt::TARGZ);
    }

    #[test]
    fn it_initializes_with_defaults() {
        let ext = NodeJSPkgExt::default();
        assert_eq!(ext, NodeJSPkgExt::TARGZ);
    }

    #[test]
    fn it_initializes_from_str() {
        let ext = NodeJSPkgExt::from_str("tar.xz").unwrap();
        assert_eq!(ext, NodeJSPkgExt::TARXZ);
    }
}
