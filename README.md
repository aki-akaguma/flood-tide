# flood-tide

*flood-tide* is command line flag and option parse utilities

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
- minimum support: rustc 1.41.1 (f3e1a954d 2020-02-24)

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

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| cmp_null_void      |    1.754 kc |  323 kib |    0.000 kc |    0 kib |
| **cmp_flood_tide** |    6.654 kc |  377 kib |    4.900 kc |   54 kib |
| cmp_gumdrop        |   14.244 kc |  477 kib |   12.490 kc |  153 kib |
| cmp_pure_rust      |   16.439 kc |  389 kib |   14.685 kc |   66 kib |
| cmp_argh           |   26.279 kc |  409 kib |   24.525 kc |   85 kib |
| cmp_pico_args      |  156.589 kc |  421 kib |  154.835 kc |   98 kib |
| cmp_rustop         |  439.899 kc |  498 kib |  438.145 kc |  175 kib |
| cmp_clap           |  562.743 kc |  942 kib |  560.989 kc |  618 kib |
| cmp_structopt      |  676.121 kc | 1023 kib |  674.367 kc |  700 kib |
| cmp_getopts        |  697.585 kc |  412 kib |  695.831 kc |   89 kib |
| cmp_commander      |  762.846 kc |  424 kib |  761.092 kc |  100 kib |
| cmp_lapp           | 1109.102 kc |  464 kib | 1107.348 kc |  140 kib |
| cmp_args           | 2064.578 kc |  464 kib | 2062.824 kc |  140 kib |
| cmp_app            | 2379.029 kc |  714 kib | 2377.275 kc |  390 kib |
| cmp_docopt         | 5714.188 kc | 1694 kib | 5712.434 kc | 1370 kib |

- `us` is micro seconds
- `.text` is elf .text section size
- `Δ`(delta) is the difference from cmp_null_void
- `cmp_null_void` is non parser, support only `--help` and `--version`
- `cmp_pure_rust` is newly written with sting match
- compile by rustc 1.49.0 (e1884a8e3 2020-12-29)
- bench on intel Q6600 @ 2.40GHz
- refer [comparison of various parsers](https://github.com/aki-akaguma/cmp_cmdopts_parsing)
