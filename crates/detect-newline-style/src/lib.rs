#![doc = include_str!("../README.md")]

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Returned by [`LineEnding::from_str`] when the input is not one of `"\r"`,
/// `"\n"` or `"\r\n"`
///
/// This is a concrete type rather than `Box<dyn Error>` so it is `Send + Sync`
/// and can cross thread boundaries
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ParseLineEndingError {
    input: String,
}

impl ParseLineEndingError {
    /// The unrecognized input that produced this error
    pub fn input(&self) -> &str {
        &self.input
    }
}

impl Display for ParseLineEndingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "unrecognized line ending - received: '{}'", self.input)
    }
}

impl Error for ParseLineEndingError {}

const CR: &str = "\r";
const LF: &str = "\n";
const CRLF: &str = "\r\n";

/// A newline style - see [`find`](LineEnding::find) to detect which style a
/// given string prefers
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum LineEnding {
    /// CR-style line ending (`"\r"`) rarely used, mostly on older systems
    /// (e.g. classic MacOS - OS-X before 10.0)
    Cr,
    /// LF-style line ending (`"\n"`) typically used on *nix and MacOS
    #[default]
    Lf,
    /// CRLF-style line ending (`"\r\n"`) typically used on Windows
    Crlf,
}

impl LineEnding {
    /// Creates a new instance - you'll almost certainly rather use one of the
    /// [`find*`](crate::LineEnding::find) associated fns below :)
    ///
    /// # Arguments
    ///
    /// * `kind` - The line ending style you want
    ///
    /// # Examples
    ///
    /// ```rust
    /// use detect_newline_style::LineEnding;
    /// let eol = LineEnding::new("\n");
    /// assert_eq!(eol, LineEnding::Lf);
    /// ```
    pub fn new<K: AsRef<str>>(kind: K) -> LineEnding {
        // NOTE: unrecognized input falls back to the default (`Lf`) rather
        // than failing - use [`from_str`](LineEnding::from_str) if you need
        // to know the input was invalid
        LineEnding::from_str(kind.as_ref()).unwrap_or_default()
    }

    /// Determines which newline style a given string uses (CR, LF, or CRLF)
    ///
    /// # Arguments
    ///
    /// * `text` - The text you want to analyze
    /// * `default` - The default newline style to use when text has no preference
    ///
    /// # Examples
    ///
    /// ```rust
    /// use detect_newline_style::LineEnding;
    /// let eol = LineEnding::find("one\ntwo\r\nthree\n", LineEnding::Crlf);
    /// assert_eq!(eol, LineEnding::Lf);
    /// ```
    pub fn find<S: AsRef<str>>(text: S, default: LineEnding) -> LineEnding {
        // NOTE: `\r` and `\n` are ASCII, so they can never appear inside a
        // multi-byte UTF-8 sequence - scanning bytes is safe and avoids
        // pulling in (and re-compiling, on every call) a regex
        let bytes = text.as_ref().as_bytes();
        let mut crlf_count = 0;
        let mut cr_count = 0;
        let mut lf_count = 0;
        let mut i = 0;

        while i < bytes.len() {
            match bytes[i] {
                b'\r' => {
                    if bytes.get(i + 1) == Some(&b'\n') {
                        crlf_count += 1;
                        i += 2;
                    } else {
                        cr_count += 1;
                        i += 1;
                    }
                }
                b'\n' => {
                    lf_count += 1;
                    i += 1;
                }
                _ => i += 1,
            }
        }

        if crlf_count > lf_count && crlf_count > cr_count {
            return LineEnding::Crlf;
        } else if lf_count > crlf_count && lf_count > cr_count {
            return LineEnding::Lf;
        } else if cr_count > lf_count && cr_count > crlf_count {
            return LineEnding::Cr;
        }

        default
    }

    /// Determines which newline style a given string uses (CR, LF, or CRLF)
    /// defaulting to CRLF-style endings
    ///
    /// # Arguments
    ///
    /// * `text` - The text you want to analyze
    ///
    /// # Examples
    ///
    /// ```rust
    /// use detect_newline_style::LineEnding;
    /// let eol = LineEnding::find_or_use_crlf("one\ntwo\r\nthree\n");
    /// assert_eq!(eol, LineEnding::Lf);
    /// let eol = LineEnding::find_or_use_crlf("one\ntwo\r\nthree\r");
    /// assert_eq!(eol, LineEnding::Crlf);
    /// ```
    pub fn find_or_use_crlf<S: AsRef<str>>(s: S) -> LineEnding {
        LineEnding::find(s, LineEnding::Crlf)
    }

    /// Determines which newline style a given string uses (CR, LF, or CRLF)
    /// defaulting to LF-style endings
    ///
    /// # Arguments
    ///
    /// * `text` - The text you want to analyze
    ///
    /// # Examples
    ///
    /// ```rust
    /// use detect_newline_style::LineEnding;
    /// let eol = LineEnding::find_or_use_lf("one\r\ntwo\nthree\r\n");
    /// assert_eq!(eol, LineEnding::Crlf);
    /// let eol = LineEnding::find_or_use_lf("one\ntwo\r\nthree\r");
    /// assert_eq!(eol, LineEnding::Lf);
    /// ```
    pub fn find_or_use_lf<S: AsRef<str>>(s: S) -> LineEnding {
        LineEnding::find(s, LineEnding::Lf)
    }

    /// Determines which newline style a given string uses (CR, LF, or CRLF)
    /// defaulting to CR-style endings
    ///
    /// # Arguments
    ///
    /// * `text` - The text you want to analyze
    ///
    /// # Examples
    ///
    /// ```rust
    /// use detect_newline_style::LineEnding;
    /// let eol = LineEnding::find_or_use_cr("one\ntwo\r\nthree\n");
    /// assert_eq!(eol, LineEnding::Lf);
    /// let eol = LineEnding::find_or_use_cr("one\ntwo\r\nthree\r");
    /// assert_eq!(eol, LineEnding::Cr);
    /// ```
    pub fn find_or_use_cr<S: AsRef<str>>(s: S) -> LineEnding {
        LineEnding::find(s, LineEnding::Cr)
    }
}

impl Display for LineEnding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let eol = match self {
            LineEnding::Cr => CR,
            LineEnding::Lf => LF,
            LineEnding::Crlf => CRLF,
        };

        write!(f, "{eol}")
    }
}

impl FromStr for LineEnding {
    type Err = ParseLineEndingError;

    fn from_str(s: &str) -> Result<LineEnding, ParseLineEndingError> {
        match s {
            CR => Ok(LineEnding::Cr),
            LF => Ok(LineEnding::Lf),
            CRLF => Ok(LineEnding::Crlf),
            _ => Err(ParseLineEndingError {
                input: s.to_owned(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes_a_line_ending() {
        let eol = LineEnding::new("\r");

        assert_eq!(eol, LineEnding::Cr);

        let eol = LineEnding::new("\n");

        assert_eq!(eol, LineEnding::Lf);

        let eol = LineEnding::new("\r\n");

        assert_eq!(eol, LineEnding::Crlf);
    }

    #[test]
    fn it_uses_lf_line_ending_when_kind_is_unrecognized() {
        let eol = LineEnding::new("NOPE!");
        assert_eq!(eol, LineEnding::Lf);
    }

    #[test]
    fn it_serializes_a_line_ending() {
        assert_eq!("\r", format!("{}", LineEnding::Cr));
        assert_eq!("\n", format!("{}", LineEnding::Lf));
        assert_eq!("\r\n", format!("{}", LineEnding::Crlf));
    }

    #[test]
    fn it_finds_preferred_line_ending_when_input_prefers_unix_style_endings() {
        let input = "\nthis\nprefers\nunix-style endings\r\n";
        let eol = LineEnding::find(input, LineEnding::Crlf);
        assert_eq!(eol, LineEnding::Lf);
    }

    #[test]
    fn it_finds_preferred_line_ending_when_input_prefers_windows_style_endings() {
        let input = "\r\nthis\r\nprefers\r\nwindows-style endings\n";
        let eol = LineEnding::find(input, LineEnding::Crlf);
        assert_eq!(eol, LineEnding::Crlf);
    }

    #[test]
    fn it_finds_preferred_line_ending_when_input_prefers_obsolete_style_endings() {
        let input = "\rthis\rprefers\r\nobsolete endings\n";
        let eol = LineEnding::find(input, LineEnding::Crlf);
        assert_eq!(eol, LineEnding::Cr);
    }

    #[test]
    fn it_uses_default_when_preference_cannot_be_determined() {
        let input = "\r\nthis\r\nis\nambiguous\n?\r\r";
        let eol = LineEnding::find(input, LineEnding::Lf);
        assert_eq!(eol, LineEnding::Lf);
    }

    #[test]
    fn it_counts_a_lone_cr_followed_by_crlf_separately() {
        // "\r\r\n" is one CR then one CRLF - not two CRs, and not a CR plus
        // an LF.
        let eol = LineEnding::find("a\r\r\nb\r\nc\r\nd", LineEnding::Lf);
        assert_eq!(eol, LineEnding::Crlf);

        // ...and with the CRLFs removed, the lone CR wins
        let eol = LineEnding::find("a\r\r\nb", LineEnding::Lf);
        assert_eq!(eol, LineEnding::Lf);
    }

    #[test]
    fn it_counts_line_breaks_around_multi_byte_characters() {
        // `\r` / `\n` are ASCII and cannot appear inside a UTF-8 sequence
        let eol = LineEnding::find("日本\r\n語\r\n🦀\n", LineEnding::Lf);
        assert_eq!(eol, LineEnding::Crlf);
    }

    #[test]
    fn it_uses_default_when_text_has_no_line_breaks() {
        let input = "no line breaks";
        let eol = LineEnding::find(input, LineEnding::Lf);
        assert_eq!(eol, LineEnding::Lf);
    }

    #[test]
    fn it_uses_default_when_text_is_empty() {
        let input = "";
        let eol = LineEnding::find(input, LineEnding::Lf);
        assert_eq!(eol, LineEnding::Lf);
    }

    #[test]
    fn it_finds_preferred_line_ending_defaulting_to_cr_endings() {
        let input = "\rthis\rprefers\r\nobsolete endings\n";
        let eol = LineEnding::find_or_use_cr(input);

        assert_eq!(eol, LineEnding::Cr);

        let input = "\r\nthis\r\nis\nambiguous\n?\r\r";
        let eol = LineEnding::find_or_use_cr(input);

        assert_eq!(eol, LineEnding::Cr);
    }

    #[test]
    fn it_finds_preferred_line_ending_defaulting_to_lf_endings() {
        let input = "\nthis\nprefers\nunix-style endings\r\n";
        let eol = LineEnding::find_or_use_lf(input);

        assert_eq!(eol, LineEnding::Lf);

        let input = "\r\nthis\r\nis\nambiguous\n?\r\r";
        let eol = LineEnding::find_or_use_lf(input);

        assert_eq!(eol, LineEnding::Lf);
    }

    #[test]
    fn it_finds_preferred_line_ending_defaulting_to_crlf_endings() {
        let input = "\r\nthis\r\nprefers\r\nwindows-style endings\n";
        let eol = LineEnding::find_or_use_crlf(input);

        assert_eq!(eol, LineEnding::Crlf);

        let input = "\r\nthis\r\nis\nambiguous\n?\r\r";
        let eol = LineEnding::find_or_use_crlf(input);

        assert_eq!(eol, LineEnding::Crlf);
    }
}
