/*!
Command line flag and option parse utilities.

# Features

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

# Todos

- [x] multiple errors
- [x] `no_std`
- [ ] option suggestion (do you mean? '--abc')
- [ ] windows style (like `/a`)
- [ ] source code generator support tools
- [ ] more easy use

# Non-Supports

- procedural macro style
- traditional macro style
- non UTF-8 arguments, multibyte or wide charactor

# Examples

in [examples](https://github.com/aki-akaguma/flood-tide/tree/main/examples) directory.

- manual coding style: bsd-sed.rs, gnu-cat.rs
- single long options: ffmpeg.rs
- source code generating by xtask and parse_simple_gnu_style(): curl.rs

# Supports

- [flood-tide-gen](https://crates.io/crates/flood-tide-gen) - the generating *flood-tide* tables
- [aki-gsub](https://crates.io/crates/aki-gsub) - the sample used *flood-tide*

# Alternatives

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

*/
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

#[cfg(any(not(feature = "single_error"), feature = "dox"))]
pub use err::OptParseErrors;

/// Option parse error type
#[cfg(any(not(feature = "single_error"), feature = "dox"))]
pub type OpErr = OptParseErrors;

/// Option parse error type
#[cfg(feature = "single_error")]
pub type OpErr = OptParseError;

/// Option number type
#[cfg(feature = "optnum_u16")]
pub type OptNum = u16;

/// Option number type
#[cfg(any(not(feature = "optnum_u16"), feature = "dox"))]
pub type OptNum = u8;

pub use err::OptParseErrorKind;

/// check help and version of conf
pub trait HelpVersion {
    fn is_help(&self) -> bool;
    fn is_version(&self) -> bool;
}

/// setter subcmd of conf
pub trait SubCommand {
    fn set_subcmd(&mut self, subcmd: String);
}

/// Parse simple gnu style.
#[cfg(any(feature = "stop_at_mm", feature = "dox"))]
pub fn parse_simple_gnu_style<'a, T, F>(
    conf: &mut T,
    opt_ary: &'a [Opt],
    sho_idx_ary: &'a [(u8, usize)],
    args: &'a [&'a str],
    parse_match: F,
) -> (Option<Vec<String>>, Result<(), OpErr>)
where
    F: Fn(&mut T, &NameVal<'_>) -> Result<(), OptParseError>,
    T: HelpVersion,
{
    let lex = Lex::create_with(opt_ary, sho_idx_ary);
    let tokens = match lex.tokens_from(args) {
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
        match parse_match(conf, nv) {
            Ok(_) => {}
            Err(err) => {
                #[cfg(feature = "single_error")]
                return (None, Err(err));
                #[cfg(not(feature = "single_error"))]
                errs.push(err);
            }
        }
        if conf.is_help() || conf.is_version() {
            break;
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

/// Parse simple gnu style with sub command.
#[cfg(any(all(feature = "stop_at_mm", feature = "subcommand"), feature = "dox"))]
pub fn parse_simple_gnu_style_subcmd<'a, T, F>(
    conf: &mut T,
    opt_ary: &'a [Opt],
    sho_idx_ary: &'a [(u8, usize)],
    args: &'a [&'a str],
    parse_match: F,
    subcmds: &'a [&'a str],
) -> (Option<Vec<String>>, Result<(), OpErr>)
where
    F: Fn(&mut T, &NameVal<'_>) -> Result<(), OptParseError>,
    T: HelpVersion + SubCommand,
{
    let lex = Lex::create_with(opt_ary, sho_idx_ary).subcmd(subcmds);
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
        if conf.is_help() || conf.is_version() {
            break;
        }
    }
    //
    match tokens.subcmd {
        Some(s) => conf.set_subcmd(String::from(s)),
        None => {
            #[cfg(feature = "single_error")]
            return (None, Err(OptParseError::missing_subcommand("<command>")));
            #[cfg(not(feature = "single_error"))]
            errs.push(OptParseError::missing_subcommand("<command>"));
        }
    };
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Opt<'a> {
    /// short name
    pub sho: u8,
    /// long name
    pub lon: &'a str,
    /// has arg / option argument
    #[cfg(any(feature = "option_argument", feature = "dox"))]
    pub has: Arg,
    /// uniq number
    pub num: OptNum,
}
impl Opt<'_> {
    /// long or short name
    pub fn lon_or_sho(&self) -> String {
        if !self.lon.is_empty() {
            self.lon.to_string()
        } else if self.sho != 0_u8 {
            let v = vec![self.sho];
            String::from_utf8_lossy(&v).to_string()
        } else {
            "".to_string()
        }
    }
}

/// Entity as the result of lex
#[derive(Debug)]
pub struct NameVal<'a> {
    pub opt: &'a Opt<'a>,
    #[cfg(any(feature = "option_argument", feature = "dox"))]
    pub val: Option<&'a str>,
    #[cfg(any(feature = "was_long", feature = "dox"))]
    pub was_long: bool,
}

impl NameVal<'_> {
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
    #[cfg(any(feature = "stop_at_mm", feature = "dox"))]
    pub double_m: bool,
    #[cfg(any(feature = "subcommand", feature = "dox"))]
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
    #[cfg(any(feature = "subcommand", feature = "dox"))]
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
    #[cfg(any(feature = "subcommand", feature = "dox"))]
    #[inline]
    pub fn subcmd(mut self, subcmd_ary: &'a [&'a str]) -> Self {
        self.subcmds = subcmd_ary;
        self
    }
    /// analyze and return tokens
    pub fn tokens_from(&'a self, args: &'a [&'a str]) -> Result<Tokens<'a>, OpErr> {
        #[cfg(not(feature = "single_error"))]
        let mut v_errs = OpErr::new();
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
    fn find_abbreviate(&'a self, name: &'a str) -> Result<&'a Opt<'a>, OptParseError> {
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
                    {
                        self.find_abbreviate(name)?
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
    ) -> Result<(), OpErr> {
        #[cfg(not(feature = "single_error"))]
        let mut errs = OpErr::new();
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
    ) -> Result<(), OpErr> {
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
                                let mut errs = OpErr::new();
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
                        let mut errs = OpErr::new();
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
fn mkerr_ambiguous_option<'a, T>(
    name: &'a str,
    ambiguous: &[&Opt<'a>],
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
