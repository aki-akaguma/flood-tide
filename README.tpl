# {{crate}}

{{readme}}

## Benchmarks

The comparing performance and .text size.

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
- compile by rustc 1.56.1 (59eed8a2a 2021-11-01)
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
