#[allow(unused_macros)]
#[macro_use]
mod helper;

#[cfg(not(feature = "long_only"))]
mod test_more1 {
    use flood_tide::check;
    #[cfg(feature = "option_argument")]
    use flood_tide::Arg;
    use flood_tide::Lex;
    use flood_tide::Opt;
    use flood_tide::OptNum;

    #[cfg(not(feature = "long_only"))]
    #[test]
    fn tokens_simple_combined() {
        #[rustfmt::skip]
        let args = vec![
            "-a", "--verbose", "-f", "file.txt", "--", "free_arg2",
        ];

        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            Verbose = 2,
            File = 3,
        }

        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }

        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'a',  lon: "",        has: Arg::No,  num: CmdOP::A.to(), },
            Opt { sho: b'f',  lon: "file",    has: Arg::Yes, num: CmdOP::File.to(), },
            Opt { sho: b'v',  lon: "verbose", has: Arg::No,  num: CmdOP::Verbose.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'a',  lon: "",        num: CmdOP::A.to(), },
            Opt { sho: b'f',  lon: "file",    num: CmdOP::File.to(), },
            Opt { sho: b'v',  lon: "verbose", num: CmdOP::Verbose.to(), },
        ];

        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0),(b'f',1),(b'v',2)];

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

        assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
        assert_eq_tokens_namevals!(tokens, 1, b'v', "verbose", None, CmdOP::Verbose);
        assert_eq_tokens_namevals!(tokens, 2, b'f', "file", Some("file.txt"), CmdOP::File);

        #[cfg(feature = "option_argument")]
        {
            assert_eq_tokens_free!(tokens, 0, "free_arg2");

            #[cfg(feature = "stop_at_mm")]
            assert!(tokens.double_m);
        }
        #[cfg(not(feature = "option_argument"))]
        {
            #[cfg(feature = "stop_at_mm")]
            {
                assert_eq_tokens_free!(tokens, 0, "file.txt");
                assert_eq_tokens_free!(tokens, 1, "--");
                assert_eq_tokens_free!(tokens, 2, "free_arg2");
            }
            #[cfg(not(feature = "stop_at_mm"))]
            {
                #[cfg(feature = "stop_at_free")]
                {
                    assert_eq_tokens_free!(tokens, 0, "file.txt");
                    assert_eq_tokens_free!(tokens, 1, "--");
                    assert_eq_tokens_free!(tokens, 2, "free_arg2");
                }
                #[cfg(not(feature = "stop_at_free"))]
                {
                    assert_eq_tokens_free!(tokens, 0, "file.txt");
                    assert_eq_tokens_free!(tokens, 1, "free_arg2");
                }
            }
            #[cfg(feature = "stop_at_mm")]
            assert!(!tokens.double_m);
        }
    }

    #[cfg(not(feature = "long_only"))]
    #[test]
    fn tokens_mixed_short_options_and_free_args() {
        #[rustfmt::skip]
        let args = vec![
            "-a", "-bval", "-c", "-d", "d_arg", "free1", "-e", "free2",
        ];

        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            B = 2,
            C = 3,
            D = 4,
            E = 5,
        }

        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }

        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'a',  lon: "", has: Arg::No,    num: CmdOP::A.to(), },
            Opt { sho: b'b',  lon: "", has: Arg::Yes,   num: CmdOP::B.to(), },
            Opt { sho: b'c',  lon: "", has: Arg::No,    num: CmdOP::C.to(), },
            Opt { sho: b'd',  lon: "", has: Arg::Maybe, num: CmdOP::D.to(), },
            Opt { sho: b'e',  lon: "", has: Arg::No,    num: CmdOP::E.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'a',  lon: "", num: CmdOP::A.to(), },
            Opt { sho: b'b',  lon: "", num: CmdOP::B.to(), },
            Opt { sho: b'c',  lon: "", num: CmdOP::C.to(), },
            Opt { sho: b'd',  lon: "", num: CmdOP::D.to(), },
            Opt { sho: b'e',  lon: "", num: CmdOP::E.to(), },
        ];

        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0),(b'b',1),(b'c',2),(b'd',3),(b'e',4)];

        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };

        #[cfg(feature = "option_argument")]
        {
            let tokens = match lex.tokens_from(&args) {
                Ok(t) => t,
                Err(e) => {
                    assert_eq!(format!("{}", e), "");
                    unreachable!();
                }
            };

            #[cfg(feature = "stop_at_free")]
            {
                assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
                assert_eq_tokens_namevals!(tokens, 1, b'b', "", Some("val"), CmdOP::B);
                assert_eq_tokens_namevals!(tokens, 2, b'c', "", None, CmdOP::C);
                assert_eq_tokens_namevals!(tokens, 3, b'd', "", Some(""), CmdOP::D);

                assert_eq_tokens_free!(tokens, 0, "d_arg");
                assert_eq_tokens_free!(tokens, 1, "free1");
                assert_eq_tokens_free!(tokens, 2, "-e");
                assert_eq_tokens_free!(tokens, 3, "free2");
            }
            #[cfg(not(feature = "stop_at_free"))]
            {
                assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
                assert_eq_tokens_namevals!(tokens, 1, b'b', "", Some("val"), CmdOP::B);
                assert_eq_tokens_namevals!(tokens, 2, b'c', "", None, CmdOP::C);
                assert_eq_tokens_namevals!(tokens, 3, b'd', "", Some(""), CmdOP::D);
                assert_eq_tokens_namevals!(tokens, 4, b'e', "", None, CmdOP::E);

                assert_eq_tokens_free!(tokens, 0, "d_arg");
                assert_eq_tokens_free!(tokens, 1, "free1");
                assert_eq_tokens_free!(tokens, 2, "free2");
            }
        }
        #[cfg(not(feature = "option_argument"))]
        {
            match lex.tokens_from(&args) {
                Ok(t) => {
                    assert_eq!(format!("{:?}", t), "");
                    unreachable!();
                }
                Err(e) => {
                    #[cfg(feature = "single_error")]
                    assert_eq!(format!("{}", e), "Invalid option: v");
                    #[cfg(not(feature = "single_error"))]
                    assert_eq!(format!("{}", e), "Invalid option: v\nInvalid option: l");
                }
            };
        }
    }

    #[cfg(not(feature = "long_only"))]
    #[test]
    fn tokens_long_options_with_arguments() {
        #[rustfmt::skip]
        let args = vec![
            "--alpha", "--beta=b_arg", "--gamma", "g_arg", "--delta", "free1",
        ];

        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Alpha = 1,
            Beta = 2,
            Gamma = 3,
            Delta = 4,
        }

        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }

        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'\0', lon: "alpha", has: Arg::No,    num: CmdOP::Alpha.to(), },
            Opt { sho: b'\0', lon: "beta",  has: Arg::Yes,   num: CmdOP::Beta.to(), },
            Opt { sho: b'\0', lon: "delta", has: Arg::No,    num: CmdOP::Delta.to(), },
            Opt { sho: b'\0', lon: "gamma", has: Arg::Maybe, num: CmdOP::Gamma.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'\0', lon: "alpha", num: CmdOP::Alpha.to(), },
            Opt { sho: b'\0', lon: "beta",  num: CmdOP::Beta.to(), },
            Opt { sho: b'\0', lon: "delta", num: CmdOP::Delta.to(), },
            Opt { sho: b'\0', lon: "gamma", num: CmdOP::Gamma.to(), },
        ];

        #[rustfmt::skip]
        let opt_ary_sho_idx = [];

        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };

        #[cfg(feature = "option_argument")]
        {
            let tokens = match lex.tokens_from(&args) {
                Ok(t) => t,
                Err(e) => {
                    assert_eq!(format!("{}", e), "");
                    unreachable!();
                }
            };

            #[cfg(feature = "stop_at_free")]
            {
                assert_eq_tokens_namevals!(tokens, 0, 0u8, "alpha", None, CmdOP::Alpha);
                assert_eq_tokens_namevals!(tokens, 1, 0u8, "beta", Some("b_arg"), CmdOP::Beta);
                assert_eq_tokens_namevals!(tokens, 2, 0u8, "gamma", Some(""), CmdOP::Gamma);

                assert_eq_tokens_free!(tokens, 0, "g_arg");
                assert_eq_tokens_free!(tokens, 1, "--delta");
                assert_eq_tokens_free!(tokens, 2, "free1");
            }
            #[cfg(not(feature = "stop_at_free"))]
            {
                assert_eq_tokens_namevals!(tokens, 0, 0u8, "alpha", None, CmdOP::Alpha);
                assert_eq_tokens_namevals!(tokens, 1, 0u8, "beta", Some("b_arg"), CmdOP::Beta);
                assert_eq_tokens_namevals!(tokens, 2, 0u8, "gamma", Some(""), CmdOP::Gamma);
                assert_eq_tokens_namevals!(tokens, 3, 0u8, "delta", None, CmdOP::Delta);

                assert_eq_tokens_free!(tokens, 0, "g_arg");
                assert_eq_tokens_free!(tokens, 1, "free1");
            }
        }
        #[cfg(not(feature = "option_argument"))]
        {
            match lex.tokens_from(&args) {
                Ok(t) => {
                    assert_eq!(format!("{:?}", t), "");
                    unreachable!();
                }
                Err(e) => {
                    assert_eq!(format!("{}", e), "Invalid option: beta=b_arg");
                }
            };
        }
    }

    #[cfg(not(feature = "long_only"))]
    #[test]
    fn tokens_comprehensive_parsing() {
        #[rustfmt::skip]
        let args = vec![
            "-a", "-bval", "--long-opt", "long_val", "-c", "--", "free1", "-d", "free2",
        ];

        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            B = 2,
            LongOpt = 3,
            C = 4,
        }

        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }

        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'a',  lon: "",         has: Arg::No,  num: CmdOP::A.to(), },
            Opt { sho: b'b',  lon: "",         has: Arg::Yes, num: CmdOP::B.to(), },
            Opt { sho: b'c',  lon: "",         has: Arg::No,  num: CmdOP::C.to(), },
            Opt { sho: b'\0', lon: "long-opt", has: Arg::Yes, num: CmdOP::LongOpt.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'a',  lon: "",         num: CmdOP::A.to(), },
            Opt { sho: b'b',  lon: "",         num: CmdOP::B.to(), },
            Opt { sho: b'c',  lon: "",         num: CmdOP::C.to(), },
            Opt { sho: b'\0', lon: "long-opt", num: CmdOP::LongOpt.to(), },
        ];

        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0),(b'b',1),(b'c',2)];

        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };

        #[cfg(feature = "option_argument")]
        {
            #[cfg(feature = "stop_at_mm")]
            {
                let tokens = match lex.tokens_from(&args) {
                    Ok(t) => t,
                    Err(e) => {
                        assert_eq!(format!("{}", e), "");
                        unreachable!();
                    }
                };

                assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
                assert_eq_tokens_namevals!(tokens, 1, b'b', "", Some("val"), CmdOP::B);
                assert_eq_tokens_namevals!(
                    tokens,
                    2,
                    0u8,
                    "long-opt",
                    Some("long_val"),
                    CmdOP::LongOpt
                );
                assert_eq_tokens_namevals!(tokens, 3, b'c', "", None, CmdOP::C);
                assert_eq_tokens_free!(tokens, 0, "free1");
                assert_eq_tokens_free!(tokens, 1, "-d");
                assert_eq_tokens_free!(tokens, 2, "free2");
                #[cfg(feature = "stop_at_mm")]
                assert!(tokens.double_m);
            }
            #[cfg(not(feature = "stop_at_mm"))]
            {
                #[cfg(feature = "stop_at_free")]
                {
                    let tokens = match lex.tokens_from(&args) {
                        Ok(t) => t,
                        Err(e) => {
                            assert_eq!(format!("{}", e), "");
                            unreachable!();
                        }
                    };

                    assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
                    assert_eq_tokens_namevals!(tokens, 1, b'b', "", Some("val"), CmdOP::B);
                    assert_eq_tokens_namevals!(
                        tokens,
                        2,
                        0u8,
                        "long-opt",
                        Some("long_val"),
                        CmdOP::LongOpt
                    );
                    assert_eq_tokens_namevals!(tokens, 3, b'c', "", None, CmdOP::C);
                    assert_eq_tokens_free!(tokens, 0, "free1");
                    assert_eq_tokens_free!(tokens, 1, "-d");
                    assert_eq_tokens_free!(tokens, 2, "free2");
                }
                #[cfg(not(feature = "stop_at_free"))]
                match lex.tokens_from(&args) {
                    Ok(t) => {
                        assert_eq!(format!("{:?}", t), "");
                        unreachable!();
                    }
                    Err(e) => {
                        assert_eq!(format!("{}", e), "Invalid option: d");
                    }
                };
            }
        }
        #[cfg(not(feature = "option_argument"))]
        {
            match lex.tokens_from(&args) {
                Ok(t) => {
                    assert_eq!(format!("{:?}", t), "");
                    unreachable!();
                }
                Err(e) => {
                    #[cfg(feature = "single_error")]
                    {
                        assert_eq!(format!("{}", e), "Invalid option: v");
                    }
                    #[cfg(not(feature = "single_error"))]
                    {
                        #[cfg(feature = "stop_at_mm")]
                        {
                            assert_eq!(format!("{}", e), "Invalid option: v\nInvalid option: l");
                        }
                        #[cfg(not(feature = "stop_at_mm"))]
                        {
                            #[cfg(feature = "stop_at_free")]
                            assert_eq!(format!("{}", e), "Invalid option: v\nInvalid option: l");
                            #[cfg(not(feature = "stop_at_free"))]
                            assert_eq!(
                                format!("{}", e),
                                "Invalid option: v\nInvalid option: l\nInvalid option: d"
                            );
                        }
                    }
                }
            };
        }
    }

    #[cfg(not(feature = "long_only"))]
    #[test]
    fn tokens_no_option_argument_feature() {
        #[rustfmt::skip]
        let args = vec![
            "-a", "-b", "b_arg", "--long-yes", "long_yes_arg", "--long-maybe", "long_maybe_arg", "free1",
        ];

        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            B = 2,
            LongYes = 3,
            LongMaybe = 4,
        }

        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }

        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'a',  lon: "",           has: Arg::No,    num: CmdOP::A.to(), },
            Opt { sho: b'b',  lon: "",           has: Arg::Yes,   num: CmdOP::B.to(), },
            Opt { sho: b'\0', lon: "long-maybe", has: Arg::Maybe, num: CmdOP::LongMaybe.to(), },
            Opt { sho: b'\0', lon: "long-yes",   has: Arg::Yes,   num: CmdOP::LongYes.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'a',  lon: "",           num: CmdOP::A.to(), },
            Opt { sho: b'b',  lon: "",           num: CmdOP::B.to(), },
            Opt { sho: b'\0', lon: "long-maybe", num: CmdOP::LongMaybe.to(), },
            Opt { sho: b'\0', lon: "long-yes",   num: CmdOP::LongYes.to(), },
        ];

        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0),(b'b',1)];

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

        #[cfg(feature = "option_argument")]
        {
            assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
            assert_eq_tokens_namevals!(tokens, 1, b'b', "", Some("b_arg"), CmdOP::B);
            assert_eq_tokens_namevals!(
                tokens,
                2,
                0u8,
                "long-yes",
                Some("long_yes_arg"),
                CmdOP::LongYes
            );
            assert_eq_tokens_namevals!(tokens, 3, 0u8, "long-maybe", Some(""), CmdOP::LongMaybe);

            assert_eq_tokens_free!(tokens, 0, "long_maybe_arg");
            assert_eq_tokens_free!(tokens, 1, "free1");
        }
        #[cfg(not(feature = "option_argument"))]
        {
            #[cfg(feature = "stop_at_free")]
            {
                assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
                assert_eq_tokens_namevals!(tokens, 1, b'b', "", None, CmdOP::B);

                assert_eq_tokens_free!(tokens, 0, "b_arg");
                assert_eq_tokens_free!(tokens, 1, "--long-yes");
                assert_eq_tokens_free!(tokens, 2, "long_yes_arg");
                assert_eq_tokens_free!(tokens, 3, "--long-maybe");
                assert_eq_tokens_free!(tokens, 4, "long_maybe_arg");
                assert_eq_tokens_free!(tokens, 5, "free1");
            }
            #[cfg(not(feature = "stop_at_free"))]
            {
                assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
                assert_eq_tokens_namevals!(tokens, 1, b'b', "", None, CmdOP::B);
                assert_eq_tokens_namevals!(tokens, 2, 0u8, "long-yes", None, CmdOP::LongYes);
                assert_eq_tokens_namevals!(tokens, 3, 0u8, "long-maybe", None, CmdOP::LongMaybe);

                assert_eq_tokens_free!(tokens, 0, "b_arg");
                assert_eq_tokens_free!(tokens, 1, "long_yes_arg");
                assert_eq_tokens_free!(tokens, 2, "long_maybe_arg");
                assert_eq_tokens_free!(tokens, 3, "free1");
            }
        }
    }

    #[cfg(not(feature = "long_only"))]
    #[cfg(feature = "stop_at_mm")]
    #[test]
    fn tokens_stop_at_mm_feature() {
        #[rustfmt::skip]
        let args = vec![
            "-a", "--long-opt", "--", "-b", "free1", "--another-opt", "free2",
        ];

        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            LongOpt = 2,
        }

        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }

        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'a',  lon: "",         has: Arg::No,  num: CmdOP::A.to(), },
            Opt { sho: b'\0', lon: "long-opt", has: Arg::No,  num: CmdOP::LongOpt.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'a',  lon: "",         num: CmdOP::A.to(), },
            Opt { sho: b'\0', lon: "long-opt", num: CmdOP::LongOpt.to(), },
        ];

        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0)];

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

        assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
        assert_eq_tokens_namevals!(tokens, 1, 0u8, "long-opt", None, CmdOP::LongOpt);

        assert_eq_tokens_free!(tokens, 0, "-b");
        assert_eq_tokens_free!(tokens, 1, "free1");
        assert_eq_tokens_free!(tokens, 2, "--another-opt");
        assert_eq_tokens_free!(tokens, 3, "free2");

        assert!(tokens.double_m);
    }
}
