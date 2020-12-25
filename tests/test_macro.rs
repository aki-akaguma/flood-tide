#[allow(unused_macros)]
macro_rules! assert_eq_tokens_namevals {
    ($tokens:ident, $idx:expr, $st_nm:expr, $lg_nm:expr, $val:expr, $num:expr) => {
        let nmvl = if let Some(v) = $tokens.namevals.get($idx) {
            v
        } else {
            unreachable!()
        };
        assert_eq!(nmvl.opt.sho, $st_nm);
        assert_eq!(nmvl.opt.lon, $lg_nm);
        #[cfg(feature = "option_argument")]
        assert_eq!(nmvl.val, $val);
        assert_eq!(nmvl.opt.num, $num as OptNum);
    };
}

#[allow(unused_macros)]
macro_rules! assert_eq_tokens_free {
    ($tokens:ident, $idx:expr, $val:expr) => {
        let free = if let Some(&v) = $tokens.free.get($idx) {
            v
        } else {
            unreachable!()
        };
        assert_eq!(free, $val);
    };
}

#[allow(unused_macros)]
macro_rules! assert_eq_tokens_subcmd {
    ($tokens:ident, $val:expr) => {
        let subcmd = if let Some(s) = $tokens.subcmd {
            s
        } else {
            unreachable!()
        };
        assert_eq!(subcmd, $val);
    };
}
