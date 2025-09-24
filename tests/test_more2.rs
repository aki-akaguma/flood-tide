#[allow(unused_macros)]
#[macro_use]
mod helper;

#[cfg(feature = "stop_at_free")]
mod test_more2 {
    use flood_tide::check;
    #[cfg(feature = "option_argument")]
    use flood_tide::Arg;
    use flood_tide::Lex;
    use flood_tide::Opt;
    use flood_tide::OptNum;

    #[cfg(feature = "stop_at_free")]
    #[test]
    fn tokens_stop_at_free_feature() {
        #[rustfmt::skip]
        let args = vec![
            "-a", "free1", "-b", "--long-opt", "free2",
        ];

        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            B = 2,
            LongOpt = 3,
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
            Opt { sho: b'b',  lon: "",         has: Arg::No,  num: CmdOP::B.to(), },
            Opt { sho: b'\0', lon: "long-opt", has: Arg::No,  num: CmdOP::LongOpt.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'a',  lon: "",         num: CmdOP::A.to(), },
            Opt { sho: b'b',  lon: "",         num: CmdOP::B.to(), },
            Opt { sho: b'\0', lon: "long-opt", num: CmdOP::LongOpt.to(), },
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

        assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);

        assert_eq_tokens_free!(tokens, 0, "free1");
        assert_eq_tokens_free!(tokens, 1, "-b");
        assert_eq_tokens_free!(tokens, 2, "--long-opt");
        assert_eq_tokens_free!(tokens, 3, "free2");

        #[cfg(feature = "stop_at_mm")]
        assert!(!tokens.double_m);
    }
}
