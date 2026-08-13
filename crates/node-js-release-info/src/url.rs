#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct NodeJsUrlFormatter {
    pub protocol: String,
    pub host: String,
    pub pathname: String,
}

impl Default for NodeJsUrlFormatter {
    fn default() -> Self {
        NodeJsUrlFormatter::new()
    }
}

impl NodeJsUrlFormatter {
    pub(crate) fn new() -> NodeJsUrlFormatter {
        NodeJsUrlFormatter {
            protocol: String::from("https:"),
            host: String::from("nodejs.org"),
            pathname: String::from("/download/release"),
        }
    }

    pub(crate) fn info<V: AsRef<str>>(&self, version: V) -> String {
        format!(
            "{}//{}{}",
            self.protocol,
            self.host,
            self.info_pathname(version),
        )
    }

    pub(crate) fn info_pathname<V: AsRef<str>>(&self, version: V) -> String {
        format!(
            "{}/v{}/SHASUMS256.txt",
            self.pathname,
            version.as_ref().to_owned(),
        )
    }

    pub(crate) fn pkg<V: AsRef<str>, F: AsRef<str>>(&self, version: V, filename: F) -> String {
        format!(
            "{}//{}{}",
            self.protocol,
            self.host,
            self.pkg_pathname(version, filename),
        )
    }

    pub(crate) fn pkg_pathname<V: AsRef<str>, F: AsRef<str>>(
        &self,
        version: V,
        filename: F,
    ) -> String {
        format!(
            "{}/v{}/{}",
            self.pathname,
            version.as_ref().to_owned(),
            filename.as_ref().to_owned(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let url_fmt = NodeJsUrlFormatter::new();
        assert_eq!(url_fmt.protocol, "https:");
        assert_eq!(url_fmt.host, "nodejs.org");
        assert_eq!(url_fmt.pathname, "/download/release");
    }

    #[test]
    fn it_initializes_with_defaults() {
        let url_fmt = NodeJsUrlFormatter::default();
        assert_eq!(url_fmt, NodeJsUrlFormatter::new());
    }

    #[test]
    fn it_formats_url_for_node_js_release_info() {
        let url_fmt = NodeJsUrlFormatter::new();
        assert_eq!(
            url_fmt.info("1.0.0"),
            "https://nodejs.org/download/release/v1.0.0/SHASUMS256.txt"
        );
    }

    #[test]
    fn it_formats_url_for_node_js_package() {
        let url_fmt = NodeJsUrlFormatter::new();
        assert_eq!(
            url_fmt.pkg("1.0.0", "fake-filename"),
            "https://nodejs.org/download/release/v1.0.0/fake-filename"
        );
    }
}
