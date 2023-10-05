#![doc = include_str!("../README.md")]

use regex::RegexBuilder;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const CR: &str = "\r";
const LF: &str = "\n";
const CRLF: &str = "\r\n";

#[derive(Clone, Debug, Default, PartialEq)]
pub enum LineEnding {
    /// CR-style line ending (`"\r"`) rarely used, mostly on older systems
    /// (e.g. classic MacOS - OS-X before 10.0)
    CR,
    /// LF-style line ending (`"\n"`) typically used on *nix and MacOS
    #[default]
    LF,
    /// CRLF-style line ending (`"\r\n"`) typically used on Windows
    CRLF,
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
    /// assert_eq!(eol, LineEnding::LF);
    /// ```
    pub fn new<K: AsRef<str>>(kind: K) -> LineEnding {
        let kind = LineEnding::from_str(kind.as_ref());

        if kind.is_err() {
            return LineEnding::LF;
        }

        kind.unwrap()
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
    /// let eol = LineEnding::find("one\ntwo\r\nthree\n", LineEnding::CRLF);
    /// assert_eq!(eol, LineEnding::LF);
    /// ```
    pub fn find<S: AsRef<str>>(text: S, default: LineEnding) -> LineEnding {
        let text = text.as_ref();
        let ptn = r"(?:\r\n?|\n)";
        let re = RegexBuilder::new(ptn)
            .case_insensitive(true)
            .multi_line(true)
            .build()
            .unwrap();

        let matches = re.find_iter(text);
        let mut crlf_count = 0;
        let mut cr_count = 0;
        let mut lf_count = 0;

        for item in matches {
            let x = item.as_str();

            if x == CRLF {
                crlf_count += 1;
            } else if x == LF {
                lf_count += 1;
            } else if x == CR {
                cr_count += 1;
            }
        }

        if crlf_count > lf_count && crlf_count > cr_count {
            return LineEnding::CRLF;
        } else if lf_count > crlf_count && lf_count > cr_count {
            return LineEnding::LF;
        } else if cr_count > lf_count && cr_count > crlf_count {
            return LineEnding::CR;
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
    /// assert_eq!(eol, LineEnding::LF);
    /// let eol = LineEnding::find_or_use_crlf("one\ntwo\r\nthree\r");
    /// assert_eq!(eol, LineEnding::CRLF);
    /// ```
    pub fn find_or_use_crlf<S: AsRef<str>>(s: S) -> LineEnding {
        LineEnding::find(s, LineEnding::CRLF)
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
    /// assert_eq!(eol, LineEnding::CRLF);
    /// let eol = LineEnding::find_or_use_lf("one\ntwo\r\nthree\r");
    /// assert_eq!(eol, LineEnding::LF);
    /// ```
    pub fn find_or_use_lf<S: AsRef<str>>(s: S) -> LineEnding {
        LineEnding::find(s, LineEnding::LF)
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
    /// assert_eq!(eol, LineEnding::LF);
    /// let eol = LineEnding::find_or_use_cr("one\ntwo\r\nthree\r");
    /// assert_eq!(eol, LineEnding::CR);
    /// ```
    pub fn find_or_use_cr<S: AsRef<str>>(s: S) -> LineEnding {
        LineEnding::find(s, LineEnding::CR)
    }
}

impl Display for LineEnding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let eol = match self {
            LineEnding::CR => CR,
            LineEnding::LF => LF,
            LineEnding::CRLF => CRLF,
        };

        write!(f, "{}", eol)
    }
}

impl FromStr for LineEnding {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<LineEnding, Box<dyn Error>> {
        match s.to_lowercase().as_str() {
            CR => Ok(LineEnding::CR),
            LF => Ok(LineEnding::LF),
            CRLF => Ok(LineEnding::CRLF),
            _ => Err(format!("Unrecognized input: {}", s).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes_a_line_ending() {
        let eol = LineEnding::new("\r");

        assert_eq!(eol, LineEnding::CR);

        let eol = LineEnding::new("\n");

        assert_eq!(eol, LineEnding::LF);

        let eol = LineEnding::new("\r\n");

        assert_eq!(eol, LineEnding::CRLF);
    }

    #[test]
    fn it_uses_lf_line_ending_when_kind_is_unrecognized() {
        let eol = LineEnding::new("NOPE!");
        assert_eq!(eol, LineEnding::LF);
    }

    #[test]
    fn it_serializes_a_line_ending() {
        assert_eq!("\r", format!("{}", LineEnding::CR));
        assert_eq!("\n", format!("{}", LineEnding::LF));
        assert_eq!("\r\n", format!("{}", LineEnding::CRLF));
    }

    #[test]
    fn it_finds_preferred_line_ending_when_input_prefers_unix_style_endings() {
        let input = "\nthis\nprefers\nunix-style endings\r\n";
        let eol = LineEnding::find(input, LineEnding::CRLF);
        assert_eq!(eol, LineEnding::LF);
    }

    #[test]
    fn it_finds_preferred_line_ending_when_input_prefers_windows_style_endings() {
        let input = "\r\nthis\r\nprefers\r\nwindows-style endings\n";
        let eol = LineEnding::find(input, LineEnding::CRLF);
        assert_eq!(eol, LineEnding::CRLF);
    }

    #[test]
    fn it_finds_preferred_line_ending_when_input_prefers_obsolete_style_endings() {
        let input = "\rthis\rprefers\r\nobsolete endings\n";
        let eol = LineEnding::find(input, LineEnding::CRLF);
        assert_eq!(eol, LineEnding::CR);
    }

    #[test]
    fn it_uses_default_when_preference_cannot_be_determined() {
        let input = "\r\nthis\r\nis\nambiguous\n?\r\r";
        let eol = LineEnding::find(input, LineEnding::LF);
        assert_eq!(eol, LineEnding::LF);
    }

    #[test]
    fn it_uses_default_when_text_has_no_line_breaks() {
        let input = "no line breaks";
        let eol = LineEnding::find(input, LineEnding::LF);
        assert_eq!(eol, LineEnding::LF);
    }

    #[test]
    fn it_uses_default_when_text_is_empty() {
        let input = "";
        let eol = LineEnding::find(input, LineEnding::LF);
        assert_eq!(eol, LineEnding::LF);
    }

    #[test]
    fn it_finds_preferred_line_ending_defaulting_to_cr_endings() {
        let input = "\rthis\rprefers\r\nobsolete endings\n";
        let eol = LineEnding::find_or_use_cr(input);

        assert_eq!(eol, LineEnding::CR);

        let input = "\r\nthis\r\nis\nambiguous\n?\r\r";
        let eol = LineEnding::find_or_use_cr(input);

        assert_eq!(eol, LineEnding::CR);
    }

    #[test]
    fn it_finds_preferred_line_ending_defaulting_to_lf_endings() {
        let input = "\nthis\nprefers\nunix-style endings\r\n";
        let eol = LineEnding::find_or_use_lf(input);

        assert_eq!(eol, LineEnding::LF);

        let input = "\r\nthis\r\nis\nambiguous\n?\r\r";
        let eol = LineEnding::find_or_use_lf(input);

        assert_eq!(eol, LineEnding::LF);
    }

    #[test]
    fn it_finds_preferred_line_ending_defaulting_to_crlf_endings() {
        let input = "\r\nthis\r\nprefers\r\nwindows-style endings\n";
        let eol = LineEnding::find_or_use_crlf(input);

        assert_eq!(eol, LineEnding::CRLF);

        let input = "\r\nthis\r\nis\nambiguous\n?\r\r";
        let eol = LineEnding::find_or_use_crlf(input);

        assert_eq!(eol, LineEnding::CRLF);
    }
}
