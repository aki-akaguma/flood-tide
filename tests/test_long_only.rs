#[allow(unused_macros)]
#[macro_use]
mod helper;

#[cfg(feature = "long_only")]
mod test_long_only {
    use flood_tide::check;
    #[cfg(feature = "option_argument")]
    use flood_tide::Arg;
    use flood_tide::Lex;
    use flood_tide::Opt;
    use flood_tide::OptNum;

    #[cfg(feature = "long_only")]
    #[cfg(not(feature = "subcommand"))]
    #[test]
    fn tokens_long_only_feature() {
        #[rustfmt::skip]
        let args = vec![
            "-a", "-b", "b_arg", "-long-yes", "long_yes_arg", "-long-maybe", "long_maybe_arg", "free1",
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
            Opt { sho: b'\0', lon: "a",          has: Arg::No,    num: CmdOP::A.to(), },
            Opt { sho: b'\0', lon: "b",          has: Arg::Yes,   num: CmdOP::B.to(), },
            Opt { sho: b'\0', lon: "long-maybe", has: Arg::Maybe, num: CmdOP::LongMaybe.to(), },
            Opt { sho: b'\0', lon: "long-yes",   has: Arg::Yes,   num: CmdOP::LongYes.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'\0', lon: "a",          num: CmdOP::A.to(), },
            Opt { sho: b'\0', lon: "b",          num: CmdOP::B.to(), },
            Opt { sho: b'\0', lon: "long-maybe", num: CmdOP::LongMaybe.to(), },
            Opt { sho: b'\0', lon: "long-yes",   num: CmdOP::LongYes.to(), },
        ];

        #[rustfmt::skip]
        let opt_ary_sho_idx = []; // No short options when long_only is enabled

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
            assert_eq_tokens_namevals!(tokens, 0, 0u8, "a", None, CmdOP::A);
            assert_eq_tokens_namevals!(tokens, 1, 0u8, "b", Some("b_arg"), CmdOP::B);
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
                assert_eq_tokens_namevals!(tokens, 0, 0u8, "a", None, CmdOP::A);
                assert_eq_tokens_namevals!(tokens, 1, 0u8, "b", None, CmdOP::B);

                assert_eq_tokens_free!(tokens, 0, "b_arg");
                assert_eq_tokens_free!(tokens, 1, "-long-yes");
                assert_eq_tokens_free!(tokens, 2, "long_yes_arg");
                assert_eq_tokens_free!(tokens, 3, "-long-maybe");
                assert_eq_tokens_free!(tokens, 4, "long_maybe_arg");
                assert_eq_tokens_free!(tokens, 5, "free1");
            }
            #[cfg(not(feature = "stop_at_free"))]
            {
                assert_eq_tokens_namevals!(tokens, 0, 0u8, "a", None, CmdOP::A);
                assert_eq_tokens_namevals!(tokens, 1, 0u8, "b", None, CmdOP::B);
                assert_eq_tokens_namevals!(tokens, 2, 0u8, "long-yes", None, CmdOP::LongYes);
                assert_eq_tokens_namevals!(tokens, 3, 0u8, "long-maybe", None, CmdOP::LongMaybe);

                assert_eq_tokens_free!(tokens, 0, "b_arg");
                assert_eq_tokens_free!(tokens, 1, "long_yes_arg");
                assert_eq_tokens_free!(tokens, 2, "long_maybe_arg");
                assert_eq_tokens_free!(tokens, 3, "free1");
            }
        }
    }

    #[cfg(all(feature = "long_only", feature = "stop_at_free"))]
    #[test]
    fn tokens_long_only_and_stop_at_free_features() {
        #[rustfmt::skip]
        let args = vec![
            "-a", "-b", "free1", "-c", "--long-opt", "free2",
        ];

        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            B = 2,
            C = 3,
            LongOpt = 4,
        }

        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }

        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'\0', lon: "a",          has: Arg::No,  num: CmdOP::A.to(), },
            Opt { sho: b'\0', lon: "b",          has: Arg::No,  num: CmdOP::B.to(), },
            Opt { sho: b'\0', lon: "c",          has: Arg::No,  num: CmdOP::C.to(), },
            Opt { sho: b'\0', lon: "long-opt", has: Arg::No,  num: CmdOP::LongOpt.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'\0', lon: "a",          num: CmdOP::A.to(), },
            Opt { sho: b'\0', lon: "b",          num: CmdOP::B.to(), },
            Opt { sho: b'\0', lon: "c",          num: CmdOP::C.to(), },
            Opt { sho: b'\0', lon: "long-opt", num: CmdOP::LongOpt.to(), },
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

        let tokens = match lex.tokens_from(&args) {
            Ok(t) => t,
            Err(e) => {
                assert_eq!(format!("{}", e), "");
                unreachable!();
            }
        };

        assert_eq_tokens_namevals!(tokens, 0, 0u8, "a", None, CmdOP::A);
        assert_eq_tokens_namevals!(tokens, 1, 0u8, "b", None, CmdOP::B);

        assert_eq_tokens_free!(tokens, 0, "free1");
        assert_eq_tokens_free!(tokens, 1, "-c");
        assert_eq_tokens_free!(tokens, 2, "--long-opt");
        assert_eq_tokens_free!(tokens, 3, "free2");

        #[cfg(feature = "stop_at_mm")]
        assert!(!tokens.double_m);
    }

    #[cfg(all(feature = "long_only", feature = "stop_at_mm"))]
    #[test]
    fn tokens_long_only_and_stop_at_mm_features() {
        #[rustfmt::skip]
        let args = vec![
            "-a", "-b", "--", "-c", "--long-opt", "free1",
        ];

        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            B = 2,
            C = 3,
            LongOpt = 4,
        }

        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }

        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'\0', lon: "a",          has: Arg::No,  num: CmdOP::A.to(), },
            Opt { sho: b'\0', lon: "b",          has: Arg::No,  num: CmdOP::B.to(), },
            Opt { sho: b'\0', lon: "c",          has: Arg::No,  num: CmdOP::C.to(), },
            Opt { sho: b'\0', lon: "long-opt", has: Arg::No,  num: CmdOP::LongOpt.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'\0', lon: "a",          num: CmdOP::A.to(), },
            Opt { sho: b'\0', lon: "b",          num: CmdOP::B.to(), },
            Opt { sho: b'\0', lon: "c",          num: CmdOP::C.to(), },
            Opt { sho: b'\0', lon: "long-opt", num: CmdOP::LongOpt.to(), },
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

        let tokens = match lex.tokens_from(&args) {
            Ok(t) => t,
            Err(e) => {
                assert_eq!(format!("{}", e), "");
                unreachable!();
            }
        };

        assert_eq_tokens_namevals!(tokens, 0, 0u8, "a", None, CmdOP::A);
        assert_eq_tokens_namevals!(tokens, 1, 0u8, "b", None, CmdOP::B);

        assert_eq_tokens_free!(tokens, 0, "-c");
        assert_eq_tokens_free!(tokens, 1, "--long-opt");
        assert_eq_tokens_free!(tokens, 2, "free1");

        assert!(tokens.double_m);
    }
}
