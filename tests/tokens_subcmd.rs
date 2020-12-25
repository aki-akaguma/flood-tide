#[allow(unused_macros)]
#[macro_use]
mod test_macro;

#[cfg(feature = "option_argument")]
#[cfg(feature = "subcommand")]
mod subcommand {
    use flood_tide::check;
    use flood_tide::Arg;
    use flood_tide::Lex;
    use flood_tide::Opt;
    use flood_tide::OptNum;
    //
    #[test]
    fn tokens_subcommand() {
        #[cfg(not(feature = "long_only"))]
        #[rustfmt::skip]
        let args = vec!["-a", "val1", "--long2=val2", "-c", "cmd1"];
        #[cfg(feature = "long_only")]
        #[rustfmt::skip]
        let args = vec!["-a", "val1", "-long2=val2", "-c", "cmd1"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long1 = 1,
            Long2 = 2,
            Long3 = 3,
        };
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'a', lon: "long1", has: Arg::Yes, num: CmdOP::Long1.to(), },
            Opt { sho: 0u8,  lon: "long2", has: Arg::Yes, num: CmdOP::Long2.to(), },
            Opt { sho: b'c', lon: "long3", has: Arg::No,  num: CmdOP::Long3.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0),(b'c',2)];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx).subcmd(&["cmd1", "cmd2", "cmd3"])
        };
        let tokens = match lex.tokens_from(&args) {
            Ok(t) => t,
            Err(e) => {
                assert_eq!(format!("{}", e), "");
                unreachable!();
            } //_ => unreachable!(),
        };
        //
        assert_eq_tokens_namevals!(tokens, 0, b'a', "long1", Some("val1"), CmdOP::Long1);
        assert_eq_tokens_namevals!(tokens, 1, 0u8, "long2", Some("val2"), CmdOP::Long2);
        assert_eq_tokens_namevals!(tokens, 2, b'c', "long3", None, CmdOP::Long3);
        assert_eq_tokens_subcmd!(tokens, "cmd1");
        //
        #[cfg(feature = "stop_at_mm")]
        assert_eq!(tokens.double_m, false);
    }
    #[test]
    fn tokens_subcommand_invalid_subcommand() {
        #[cfg(not(feature = "long_only"))]
        #[rustfmt::skip]
        let args = vec!["-a", "val1", "--long2=val2", "-c", "cmd4"];
        #[cfg(feature = "long_only")]
        #[rustfmt::skip]
        let args = vec!["-a", "val1", "-long2=val2", "-c", "cmd4"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long1 = 1,
            Long2 = 2,
            Long3 = 3,
        };
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'a', lon: "long1", has: Arg::Yes, num: CmdOP::Long1.to(), },
            Opt { sho: 0u8,  lon: "long2", has: Arg::Yes, num: CmdOP::Long2.to(), },
            Opt { sho: b'c', lon: "long3", has: Arg::No,  num: CmdOP::Long3.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0),(b'c',2)];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx).subcmd(&["cmd1", "cmd2", "cmd3"])
        };
        let _tokens = match lex.tokens_from(&args) {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                let expect = "Invalid subcommand: cmd4";
                assert_eq!(thing, expect);
            }
        };
    }
    #[test]
    fn tokens_subcommand_ok_nealy_missing_subcommand() {
        #[cfg(not(feature = "long_only"))]
        #[rustfmt::skip]
        let args = vec!["-a", "val1", "--long2=val2", "-c"];
        #[cfg(feature = "long_only")]
        #[rustfmt::skip]
        let args = vec!["-a", "val1", "-long2=val2", "-c"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long1 = 1,
            Long2 = 2,
            Long3 = 3,
        };
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'a', lon: "long1", has: Arg::Yes, num: CmdOP::Long1.to(), },
            Opt { sho: 0u8,  lon: "long2", has: Arg::Yes, num: CmdOP::Long2.to(), },
            Opt { sho: b'c', lon: "long3", has: Arg::No,  num: CmdOP::Long3.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0),(b'c',2)];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx).subcmd(&["cmd1", "cmd2", "cmd3"])
        };
        let tokens = match lex.tokens_from(&args) {
            Ok(t) => t,
            Err(e) => {
                assert_eq!(format!("{}", e), "");
                unreachable!();
            } //_ => unreachable!(),
        };
        //
        assert_eq_tokens_namevals!(tokens, 0, b'a', "long1", Some("val1"), CmdOP::Long1);
        assert_eq_tokens_namevals!(tokens, 1, 0u8, "long2", Some("val2"), CmdOP::Long2);
        assert_eq_tokens_namevals!(tokens, 2, b'c', "long3", None, CmdOP::Long3);
        assert_eq!(tokens.subcmd, None);
        //
        #[cfg(feature = "stop_at_mm")]
        assert_eq!(tokens.double_m, false);
    }
    #[test]
    fn tokens_subcommand_abbreviate() {
        #[cfg(not(feature = "long_only"))]
        #[rustfmt::skip]
        let args = vec!["-a", "val1", "--long2=val2", "-c", "c"];
        #[cfg(feature = "long_only")]
        #[rustfmt::skip]
        let args = vec!["-a", "val1", "-long2=val2", "-c", "c"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long1 = 1,
            Long2 = 2,
            Long3 = 3,
        };
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'a', lon: "long1", has: Arg::Yes, num: CmdOP::Long1.to(), },
            Opt { sho: 0u8,  lon: "long2", has: Arg::Yes, num: CmdOP::Long2.to(), },
            Opt { sho: b'c', lon: "long3", has: Arg::No,  num: CmdOP::Long3.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0),(b'c',2)];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx).subcmd(&["cmd1"])
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
            //
            assert_eq_tokens_namevals!(tokens, 0, b'a', "long1", Some("val1"), CmdOP::Long1);
            assert_eq_tokens_namevals!(tokens, 1, 0u8, "long2", Some("val2"), CmdOP::Long2);
            assert_eq_tokens_namevals!(tokens, 2, b'c', "long3", None, CmdOP::Long3);
            assert_eq_tokens_subcmd!(tokens, "cmd1");
            //
            #[cfg(feature = "stop_at_mm")]
            assert_eq!(tokens.double_m, false);
        };
        #[cfg(not(feature = "abbreviate"))]
        {
            let _tokens = match lex.tokens_from(&args) {
                Ok(_) => unreachable!(),
                Err(e) => {
                    let thing = format!("{}", e);
                    let expect = "Invalid subcommand: c";
                    assert_eq!(thing, expect);
                }
            };
        };
    }
    #[test]
    fn tokens_subcommand_ambiguous() {
        #[cfg(not(feature = "long_only"))]
        #[rustfmt::skip]
        let args = vec!["-a", "val1", "--long2=val2", "-c", "c"];
        #[cfg(feature = "long_only")]
        #[rustfmt::skip]
        let args = vec!["-a", "val1", "-long2=val2", "-c", "c"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long1 = 1,
            Long2 = 2,
            Long3 = 3,
        };
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'a', lon: "long1", has: Arg::Yes, num: CmdOP::Long1.to(), },
            Opt { sho: 0u8,  lon: "long2", has: Arg::Yes, num: CmdOP::Long2.to(), },
            Opt { sho: b'c', lon: "long3", has: Arg::No,  num: CmdOP::Long3.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0),(b'c',2)];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx).subcmd(&["cmd1", "cmd2", "cmd3"])
        };
        #[cfg(feature = "abbreviate")]
        let _tokens = match lex.tokens_from(&args) {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                let expect = "Ambiguous subcommand: c: possibilities: \'cmd1\' \'cmd2\' \'cmd3\'";
                assert_eq!(thing, expect);
            }
        };
        #[cfg(not(feature = "abbreviate"))]
        let _tokens = match lex.tokens_from(&args) {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                let expect = "Invalid subcommand: c";
                assert_eq!(thing, expect);
            }
        };
    }
    #[test]
    fn tokens_subcommand_empty() {
        #[cfg(not(feature = "long_only"))]
        #[rustfmt::skip]
        let args = vec!["-a", "val1", "--long2=val2", "-c", ""];
        #[cfg(feature = "long_only")]
        #[rustfmt::skip]
        let args = vec!["-a", "val1", "-long2=val2", "-c", ""];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long1 = 1,
            Long2 = 2,
            Long3 = 3,
        };
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'a', lon: "long1", has: Arg::Yes, num: CmdOP::Long1.to(), },
            Opt { sho: 0u8,  lon: "long2", has: Arg::Yes, num: CmdOP::Long2.to(), },
            Opt { sho: b'c', lon: "long3", has: Arg::No,  num: CmdOP::Long3.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0),(b'c',2)];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx).subcmd(&["cmd1", "cmd2", "cmd3"])
        };
        let tokens = match lex.tokens_from(&args) {
            Ok(t) => t,
            Err(e) => {
                assert_eq!(format!("{}", e), "");
                unreachable!();
            } //_ => unreachable!(),
        };
        //
        assert_eq_tokens_namevals!(tokens, 0, b'a', "long1", Some("val1"), CmdOP::Long1);
        assert_eq_tokens_namevals!(tokens, 1, 0u8, "long2", Some("val2"), CmdOP::Long2);
        assert_eq_tokens_namevals!(tokens, 2, b'c', "long3", None, CmdOP::Long3);
        assert_eq!(tokens.subcmd, None);
        //
        #[cfg(feature = "stop_at_mm")]
        assert_eq!(tokens.double_m, false);
    }
    #[test]
    fn tokens_subcommand_invalid_opt() {
        #[cfg(not(feature = "long_only"))]
        #[rustfmt::skip]
        let args = vec!["-x", "val1", "--long2=val2", "-c", ""];
        #[cfg(feature = "long_only")]
        #[rustfmt::skip]
        let args = vec!["-x", "val1", "-long2=val2", "-c", ""];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Long1 = 1,
            Long2 = 2,
            Long3 = 3,
        };
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'a', lon: "long1", has: Arg::Yes, num: CmdOP::Long1.to(), },
            Opt { sho: 0u8,  lon: "long2", has: Arg::Yes, num: CmdOP::Long2.to(), },
            Opt { sho: b'c', lon: "long3", has: Arg::No,  num: CmdOP::Long3.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0),(b'c',2)];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx).subcmd(&["cmd1", "cmd2", "cmd3"])
        };
        let _tokens = match lex.tokens_from(&args) {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                let expect = "Invalid option: x";
                assert_eq!(thing, expect);
            }
        };
    }
} // mod subcommand
