# HLLV :rocket:

[![Crates.io][crates_badge]][crates]
[![Docs.rs][docs_badge]][docs]
[![Travis CI][travis_badge]][travis]
[![Appveyor][appveyor_badge]][appveyor]
[![Codecov.io][codecov_badge]][codecov]
[![License][license_badge]][license]

[crates_badge]: https://img.shields.io/crates/v/hllv.svg
[docs_badge]: https://docs.rs/hllv/badge.svg
[travis_badge]: https://api.travis-ci.org/nthend/hllv.svg
[appveyor_badge]: https://ci.appveyor.com/api/projects/status/github/nthend/hllv?branch=master&svg=true
[codecov_badge]: https://codecov.io/gh/nthend/hllv/graphs/badge.svg
[license_badge]: https://img.shields.io/crates/l/hllv.svg

[crates]: https://crates.io/crates/hllv
[docs]: https://docs.rs/hllv
[travis]: https://travis-ci.org/nthend/hllv
[appveyor]: https://ci.appveyor.com/project/nthend/hllv
[codecov]: https://codecov.io/gh/nthend/hllv
[license]: #license

Simple git-like VCS for large binary files.

## Features
+ Zero filesystem and network overhead
+ Partial upload/download, resume after failure
+ Consistency and repetition check
+ Written entirely in Rust

## Installation

```sh
cargo install --git https://github.com/nthend/hllv
```

## [TODO](TODO.md)

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
