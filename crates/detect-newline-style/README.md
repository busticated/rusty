# detect-newline-style

[![Latest Version](https://img.shields.io/crates/v/detect-newline-style.svg)](https://crates.io/crates/detect-newline-style)
[![Documentation](https://docs.rs/detect-newline-style/badge.svg)](https://docs.rs/detect-newline-style)
[![CI Status](https://github.com/busticated/rusty/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/busticated/rusty/actions)

Determine a string's preferred newline character

## Installation

```shell
cargo add detect-newline-style
```

## Examples

```rust
use detect_newline_style::LineEnding;

fn main() {
    let text = "one\rtwo\r\nthree\nfour\n";
    let eol = LineEnding::find(text, LineEnding::LF);

    assert_eq!(eol, LineEnding::LF);

    let text = "one\rtwo\r\nthree\n";
    let eol = LineEnding::find_or_use_lf(text);

    assert_eq!(eol, LineEnding::LF);

    let text = "one\rtwo\r\nthree\n";
    let eol = LineEnding::find_or_use_crlf(text);

    assert_eq!(eol, LineEnding::CRLF);

    assert_eq!(format!("{}", LineEnding::CR), "\r");
    assert_eq!(format!("{}", LineEnding::LF), "\n");
    assert_eq!(format!("{}", LineEnding::CRLF), "\r\n");
}
```

