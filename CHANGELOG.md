TBD
===
Unreleased changes. Release notes have not yet been written.

0.2.2 (2021-05-09)
=====

* update depends: regex(1.5.4)

0.2.1 (2021-04-23)
=====

* add fn Opt::lon_or_sho()

0.2.0 (2021-04-03)
=====

* add trait SubCommand
* rename OPErr to OpErr
* remove unnecessary trailing semicolon
* add simple_gnu_style_subc into features
* add fn parse_simple_gnu_style_subcmd()
* update depends

* bug fix: if the scann has subcmd, then it should stop at free.
* bug fix: can not compile benches

0.1.21 (2021-02-05)
=====

* modify README.md

0.1.20 (2021-01-31)
=====

* add impl HelpVersion for OptParseError
* add help_message() and version_message() to OptParseError
* add trait HelpVersion

0.1.19 (2021-01-09)
=====

* add "dox" into features
* bug fix README.md

0.1.18 (2021-01-03)
=====

* update rustc 1.49.0 (e1884a8e3 2020-12-29)

0.1.17 (2020-12-25)
=====

* fix README.md

0.1.16 (2020-12-25)
=====

* publish to crates.io

0.1.15 (2020-12-25)
=====

* rename package flood-tide from optpa-util-5

0.1.14 (2020-12-20)
=====

* add Lex::create_with()
* delete Lex::create_from()
* delete from features: sorted_opt_ary, create_with
* add create_with to features
* add sorted_opt_ary to features

0.1.13 (2020-12-18)
=====

* add to features: stop_at_mm, stop_at_free, no_std
* del from features: std

0.1.12 (2020-12-09)
=====

* impl std::error::Error for OptParseErrors

0.1.11 (2020-12-06)
=====

* import optpaerr-5("0.1.6" (2020-12-05))
* README.txt is added and edited
* replace crate combination("0.1.2") to itertools("0.9") in xtask

0.1.10 (2020-12-04)
=====

* modify some examples
* add error message with file name to xtask::update_file()
* fix clippy: xtask/src/gen_features_combination.rs
* fix clippy: examples/curl.rs

0.1.9 (2020-12-02)
=====

* change examples new style
* add fn parse_simple_gnu_style()
* bugfix: examples/curl.rs and xtask
* refactoring source code

0.1.8 (2020-11-28)
=====

* add gen_features_combination to xtask
* add optnum_u16 to \[features]
* add bench-one.rs
* change to public: pub struct CmdOptConf, on examples/curl.rs

0.1.7 (2020-11-25)
=====

* change Vec<&str> to &\[&str] at the param type of tokens_from()
* remove scripts/gen-parser-curl.pl
* refactoring xtask's gen_src_example_curl_cmd
* add xtask for example curl
* add example curl

0.1.6 (2020-11-18)
=====

* add README.md, COPYING, LICENSE-APACHE, LICENSE-MIT
* add no_std
* add example gnu-cat
* rename repo: optpa-util to optpa-util-1

0.1.5 (2020-10-29)
=====

* add NameVal.name()

0.1.4 (2020-10-05)
=====

* fix: argument features
* add option_argument to features
* add example/ffmpeg
* add example/bsd-sed
* add examples

0.1.3 (2020-09-20)
=====

* refactoring
* change 'is_long' to 'was_long'

0.1.2 (2020-09-17)
=====

* fork optpa-util
* a lot of things

0.1.1 (2018-05-22)
=====

* rename optpa_util to optpa-util
* fix dependencies

0.1.0 (2017-11-06)
=====
first commit
