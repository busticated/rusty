use detect_newline_style::*;

#[test]
fn it_detects_cr_style_endings_while_defaulting_to_cr_endings() {
    let input = "one\rtwo\r\nthree\rfour\n";
    let eol = LineEnding::find_or_use_cr(input);
    assert_eq!(eol, LineEnding::CR);
}

#[test]
fn it_detects_lf_style_endings_while_defaulting_to_cr_endings() {
    let input = "one\rtwo\r\nthree\nfour\n";
    let eol = LineEnding::find_or_use_cr(input);
    assert_eq!(eol, LineEnding::LF);
}

#[test]
fn it_detects_crlf_style_endings_while_defaulting_to_cr_endings() {
    let input = "one\rtwo\r\nthree\nfour\r\n";
    let eol = LineEnding::find_or_use_cr(input);
    assert_eq!(eol, LineEnding::CRLF);
}
#[test]
fn it_detects_cr_style_endings_while_defaulting_to_lf_endings() {
    let input = "one\rtwo\r\nthree\rfour\n";
    let eol = LineEnding::find_or_use_lf(input);
    assert_eq!(eol, LineEnding::CR);
}

#[test]
fn it_detects_lf_style_endings_while_defaulting_to_lf_endings() {
    let input = "one\rtwo\r\nthree\nfour\n";
    let eol = LineEnding::find_or_use_lf(input);
    assert_eq!(eol, LineEnding::LF);
}

#[test]
fn it_detects_crlf_style_endings_while_defaulting_to_lf_endings() {
    let input = "one\rtwo\r\nthree\nfour\r\n";
    let eol = LineEnding::find_or_use_lf(input);
    assert_eq!(eol, LineEnding::CRLF);
}
#[test]
fn it_detects_cr_style_endings_while_defaulting_to_crlf_endings() {
    let input = "one\rtwo\r\nthree\rfour\n";
    let eol = LineEnding::find_or_use_crlf(input);
    assert_eq!(eol, LineEnding::CR);
}

#[test]
fn it_detects_lf_style_endings_while_defaulting_to_crlf_endings() {
    let input = "one\rtwo\r\nthree\nfour\n";
    let eol = LineEnding::find_or_use_crlf(input);
    assert_eq!(eol, LineEnding::LF);
}

#[test]
fn it_detects_crlf_style_endings_while_defaulting_to_crlf_endings() {
    let input = "one\rtwo\r\nthree\nfour\r\n";
    let eol = LineEnding::find_or_use_crlf(input);
    assert_eq!(eol, LineEnding::CRLF);
}
