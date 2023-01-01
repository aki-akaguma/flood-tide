# flood-tide

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
| cmp_null_void      |    1.323 kc |  357 kib |    0.000 kc |    0 kib |
| **cmp_flood_tide** |    5.839 kc |  398 kib |    4.516 kc |   41 kib |
| cmp_gumdrop        |   11.343 kc |  479 kib |   10.020 kc |  122 kib |
| cmp_pure_rust      |   12.035 kc |  418 kib |   10.712 kc |   61 kib |
| cmp_argh           |   23.767 kc |  445 kib |   22.445 kc |   87 kib |
| cmp_pico_args      |   41.555 kc |  437 kib |   40.232 kc |   79 kib |
| cmp_rustop         |  405.701 kc |  520 kib |  404.378 kc |  162 kib |
| cmp_clap           |  476.395 kc | 1031 kib |  475.072 kc |  673 kib |
| cmp_getopts        |  668.494 kc |  445 kib |  667.171 kc |   88 kib |
| cmp_structopt      |  672.514 kc | 1020 kib |  671.191 kc |  662 kib |
| cmp_commander      |  728.809 kc |  464 kib |  727.486 kc |  107 kib |
| cmp_clap3          |  739.796 kc | 1000 kib |  738.473 kc |  642 kib |
| cmp_lapp           | 1110.026 kc |  495 kib | 1108.704 kc |  137 kib |
| cmp_args           | 2122.128 kc |  496 kib | 2120.805 kc |  138 kib |
| cmp_app            | 2406.356 kc |  752 kib | 2405.033 kc |  394 kib |

- compile by rustc 1.56.1 (59eed8a2a 2021-11-01)

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| cmp_null_void      |    1.213 kc |  342 kib |    0.000 kc |    0 kib |
| **cmp_flood_tide** |    5.439 kc |  386 kib |    4.226 kc |   43 kib |
| cmp_gumdrop        |   10.686 kc |  456 kib |    9.473 kc |  114 kib |
| cmp_pure_rust      |   11.052 kc |  390 kib |    9.839 kc |   47 kib |
| cmp_argh           |   23.644 kc |  429 kib |   22.431 kc |   86 kib |
| cmp_pico_args      |  152.401 kc |  457 kib |  151.188 kc |  114 kib |
| cmp_rustop         |  429.309 kc |  497 kib |  428.096 kc |  155 kib |
| cmp_clap           |  488.611 kc |  925 kib |  487.399 kc |  583 kib |
| cmp_getopts        |  695.832 kc |  428 kib |  694.620 kc |   85 kib |
| cmp_structopt      |  697.229 kc | 1083 kib |  696.016 kc |  741 kib |
| cmp_commander      |  712.974 kc |  437 kib |  711.762 kc |   94 kib |
| cmp_lapp           | 1113.328 kc |  476 kib | 1112.115 kc |  133 kib |
| cmp_args           | 2017.061 kc |  489 kib | 2015.848 kc |  146 kib |
| cmp_app            | 2348.416 kc |  720 kib | 2347.203 kc |  377 kib |

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
