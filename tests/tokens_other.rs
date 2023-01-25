#[allow(unused_macros)]
#[macro_use]
mod test_macro;

mod plain {
    use flood_tide::check;
    #[cfg(feature = "option_argument")]
    use flood_tide::Arg;
    use flood_tide::Lex;
    use flood_tide::Opt;
    use flood_tide::OptNum;
    //{{{ free
    #[test]
    fn tokens_free() {
        #[rustfmt::skip]
        let args = vec!["-a", "other1", "other2"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
        }
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
        let tokens = match lex.tokens_from(&args) {
            Ok(t) => t,
            _ => unreachable!(),
        };
        //
        assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
        //
        assert_eq_tokens_free!(tokens, 0, "other1");
        assert_eq_tokens_free!(tokens, 1, "other2");
        //
        #[cfg(feature = "stop_at_mm")]
        assert!(!tokens.double_m);
    }
    //}}} free
    //
    //{{{ stop at
    /*
    #[cfg(feature = "stop_at_mm")]
    #[test]
    fn tokens_stop_at() {
        #[rustfmt::skip]
        let args = vec!["-a", "other1", "a", "-a", "other2"];
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
        //
        let lex = Lex::create_from(&opt_ary).stop_at(&["a"]);
        let tokens = match lex.tokens_from(&args) {
            Ok(t) => t,
            Err(e) => {
                assert_eq!(format!("{}", e), "");
                unreachable!();
            } //_ => unreachable!(),
        };
        //
        assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
        //
        if let Some(_) = tokens.namevals.get(1) {
            //assert_eq!(format!("{:?}",tokens.namevals), "");
            unreachable!()
        };
        //
        assert_eq_tokens_free!(tokens, 0, "other1");
        assert_eq_tokens_free!(tokens, 1, "a");
        assert_eq_tokens_free!(tokens, 2, "-a");
        assert_eq_tokens_free!(tokens, 3, "other2");
        //
        assert_eq!(tokens.double_m, false);
    }
    */
    //}}} stop at
    //
    //{{{ stop at first free
    #[cfg(feature = "stop_at_free")]
    #[test]
    fn tokens_stop_at_first_free() {
        #[rustfmt::skip]
        let args = vec!["-a", "other1", "a", "-a", "other2"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
        }
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
        let tokens = match lex.tokens_from(&args) {
            Ok(t) => t,
            Err(e) => {
                assert_eq!(format!("{}", e), "");
                unreachable!();
            } //_ => unreachable!(),
        };
        //
        assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
        //
        if let Some(_) = tokens.namevals.get(1) {
            //assert_eq!(format!("{:?}",tokens.namevals), "");
            unreachable!()
        };
        //
        assert_eq_tokens_free!(tokens, 0, "other1");
        assert_eq_tokens_free!(tokens, 1, "a");
        assert_eq_tokens_free!(tokens, 2, "-a");
        assert_eq_tokens_free!(tokens, 3, "other2");
        //
        #[cfg(feature = "stop_at_mm")]
        assert_eq!(tokens.double_m, false);
    }
    //}}} stop at first free
    //
    //{{{ stop at double minus
    #[cfg(feature = "stop_at_mm")]
    #[test]
    fn tokens_stop_at_double_minus() {
        #[rustfmt::skip]
        let args = vec!["-a", "--", "other1", "a", "-a", "other2"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
        }
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
        let tokens = match lex.tokens_from(&args) {
            Ok(t) => t,
            Err(e) => {
                assert_eq!(format!("{}", e), "");
                unreachable!();
            } //_ => unreachable!(),
        };
        //
        assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
        //
        if tokens.namevals.get(1).is_some() {
            //assert_eq!(format!("{:?}",tokens.namevals), "");
            unreachable!()
        };
        //
        assert_eq_tokens_free!(tokens, 0, "other1");
        assert_eq_tokens_free!(tokens, 1, "a");
        assert_eq_tokens_free!(tokens, 2, "-a");
        assert_eq_tokens_free!(tokens, 3, "other2");
        //
        assert!(tokens.double_m);
    }
    #[cfg(feature = "stop_at_mm")]
    #[test]
    fn tokens_stop_at_double_minus_2() {
        #[rustfmt::skip]
        let args = vec!["-a", "other1", "--", "a", "-a", "other2"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
        }
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
        let tokens = match lex.tokens_from(&args) {
            Ok(t) => t,
            Err(e) => {
                assert_eq!(format!("{}", e), "");
                unreachable!();
            } //_ => unreachable!(),
        };
        //
        assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
        //
        if tokens.namevals.get(1).is_some() {
            //assert_eq!(format!("{:?}",tokens.namevals), "");
            unreachable!()
        };
        //
        assert_eq_tokens_free!(tokens, 0, "other1");
        assert_eq_tokens_free!(tokens, 1, "--");
        assert_eq_tokens_free!(tokens, 2, "a");
        assert_eq_tokens_free!(tokens, 3, "-a");
        assert_eq_tokens_free!(tokens, 4, "other2");
        //
        assert!(!tokens.double_m);
    }
    #[cfg(feature = "stop_at_mm")]
    #[test]
    fn tokens_stop_at_double_minus_3() {
        #[rustfmt::skip]
        let args = vec!["-a", "other1", "--", "a", "-a", "other2"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
        }
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
        let tokens = match lex.tokens_from(&args) {
            Ok(t) => t,
            Err(e) => {
                assert_eq!(format!("{}", e), "");
                unreachable!();
            } //_ => unreachable!(),
        };
        //
        assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
        //
        if tokens.namevals.get(1).is_some() {
            //assert_eq!(format!("{:?}",tokens.namevals), "");
            unreachable!()
        };
        //
        assert_eq_tokens_free!(tokens, 0, "other1");
        assert_eq_tokens_free!(tokens, 1, "--");
        assert_eq_tokens_free!(tokens, 2, "a");
        assert_eq_tokens_free!(tokens, 3, "-a");
        assert_eq_tokens_free!(tokens, 4, "other2");
        //
        assert!(!tokens.double_m);
    }
    #[cfg(feature = "stop_at_mm")]
    #[test]
    fn tokens_stop_at_double_minus_4() {
        #[rustfmt::skip]
        let args = vec!["-a", "other1", "a", "-a", "--", "other2"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
        }
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
        let tokens = match lex.tokens_from(&args) {
            Ok(t) => t,
            Err(e) => {
                assert_eq!(format!("{}", e), "");
                unreachable!();
            } //_ => unreachable!(),
        };
        //
        assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
        #[cfg(not(feature = "stop_at_free"))]
        assert_eq_tokens_namevals!(tokens, 1, b'a', "", None, CmdOP::A);
        //
        if tokens.namevals.get(2).is_some() {
            //assert_eq!(format!("{:?}",tokens.namevals), "");
            unreachable!()
        };
        //
        #[cfg(feature = "stop_at_free")]
        {
            assert_eq_tokens_free!(tokens, 0, "other1");
            assert_eq_tokens_free!(tokens, 1, "a");
            assert_eq_tokens_free!(tokens, 2, "-a");
            assert_eq_tokens_free!(tokens, 3, "--");
            assert_eq_tokens_free!(tokens, 4, "other2");
        }
        #[cfg(not(feature = "stop_at_free"))]
        {
            assert_eq_tokens_free!(tokens, 0, "other1");
            assert_eq_tokens_free!(tokens, 1, "a");
            assert_eq_tokens_free!(tokens, 2, "--");
            assert_eq_tokens_free!(tokens, 3, "other2");
        }
        //
        assert!(!tokens.double_m);
    }
    //}}} stop at double minus
}
