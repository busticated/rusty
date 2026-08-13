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


## Migrations

<details id="migrate-0x-to-1x">
<summary><b>0.x -> 1.x</b></summary>
<p>

**Variant names now follow [RFC 430](https://rust-lang.github.io/rfcs/0430-finalizing-naming-conventions.html)**

| before | after |
| --- | --- |
| `LineEnding::CR` | `LineEnding::Cr` |
| `LineEnding::LF` | `LineEnding::Lf` |
| `LineEnding::CRLF` | `LineEnding::Crlf` |

**`FromStr` returns a concrete error type**

`LineEnding::from_str` now fails with `ParseLineEndingError` instead of `Box<dyn Error>`. The old type was neither `Send` nor `Sync`, so the error could not cross a thread boundary or compose with most application error types.

Code that propagates with `?` into a function returning `Box<dyn Error>` keeps working unchanged. Code that names the error type explicitly needs updating:

```rust,ignore
// before
let eol: Result<LineEnding, Box<dyn Error>> = LineEnding::from_str("\n");
// after
let eol: Result<LineEnding, ParseLineEndingError> = LineEnding::from_str("\n");
```

`from_str` also no longer lowercases its input. This has no observable effect, since the only valid inputs are `"\r"`, `"\n"` and `"\r\n"`.

**`LineEnding` is now `Copy`**

Existing code is unaffected - values that were previously moved are now copied.

**The `regex` dependency is gone**

This crate now has no dependencies at all. Behaviour is unchanged; see the tests covering `"\r\r\n"` and multi-byte input for the edge cases.

</p>
</details>
