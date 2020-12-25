//! Parse error module.
//!
//! # Examples
//! ```
//! use flood_tide::err::OptParseError;
//! let err = OptParseError::invalid_option("abc");
//! ```
//!
#[cfg(not(feature = "no_std"))]
use std::fmt::{Display, Error, Formatter};
#[cfg(not(feature = "no_std"))]
use std::slice::Iter;

#[cfg(feature = "no_std")]
use core::fmt::{Display, Error, Formatter};
#[cfg(feature = "no_std")]
use core::slice::Iter;

#[cfg(feature = "no_std")]
use alloc::string::{String, ToString};
#[cfg(feature = "no_std")]
use alloc::vec::Vec;

#[derive(Debug, PartialEq, Clone)]
pub enum OptParseErrorKind {
    InvalidOption,
    MissingOption,
    //
    #[cfg(feature = "option_argument")]
    InvalidOptionArgument,
    #[cfg(feature = "option_argument")]
    UnexpectedOptionArgument,
    #[cfg(feature = "option_argument")]
    MissingOptionArgument,
    //
    #[cfg(feature = "argument")]
    UnexpectedArgument,
    #[cfg(feature = "argument")]
    MissingArgument,
    //
    #[cfg(feature = "subcommand")]
    InvalidSubcommand,
    #[cfg(feature = "subcommand")]
    MissingSubcommand,
    //
    #[cfg(feature = "abbreviate")]
    AmbiguousOption,
    #[cfg(all(feature = "abbreviate", feature = "subcommand"))]
    AmbiguousSubcommand,
}

/// Single option parse error
#[derive(Debug, PartialEq)]
pub struct OptParseError {
    kind: OptParseErrorKind,
    desc1: String,
    desc2: Option<String>,
}

impl OptParseError {
    pub fn kind(&self) -> OptParseErrorKind {
        self.kind.clone()
    }
    pub fn desc1_str(&self) -> &str {
        self.desc1.as_str()
    }
}

impl OptParseError {
    #[inline(never)]
    fn new_p1(a_kind: OptParseErrorKind, a_desc1: &str) -> Self {
        Self {
            kind: a_kind,
            desc1: a_desc1.to_string(),
            desc2: None,
        }
    }
    #[cfg(any(feature = "option_argument", feature = "abbreviate"))]
    #[inline(never)]
    fn new_p2(a_kind: OptParseErrorKind, a_desc1: &str, a_desc2: &str) -> Self {
        let mut r = Self::new_p1(a_kind, a_desc1);
        r.desc2 = Some(a_desc2.to_string());
        r
    }
    pub fn invalid_option(desc1: &str) -> Self {
        Self::new_p1(OptParseErrorKind::InvalidOption, desc1)
    }
    pub fn missing_option(desc1: &str) -> Self {
        Self::new_p1(OptParseErrorKind::MissingOption, desc1)
    }
    //
    #[cfg(feature = "option_argument")]
    pub fn invalid_option_argument(desc1: &str, desc2: &str) -> Self {
        Self::new_p2(OptParseErrorKind::InvalidOptionArgument, desc1, desc2)
    }
    #[cfg(feature = "option_argument")]
    pub fn unexpected_option_argument(desc1: &str, desc2: &str) -> Self {
        Self::new_p2(OptParseErrorKind::UnexpectedOptionArgument, desc1, desc2)
    }
    #[cfg(feature = "option_argument")]
    pub fn missing_option_argument(desc1: &str) -> Self {
        Self::new_p1(OptParseErrorKind::MissingOptionArgument, desc1)
    }
    //
    #[cfg(feature = "argument")]
    pub fn unexpected_argument(desc1: &str) -> Self {
        Self::new_p1(OptParseErrorKind::UnexpectedArgument, desc1)
    }
    #[cfg(feature = "argument")]
    pub fn missing_argument(desc1: &str) -> Self {
        Self::new_p1(OptParseErrorKind::MissingArgument, desc1)
    }
    //
    #[cfg(feature = "subcommand")]
    pub fn invalid_subcommand(desc1: &str) -> Self {
        Self::new_p1(OptParseErrorKind::InvalidSubcommand, desc1)
    }
    #[cfg(feature = "subcommand")]
    pub fn missing_subcommand(desc1: &str) -> Self {
        Self::new_p1(OptParseErrorKind::MissingSubcommand, desc1)
    }
    //
    #[cfg(feature = "abbreviate")]
    pub fn ambiguous_option(desc1: &str, desc2: &str) -> Self {
        Self::new_p2(OptParseErrorKind::AmbiguousOption, desc1, desc2)
    }
    #[cfg(all(feature = "abbreviate", feature = "subcommand"))]
    pub fn ambiguous_subcommand(desc1: &str, desc2: &str) -> Self {
        Self::new_p2(OptParseErrorKind::AmbiguousSubcommand, desc1, desc2)
    }
}

impl Display for OptParseError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::OptParseErrorKind::*;
        //
        let msg: &str = match self.kind {
            InvalidOption => "Invalid option",
            MissingOption => "Missing option",
            //
            #[cfg(feature = "option_argument")]
            InvalidOptionArgument => "Invalid option argument",
            #[cfg(feature = "option_argument")]
            UnexpectedOptionArgument => "Unexpected option argument",
            #[cfg(feature = "option_argument")]
            MissingOptionArgument => "Missing option argument",
            //
            #[cfg(feature = "argument")]
            UnexpectedArgument => "Unexpected argument",
            #[cfg(feature = "argument")]
            MissingArgument => "Missing argument",
            //
            #[cfg(feature = "subcommand")]
            InvalidSubcommand => "Invalid subcommand",
            #[cfg(feature = "subcommand")]
            MissingSubcommand => "Missing subcommand",
            //
            #[cfg(feature = "abbreviate")]
            AmbiguousOption => "Ambiguous option",
            #[cfg(all(feature = "abbreviate", feature = "subcommand"))]
            AmbiguousSubcommand => "Ambiguous subcommand",
        };
        match self.desc2 {
            Some(ref s) => write!(fmt, "{}: {}: {}", msg, &self.desc1, &s),
            None => write!(fmt, "{}: {}", msg, &self.desc1),
        }
    }
}

#[cfg(not(feature = "no_std"))]
impl std::error::Error for OptParseError {}

/// Multiple option parse errors
#[derive(Debug, PartialEq)]
pub struct OptParseErrors(Vec<OptParseError>);

impl OptParseErrors {
    pub fn new() -> OptParseErrors {
        OptParseErrors(Vec::with_capacity(0))
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn push(&mut self, e: OptParseError) {
        self.0.push(e)
    }
    pub fn iter(&self) -> Iter<OptParseError> {
        self.0.iter()
    }
    pub fn append(&mut self, other: Self) {
        self.0.extend(other.0.into_iter())
    }
}
impl Default for OptParseErrors {
    fn default() -> Self {
        Self::new()
    }
}
impl Display for OptParseErrors {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        if self.is_empty() {
            write!(fmt, "")
        } else {
            let mut s = String::new();
            for err in self.iter() {
                s += &format!("{}\n", err);
            }
            write!(fmt, "{}", &s[0..(s.len() - 1)])
        }
    }
}
#[cfg(not(feature = "no_std"))]
impl std::error::Error for OptParseErrors {}