# {{crate}}

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

{{readme}}

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

[This crate's changelog here.](https://github.com/aki-akaguma/{{crate}}/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/{{crate}}.svg
[crate-link]: https://crates.io/crates/{{crate}}
[docs-image]: https://docs.rs/{{crate}}/badge.svg
[docs-link]: https://docs.rs/{{crate}}/
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-windows.yml
