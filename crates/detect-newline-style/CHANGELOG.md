# `detect-newline-style` Changelog
<!-- next-version-start -->
<!-- next-version-end -->
## v1.0.0

* add migrations section to README with v0.x -> v1.x notes
* rename variants per RFC 430, add concrete error type
* BREAKING: `LineEnding::{CR,LF,CRLF}` are now `{Cr,Lf,Crlf}`
* BREAKING: `FromStr::Err` is now `ParseLineEndingError`, not `Box<dyn Error>`
* derive `Copy` on `LineEnding`
* remove regex dependency
* inherit dependencies and lint settings from workspace
* clarify licensing
* set minimum supported rust version to v1.86


## v0.1.2

* add new version markers to changelog


## v0.1.1

* add tests to cover no-op cases


## v0.1.0

* Initial release 🎊🎉

