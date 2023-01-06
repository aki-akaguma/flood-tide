#[allow(unused_macros)]
#[macro_use]
mod test_macro;

mod basic {
    use flood_tide::check;
    use flood_tide::Arg;
    use flood_tide::Lex;
    use flood_tide::NameVal;
    use flood_tide::Opt;
    use flood_tide::OptNum;
    use flood_tide::Tokens;
    //
    #[test]
    fn size_of() {
        #[cfg(target_pointer_width = "64")]
        {
            assert_eq!(std::mem::size_of::<&str>(), 16);
            assert_eq!(std::mem::size_of::<Option<&str>>(), 16);
            assert_eq!(std::mem::size_of::<String>(), 24);
            assert_eq!(std::mem::size_of::<Option<String>>(), 24);
            assert_eq!(std::mem::size_of::<Vec<&str>>(), 24);
            assert_eq!(std::mem::size_of::<Opt>(), 24);
        }
        #[cfg(target_pointer_width = "32")]
        {
            assert_eq!(std::mem::size_of::<&str>(), 8);
            assert_eq!(std::mem::size_of::<Option<&str>>(), 8);
            assert_eq!(std::mem::size_of::<String>(), 12);
            assert_eq!(std::mem::size_of::<Option<String>>(), 12);
            assert_eq!(std::mem::size_of::<Vec<&str>>(), 12);
            assert_eq!(std::mem::size_of::<Opt>(), 12);
        }
        //
        assert_eq!(std::mem::size_of::<Arg>(), 1);
        //
        #[cfg(target_pointer_width = "64")]
        {
            let len = 8;
            #[cfg(feature = "option_argument")]
            let len = len + 16;
            #[cfg(feature = "was_long")]
            let len = len + 8;
            assert_eq!(std::mem::size_of::<NameVal>(), len);
        }
        #[cfg(target_pointer_width = "32")]
        {
            let len = 4;
            #[cfg(feature = "option_argument")]
            let len = len + 8;
            #[cfg(feature = "was_long")]
            let len = len + 4;
            assert_eq!(std::mem::size_of::<NameVal>(), len);
        }
        //
        #[cfg(target_pointer_width = "64")]
        {
            let len = 48;
            #[cfg(feature = "stop_at_mm")]
            let len = len + 8;
            #[cfg(feature = "subcommand")]
            let len = len + 16;
            assert_eq!(std::mem::size_of::<Tokens>(), len);
        }
        #[cfg(target_pointer_width = "32")]
        {
            let len = 24;
            #[cfg(feature = "stop_at_mm")]
            let len = len + 4;
            #[cfg(feature = "subcommand")]
            let len = len + 8;
            assert_eq!(std::mem::size_of::<Tokens>(), len);
        }
    }

    //{{{ complex tests
    #[cfg(not(feature = "long_only"))]
    #[test]
    fn tokens_complex() {
        //
        #[rustfmt::skip]
        let args = vec![
            "-ab", "--barn", "-cd", "--date=1029", "--eat", "jum",
            "-f", "other1", "other2",
        ];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            A = 1,
            Barn = 2,
            C = 3,
            Date = 4,
            Eat = 5,
            F = 6,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        assert_eq!(std::mem::size_of::<CmdOP>(), 1);
        //
        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b'a',  lon: "",     has: Arg::No,  num: CmdOP::A.to(), },
            Opt { sho: b'c',  lon: "",     has: Arg::Yes, num: CmdOP::C.to(), },
            Opt { sho: b'f',  lon: "",     has: Arg::No,  num: CmdOP::F.to(), },
            Opt { sho: b'b',  lon: "barn", has: Arg::No,  num: CmdOP::Barn.to(), },
            Opt { sho: b'd',  lon: "date", has: Arg::Yes, num: CmdOP::Date.to(), },
            Opt { sho: b'\0', lon: "eat",  has: Arg::Yes, num: CmdOP::Eat.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b'a',  lon: "",     num: CmdOP::A.to(), },
            Opt { sho: b'c',  lon: "",     num: CmdOP::C.to(), },
            Opt { sho: b'f',  lon: "",     num: CmdOP::F.to(), },
            Opt { sho: b'b',  lon: "barn", num: CmdOP::Barn.to(), },
            Opt { sho: b'd',  lon: "date", num: CmdOP::Date.to(), },
            Opt { sho: b'\0', lon: "eat",  num: CmdOP::Eat.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'a',0),(b'b',3),(b'c',1),(b'd',4),(b'f',2)];
        //
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
                } //_ => unreachable!(),
            };
            assert_eq_tokens_namevals!(tokens, 0, b'a', "", None, CmdOP::A);
            assert_eq_tokens_namevals!(tokens, 1, b'b', "barn", None, CmdOP::Barn);
            assert_eq_tokens_namevals!(tokens, 2, b'b', "barn", None, CmdOP::Barn);
            assert_eq_tokens_namevals!(tokens, 3, b'c', "", Some("d"), CmdOP::C);
            assert_eq_tokens_namevals!(tokens, 4, b'd', "date", Some("1029"), CmdOP::Date);
            assert_eq_tokens_namevals!(tokens, 5, 0u8, "eat", Some("jum"), CmdOP::Eat);
            assert_eq_tokens_namevals!(tokens, 6, b'f', "", None, CmdOP::F);
            //
            assert_eq_tokens_free!(tokens, 0, "other1");
            assert_eq_tokens_free!(tokens, 1, "other2");
            //
            #[cfg(feature = "stop_at_mm")]
            assert_eq!(tokens.double_m, false);
        }
        #[cfg(not(feature = "option_argument"))]
        let _tokens = match lex.tokens_from(&args) {
            Ok(t) => {
                assert_eq!(format!("{:?}", t), "");
                unreachable!();
            }
            Err(e) => {
                assert_eq!(format!("{}", e), "Invalid option: date=1029");
            }
        };
    }
    #[cfg(feature = "long_only")]
    #[test]
    fn tokens_complex() {
        //
        #[rustfmt::skip]
        let args = vec![
            "-ab", "-barn", "-cd", "-date=1029", "-eat", "jum",
            "-f", "other1", "other2",
        ];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Ab = 1,
            Barn = 2,
            Cd = 3,
            Date = 4,
            Eat = 5,
            F = 6,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        assert_eq!(std::mem::size_of::<CmdOP>(), 1);
        //
        #[rustfmt::skip]
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: 0u8, lon: "ab",   has: Arg::No,  num: CmdOP::Ab.to(), },
            Opt { sho: 0u8, lon: "barn", has: Arg::No,  num: CmdOP::Barn.to(), },
            Opt { sho: 0u8, lon: "cd",   has: Arg::No,  num: CmdOP::Cd.to(), },
            Opt { sho: 0u8, lon: "date", has: Arg::Yes, num: CmdOP::Date.to(), },
            Opt { sho: 0u8, lon: "eat",  has: Arg::Yes, num: CmdOP::Eat.to(), },
            Opt { sho: 0u8, lon: "f",    has: Arg::No,  num: CmdOP::F.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: 0u8, lon: "ab",   num: CmdOP::Ab.to(), },
            Opt { sho: 0u8, lon: "barn", num: CmdOP::Barn.to(), },
            Opt { sho: 0u8, lon: "cd",   num: CmdOP::Cd.to(), },
            Opt { sho: 0u8, lon: "date", num: CmdOP::Date.to(), },
            Opt { sho: 0u8, lon: "eat",  num: CmdOP::Eat.to(), },
            Opt { sho: 0u8, lon: "f",    num: CmdOP::F.to(), },
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
        #[cfg(feature = "option_argument")]
        {
            let tokens = match lex.tokens_from(&args) {
                Ok(t) => t,
                Err(e) => {
                    assert_eq!(format!("{}", e), "");
                    unreachable!();
                } //_ => unreachable!(),
            };
            assert_eq_tokens_namevals!(tokens, 0, 0u8, "ab", None, CmdOP::Ab);
            assert_eq_tokens_namevals!(tokens, 1, 0u8, "barn", None, CmdOP::Barn);
            assert_eq_tokens_namevals!(tokens, 2, 0u8, "cd", None, CmdOP::Cd);
            assert_eq_tokens_namevals!(tokens, 3, 0u8, "date", Some("1029"), CmdOP::Date);
            assert_eq_tokens_namevals!(tokens, 4, 0u8, "eat", Some("jum"), CmdOP::Eat);
            assert_eq_tokens_namevals!(tokens, 5, 0u8, "f", None, CmdOP::F);
            //
            assert_eq_tokens_free!(tokens, 0, "other1");
            assert_eq_tokens_free!(tokens, 1, "other2");
            //
            #[cfg(feature = "stop_at_mm")]
            assert_eq!(tokens.double_m, false);
        }
        #[cfg(not(feature = "option_argument"))]
        let _tokens = match lex.tokens_from(&args) {
            Ok(t) => {
                assert_eq!(format!("{:?}", t), "");
                unreachable!();
            }
            Err(e) => {
                assert_eq!(format!("{}", e), "Invalid option: date=1029");
            }
        };
    }
    //}}} complex tests
    //
    //
    //{{{ maybe fix
    #[cfg(not(feature = "long_only"))]
    #[test]
    fn tokens_val_tab() {
        #[rustfmt::skip]
        let args = vec!["-s", "\t", "--long2=\t", "--long3", "\t"];
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            S = 1,
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
        #[cfg(feature = "option_argument")]
        let opt_ary = [
            Opt { sho: b's',  lon: "",      has: Arg::Yes, num: CmdOP::S.to(), },
            Opt { sho: b'\0', lon: "long2", has: Arg::Yes, num: CmdOP::Long2.to(), },
            Opt { sho: b'\0', lon: "long3", has: Arg::Yes, num: CmdOP::Long3.to(), },
        ];
        #[rustfmt::skip]
        #[cfg(not(feature = "option_argument"))]
        let opt_ary = [
            Opt { sho: b's',  lon: "",      num: CmdOP::S.to(), },
            Opt { sho: b'\0', lon: "long2", num: CmdOP::Long2.to(), },
            Opt { sho: b'\0', lon: "long3", num: CmdOP::Long3.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b's',0)];
        //
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
            //
            assert_eq_tokens_namevals!(tokens, 0, b's', "", Some("\t"), CmdOP::S);
            assert_eq_tokens_namevals!(tokens, 1, 0u8, "long2", Some("\t"), CmdOP::Long2);
            assert_eq_tokens_namevals!(tokens, 2, 0u8, "long3", Some("\t"), CmdOP::Long3);
            //
            #[cfg(feature = "stop_at_mm")]
            assert_eq!(tokens.double_m, false);
        }
        #[cfg(not(feature = "option_argument"))]
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
                //
                assert_eq_tokens_namevals!(tokens, 0, b's', "", None, CmdOP::S);
                assert_eq_tokens_free!(tokens, 0, "\t");
                assert_eq_tokens_free!(tokens, 1, "--long2=\t");
                assert_eq_tokens_free!(tokens, 2, "--long3");
                assert_eq_tokens_free!(tokens, 3, "\t");
                //
                #[cfg(feature = "stop_at_mm")]
                assert_eq!(tokens.double_m, false);
            }
            #[cfg(not(feature = "stop_at_free"))]
            let _tokens = match lex.tokens_from(&args) {
                Ok(t) => {
                    assert_eq!(format!("{:?}", t), "");
                    unreachable!();
                }
                Err(e) => {
                    assert_eq!(format!("{}", e), "Invalid option: long2=\t");
                }
            };
        }
    }
    //}}}
} // mod plain
