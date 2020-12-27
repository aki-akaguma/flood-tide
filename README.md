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
| cmp_null_void      |    1.795 kc |  318 kib |    0.000 kc |    0 kib |
| **cmp_flood_tide** |    7.030 kc |  372 kib |    5.234 kc |   54 kib |
| cmp_gumdrop        |   14.357 kc |  474 kib |   12.562 kc |  156 kib |
| cmp_pure_rust      |   17.833 kc |  385 kib |   16.038 kc |   67 kib |
| cmp_argh           |   26.372 kc |  404 kib |   24.577 kc |   85 kib |
| cmp_pico_args      |  160.434 kc |  413 kib |  158.639 kc |   94 kib |
| cmp_rustop         |  437.199 kc |  495 kib |  435.404 kc |  177 kib |
| cmp_clap           |  589.117 kc |  934 kib |  587.322 kc |  616 kib |
| cmp_getopts        |  683.997 kc |  408 kib |  682.202 kc |   90 kib |
| cmp_structopt      |  723.818 kc | 1015 kib |  722.023 kc |  697 kib |
| cmp_commander      |  752.331 kc |  421 kib |  750.535 kc |  102 kib |
| cmp_lapp           | 1123.426 kc |  461 kib | 1121.631 kc |  142 kib |
| cmp_args           | 2097.303 kc |  459 kib | 2095.508 kc |  140 kib |
| cmp_app            | 2378.484 kc |  707 kib | 2376.689 kc |  388 kib |
| cmp_docopt         | 5849.126 kc | 1690 kib | 5847.331 kc | 1371 kib |

- `us` is micro seconds
- `.text` is elf .text section size
- `Δ`(delta) is the difference from cmp_null_void
- `cmp_null_void` is non parser, support only `--help` and `--version`
- `cmp_pure_rust` is newly written with sting match
- refer [comparison of various parsers](https://github.com/aki-akaguma/cmp_cmdopts_parsing)
