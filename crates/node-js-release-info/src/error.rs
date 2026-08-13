use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// The error type returned by all fallible operations in this crate
///
/// Non-exhaustive: new variants may appear in a minor release
#[derive(Debug)]
#[non_exhaustive]
pub enum NodeJsRelInfoError {
    /// The operating system for the Node.js distributable you are targeting is
    /// unrecognized - see: [`NodeJsOs`](crate::NodeJsOs) for options
    UnrecognizedOs(String),
    /// The CPU architecture for the Node.js distributable you are targeting is
    /// unrecognized - see: [`NodeJsArch`](crate::NodeJsArch) for options
    UnrecognizedArch(String),
    /// The file extension of the Node.js distributable you are targeting is
    /// unrecognized - see: [`NodeJsPkgExt`](crate::NodeJsPkgExt) for options
    UnrecognizedExt(String),
    /// The version string provided is invalid - see: [semver](https://semver.org)
    InvalidVersion(String),
    /// The version of Node.js you are targeting is not available
    UnrecognizedVersion(String),
    /// The Node.js configuration you are targeting is not available
    UnrecognizedConfiguration(String),
    /// Something went wrong issuing or processing the HTTP GET request to the Node.js [downloads server](https://nodejs.org/download/release/)
    HttpError(reqwest::Error),
}

impl Error for NodeJsRelInfoError {
    /// Exposes the underlying [`reqwest::Error`] behind
    /// [`HttpError`](NodeJsRelInfoError::HttpError) so callers (and error
    /// reporters like `anyhow`) can walk the full cause chain
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NodeJsRelInfoError::HttpError(e) => Some(e),
            _ => None,
        }
    }
}

impl Display for NodeJsRelInfoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let message = match self {
            NodeJsRelInfoError::UnrecognizedOs(input) => {
                format!("unrecognized os - received: '{input}'")
            }
            NodeJsRelInfoError::UnrecognizedArch(input) => {
                format!("unrecognized arch - received: '{input}'")
            }
            NodeJsRelInfoError::UnrecognizedExt(input) => {
                format!("unrecognized file extension - received: '{input}'")
            }
            NodeJsRelInfoError::InvalidVersion(input) => {
                format!("invalid version - received: '{input}'")
            }
            NodeJsRelInfoError::UnrecognizedVersion(input) => {
                format!("unrecognized version - received: '{input}'")
            }
            NodeJsRelInfoError::UnrecognizedConfiguration(input) => {
                format!("unrecognized configuration - received: '{input}'")
            }
            NodeJsRelInfoError::HttpError(e) => return write!(f, "{e}"),
        };

        write!(f, "{message}")
    }
}

impl From<reqwest::Error> for NodeJsRelInfoError {
    fn from(e: reqwest::Error) -> Self {
        NodeJsRelInfoError::HttpError(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_prints_expected_message_when_os_is_unrecognized() {
        let err = NodeJsRelInfoError::UnrecognizedOs("unknown-os".to_string());
        assert_eq!(format!("{err}"), "unrecognized os - received: 'unknown-os'");
    }

    #[test]
    fn it_prints_expected_message_when_arch_is_unrecognized() {
        let err = NodeJsRelInfoError::UnrecognizedArch("unknown-arch".to_string());
        assert_eq!(
            format!("{err}"),
            "unrecognized arch - received: 'unknown-arch'"
        );
    }

    #[test]
    fn it_prints_expected_message_when_extension_is_unrecognized() {
        let err = NodeJsRelInfoError::UnrecognizedExt("unknown-ext".to_string());
        assert_eq!(
            format!("{err}"),
            "unrecognized file extension - received: 'unknown-ext'"
        );
    }

    #[test]
    fn it_prints_expected_message_when_version_is_invalid() {
        let err = NodeJsRelInfoError::InvalidVersion("invalid-ver".to_string());
        assert_eq!(
            format!("{err}"),
            "invalid version - received: 'invalid-ver'"
        );
    }

    #[test]
    fn it_prints_expected_message_when_version_is_unrecognized() {
        let err = NodeJsRelInfoError::UnrecognizedVersion("unknown-ver".to_string());
        assert_eq!(
            format!("{err}"),
            "unrecognized version - received: 'unknown-ver'"
        );
    }

    #[test]
    fn it_prints_expected_message_when_configuration_is_unrecognized() {
        let err = NodeJsRelInfoError::UnrecognizedConfiguration("unknown-cfg".to_string());
        assert_eq!(
            format!("{err}"),
            "unrecognized configuration - received: 'unknown-cfg'"
        );
    }

    #[tokio::test]
    async fn it_prints_expected_message_upon_http_error() {
        let source = reqwest::get("not-a-url").await.unwrap_err();
        // NOTE: `HttpError` delegates to the wrapped `reqwest::Error` verbatim
        // so assert on that rather than on reqwest's exact wording, which
        // changes between releases
        let expected = source.to_string();
        let err = NodeJsRelInfoError::from(source);

        assert_eq!(format!("{err}"), expected);
    }
}
