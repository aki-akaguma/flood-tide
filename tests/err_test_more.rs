#[allow(unused_macros)]
#[macro_use]
mod helper;

mod err_test_more {
    use flood_tide::check;
    #[cfg(feature = "option_argument")]
    use flood_tide::Arg;
    use flood_tide::Lex;
    use flood_tide::Opt;
    use flood_tide::OptNum;

    #[cfg(not(feature = "long_only"))]
    #[test]
    fn tokens_invalid_option_error() {
        #[rustfmt::skip]
        let args = vec![
            "-a", "-x", "--long",
        ];

        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            Long = 2,
        }

        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }

        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'a',  lon: "",     has: Arg::No,  num: CmdOP::A.to(), },
            Opt { sho: b'\0', lon: "long", has: Arg::No,  num: CmdOP::Long.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'a',  lon: "",     num: CmdOP::A.to(), },
            Opt { sho: b'\0', lon: "long", num: CmdOP::Long.to(), },
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

        let tokens = lex.tokens_from(&args);

        match tokens {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                let expect = "Invalid option: x";
                assert_eq!(thing, expect);
            }
        }
    }

    #[cfg(not(feature = "long_only"))]
    #[cfg(feature = "abbreviate")]
    #[test]
    fn tokens_missing_option_argument_error() {
        #[rustfmt::skip]
        let args = vec![
            "-a", "-f", "--long",
        ];

        #[allow(dead_code)]
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            File = 2,
            Long = 3,
        }

        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }

        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'a',  lon: "",     has: Arg::No,  num: CmdOP::A.to(), },
            Opt { sho: b'f',  lon: "file", has: Arg::Yes, num: CmdOP::File.to(), },
            Opt { sho: b'\0', lon: "long", has: Arg::No,  num: CmdOP::Long.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'a',  lon: "",     num: CmdOP::A.to(), },
            Opt { sho: b'\0', lon: "long", num: CmdOP::Long.to(), },
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

        let tokens = lex.tokens_from(&args);

        #[cfg(feature = "option_argument")]
        match tokens {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                let expect = "Invalid option: f";
                assert_eq!(thing, expect);
            }
        }
        #[cfg(not(feature = "option_argument"))]
        match tokens {
            Ok(t) => {
                assert_eq!(format!("{:?}", t), "");
                unreachable!();
            }
            Err(e) => {
                let thing = format!("{}", e);
                let expect = "Invalid option: f";
                assert_eq!(thing, expect);
            }
        }
    }

    #[cfg(not(feature = "long_only"))]
    #[cfg(feature = "abbreviate")]
    #[test]
    fn tokens_ambiguous_abbreviation_error() {
        #[rustfmt::skip]
        let args = vec![
            "-a", "--fo",
        ];

        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            Foo = 2,
            Foobar = 3,
        }

        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }

        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'a',  lon: "",       has: Arg::No, num: CmdOP::A.to(), },
            Opt { sho: b'\0', lon: "foo",    has: Arg::No, num: CmdOP::Foo.to(), },
            Opt { sho: b'\0', lon: "foobar", has: Arg::No, num: CmdOP::Foobar.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'a',  lon: "",       num: CmdOP::A.to(), },
            Opt { sho: b'\0', lon: "foo",    num: CmdOP::Foo.to(), },
            Opt { sho: b'\0', lon: "foobar", num: CmdOP::Foobar.to(), },
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

        let tokens = lex.tokens_from(&args);

        match tokens {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                let expect = "Ambiguous option: fo: possibilities: '--foo' '--foobar'";
                assert_eq!(thing, expect);
            }
        }
    }

    #[cfg(feature = "long_only")]
    #[test]
    fn tokens_long_only_invalid_option_error() {
        #[rustfmt::skip]
        let args = vec![
            "-a", "-invalid-opt", "-b",
        ];

        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            B = 2,
        }

        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }

        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'\0', lon: "a", has: Arg::No, num: CmdOP::A.to(), },
            Opt { sho: b'\0', lon: "b", has: Arg::No, num: CmdOP::B.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'\0', lon: "a", num: CmdOP::A.to(), },
            Opt { sho: b'\0', lon: "b", num: CmdOP::B.to(), },
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

        let tokens = lex.tokens_from(&args);

        match tokens {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                let expect = "Invalid option: invalid-opt";
                assert_eq!(thing, expect);
            }
        }
    }

    #[cfg(all(feature = "long_only", feature = "abbreviate"))]
    #[test]
    fn tokens_long_only_ambiguous_abbreviation_error() {
        #[rustfmt::skip]
        let args = vec![
            "-f",
        ];

        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Foo = 1,
            Foobar = 2,
        }

        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }

        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'\0', lon: "foo",    has: Arg::No, num: CmdOP::Foo.to(), },
            Opt { sho: b'\0', lon: "foobar", has: Arg::No, num: CmdOP::Foobar.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'\0', lon: "foo",    num: CmdOP::Foo.to(), },
            Opt { sho: b'\0', lon: "foobar", num: CmdOP::Foobar.to(), },
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

        let tokens = lex.tokens_from(&args);

        match tokens {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                let expect = "Ambiguous option: f: possibilities: '--foo' '--foobar'";
                assert_eq!(thing, expect);
            }
        }
    }

    #[cfg(feature = "subcommand")]
    #[cfg(not(feature = "long_only"))]
    #[test]
    fn tokens_invalid_subcommand_error() {
        #[rustfmt::skip]
        let args = vec![
            "--global-opt", "val", "invalid-subcmd", "-s",
        ];

        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            GlobalOpt = 1,
        }

        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }

        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'\0', lon: "global-opt", has: Arg::Yes, num: CmdOP::GlobalOpt.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'\0', lon: "global-opt", num: CmdOP::GlobalOpt.to(), },
        ];

        #[rustfmt::skip]
        let opt_ary_sho_idx = [];

        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx).subcmd(&["subcmd"])
        };

        let tokens = lex.tokens_from(&args);

        match tokens {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                #[cfg(feature = "option_argument")]
                {
                    let expect = "Invalid subcommand: invalid-subcmd";
                    assert_eq!(thing, expect);
                }
                #[cfg(not(feature = "option_argument"))]
                {
                    let expect = "Invalid subcommand: val";
                    assert_eq!(thing, expect);
                }
            }
        }
    }

    #[cfg(all(feature = "subcommand", feature = "abbreviate"))]
    #[cfg(not(feature = "long_only"))]
    #[test]
    fn tokens_ambiguous_subcommand_error() {
        #[rustfmt::skip]
        let args = vec![
            "--global-opt", "val", "sub",
        ];

        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            GlobalOpt = 1,
        }

        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }

        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'\0', lon: "global-opt", has: Arg::Yes, num: CmdOP::GlobalOpt.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'\0', lon: "global-opt", num: CmdOP::GlobalOpt.to(), },
        ];

        #[rustfmt::skip]
        let opt_ary_sho_idx = [];

        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx).subcmd(&["subcmd1", "subcmd2"])
        };

        let tokens = lex.tokens_from(&args);

        match tokens {
            Ok(_) => unreachable!(),
            Err(e) => {
                let thing = format!("{}", e);
                #[cfg(feature = "option_argument")]
                let expect = "Ambiguous subcommand: sub: possibilities: 'subcmd1' 'subcmd2'";
                #[cfg(not(feature = "option_argument"))]
                let expect = "Invalid subcommand: val";
                assert_eq!(thing, expect);
            }
        }
    }
}
