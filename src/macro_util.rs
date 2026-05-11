//! Utilities for argparse macro.

use crate::Opt;
use core::cmp::Ordering;

#[cfg(feature = "no_std")]
use alloc::string::{String, ToString};

pub const fn str_cmp(a: &str, b: &str) -> Ordering {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let len = if a.len() < b.len() { a.len() } else { b.len() };
    let mut i = 0;
    while i < len {
        if a[i] < b[i] {
            return Ordering::Less;
        }
        if a[i] > b[i] {
            return Ordering::Greater;
        }
        i += 1;
    }
    if a.len() < b.len() {
        return Ordering::Less;
    }
    if a.len() > b.len() {
        return Ordering::Greater;
    }
    Ordering::Equal
}

pub const fn opt_cmp(a: &Opt, b: &Opt) -> Ordering {
    match str_cmp(a.lon, b.lon) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => {
            if a.sho < b.sho {
                Ordering::Less
            } else if a.sho > b.sho {
                Ordering::Greater
            } else {
                if a.num < b.num {
                    Ordering::Less
                } else if a.num > b.num {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        }
    }
}

pub const fn sort_opts<const N: usize>(mut opts: [Opt; N]) -> [Opt; N] {
    let mut i = 0;
    while i < N {
        let mut j = i + 1;
        while j < N {
            if let Ordering::Greater = opt_cmp(&opts[i], &opts[j]) {
                let tmp = opts[i];
                opts[i] = opts[j];
                opts[j] = tmp;
            }
            j += 1;
        }
        i += 1;
    }
    opts
}

pub const fn count_short_opts<const N: usize>(opts: &[Opt; N]) -> usize {
    let mut i = 0;
    let mut count = 0;
    while i < N {
        if opts[i].sho != 0 {
            count += 1;
        }
        i += 1;
    }
    count
}

pub const fn gen_sho_idx<const N: usize, const M: usize>(opts: &[Opt; N]) -> [(u8, usize); M] {
    let mut res = [(0, 0); M];
    let mut i = 0;
    let mut count = 0;
    while i < N {
        if opts[i].sho != 0 {
            res[count] = (opts[i].sho, i);
            count += 1;
        }
        i += 1;
    }
    // sort res by sho (u8)
    let mut i = 0;
    while i < M {
        let mut j = i + 1;
        while j < M {
            if res[i].0 > res[j].0 {
                let tmp = res[i];
                res[i] = res[j];
                res[j] = tmp;
            }
            j += 1;
        }
        i += 1;
    }
    res
}

/// Trait for setting field from option value.
pub trait ArgparseSet {
    fn argparse_set(&mut self, val: Option<&str>, name: &str) -> Result<(), crate::OptParseError>;
}

impl ArgparseSet for bool {
    fn argparse_set(&mut self, _val: Option<&str>, _name: &str) -> Result<(), crate::OptParseError> {
        *self = true;
        Ok(())
    }
}

impl ArgparseSet for String {
    fn argparse_set(&mut self, val: Option<&str>, _name: &str) -> Result<(), crate::OptParseError> {
        if let Some(s) = val {
            *self = s.to_string();
        }
        Ok(())
    }
}

impl ArgparseSet for Option<String> {
    fn argparse_set(&mut self, val: Option<&str>, _name: &str) -> Result<(), crate::OptParseError> {
        if let Some(s) = val {
            *self = Some(s.to_string());
        } else {
            *self = None;
        }
        Ok(())
    }
}

macro_rules! impl_argparse_set_parse {
    ($($t:ty),*) => {
        $(
            impl ArgparseSet for $t {
                fn argparse_set(&mut self, val: Option<&str>, name: &str) -> Result<(), crate::OptParseError> {
                    if let Some(s) = val {
                        match s.parse::<$t>() {
                            Ok(v) => {
                                *self = v;
                                Ok(())
                            }
                            Err(_) => {
                                #[cfg(any(feature = "option_argument", feature = "dox"))]
                                return Err(crate::OptParseError::invalid_option_argument(name, s));
                                #[cfg(not(any(feature = "option_argument", feature = "dox")))]
                                return Err(crate::OptParseError::invalid_option(name));
                            }
                        }
                    } else {
                        Ok(())
                    }
                }
            }
        )*
    };
}

impl_argparse_set_parse!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, usize, isize);
