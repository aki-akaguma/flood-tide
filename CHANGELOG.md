# Changelog: flood-tide

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.15] - 2026-05-27
### Changed
* Bump crates: criterion(0.8), criterion-cycles-per-byte(0.8), clf(0.2)

### Fixed
* Resolve `clippy::useless_borrows_in_formatting`

## [0.2.14] - 2026-05-17
### Changed
* Optimize compile-time sorting algorithm from bubble sort to shell sort in `argparse!` macro.
* Refactor `src/lib.rs` to reduce `cfg` attribute noise by extracting feature-gated logic into helper methods.
* Refactor `str_cmp` in `src/macro_util.rs` for improved readability while maintaining Rust 1.60.0 compatibility.

## [0.2.13] - 2026-05-11
### Added
* Introduce `argparse!` macro

### Fixed
* Resolve `clippy::needless_return`

## [0.2.12] - 2025-09-24
### Added
* Provide `specs`
* Include more tests

### Changed
* Adjust some bits codes
* Bump rust-version to "1.60.0"

### Fixed
* Resolve `clippy::uninlined_format_args`

## [0.2.11] - 2024-06-09
### Changed
* Modify filename: `config` to `config.toml`
* Bump crates: criterion(0.5), itertools(0.13)
* Adjust test support for 1.60.0 on github workflows
* Adjust build support for 1.60.0 on github workflows

### Fixed
* Resolve `clippy::useless_conversion`
* Resolve `clippy::needless_lifetimes`
* Resolve `clippy::let_unit_value`

## [0.2.10] - 2023-02-12
### Added
* Introduce `.github/workflows/test-ubuntu.yml`
* Introduce `.github/workflows/test-macos.yml`
* Introduce `.github/workflows/test-windows.yml`
* Include test status badges into `README.tpl`

### Changed
* Refactor `Makefile`

### Removed
* Discard `COPYING`

### Fixed
* Correct `LICENSE-APACHE`, `LICENSE-MIT`
* Resolve `clippy::needless_borrow`
* Resolve `clippy::bool_assert_comparison`
* Resolve `clippy::map_identity`
* Resolve `clippy::ptr_arg`

## [0.2.9] - 2023-01-28
### Added
* Introduce `.github/workflows/test.yml`
* Include test status badges into `README.tpl`

### Fixed
* Correct Makefile: rustc version `1.66.0` to `1.66.1`
* Resolve `clippy::bool_assert_comparison`
* Resolve `clippy::redundant_clone`
* Resolve `clippy::redundant_pattern_matching`
* Resolve `clippy::while_let_on_iterator`
* Resolve `clippy::let_unit_value`
* Resolve `clippy::single_match`
* Correct `LICENSE` files

## [0.2.8] - 2023-01-10
### Added
* Include version difference link into `CHANGELOG.md`
* Introduce `rust-version = "1.57.0"` into `Cargo.toml`
* Provide `all-test-version` target into `Makefile`
* Include badges into `README.tpl`

### Changed
* Enhance `examples/curl.cmd.txt`
* Relocate benches into xbench

## [0.2.7] - 2023-01-05
### Changed
* Adjust formatting of `CHANGELOG.md`

## [0.2.6] - 2023-01-02
### Changed
* Bump crates: criterion(0.4)

### Fixed
* Resolve clippy: you are deriving `PartialEq` and can implement `Eq`
* Resolve `clippy::format_push_string`

## [0.2.5] - 2022-06-13
### Changed
* Transition to edition 2021

## [0.2.4] - 2021-11-14
### Added
* Provide more documents

### Changed
* Refactor source codes

## [0.2.3] - 2021-09-10
### Changed
* Bump crates: criterion(0.3.5)

## [0.2.2] - 2021-05-09
### Changed
* Bump dependencies: regex(1.5.4)

## [0.2.1] - 2021-04-23
### Added
* Introduce `fn Opt::lon_or_sho()`

## [0.2.0] - 2021-04-03
### Added
* Introduce `trait SubCommand`
* Include `simple_gnu_style_subc` into features
* Introduce `fn parse_simple_gnu_style_subcmd()`

### Changed
* Modify name: `OPErr` to `OpErr`
* Bump dependencies

### Fixed
* Resolve bug: if the scan has `subcmd`, then it should stop at free.
* Resolve bug: can not compile benches

### Removed
* Discard unnecessary trailing semicolon

## [0.1.21] - 2021-02-05
### Changed
* Adjust `README.md`

## [0.1.20] - 2021-01-31
### Added
* Provide `impl HelpVersion for OptParseError`
* Provide `help_message()` and `version_message()` to `OptParseError`
* Introduce `trait HelpVersion`

## [0.1.19] - 2021-01-09
### Added
* Include `dox` into features

### Fixed
* Resolve bug in `README.md`

## [0.1.18] - 2021-01-03
### Changed
* Adjust for rustc 1.49.0 (e1884a8e3 2020-12-29)

## [0.1.17] - 2020-12-25
### Fixed
* Correct `README.md`

## [0.1.16] - 2020-12-25
### Changed
* Initial release to crates.io

## [0.1.15] - 2020-12-25
### Changed
* Modify package name: `flood-tide` from `optpa-util-5`

## [0.1.14] - 2020-12-20
### Added
* Introduce `Lex::create_with()`
* Include `create_with` to features
* Include `sorted_opt_ary` to features

### Removed
* Discard `Lex::create_from()`
* Discard from features: `sorted_opt_ary`, `create_with`

## [0.1.13] - 2020-12-18
### Added
* Include in features: `stop_at_mm`, `stop_at_free`, `no_std`

### Removed
* Discard from features: `std`

## [0.1.12] - 2020-12-09
### Changed
* Adjust `impl std::error::Error for OptParseErrors`

## [0.1.11] - 2020-12-06
### Changed
* Introduce `optpaerr-5("0.1.6" (2020-12-05))`
* Refactor `README.txt`
* Modify crate dependency: combination("0.1.2") to itertools("0.9") in xtask

## [0.1.10] - 2020-12-04
### Added
* Provide error message with file name to `xtask::update_file()`

### Changed
* Adjust some examples

### Fixed
* Resolve clippy: `xtask/src/gen_features_combination.rs`
* Resolve clippy: `examples/curl.rs`

## [0.1.9] - 2020-12-02
### Added
* Introduce `fn parse_simple_gnu_style()`

### Changed
* Adjust examples to new style
* Refactor source code

### Fixed
* Resolve bug: `examples/curl.rs` and xtask

## [0.1.8] - 2020-11-28
### Added
* Provide `gen_features_combination` to xtask
* Include `optnum_u16` in `[features]`
* Provide `bench-one.rs`

### Changed
* Adjust visibility: `pub struct CmdOptConf`, on `examples/curl.rs`

## [0.1.7] - 2020-11-25
### Added
* Provide `xtask` for example `curl`
* Provide example `curl`

### Changed
* Modify `Vec<&str>` to `&[&str]` at the param type of `tokens_from()`
* Refactor xtask's `gen_src_example_curl_cmd`

### Removed
* Discard `scripts/gen-parser-curl.pl`

## [0.1.6] - 2020-11-18
### Added
* Include `README.md`, `COPYING`, `LICENSE-APACHE`, `LICENSE-MIT`
* Introduce `no_std` support
* Provide example `gnu-cat`

### Changed
* Modify repo name: `optpa-util` to `optpa-util-1`

## [0.1.5] - 2020-10-29
### Added
* Introduce `NameVal.name()`

## [0.1.4] - 2020-10-05
### Added
* Include `option_argument` in features
* Provide `example/ffmpeg`
* Provide `example/bsd-sed`
* Provide `examples`

### Fixed
* Resolve issues in argument features

## [0.1.3] - 2020-09-20
### Changed
* Refactor codebase
* Modify `is_long` to `was_long`

## [0.1.2] - 2020-09-17
### Changed
* Initialize fork from `optpa-util`
* Modify various components

## [0.1.1] - 2018-05-22
### Changed
* Modify name: `optpa_util` to `optpa-util`

### Fixed
* Resolve dependency issues

## [0.1.0] - 2017-11-06
### Added
* Initial commit

[Unreleased]: https://github.com/aki-akaguma/flood-tide/compare/v0.2.15..HEAD
[0.2.15]: https://github.com/aki-akaguma/flood-tide/compare/v0.2.14..v0.2.15
[0.2.14]: https://github.com/aki-akaguma/flood-tide/compare/v0.2.13..v0.2.14
[0.2.13]: https://github.com/aki-akaguma/flood-tide/compare/v0.2.12..v0.2.13
[0.2.12]: https://github.com/aki-akaguma/flood-tide/compare/v0.2.11..v0.2.12
[0.2.11]: https://github.com/aki-akaguma/flood-tide/compare/v0.2.10..v0.2.11
[0.2.10]: https://github.com/aki-akaguma/flood-tide/compare/v0.2.9..v0.2.10
[0.2.9]: https://github.com/aki-akaguma/flood-tide/compare/v0.2.8..v0.2.9
[0.2.8]: https://github.com/aki-akaguma/flood-tide/compare/v0.2.7..v0.2.8
[0.2.7]: https://github.com/aki-akaguma/flood-tide/compare/v0.2.6..v0.2.7
[0.2.6]: https://github.com/aki-akaguma/flood-tide/compare/v0.2.5..v0.2.6
[0.2.5]: https://github.com/aki-akaguma/flood-tide/compare/v0.2.4..v0.2.5
[0.2.4]: https://github.com/aki-akaguma/flood-tide/compare/v0.2.3..v0.2.4
[0.2.3]: https://github.com/aki-akaguma/flood-tide/compare/v0.2.2..v0.2.3
[0.2.2]: https://github.com/aki-akaguma/flood-tide/compare/v0.2.1..v0.2.2
[0.2.1]: https://github.com/aki-akaguma/flood-tide/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/aki-akaguma/flood-tide/compare/v0.1.21..v0.2.0
[0.1.21]: https://github.com/aki-akaguma/flood-tide/compare/v0.1.20..v0.1.21
[0.1.20]: https://github.com/aki-akaguma/flood-tide/compare/v0.1.19..v0.1.20
[0.1.19]: https://github.com/aki-akaguma/flood-tide/compare/v0.1.18..v0.1.19
[0.1.18]: https://github.com/aki-akaguma/flood-tide/compare/v0.1.17..v0.1.18
[0.1.17]: https://github.com/aki-akaguma/flood-tide/compare/v0.1.16..v0.1.17
[0.1.16]: https://github.com/aki-akaguma/flood-tide/compare/v0.1.15..v0.1.16
[0.1.15]: https://github.com/aki-akaguma/flood-tide/releases/tag/v0.1.15
