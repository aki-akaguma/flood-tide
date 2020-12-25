#[allow(unused_macros)]
#[macro_use]
mod test_macro;

#[cfg(not(feature = "long_only"))]
mod plain {
    use flood_tide::check;
    #[cfg(feature = "option_argument")]
    use flood_tide::Arg;
    use flood_tide::Lex;
    use flood_tide::Opt;
    use flood_tide::OptNum;
    //
    #[test]
    fn tokens_short_name() {
        #[rustfmt::skip]
        let args = vec!["-a", "-b", "-cd"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            B = 2,
            C = 3,
            D = 4,
        };
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'a', lon: "", has: Arg::No, num: CmdOP::A.to(), },
            Opt { sho: b'b', lon: "", has: Arg::No, num: CmdOP::B.to(), },
            Opt { sho: b'c', lon: "", has: Arg::No, num: CmdOP::C.to(), },
            Opt { sho: b'd', lon: "", has: Arg::No, num: CmdOP::D.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'a', lon: "", num: CmdOP::A.to(), },
            Opt { sho: b'b', lon: "", num: CmdOP::B.to(), },
            Opt { sho: b'c', lon: "", num: CmdOP::C.to(), },
            Opt { sho: b'd', lon: "", num: CmdOP::D.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0),(b'b',1),(b'c',2),(b'd',3)];
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
        assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
        assert_eq_tokens_namevals!(tokens, 1, b'b', "", None, CmdOP::B);
        assert_eq_tokens_namevals!(tokens, 2, b'c', "", None, CmdOP::C);
        assert_eq_tokens_namevals!(tokens, 3, b'd', "", None, CmdOP::D);
        //
        #[cfg(feature = "stop_at_mm")]
        assert_eq!(tokens.double_m, false);
    }
    #[cfg(feature = "option_argument")]
    #[test]
    fn tokens_short_name_maybe() {
        #[rustfmt::skip]
        let args = vec!["-a", "-ab", "-a", "c"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
        };
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'a', lon: "", has: Arg::Maybe, num: CmdOP::A.to(), }
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0)];
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
        assert_eq_tokens_namevals!(tokens, 0, b'a', "", Some(""), CmdOP::A);
        assert_eq_tokens_namevals!(tokens, 1, b'a', "", Some("b"), CmdOP::A);
        assert_eq_tokens_namevals!(tokens, 2, b'a', "", Some(""), CmdOP::A);
        //
        assert_eq_tokens_free!(tokens, 0, "c");
        //
        #[cfg(feature = "stop_at_mm")]
        assert_eq!(tokens.double_m, false);
    }
    #[test]
    fn tokens_short_name_invalid_option() {
        #[rustfmt::skip]
        let args = vec!["-ab"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
        };
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'a', lon: "", has: Arg::No, num: CmdOP::A.to(), }
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'a', lon: "", num: CmdOP::A.to(), }
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0)];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };
        let _tokens = match lex.tokens_from(&args) {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                let expect = "Invalid option: b";
                assert_eq!(thing, expect);
            }
        };
    }
    #[cfg(feature = "option_argument")]
    #[test]
    fn tokens_short_name_missing_option_argument() {
        #[rustfmt::skip]
        let args = vec!["-a"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
        };
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'a', lon: "", has: Arg::Yes, num: CmdOP::A.to(), }
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0)];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };
        let _tokens = match lex.tokens_from(&args) {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                let expect = "Missing option argument: a";
                assert_eq!(thing, expect);
            }
        };
    }
    #[cfg(feature = "option_argument")]
    #[test]
    fn tokens_short_name_ok_nealy_missing_option_argument() {
        #[rustfmt::skip]
        let args = vec!["-ab"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            B = 2,
        };
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'a', lon: "", has: Arg::Yes, num: CmdOP::A.to(), },
            Opt { sho: b'b', lon: "", has: Arg::No,  num: CmdOP::B.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0),(b'b',1)];
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
        assert_eq_tokens_namevals!(tokens, 0, b'a', "", Some("b"), CmdOP::A);
        //
        //assert_eq!(format!("{:?}",tokens.namevals.get(1)),"");
        match tokens.namevals.get(1) {
            Some(_) => unreachable!(),
            _ => {}
        }
        //
        #[cfg(feature = "stop_at_mm")]
        assert_eq!(tokens.double_m, false);
    }
    #[cfg(feature = "option_argument")]
    #[test]
    fn tokens_short_name_ok_nealy_missing_option_argument_2() {
        #[rustfmt::skip]
        let args = vec!["-a", "-b"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            B = 2,
        };
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'a', lon: "", has: Arg::Yes, num: CmdOP::A.to(), },
            Opt { sho: b'b', lon: "", has: Arg::No,  num: CmdOP::B.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0),(b'b',1)];
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
        assert_eq_tokens_namevals!(tokens, 0, b'a', "", Some("-b"), CmdOP::A);
        //
        match tokens.namevals.get(1) {
            Some(_) => unreachable!(),
            _ => {}
        }
    }
}
