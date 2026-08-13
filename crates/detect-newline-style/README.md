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
    let eol = LineEnding::find(text, LineEnding::Lf);

    assert_eq!(eol, LineEnding::Lf);

    let text = "one\rtwo\r\nthree\n";
    let eol = LineEnding::find_or_use_lf(text);

    assert_eq!(eol, LineEnding::Lf);

    let text = "one\rtwo\r\nthree\n";
    let eol = LineEnding::find_or_use_crlf(text);

    assert_eq!(eol, LineEnding::Crlf);

    assert_eq!(format!("{}", LineEnding::Cr), "\r");
    assert_eq!(format!("{}", LineEnding::Lf), "\n");
    assert_eq!(format!("{}", LineEnding::Crlf), "\r\n");
}
```

