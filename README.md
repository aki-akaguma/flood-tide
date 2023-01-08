# flood-tide

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]

Command line flag and option parse utilities.

## Features

- `no_std` and `std` are supported.
- flags, options, subcommand and free arguments
- short flags and options (like `-a`)
- long flags and options (like `--long`)
- combined short flags (like `-abc` ::= `-a` `-b` `-c`)
- single long options (like `-long`)
- abbreviate long options (like `--abbr` ::= `--abbreviate`)
- single error or multiple errors
- only UTF-8 arguments
- it can be used optimally by a compile switch with many features.
- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

## Todos

- [x] multiple errors
- [x] `no_std`
- [ ] option suggestion (do you mean? '--abc')
- [ ] windows style (like `/a`)
- [ ] source code generator support tools
- [ ] more easy use

## Non-Supports

- procedural macro style
- traditional macro style
- non UTF-8 arguments, multibyte or wide charactor

## Examples

in [examples](https://github.com/aki-akaguma/flood-tide/tree/main/examples) directory.

- manual coding style: bsd-sed.rs, gnu-cat.rs
- single long options: ffmpeg.rs
- source code generating by xtask and parse_simple_gnu_style(): curl.rs

## Supports

- [flood-tide-gen](https://crates.io/crates/flood-tide-gen) - the generating *flood-tide* tables
- [aki-gsub](https://crates.io/crates/aki-gsub) - the sample used *flood-tide*

## Alternatives

This parser is *not* a new special idea. It's just comparing characters one by one.
Is there anything simpler than this?

- [clap](https://crates.io/crates/clap) - is the most popular and complete one
- [structopt](https://crates.io/crates/structopt) - clap parser that uses procedural macros
- [gumdrop](https://crates.io/crates/gumdrop) - a simple parser that uses procedural macros
- [argh](https://crates.io/crates/argh) - procedural macros
- [rustop](https://crates.io/crates/rustop) - traditional macro
- [pico-args](https://crates.io/crates/pico-args) - a simple use
- [getopts](https://crates.io/crates/getopts) - a simple use
- [docopt](https://crates.io/crates/docopt) - a simple use


## Benchmarks

The comparing performance and .text size.

- compiled by rustc 1.66.0 (69f9c33d7 2022-12-12)

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| cmp_null_void      |    1.327 kc |  316 kib |    0.000 kc |    0 kib |
| **cmp_flood_tide** |    6.315 kc |  356 kib |    4.988 kc |   40 kib |
| cmp_pure_rust      |    7.951 kc |  368 kib |    6.624 kc |   52 kib |
| cmp_gumdrop        |   11.346 kc |  432 kib |   10.019 kc |  116 kib |
| cmp_argh           |   20.851 kc |  385 kib |   19.524 kc |   69 kib |
| cmp_pico_args      |   39.187 kc |  393 kib |   37.860 kc |   77 kib |
| cmp_rustop         |  379.726 kc |  465 kib |  378.399 kc |  149 kib |
| cmp_clap           |  415.422 kc |  988 kib |  414.095 kc |  671 kib |
| cmp_clap3          |  495.219 kc |  840 kib |  493.893 kc |  524 kib |
| cmp_structopt      |  553.679 kc |  862 kib |  552.352 kc |  546 kib |
| cmp_getopts        |  637.986 kc |  395 kib |  636.659 kc |   78 kib |
| cmp_commander      |  665.407 kc |  412 kib |  664.080 kc |   95 kib |
| cmp_lapp           | 1115.093 kc |  451 kib | 1113.766 kc |  135 kib |
| cmp_args           | 2101.706 kc |  427 kib | 2100.379 kc |  110 kib |
| cmp_app            | 2192.245 kc |  630 kib | 2190.918 kc |  313 kib |


- compiled by rustc 1.57.0 (f1edd0429 2021-11-29)

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| cmp_null_void      |    1.405 kc |  316 kib |    0.000 kc |    0 kib |
| **cmp_flood_tide** |    5.603 kc |  356 kib |    4.197 kc |   40 kib |
| cmp_pure_rust      |    7.845 kc |  368 kib |    6.439 kc |   52 kib |
| cmp_gumdrop        |    8.737 kc |  432 kib |    7.332 kc |  116 kib |
| cmp_argh           |   23.114 kc |  385 kib |   21.708 kc |   69 kib |
| cmp_pico_args      |   41.325 kc |  393 kib |   39.920 kc |   77 kib |
| cmp_rustop         |  394.432 kc |  465 kib |  393.026 kc |  149 kib |
| cmp_clap           |  426.678 kc |  988 kib |  425.273 kc |  671 kib |
| cmp_clap3          |  495.857 kc |  840 kib |  494.452 kc |  524 kib |
| cmp_structopt      |  576.224 kc |  862 kib |  574.818 kc |  546 kib |
| cmp_getopts        |  657.353 kc |  395 kib |  655.948 kc |   78 kib |
| cmp_commander      |  673.761 kc |  412 kib |  672.356 kc |   95 kib |
| cmp_lapp           | 1089.452 kc |  451 kib | 1088.047 kc |  135 kib |
| cmp_args           | 2066.320 kc |  427 kib | 2064.915 kc |  110 kib |
| cmp_app            | 2171.903 kc |  630 kib | 2170.498 kc |  313 kib |

- `us` is micro seconds
- `.text` is elf .text section size
- `Δ`(delta) is the difference from cmp_null_void
- `cmp_null_void` is non parser, support only `--help` and `--version`
- `cmp_pure_rust` is newly written with sting match
- bench on intel Q6600 @ 2.40GHz
- refer [comparison of various parsers](https://github.com/aki-akaguma/cmp_cmdopts_parsing)

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/flood-tide/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/flood-tide.svg
[crate-link]: https://crates.io/crates/flood-tide
[docs-image]: https://docs.rs/flood-tide/badge.svg
[docs-link]: https://docs.rs/flood-tide/
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
