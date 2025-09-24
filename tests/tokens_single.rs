#[allow(unused_macros)]
#[macro_use]
mod helper;

#[cfg(feature = "long_only")]
mod plain {
    use flood_tide::check;
    #[cfg(feature = "option_argument")]
    use flood_tide::Arg;
    use flood_tide::Lex;
    use flood_tide::Opt;
    use flood_tide::OptNum;
    //
    #[test]
    fn tokens_long_only_1() {
        #[rustfmt::skip]
        let args = vec!["-a", "-ab", "-abc"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            B = 2,
            C = 3,
            D = 4,
            E = 5,
            Abcde = 6,
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
            Opt { sho: b'a', lon: "", has: Arg::No, num: CmdOP::A.to(), },
            Opt { sho: b'b', lon: "", has: Arg::No, num: CmdOP::B.to(), },
            Opt { sho: b'c', lon: "", has: Arg::No, num: CmdOP::C.to(), },
            Opt { sho: b'd', lon: "", has: Arg::No, num: CmdOP::D.to(), },
            Opt { sho: b'e', lon: "", has: Arg::No, num: CmdOP::E.to(), },
            Opt { sho: 0u8,  lon: "abcde", has: Arg::No, num: CmdOP::Abcde.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'a', lon: "", num: CmdOP::A.to(), },
            Opt { sho: b'b', lon: "", num: CmdOP::B.to(), },
            Opt { sho: b'c', lon: "", num: CmdOP::C.to(), },
            Opt { sho: b'd', lon: "", num: CmdOP::D.to(), },
            Opt { sho: b'e', lon: "", num: CmdOP::E.to(), },
            Opt { sho: 0u8,  lon: "abcde", num: CmdOP::Abcde.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0),(b'b',1),(b'c',2),(b'd',3),(b'e',4)];
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
                Err(e) => {
                    assert_eq!(format!("{}", e), "");
                    unreachable!();
                } //_ => unreachable!(),
            };
            assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
            assert_eq_tokens_namevals!(tokens, 1, 0u8, "abcde", None, CmdOP::Abcde);
            assert_eq_tokens_namevals!(tokens, 2, 0u8, "abcde", None, CmdOP::Abcde);
            //
            #[cfg(feature = "stop_at_mm")]
            assert_eq!(tokens.double_m, false);
        }
        #[cfg(not(feature = "abbreviate"))]
        match lex.tokens_from(&args) {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                #[cfg(feature = "single_error")]
                let expect = "Invalid option: ab";
                #[cfg(not(feature = "single_error"))]
                let expect = concat!("Invalid option: ab\n", "Invalid option: abc",);
                assert_eq!(thing, expect);
            }
        };
    }
    #[cfg(feature = "option_argument")]
    #[test]
    fn tokens_long_only_2() {
        #[rustfmt::skip]
        let args = vec!["-a", "-one", "-two=MANDATORY", "-three=OPTIONAL"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            One = 2,
            Two = 3,
            Three = 4,
            Four = 5,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'a', lon: "",      has: Arg::No,    num: CmdOP::A.to(), },
            Opt { sho: 0u8,  lon: "four",  has: Arg::Maybe, num: CmdOP::Four.to(), },
            Opt { sho: 0u8,  lon: "one",   has: Arg::No,    num: CmdOP::One.to(), },
            Opt { sho: 0u8,  lon: "three", has: Arg::Maybe, num: CmdOP::Three.to(), },
            Opt { sho: 0u8,  lon: "two",   has: Arg::Yes,   num: CmdOP::Two.to(), },
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
        assert_eq_tokens_namevals!(tokens, 1, 0u8, "one", None, CmdOP::One);
        assert_eq_tokens_namevals!(tokens, 2, 0u8, "two", Some("MANDATORY"), CmdOP::Two);
        assert_eq_tokens_namevals!(tokens, 3, 0u8, "three", Some("OPTIONAL"), CmdOP::Three);
        //
        #[cfg(feature = "stop_at_mm")]
        assert!(!tokens.double_m);
    }
    #[cfg(feature = "option_argument")]
    #[test]
    fn tokens_long_only_3() {
        #[rustfmt::skip]
        let args = vec!["-a", "-one", "-two", "MANDATORY", "-three=OPTIONAL"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            One = 2,
            Two = 3,
            Three = 4,
            Four = 5,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'a', lon: "",      has: Arg::No,    num: CmdOP::A.to(), },
            Opt { sho: 0u8,  lon: "four",  has: Arg::Maybe, num: CmdOP::Four.to(), },
            Opt { sho: 0u8,  lon: "one",   has: Arg::No,    num: CmdOP::One.to(), },
            Opt { sho: 0u8,  lon: "three", has: Arg::Maybe, num: CmdOP::Three.to(), },
            Opt { sho: 0u8,  lon: "two",   has: Arg::Yes,   num: CmdOP::Two.to(), },
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
            }
        };
        //
        assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
        assert_eq_tokens_namevals!(tokens, 1, 0u8, "one", None, CmdOP::One);
        assert_eq_tokens_namevals!(tokens, 2, 0u8, "two", Some("MANDATORY"), CmdOP::Two);
        assert_eq_tokens_namevals!(tokens, 3, 0u8, "three", Some("OPTIONAL"), CmdOP::Three);
        //
        #[cfg(feature = "stop_at_mm")]
        assert!(!tokens.double_m);
    }
    #[cfg(feature = "option_argument")]
    #[test]
    fn tokens_long_only_4() {
        #[rustfmt::skip]
        let args = vec!["-ab"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            B = 2,
            One = 3,
            Two = 4,
            Three = 5,
            Four = 6,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'a', lon: "",      has: Arg::No,    num: CmdOP::A.to(), },
            Opt { sho: b'b', lon: "",      has: Arg::No,    num: CmdOP::B.to(), },
            Opt { sho: 0u8,  lon: "four",  has: Arg::Maybe, num: CmdOP::Four.to(), },
            Opt { sho: 0u8,  lon: "one",   has: Arg::No,    num: CmdOP::One.to(), },
            Opt { sho: 0u8,  lon: "three", has: Arg::Maybe, num: CmdOP::Three.to(), },
            Opt { sho: 0u8,  lon: "two",   has: Arg::Yes,   num: CmdOP::Two.to(), },
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
        match lex.tokens_from(&args) {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                let expect = "Invalid option: ab";
                assert_eq!(thing, expect);
            }
        };
    }
    #[cfg(feature = "option_argument")]
    #[test]
    fn tokens_long_only_5() {
        #[rustfmt::skip]
        let args = vec!["-ab"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            One = 2,
            Two = 3,
            Three = 4,
            Four = 5,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'a', lon: "",     has: Arg::No,    num: CmdOP::A.to(), },
            Opt { sho: 0u8, lon: "four",  has: Arg::Maybe, num: CmdOP::Four.to(), },
            Opt { sho: 0u8, lon: "one",   has: Arg::No,    num: CmdOP::One.to(), },
            Opt { sho: 0u8, lon: "three", has: Arg::Maybe, num: CmdOP::Three.to(), },
            Opt { sho: 0u8, lon: "two",   has: Arg::Yes,   num: CmdOP::Two.to(), },
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
        match lex.tokens_from(&args) {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                let expect = "Invalid option: ab";
                assert_eq!(thing, expect);
            }
        };
    }
    #[cfg(feature = "option_argument")]
    #[test]
    fn tokens_long_only_6() {
        #[rustfmt::skip]
        let args = vec!["-a", "-b"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            X = 1,
            A = 2,
            B = 3,
            C = 4,
            One = 5,
            Two = 6,
            Three = 7,
            Four = 8,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'x', lon: "",      has: Arg::No,    num: CmdOP::X.to(), },
            Opt { sho: 0u8,  lon: "a",     has: Arg::No,    num: CmdOP::A.to(), },
            Opt { sho: 0u8,  lon: "b",     has: Arg::No,    num: CmdOP::B.to(), },
            Opt { sho: 0u8,  lon: "c",     has: Arg::No,    num: CmdOP::C.to(), },
            Opt { sho: 0u8,  lon: "four",  has: Arg::Maybe, num: CmdOP::Four.to(), },
            Opt { sho: 0u8,  lon: "one",   has: Arg::No,    num: CmdOP::One.to(), },
            Opt { sho: 0u8,  lon: "three", has: Arg::Maybe, num: CmdOP::Three.to(), },
            Opt { sho: 0u8,  lon: "two",   has: Arg::Yes,   num: CmdOP::Two.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'x',0)];
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
            }
        };
        //
        assert_eq_tokens_namevals!(tokens, 0, 0u8, "a", None, CmdOP::A);
        assert_eq_tokens_namevals!(tokens, 1, 0u8, "b", None, CmdOP::B);
        //
        #[cfg(feature = "stop_at_mm")]
        assert!(!tokens.double_m);
    }
    #[cfg(feature = "option_argument")]
    #[test]
    fn tokens_long_only_7() {
        #[rustfmt::skip]
        let args = vec!["-a", "-b"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            X = 1,
            C = 4,
            One = 5,
            Two = 6,
            Three = 7,
            Four = 8,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'x', lon: "",      has: Arg::No,    num: CmdOP::X.to(), },
            Opt { sho: 0u8,  lon: "c",     has: Arg::No,    num: CmdOP::C.to(), },
            Opt { sho: 0u8,  lon: "four",  has: Arg::Maybe, num: CmdOP::Four.to(), },
            Opt { sho: 0u8,  lon: "one",   has: Arg::No,    num: CmdOP::One.to(), },
            Opt { sho: 0u8,  lon: "three", has: Arg::Maybe, num: CmdOP::Three.to(), },
            Opt { sho: 0u8,  lon: "two",   has: Arg::Yes,   num: CmdOP::Two.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'x',0)];
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
                #[cfg(feature = "single_error")]
                let expect = "Invalid option: a";
                #[cfg(not(feature = "single_error"))]
                let expect = "Invalid option: a\nInvalid option: b";
                assert_eq!(thing, expect);
            }
        };
    }
    #[test]
    fn tokens_long_only_free() {
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
            Err(e) => {
                assert_eq!(format!("{}", e), "");
                unreachable!();
            }
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
    /*
    #[test]
    fn tokens_long_only_stop_at() {
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
            }
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
    #[cfg(feature = "stop_at_free")]
    #[test]
    fn tokens_long_only_stop_at_first_free() {
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
        #[cfg(feature = "stop_at_mm")]
        assert!(!tokens.double_m);
    }
    #[cfg(feature = "stop_at_mm")]
    #[test]
    fn tokens_long_only_stop_at_double_minus() {
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
    fn tokens_long_only_stop_at_double_minus_2() {
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
    fn tokens_long_only_stop_at_double_minus_3() {
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
    fn tokens_long_only_stop_at_double_minus_4() {
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
} // mod plain
