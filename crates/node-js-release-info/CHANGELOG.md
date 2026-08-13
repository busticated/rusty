# `node-js-release-info` Changelog
<!-- next-version-start -->
<!-- next-version-end -->
## v2.0.0

* add migrations section to README with v1.x -> v2.x notes
* rename types + variants per RFC 430, consume self in builders
* BREAKING: types are now `NodeJsRelInfo`, `NodeJsOs`, `NodeJsArch`, `NodeJsPkgExt`, `NodeJsRelInfoError`
* BREAKING: variants are now UpperCamelCase - e.g. `AIX` -> `Aix`, `PPC64LE` -> `Ppc64le`
* BREAKING: builder methods consume and return `Self`, so `to_owned()` is gone
* BREAKING: `fetch()` takes `self` and returns the populated value
* BREAKING: os/arch/ext/error enums are `#[non_exhaustive]`
* BREAKING: error messages drop the `Error: ` prefix and are lowercased
* add `NodeJsOs::SunOs` and `NodeJsArch::Armv6l` for older Node.js releases
* derive `Copy` on the os/arch/ext enums
* return an error instead of panicking on a malformed checksum line
* target Node.js v24.19.0 in integration tests and doc examples
* expose error source, derive Eq + Hash, document json feature
* assert on returned errors instead of panic text
* inherit dependencies and lint settings from workspace
* clarify licensing
* update cargo dependencies to latest
* set minimum supported rust version to v1.86
* fix lint issues
* fix tokio for doctests, serde for serialization tests
* fix typo


## v1.1.1

* add new version markers to changelog
* add integration test for .fetch_all()
* lintings


## v1.1.0

* add docs and examples for .fetch_all() method
* add 7z to the list of recognized exts
* add ppc64 and s390x to the list of recognized archs
* add aix to the list of recognized oses
* add .fetch_all() method to retrieve the list of supported configurations


## v1.0.0

* add docs for json feature
* add test to verify thread-safe types
* breaking: remove .to_json_string() method now that the json feature is available
* add json feature to enable full serialization + deserialization


## v0.1.1

* clarify example usage instructions
* add tests for serializing enums


## v0.1.0

* Initial release 🎊🎉

