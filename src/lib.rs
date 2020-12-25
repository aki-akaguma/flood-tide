//! Command line flag and option parse utilities.

#![cfg_attr(feature = "no_std", no_std)]
#![allow(unused_labels)]

#[cfg(feature = "no_std")]
#[macro_use]
extern crate alloc;

#[cfg(feature = "no_std")]
use alloc::string::{String, ToString};
#[cfg(feature = "no_std")]
use alloc::vec::Vec;

pub mod check;
pub mod err;
pub use err::OptParseError;

#[cfg(not(feature = "single_error"))]
pub use err::OptParseErrors;

/// Option parse error type
#[cfg(not(feature = "single_error"))]
pub type OPErr = OptParseErrors;

/// Option parse error type
#[cfg(feature = "single_error")]
pub type OPErr = OptParseError;

/// Option number type
#[cfg(feature = "optnum_u16")]
pub type OptNum = u16;

/// Option number type
#[cfg(not(feature = "optnum_u16"))]
pub type OptNum = u8;

pub use err::OptParseErrorKind;

/// Parse simple gnu style.
///
///
#[cfg(feature = "stop_at_mm")]
pub fn parse_simple_gnu_style<'a, T, F>(
    conf: &mut T,
    opt_ary: &'a [Opt],
    sho_idx_ary: &'a [(u8, usize)],
    args: &'a [&'a str],
    parse_match: F,
) -> (Option<Vec<String>>, Result<(), OPErr>)
where
    F: Fn(&mut T, &NameVal<'_>) -> Result<(), OptParseError>,
{
    let lex = Lex::create_with(opt_ary, sho_idx_ary);
    let tokens = match lex.tokens_from(&args) {
        Ok(t) => t,
        Err(errs) => {
            return (None, Err(errs));
        }
    };
    //
    #[cfg(not(feature = "single_error"))]
    let mut errs = OptParseErrors::new();
    //
    for nv in tokens.namevals.iter() {
        match parse_match(conf, &nv) {
            Ok(_) => {}
            Err(err) => {
                #[cfg(feature = "single_error")]
                return (None, Err(err));
                #[cfg(not(feature = "single_error"))]
                errs.push(err);
            }
        }
    }
    //
    let mut v: Vec<String> = Vec::new();
    v.extend(tokens.free.iter().map(|&s| s.to_string()));
    //
    #[cfg(feature = "single_error")]
    return (Some(v), Ok(()));
    #[cfg(not(feature = "single_error"))]
    return (Some(v), Err(errs));
}

/// Option argument
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arg {
    No = 0,
    Yes,
    Maybe,
}

/// Record type of opt ary table
///
/// # Examples
/// ```
/// #[cfg(feature = "option_argument")]
/// {
///     use flood_tide::Arg;
///     use flood_tide::Lex;
///     use flood_tide::Opt;
///     use flood_tide::OptNum;
///    
///     #[rustfmt::skip]
///     #[repr(u8)]
///     #[derive(Debug, PartialEq)]
///     enum CmdOP { A = 1, Barn, Eat, };
///     impl CmdOP { pub const fn to(self) -> OptNum { self as OptNum } }
///    
///     #[rustfmt::skip]
///     const OPT_ARY: [Opt;3] = [
///         Opt { sho: b'a', lon: "",     has: Arg::No,  num: CmdOP::A.to(), },
///         Opt { sho: b'b', lon: "barn", has: Arg::No,  num: CmdOP::Barn.to(), },
///         Opt { sho: 0u8,  lon: "eat",  has: Arg::Yes, num: CmdOP::Eat.to(), },
///     ];
/// }
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Opt<'a> {
    /// short name
    pub sho: u8,
    /// long name
    pub lon: &'a str,
    /// has arg / option argument
    #[cfg(feature = "option_argument")]
    pub has: Arg,
    /// uniq number
    pub num: OptNum,
}

/// Entity as the result of lex
#[derive(Debug)]
pub struct NameVal<'a> {
    pub opt: &'a Opt<'a>,
    #[cfg(feature = "option_argument")]
    pub val: Option<&'a str>,
    #[cfg(feature = "was_long")]
    pub was_long: bool,
}

impl<'a> NameVal<'a> {
    /// long name or short name
    ///
    /// At the compiling with feature = "was_long",
    /// this return a result according to a command line keyword
    /// Otherwise this return long name or short name.
    pub fn name(&self) -> String {
        #[cfg(feature = "was_long")]
        let b = self.was_long;
        #[cfg(not(feature = "was_long"))]
        let b = !self.opt.lon.is_empty();
        //
        if b {
            self.opt.lon.to_string()
        } else {
            String::from_utf8_lossy(&[self.opt.sho]).to_string()
        }
    }
}

/// Tokens as the result of lex
#[derive(Debug)]
pub struct Tokens<'a> {
    pub namevals: Vec<NameVal<'a>>,
    #[cfg(feature = "stop_at_mm")]
    pub double_m: bool,
    #[cfg(feature = "subcommand")]
    pub subcmd: Option<&'a str>,
    pub free: Vec<&'a str>,
}

/// Lexical analyzer
///
/// this is analyzing command line arguments, returning tokens.
///
/// # Examples
/// ```
/// #[cfg(not(feature = "long_only"))]
/// #[cfg(feature = "option_argument")]
/// {
///     use flood_tide::{Arg, Lex, Opt, OptNum};
///     
///     #[rustfmt::skip]
///     let args = ["-ab", "--barn", "--eat", "jum"];
///     
///     #[rustfmt::skip]
///     #[repr(u8)]
///     #[derive(Debug, PartialEq)]
///     enum CmdOP { A = 1, Barn, Eat, };
///     impl CmdOP { pub const fn to(self) -> OptNum { self as OptNum } }
///      
///     #[rustfmt::skip]
///     const OPT_ARY: [Opt;3] = [
///         Opt { sho: b'a', lon: "",     has: Arg::No,  num: CmdOP::A.to(), },
///         Opt { sho: b'b', lon: "barn", has: Arg::No,  num: CmdOP::Barn.to(), },
///         Opt { sho: 0u8,  lon: "eat",  has: Arg::Yes, num: CmdOP::Eat.to(), },
///     ];
///     #[rustfmt::skip]
///     const OPT_ARY_SHO_IDX: [(u8,usize);2] = [(b'a',0),(b'b',1)];
///    
///     let lex = Lex::create_with(&OPT_ARY, &OPT_ARY_SHO_IDX);
///     let tokens = match lex.tokens_from(&args) {
///         Ok(t) => t,
///         Err(e) => unreachable!(),
///     };
/// }
/// ```
#[derive(Debug)]
pub struct Lex<'a> {
    opts: &'a [Opt<'a>],
    sho_idx: &'a [(u8, usize)],
    #[cfg(feature = "subcommand")]
    subcmds: &'a [&'a str],
}

impl<'a> Lex<'a> {
    /// create lexical analyzer
    pub fn create_with(opt_ary: &'a [Opt], sho_idx_ary: &'a [(u8, usize)]) -> Lex<'a> {
        Lex {
            opts: opt_ary,
            sho_idx: sho_idx_ary,
            #[cfg(feature = "subcommand")]
            subcmds: &[],
        }
    }
    /// setup subcommand ary
    #[cfg(feature = "subcommand")]
    #[inline]
    pub fn subcmd(mut self, subcmd_ary: &'a [&'a str]) -> Self {
        self.subcmds = subcmd_ary;
        self
    }
    /// analyze and return tokens
    pub fn tokens_from(&'a self, args: &'a [&'a str]) -> Result<Tokens<'a>, OPErr> {
        #[cfg(not(feature = "single_error"))]
        let mut v_errs = OPErr::new();
        let mut v_free: Vec<&str> = Vec::new();
        let mut v_namevals: Vec<NameVal> = Vec::new();
        //
        let mut cursor = args.iter();
        'itr_cursor: while let Some(cur) = cursor.next() {
            #[cfg(feature = "stop_at_mm")]
            {
                if *cur == "--" {
                    // stop on
                    v_free.push(cur);
                    v_free.extend(cursor);
                    break 'itr_cursor;
                }
            }
            let f_single = if !cur.starts_with('-') {
                // free
                v_free.push(cur);
                #[cfg(feature = "stop_at_free")]
                {
                    v_free.extend(cursor);
                    break 'itr_cursor;
                }
                #[cfg(not(feature = "stop_at_free"))]
                false
            } else {
                true
            };
            #[cfg(not(feature = "long_only"))]
            let f_single = if f_single && cur.starts_with("--") {
                // option: long name
                match self.parse_long_name(&mut cursor, &cur[2..]) {
                    Ok(nv) => v_namevals.push(nv),
                    Err(err) => {
                        #[cfg(feature = "single_error")]
                        return Err(err);
                        #[cfg(not(feature = "single_error"))]
                        v_errs.push(err)
                    }
                };
                false
            } else {
                f_single
            };
            if f_single {
                // option: short name or long only
                //
                #[cfg(not(feature = "long_only"))]
                {
                    if let Err(errs) =
                        self.parse_short_name(&mut cursor, &cur[1..], &mut v_namevals)
                    {
                        #[cfg(feature = "single_error")]
                        return Err(errs);
                        #[cfg(not(feature = "single_error"))]
                        v_errs.append(errs);
                    }
                }
                #[cfg(feature = "long_only")]
                {
                    if let Err(errs) = self.parse_long_only(&mut cursor, &cur, &mut v_namevals) {
                        #[cfg(feature = "single_error")]
                        return Err(errs);
                        #[cfg(not(feature = "single_error"))]
                        v_errs.append(errs);
                    }
                }
            }
        }
        //
        #[cfg(not(feature = "single_error"))]
        {
            if !v_errs.is_empty() {
                return Err(v_errs);
            }
        }
        //
        #[cfg(feature = "stop_at_mm")]
        let is_stop_at_double_m = {
            if !v_free.is_empty() && v_free[0] == "--" {
                v_free.remove(0);
                true
            } else {
                false
            }
        };
        //
        #[cfg(feature = "subcommand")]
        {
            #[cfg(feature = "stop_at_mm")]
            let b = !self.subcmds.is_empty() && !is_stop_at_double_m;
            #[cfg(not(feature = "stop_at_mm"))]
            let b = !self.subcmds.is_empty();
            let v_cmd = if b {
                match self.parse_subcmd(&v_free) {
                    Ok((opt, remove_1st)) => {
                        if remove_1st {
                            v_free.remove(0);
                        }
                        opt
                    }
                    Err(err) => {
                        #[cfg(feature = "single_error")]
                        return Err(err);
                        #[cfg(not(feature = "single_error"))]
                        {
                            v_errs.push(err);
                            return Err(v_errs);
                        }
                    }
                }
            } else {
                None
            };
            Ok(Tokens {
                namevals: v_namevals,
                free: v_free,
                #[cfg(feature = "stop_at_mm")]
                double_m: is_stop_at_double_m,
                subcmd: v_cmd,
            })
        }
        #[cfg(not(feature = "subcommand"))]
        {
            Ok(Tokens {
                namevals: v_namevals,
                free: v_free,
                #[cfg(feature = "stop_at_mm")]
                double_m: is_stop_at_double_m,
            })
        }
    }
}

impl<'a> Lex<'a> {
    // parse
    //
    #[cfg(feature = "abbreviate")]
    fn find_abbreviate(&'a self, name: &'a str) -> Result<&Opt<'a>, OptParseError> {
        #[rustfmt::skip]
        let ambiguous: Vec<&Opt<'a>> = self.opts.iter()
            .filter(|&o| o.lon.starts_with(name)).collect();
        match ambiguous.len() {
            1 => Ok(ambiguous[0]),
            0 => mkerr_invalid_option(name),
            _ => mkerr_ambiguous_option(name, &ambiguous),
        }
    }
    //
    #[cfg(feature = "subcommand")]
    #[cfg(feature = "abbreviate")]
    fn find_abbreviate_subcmd<'b>(&'a self, name: &'b str) -> Result<&'a str, OptParseError> {
        #[rustfmt::skip]
        let ambiguous: Vec<&'a str> = self.subcmds.iter()
            .filter(|&o| o.starts_with(name)).copied().collect();
        match ambiguous.len() {
            1 => Ok(ambiguous[0]),
            0 => mkerr_invalid_subcommand(name),
            _ => mkerr_ambiguous_subcommand(name, &ambiguous),
        }
    }
    //
    #[cfg(feature = "subcommand")]
    #[cfg(not(feature = "abbreviate"))]
    fn find_match_subcmd<'b>(&'a self, name: &'b str) -> Result<&'a str, OptParseError> {
        #[rustfmt::skip]
        let ambiguous: Vec<&'a str> = self.subcmds.iter()
            .filter(|&o| o == &name).copied().collect();
        match ambiguous.len() {
            1 => Ok(ambiguous[0]),
            _ => mkerr_invalid_subcommand(name),
        }
    }
    //
    fn parse_long_name(
        &'a self,
        _cursor: &mut dyn Iterator<Item = &&'a str>,
        tail: &'a str,
    ) -> Result<NameVal<'a>, OptParseError> {
        #[cfg(feature = "option_argument")]
        let (name, val) = {
            let eq_idx = tail.find('=');
            match eq_idx {
                Some(usz) => (&tail[0..usz], Some(&tail[usz + 1..])),
                None => (tail, None),
            }
        };
        #[cfg(not(feature = "option_argument"))]
        let name = tail;
        //
        let v_opt = {
            let found = self.opts.binary_search_by_key(&name, |&o| o.lon);
            match found {
                Ok(idx) => &self.opts[idx],
                _ => {
                    #[cfg(feature = "abbreviate")]
                    match self.find_abbreviate(name) {
                        Ok(o) => o,
                        Err(err) => return Err(err),
                    }
                    #[cfg(not(feature = "abbreviate"))]
                    return mkerr_invalid_option(name);
                }
            }
        };
        //
        #[cfg(feature = "option_argument")]
        let val2 = match v_opt.has {
            Arg::No => {
                if let Some(v) = val {
                    return mkerr_unexpected_option_argument(name, v);
                }
                val
            }
            Arg::Maybe => {
                if val.is_none() {
                    Some(&tail[0..0])
                } else {
                    val
                }
            }
            Arg::Yes => {
                if val.is_none() {
                    if let Some(&cur_val) = _cursor.next() {
                        Some(cur_val)
                    } else {
                        return mkerr_missing_option_argument(name);
                    }
                } else {
                    val
                }
            }
        };
        //
        Ok(NameVal {
            opt: v_opt,
            #[cfg(feature = "option_argument")]
            val: val2,
            #[cfg(feature = "was_long")]
            was_long: true,
        })
    }
    //
    fn parse_short_name(
        &'a self,
        _cursor: &mut dyn Iterator<Item = &&'a str>,
        tail: &'a str,
        namevals: &mut Vec<NameVal<'a>>,
    ) -> Result<(), OPErr> {
        #[cfg(not(feature = "single_error"))]
        let mut errs = OPErr::new();
        let tail_len = tail.len();
        '_ic_iter: for i in 0..tail_len {
            let c_name = &tail[i..=i];
            let b_name = c_name.as_bytes()[0];
            let v_opt = {
                let found = self.sho_idx.binary_search_by_key(&b_name, |&o| o.0);
                match found {
                    Ok(idx) => &self.opts[self.sho_idx[idx].1],
                    _ => {
                        #[cfg(feature = "single_error")]
                        return Err(OptParseError::invalid_option(c_name));
                        #[cfg(not(feature = "single_error"))]
                        {
                            errs.push(OptParseError::invalid_option(c_name));
                            continue '_ic_iter;
                        }
                    }
                }
            };
            #[cfg(feature = "option_argument")]
            let c_val = if v_opt.has == Arg::No {
                None
            } else if i < tail_len - 1 {
                let rest = &tail[i + 1..];
                namevals.push(NameVal {
                    opt: v_opt,
                    val: Some(rest),
                    #[cfg(feature = "was_long")]
                    was_long: false,
                });
                break '_ic_iter;
            } else if v_opt.has == Arg::Maybe {
                let rest = &tail[tail_len..tail_len];
                namevals.push(NameVal {
                    opt: v_opt,
                    val: Some(rest),
                    #[cfg(feature = "was_long")]
                    was_long: false,
                });
                break '_ic_iter;
            } else if let Some(&cur_val) = _cursor.next() {
                Some(cur_val)
            } else {
                #[cfg(feature = "single_error")]
                return Err(OptParseError::missing_option_argument(c_name));
                #[cfg(not(feature = "single_error"))]
                {
                    errs.push(OptParseError::missing_option_argument(c_name));
                    continue '_ic_iter;
                }
            };
            //
            namevals.push(NameVal {
                opt: v_opt,
                #[cfg(feature = "option_argument")]
                val: c_val,
                #[cfg(feature = "was_long")]
                was_long: false,
            });
        }
        //
        #[cfg(not(feature = "single_error"))]
        {
            if !errs.is_empty() {
                return Err(errs);
            }
        }
        //
        Ok(())
    }
    //
    #[cfg(feature = "long_only")]
    fn parse_long_only(
        &'a self,
        mut cursor: &mut dyn Iterator<Item = &'a &'a str>,
        cur: &'a str,
        mut namevals: &mut Vec<NameVal<'a>>,
    ) -> Result<(), OPErr> {
        if cur.len() == 2 {
            //  "-f"
            // short name
            let e = self.parse_short_name(&mut cursor, &cur[1..], &mut namevals);
            if let Err(errs) = e {
                #[cfg(not(feature = "single_error"))]
                let err = &errs.iter().as_slice()[0];
                #[cfg(feature = "single_error")]
                let err = &errs;
                if err.kind() == OptParseErrorKind::InvalidOption {
                    // long name
                    match self.parse_long_name(&mut cursor, &cur[1..]) {
                        Ok(nv) => {
                            namevals.push(nv);
                        }
                        Err(err) => {
                            #[cfg(feature = "single_error")]
                            return Err(err);
                            #[cfg(not(feature = "single_error"))]
                            {
                                let mut errs = OPErr::new();
                                errs.push(err);
                                return Err(errs);
                            }
                        }
                    };
                } else {
                    return Err(errs);
                }
            }
        } else {
            match self.parse_long_name(&mut cursor, &cur[1..]) {
                Ok(nv) => namevals.push(nv),
                Err(err) => {
                    #[cfg(feature = "single_error")]
                    return Err(err);
                    #[cfg(not(feature = "single_error"))]
                    {
                        let mut errs = OPErr::new();
                        errs.push(err);
                        return Err(errs);
                    }
                }
            }
        };
        //
        Ok(())
    }
    //
    #[cfg(feature = "subcommand")]
    fn parse_subcmd(&'a self, v_free: &[&str]) -> Result<(Option<&'a str>, bool), OptParseError> {
        let mut v_cmd: Option<&'a str> = None;
        let mut remove_1st = false;
        if !v_free.is_empty() {
            let free_1st = v_free[0];
            if free_1st != "--" && !free_1st.is_empty() {
                #[cfg(feature = "abbreviate")]
                match self.find_abbreviate_subcmd(free_1st) {
                    Ok(subcmd) => {
                        v_cmd = Some(subcmd);
                        remove_1st = true;
                    }
                    Err(err) => return Err(err),
                };
                #[cfg(not(feature = "abbreviate"))]
                match self.find_match_subcmd(free_1st) {
                    Ok(subcmd) => {
                        v_cmd = Some(subcmd);
                        remove_1st = true;
                    }
                    Err(err) => return Err(err),
                };
            }
        }
        Ok((v_cmd, remove_1st))
    }
}

#[inline]
fn mkerr_invalid_option<T>(name: &str) -> Result<T, OptParseError> {
    Err(OptParseError::invalid_option(name))
}

#[cfg(feature = "option_argument")]
#[inline]
fn mkerr_unexpected_option_argument<T>(name: &str, val: &str) -> Result<T, OptParseError> {
    Err(OptParseError::unexpected_option_argument(name, val))
}

#[cfg(feature = "option_argument")]
#[inline]
fn mkerr_missing_option_argument<T>(name: &str) -> Result<T, OptParseError> {
    Err(OptParseError::missing_option_argument(name))
}

#[cfg(feature = "subcommand")]
#[inline]
fn mkerr_invalid_subcommand<T>(name: &str) -> Result<T, OptParseError> {
    Err(OptParseError::invalid_subcommand(name))
}

#[cfg(feature = "abbreviate")]
fn mkerr_ambiguous_option<'a, 'b, T>(
    name: &'a str,
    ambiguous: &'b [&Opt<'a>],
) -> Result<T, OptParseError> {
    let mut hint = "possibilities:".to_string();
    for &a in ambiguous {
        let ss = format!(" '--{}'", a.lon);
        hint.push_str(ss.as_str());
    }
    Err(OptParseError::ambiguous_option(name, hint.as_str()))
}

#[cfg(all(feature = "abbreviate", feature = "subcommand"))]
fn mkerr_ambiguous_subcommand<'a, 'b, T>(
    name: &'a str,
    ambiguous: &'b [&'a str],
) -> Result<T, OptParseError> {
    let mut hint = "possibilities:".to_string();
    for &a in ambiguous {
        let ss = format!(" '{}'", a);
        hint.push_str(ss.as_str());
    }
    Err(OptParseError::ambiguous_subcommand(name, hint.as_str()))
}
