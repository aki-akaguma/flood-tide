//! Parse tables checking utilities for test.
//!
//!

#[cfg(feature = "no_std")]
use core::cmp::Ordering;
#[cfg(not(feature = "no_std"))]
use std::cmp::Ordering;

use super::Opt;

/// Check only sorted opt ary table.
pub fn check_sorted_opt_ary_with(opt_ary: &[Opt]) -> bool {
    let mut target = opt_ary.to_vec();
    target.sort_by(|&a, &b| match a.lon.cmp(b.lon) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => match a.sho.cmp(&b.sho) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => a.num.cmp(&b.num),
        },
    });
    target == opt_ary
}

/// Check only sorted sho idx ary table.
pub fn check_sorted_sho_idx_ary_with(sho_idx_ary: &[(u8, usize)]) -> bool {
    let mut target = sho_idx_ary.to_vec();
    target.sort_by(|&a, &b| match a.0.cmp(&b.0) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => a.1.cmp(&b.1),
    });
    target == sho_idx_ary
}

/// Check sorted opt ary table and sorted sho idx ary table.
/// - opt_ary:
///     must be sorted by *long* keyword. ex) "barn"
/// - sho_idx_ary:
///     must be sorted *short* keyword. ex) b'b'
/// - return:
///     - true:  ok
///     - false: illegal tables
/// # Examples
/// ```
/// #[cfg(feature = "option_argument")]
/// {
///     use flood_tide::check;
///     use flood_tide::{Arg, Lex, Opt, OptNum};
///
///     #[rustfmt::skip]
///     #[repr(u8)]
///     #[derive(Debug, PartialEq)]
///     enum CmdOP { A = 1, Barn, Eat, };
///     impl CmdOP { pub const fn to(self) -> OptNum { self as OptNum } }
///    
///     #[rustfmt::skip]
///     let opt_ary = [
///         Opt { sho: b'a', lon: "",     has: Arg::No,  num: CmdOP::A.to(), },
///         Opt { sho: b'b', lon: "barn", has: Arg::No,  num: CmdOP::Barn.to(), },
///         Opt { sho: 0u8,  lon: "eat",  has: Arg::Yes, num: CmdOP::Eat.to(), },
///     ];
///     #[rustfmt::skip]
///     let opt_ary_sho_idx = [(b'a',0),(b'b',1)];
///     assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
///         &opt_ary,
///         &opt_ary_sho_idx
///     ));
/// }
/// ```
pub fn check_sorted_opt_ary_and_sho_idx_ary_with(
    opt_ary: &[Opt],
    sho_idx_ary: &[(u8, usize)],
) -> bool {
    let mut prev_sho: u8 = 0u8;
    for sho in sho_idx_ary {
        if (prev_sho > 0 && prev_sho >= sho.0) || opt_ary[sho.1].sho != sho.0 {
            return false;
        }
        prev_sho = sho.0;
    }
    //
    let mut prev_lon: &str = "";
    for o in opt_ary {
        if prev_lon > o.lon || (!prev_lon.is_empty() && prev_lon == o.lon) {
            return false;
        }
        prev_lon = o.lon;
    }
    //
    true
}
