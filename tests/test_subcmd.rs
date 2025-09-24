#[allow(unused_macros)]
#[macro_use]
mod helper;

#[cfg(feature = "subcommand")]
#[cfg(not(feature = "long_only"))]
mod test_subcmd {
    use flood_tide::check;
    #[cfg(feature = "option_argument")]
    use flood_tide::Arg;
    use flood_tide::Lex;
    use flood_tide::Opt;
    use flood_tide::OptNum;

    #[cfg(feature = "subcommand")]
    #[cfg(not(feature = "long_only"))]
    #[test]
    fn tokens_subcommand_parsing() {
        #[rustfmt::skip]
        let args = vec![
            "-v", "--global-opt", "global_val", "subcmd", "-s", "sub_arg", "free1",
        ];

        #[allow(dead_code)]
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            V = 1,
            GlobalOpt = 2,
            S = 3,
            SubArg = 4,
        }

        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }

        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'v',  lon: "",           has: Arg::No,  num: CmdOP::V.to(), },
            Opt { sho: b'\0', lon: "global-opt", has: Arg::Yes, num: CmdOP::GlobalOpt.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'v',  lon: "",           num: CmdOP::V.to(), },
            Opt { sho: b'\0', lon: "global-opt", num: CmdOP::GlobalOpt.to(), },
        ];

        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'v',0)];

        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx).subcmd(&["subcmd"])
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

            assert_eq_tokens_namevals!(tokens, 0, b'v', "", None, CmdOP::V);
            assert_eq_tokens_namevals!(
                tokens,
                1,
                0u8,
                "global-opt",
                Some("global_val"),
                CmdOP::GlobalOpt
            );

            assert_eq_tokens_subcmd!(tokens, "subcmd");

            assert_eq_tokens_free!(tokens, 0, "-s");
            assert_eq_tokens_free!(tokens, 1, "sub_arg");
            assert_eq_tokens_free!(tokens, 2, "free1");
        }
        #[cfg(not(feature = "option_argument"))]
        {
            match lex.tokens_from(&args) {
                Ok(t) => {
                    assert_eq!(format!("{:?}", t), "");
                    unreachable!();
                }
                Err(e) => {
                    assert_eq!(format!("{}", e), "Invalid subcommand: global_val");
                }
            };
        }
    }
}
