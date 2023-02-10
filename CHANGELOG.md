# Changelog: flood-tide

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] *
### Fixed
* `LICENSE-APACHE`, `LICENSE-MIT`


## [0.2.9] (2023-01-28)
### Added
* `.github/workflows/test.yml`
* test status badges into `README.tpl`

### Fixed
* Makefile: rustc version `1.66.0` to `1.66.1`
* clippy: `bool\_assert\_comparison`, `redundant\_clone`
* clippy: `redundant\_pattern\_matching`, `while\_let\_on\_iterator`
* clippy: `let\_unit\_value`, `single\_match`
* `LICENSE` files

## [0.2.8] (2023-01-10)
### Added
* version difference link into `CHANGELOG.md`
* rust-version = "1.57.0" into Cargo.toml
* `all-test-version` target into Makefile
* badges into README.tpl

### Changed
* update examples/curl.cmd.txt
* move benches into xbench

## [0.2.7] (2023-01-05)
### Changed
* reformat `CHANGELOG.md`

## [0.2.6] (2023-01-02)
### Changed
* update crates: criterion(0.4)

### Fixed
* clippy: you are deriving `PartialEq` and can implement `Eq`
* clippy: format_push_string

## [0.2.5] (2022-06-13)
### Changed
* changes to edition 2021

## [0.2.4] (2021-11-14)
### Added
* more documents

### Changed
* clean source codes

## [0.2.3] (2021-09-10)
### Changed
* update dates: criterion(0.3.5)

## [0.2.2] (2021-05-09)
### Changed
* update depends: regex(1.5.4)

## [0.2.1] (2021-04-23)
### Added
* add fn Opt::lon_or_sho()

## [0.2.0] (2021-04-03)
### Added
* add trait SubCommand
* add simple_gnu_style_subc into features
* add fn parse_simple_gnu_style_subcmd()

### Changed
* rename OPErr to OpErr
* update depends

### Fixed
* bug: if the scann has subcmd, then it should stop at free.
* bug: can not compile benches

### Removed
* remove unnecessary trailing semicolon

## [0.1.21] (2021-02-05)
### Changed
* modify README.md

## [0.1.20] (2021-01-31)
### Added
* add impl HelpVersion for OptParseError
* add help_message() and version_message() to OptParseError
* add trait HelpVersion

## [0.1.19] (2021-01-09)
### Added
* add "dox" into features

### Fixed
* bug: README.md

## [0.1.18] (2021-01-03)
### Changed
* update rustc 1.49.0 (e1884a8e3 2020-12-29)

## [0.1.17] (2020-12-25)
### Fixed
* README.md

## [0.1.16] (2020-12-25)
### Changed
* publish to crates.io

## [0.1.15] (2020-12-25)
### Changed
* rename package flood-tide from optpa-util-5

## [0.1.14] (2020-12-20)
### Added
* add Lex::create_with()
* add create_with to features
* add sorted_opt_ary to features

### Removed
* delete Lex::create_from()
* delete from features: sorted_opt_ary, create_with

## [0.1.13] (2020-12-18)
### Added
* add to features: stop_at_mm, stop_at_free, no_std

### Removed
* remove from features: std

## [0.1.12] (2020-12-09)
### Changed
* impl std::error::Error for OptParseErrors

## [0.1.11] (2020-12-06)
### Changed
* import optpaerr-5("0.1.6" (2020-12-05))
* README.txt is added and edited
* replace crate combination("0.1.2") to itertools("0.9") in xtask

## [0.1.10] (2020-12-04)
### Added
* add error message with file name to xtask::update_file()

### Changed
* modify some examples

### Fixed
* clippy: xtask/src/gen_features_combination.rs
* clippy: examples/curl.rs

## [0.1.9] (2020-12-02)
### Added
* add fn parse_simple_gnu_style()

### Changed
* change examples new style
* refactoring source code

### Fixed
* bug: examples/curl.rs and xtask

## [0.1.8] (2020-11-28)
### Added
* add gen_features_combination to xtask
* add optnum_u16 to \[features]
* add bench-one.rs

### Changed
* change to public: pub struct CmdOptConf, on examples/curl.rs

## [0.1.7] (2020-11-25)
### Added
* add xtask for example curl
* add example curl

### Changed
* change Vec<&str> to &\[&str] at the param type of tokens_from()
* refactoring xtask's gen_src_example_curl_cmd

### Removed
* remove scripts/gen-parser-curl.pl

## [0.1.6] (2020-11-18)
### Added
* add README.md, COPYING, LICENSE-APACHE, LICENSE-MIT
* add no_std
* add example gnu-cat

### Changed
* rename repo: optpa-util to optpa-util-1

## [0.1.5] (2020-10-29)
### Added
* add NameVal.name()

## [0.1.4] (2020-10-05)
### Added
* add option_argument to features
* add example/ffmpeg
* add example/bsd-sed
* add examples

### Fixed
* argument features

## [0.1.3] (2020-09-20)
### Changed
* refactoring
* change 'is_long' to 'was_long'

## [0.1.2] (2020-09-17)
### Changed
* fork optpa-util
* a lot of things

## [0.1.1] (2018-05-22)
### Changed
* rename optpa_util to optpa-util

### Fixed
* dependencies

## [0.1.0] (2017-11-06)
* first commit

[Unreleased]: https://github.com/aki-akaguma/flood-tide/compare/v0.2.9..HEAD
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
