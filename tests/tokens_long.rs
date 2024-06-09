#[allow(unused_macros)]
#[macro_use]
mod test_macro;

#[cfg(feature = "option_argument")]
#[cfg(not(feature = "long_only"))]
mod plain {
    use flood_tide::check;
    use flood_tide::Arg;
    use flood_tide::Lex;
    use flood_tide::Opt;
    use flood_tide::OptNum;
    //
    #[test]
    fn tokens_long_name() {
        #[rustfmt::skip]
        let args = vec!["--long1=val1", "--long2=val2", "--long3"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long1 = 1,
            Long2 = 2,
            Long3 = 3,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'\0', lon: "long1", has: Arg::Yes, num: CmdOP::Long1.to(), },
            Opt { sho: b'\0', lon: "long2", has: Arg::Yes, num: CmdOP::Long2.to(), },
            Opt { sho: b'\0', lon: "long3", has: Arg::No,  num: CmdOP::Long3.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };
        let tokens = match lex.tokens_from(&args) {
            Ok(t) => t,
            _ => unreachable!(),
        };
        //
        assert_eq_tokens_namevals!(tokens, 0, 0u8, "long1", Some("val1"), CmdOP::Long1);
        assert_eq_tokens_namevals!(tokens, 1, 0u8, "long2", Some("val2"), CmdOP::Long2);
        assert_eq_tokens_namevals!(tokens, 2, 0u8, "long3", None, CmdOP::Long3);
        //
        #[cfg(feature = "stop_at_mm")]
        assert!(!tokens.double_m);
    }
    #[test]
    fn tokens_long_name_maybe() {
        #[rustfmt::skip]
        let args = vec!["--long1=val1", "--long1=", "--long1", "a"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long1 = 1,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'\0', lon: "long1", has: Arg::Maybe, num: CmdOP::Long1.to(), }
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };
        let tokens = match lex.tokens_from(&args) {
            Ok(t) => t,
            //_ => unreachable!(),
            Err(err) => {
                assert_eq!(format!("{}", err), "");
                unreachable!();
            }
        };
        //
        assert_eq_tokens_namevals!(tokens, 0, 0u8, "long1", Some("val1"), CmdOP::Long1);
        assert_eq_tokens_namevals!(tokens, 1, 0u8, "long1", Some(""), CmdOP::Long1);
        assert_eq_tokens_namevals!(tokens, 2, 0u8, "long1", Some(""), CmdOP::Long1);
        //
        assert_eq_tokens_free!(tokens, 0, "a");
        //
        #[cfg(feature = "stop_at_mm")]
        assert!(!tokens.double_m);
    }
    #[test]
    fn tokens_long_name_invalid_option() {
        #[rustfmt::skip]
        let args = vec!["--long4=val1"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long1 = 1,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'\0', lon: "long1", has: Arg::Yes, num: CmdOP::Long1.to(), }
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };
        match lex.tokens_from(&args) {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                let expect = "Invalid option: long4";
                assert_eq!(thing, expect);
            }
        };
    }
    #[test]
    fn tokens_long_name_unexpected_option_argument() {
        #[rustfmt::skip]
        let args = vec!["--long4=val4"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long4 = 1,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'\0', lon: "long4", has: Arg::No, num: CmdOP::Long4.to(), }
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };
        match lex.tokens_from(&args) {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                let expect = "Unexpected option argument: long4: val4";
                assert_eq!(thing, expect);
            }
        };
    }
    #[test]
    fn tokens_long_name_missing_option_argument() {
        #[rustfmt::skip]
        let args = vec!["--long4"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long4 = 1,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'\0', lon: "long4", has: Arg::Yes, num: CmdOP::Long4.to(), }
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };
        match lex.tokens_from(&args) {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                let expect = "Missing option argument: long4";
                assert_eq!(thing, expect);
            }
        };
    }
    #[test]
    fn tokens_long_name_ok_nearly_missing_option_argument() {
        #[rustfmt::skip]
        let args = vec!["--long4", ""];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long4 = 1,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'\0', lon: "long4", has: Arg::Yes, num: CmdOP::Long4.to(), }
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };
        let tokens = match lex.tokens_from(&args) {
            Ok(t) => t,
            _ => unreachable!(),
        };
        //
        assert_eq_tokens_namevals!(tokens, 0, 0u8, "long4", Some(""), CmdOP::Long4);
        //
        #[cfg(feature = "stop_at_mm")]
        assert!(!tokens.double_m);
    }
    #[test]
    fn tokens_long_name_ok_nearly_missing_option_argument2() {
        #[rustfmt::skip]
        let args = vec!["--long4="];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long4 = 1,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'\0', lon: "long4", has: Arg::Yes, num: CmdOP::Long4.to(), }
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };
        let tokens = match lex.tokens_from(&args) {
            Ok(t) => t,
            _ => unreachable!(),
        };
        //
        assert_eq_tokens_namevals!(tokens, 0, 0u8, "long4", Some(""), CmdOP::Long4);
        //
        #[cfg(feature = "stop_at_mm")]
        assert!(!tokens.double_m);
    }
    #[test]
    fn tokens_long_name_abbreviate() {
        #[rustfmt::skip]
        let args = vec!["--lon", "val4"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long4 = 1,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'\0', lon: "long4", has: Arg::Yes, num: CmdOP::Long4.to(), }
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };
        #[cfg(feature = "abbreviate")]
        {
            let tokens = match lex.tokens_from(&args) {
                Ok(t) => t,
                _ => unreachable!(),
            };
            assert_eq_tokens_namevals!(tokens, 0, 0u8, "long4", Some("val4"), CmdOP::Long4);
            #[cfg(feature = "stop_at_mm")]
            assert!(!tokens.double_m);
        }
        //
        #[cfg(not(feature = "abbreviate"))]
        match lex.tokens_from(&args) {
            Ok(_) => unreachable!(),
            Err(err) => {
                let thing = format!("{}", err);
                let expect = "Invalid option: lon";
                assert_eq!(thing, expect);
            }
        };
    }
    #[test]
    fn tokens_long_name_ambiguous() {
        #[rustfmt::skip]
        let args = vec!["--lon", "val2"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long1 = 1,
            Long2 = 2,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'\0', lon: "long1", has: Arg::Yes, num: CmdOP::Long1.to(), },
            Opt { sho: b'\0', lon: "long2", has: Arg::Yes, num: CmdOP::Long2.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };
        #[cfg(feature = "abbreviate")]
        match lex.tokens_from(&args) {
            Ok(_) => unreachable!(),
            Err(err) => {
                let thing = format!("{}", err);
                let expect = "Ambiguous option: lon: possibilities: \'--long1\' \'--long2\'";
                assert_eq!(thing, expect);
            }
        };
        #[cfg(not(feature = "abbreviate"))]
        match lex.tokens_from(&args) {
            Ok(_) => unreachable!(),
            Err(err) => {
                let thing = format!("{}", err);
                let expect = "Invalid option: lon";
                assert_eq!(thing, expect);
            }
        };
    }
    #[test]
    fn tokens_long_name_ok_nearly_ambiguous() {
        #[rustfmt::skip]
        let args = vec!["--lon", "val2"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long1 = 1,
            Lon = 2,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'\0', lon: "lon",   has: Arg::Yes, num: CmdOP::Lon.to(), },
            Opt { sho: b'\0', lon: "long1", has: Arg::Yes, num: CmdOP::Long1.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };
        let tokens = match lex.tokens_from(&args) {
            Ok(t) => t,
            //_ => unreachable!(),
            Err(err) => {
                assert_eq!(format!("{}", err), "");
                unreachable!();
            }
        };
        //
        assert_eq_tokens_namevals!(tokens, 0, 0u8, "lon", Some("val2"), CmdOP::Lon);
        //
        #[cfg(feature = "stop_at_mm")]
        assert!(!tokens.double_m);
    }
    #[test]
    fn tokens_long_name_non_abbreviated() {
        #[rustfmt::skip]
        let args = vec!["--lon", "val2"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long1 = 1,
            Long = 2,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'\0', lon: "long",  has: Arg::Yes, num: CmdOP::Long.to(), },
            Opt { sho: b'\0', lon: "long1", has: Arg::Yes, num: CmdOP::Long1.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };
        #[cfg(feature = "abbreviate")]
        match lex.tokens_from(&args) {
            Ok(_) => unreachable!(),
            Err(err) => {
                let thing = format!("{}", err);
                let expect = "Ambiguous option: lon: possibilities: \'--long\' \'--long1\'";
                assert_eq!(thing, expect);
            }
        };
        #[cfg(not(feature = "abbreviate"))]
        match lex.tokens_from(&args) {
            Ok(_) => unreachable!(),
            Err(err) => {
                let thing = format!("{}", err);
                let expect = "Invalid option: lon";
                assert_eq!(thing, expect);
            }
        };
    }
}
