# flood-tide

*flood-tide* is command line flag and option parse utilities

## Features

- `no_std` and `std` are supported.
- flags, options, subcommand and free arguments
- short flags and options (like `-a`)
- long flags and options (like `--long`)
- combined short flags (like `-abc` :=: `-a` `-b` `-c`)
- single long options (like `-long`)
- abbreviate long options (like `--abbr` :=: `--abbreviate`)
- single error or multiple error
- only UTF-8 arguments
- it can be used optimally by a compile switch with many features.

## Todos

- [x] multiple error
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

in examples directory.

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

|                          |       `base raw`       |        `delta`         |
|                          |-------------|----------|-------------|----------|
|          `name`          |   `bench`   | `.text`  |   `bench`   | `.text`  |
|--------------------------|-------------|----------|-------------|----------|
| cmp_null_void            |    1.750 kc |  318 kib |    0.000 kc |    0 kib |
| cmp_flood-tide           |    6.754 kc |  372 kib |    5.004 kc |   54 kib |
| cmp_gumdrop              |   14.251 kc |  474 kib |   12.500 kc |  156 kib |
| cmp_argh                 |   26.205 kc |  404 kib |   24.455 kc |   85 kib |
| cmp_pico_args            |  156.760 kc |  413 kib |  155.010 kc |   94 kib |
| cmp_rustop               |  429.286 kc |  495 kib |  427.536 kc |  177 kib |
| cmp_clap                 |  572.439 kc |  934 kib |  570.688 kc |  616 kib |
| cmp_getopts              |  688.350 kc |  408 kib |  686.600 kc |   90 kib |
| cmp_structopt            |  714.529 kc | 1015 kib |  712.778 kc |  697 kib |
| cmp_commander            |  749.754 kc |  421 kib |  748.004 kc |  102 kib |
| cmp_lapp                 | 1130.293 kc |  461 kib | 1128.543 kc |  142 kib |
| cmp_args                 | 2070.766 kc |  459 kib | 2069.016 kc |  140 kib |
| cmp_app                  | 2394.380 kc |  707 kib | 2392.630 kc |  388 kib |
| cmp_docopt               | 5715.680 kc | 1690 kib | 5713.930 kc | 1371 kib |

- `us` is micro seconds
- `.text` is elf .text section size
- `delta` is the difference from cmp_null_void
- `cmp_null_void` is non parser, support only `--help` and `--version`
- `cmp_pure_rust` is newly written with sting match
- refer [comparison of various parsers](https://github)
