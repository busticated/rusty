use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum NodeJSRelInfoError {
    /// The operating system for the Node.js distributable you are targeting is
    /// unrecognized - see: [`NodeJSOS`](crate::NodeJSOS) for options
    UnrecognizedOs(String),
    /// The CPU architecture for the Node.js distributable you are targeting is
    /// unrecognized - see: [`NodeJSArch`](crate::NodeJSArch) for options
    UnrecognizedArch(String),
    /// The file extension of the Node.js distributable you are targeting is
    /// unrecognized - see: [`NodeJSPkgExt`](crate::NodeJSPkgExt) for options
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

impl Error for NodeJSRelInfoError {}

impl Display for NodeJSRelInfoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let message = match self {
            NodeJSRelInfoError::UnrecognizedOs(input) => {
                format!("Unrecognized OS! Received: '{}'", input)
            }
            NodeJSRelInfoError::UnrecognizedArch(input) => {
                format!("Unrecognized Arch! Received: '{}'", input)
            }
            NodeJSRelInfoError::UnrecognizedExt(input) => {
                format!("Unrecognized File Extension! Received: '{}'", input)
            }
            NodeJSRelInfoError::InvalidVersion(input) => {
                format!("Invalid Version! Received: '{}'", input)
            }
            NodeJSRelInfoError::UnrecognizedVersion(input) => {
                format!("Unrecognized Version! Received: '{}'", input)
            }
            NodeJSRelInfoError::UnrecognizedConfiguration(input) => {
                format!("Unrecognized Configuration! Received: '{}'", input)
            }
            NodeJSRelInfoError::HttpError(e) => {
                return write!(f, "{}", e)
            }
        };

        write!(f, "Error: {}", message)
    }
}

impl From<reqwest::Error> for NodeJSRelInfoError {
    fn from(e: reqwest::Error) -> Self {
        NodeJSRelInfoError::HttpError(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_prints_expected_message_when_os_is_unrecognized() {
        let err = NodeJSRelInfoError::UnrecognizedOs("unknown-os".to_string());
        assert_eq!(
            format!("{err}"),
            "Error: Unrecognized OS! Received: 'unknown-os'"
        );
    }

    #[test]
    fn it_prints_expected_message_when_arch_is_unrecognized() {
        let err = NodeJSRelInfoError::UnrecognizedArch("unknown-arch".to_string());
        assert_eq!(
            format!("{err}"),
            "Error: Unrecognized Arch! Received: 'unknown-arch'"
        );
    }

    #[test]
    fn it_prints_expected_message_when_extension_is_unrecognized() {
        let err = NodeJSRelInfoError::UnrecognizedExt("unknown-ext".to_string());
        assert_eq!(
            format!("{err}"),
            "Error: Unrecognized File Extension! Received: 'unknown-ext'"
        );
    }

    #[test]
    fn it_prints_expected_message_when_version_is_invalid() {
        let err = NodeJSRelInfoError::InvalidVersion("invalid-ver".to_string());
        assert_eq!(
            format!("{err}"),
            "Error: Invalid Version! Received: 'invalid-ver'"
        );
    }

    #[test]
    fn it_prints_expected_message_when_version_is_unrecognized() {
        let err = NodeJSRelInfoError::UnrecognizedVersion("unknown-ver".to_string());
        assert_eq!(
            format!("{err}"),
            "Error: Unrecognized Version! Received: 'unknown-ver'"
        );
    }

    #[test]
    fn it_prints_expected_message_when_configuration_is_unrecognized() {
        let err = NodeJSRelInfoError::UnrecognizedConfiguration("unknown-cfg".to_string());
        assert_eq!(
            format!("{err}"),
            "Error: Unrecognized Configuration! Received: 'unknown-cfg'"
        );
    }

    #[tokio::test]
    async fn it_prints_expected_message_upon_http_error() {
        let err = fake_http_error().await.unwrap_err();
        assert_eq!(
            format!("{err}"),
            "builder error: relative URL without a base"
        );
    }

    async fn fake_http_error() -> std::result::Result<(), NodeJSRelInfoError> {
        let error = reqwest::get("not-a-url").await.unwrap_err();
        Err(NodeJSRelInfoError::from(error))
    }
}
