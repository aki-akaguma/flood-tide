mod fmt_debug {
    use flood_tide::check;
    use flood_tide::Arg;
    use flood_tide::Lex;
    use flood_tide::NameVal;
    use flood_tide::Opt;
    use flood_tide::OptNum;
    use flood_tide::Tokens;
    //
    #[test]
    fn opt() {
        let opt = Opt {
            sho: b'n',
            lon: "name",
            #[cfg(feature = "option_argument")]
            has: Arg::Yes,
            num: 1,
        };
        //
        let thing = format!("{:?}", opt);
        #[cfg(feature = "option_argument")]
        let expect = "Opt { sho: 110, lon: \"name\", has: Yes, num: 1 }";
        #[cfg(not(feature = "option_argument"))]
        let expect = "Opt { sho: 110, lon: \"name\", num: 1 }";
        assert_eq!(thing, expect);
    }
    #[test]
    fn has_arg() {
        assert_eq!(format!("{:?}", Arg::No), "No");
        assert_eq!(format!("{:?}", Arg::Yes), "Yes");
        assert_eq!(format!("{:?}", Arg::Maybe), "Maybe");
    }
    #[test]
    fn name_val() {
        let opt = Opt {
            sho: b'n',
            lon: "name",
            #[cfg(feature = "option_argument")]
            has: Arg::Yes,
            num: 1,
        };
        let nv = NameVal {
            opt: &opt,
            #[cfg(feature = "option_argument")]
            val: Some("value"),
            #[cfg(feature = "was_long")]
            was_long: true,
        };
        //
        let thing = format!("{:?}", nv);
        let expect = "NameVal { opt: Opt { sho: 110, lon: \"name\"".to_string();
        #[cfg(feature = "option_argument")]
        let expect = expect + ", has: Yes";
        let expect = expect + ", num: 1 }";
        #[cfg(feature = "option_argument")]
        let expect = expect + ", val: Some(\"value\")";
        #[cfg(feature = "was_long")]
        let expect = expect + ", was_long: true";
        let expect = expect + " }";
        assert_eq!(thing, expect);
    }
    #[test]
    fn name_val_name() {
        let opt = Opt {
            sho: b'n',
            lon: "name",
            #[cfg(feature = "option_argument")]
            has: Arg::Yes,
            num: 1,
        };
        let nv = NameVal {
            opt: &opt,
            #[cfg(feature = "option_argument")]
            val: Some("value"),
            #[cfg(feature = "was_long")]
            was_long: false,
        };
        //
        let thing = nv.name();
        #[cfg(not(feature = "was_long"))]
        let expect = "name";
        #[cfg(feature = "was_long")]
        let expect = "n";
        assert_eq!(thing, expect);
    }
    #[test]
    fn tokens() {
        #[rustfmt::skip]
        let opt1 = Opt { sho: b'a', lon: "name1",
            #[cfg(feature = "option_argument")]
            has: Arg::Yes,
            num: 1, };
        #[rustfmt::skip]
        let opt2 = Opt { sho: b'b', lon: "name2",
            #[cfg(feature = "option_argument")]
            has: Arg::Yes,
            num: 2, };
        #[rustfmt::skip]
        let opt3 = Opt { sho: b'c', lon: "name3",
            #[cfg(feature = "option_argument")]
            has: Arg::No,
            num: 3, };
        //
        let tks = Tokens {
            namevals: vec![
                NameVal {
                    opt: &opt1,
                    #[cfg(feature = "option_argument")]
                    val: Some("value1"),
                    #[cfg(feature = "was_long")]
                    was_long: true,
                },
                NameVal {
                    opt: &opt2,
                    #[cfg(feature = "option_argument")]
                    val: Some("value2"),
                    #[cfg(feature = "was_long")]
                    was_long: true,
                },
                NameVal {
                    opt: &opt3,
                    #[cfg(feature = "option_argument")]
                    val: None,
                    #[cfg(feature = "was_long")]
                    was_long: false,
                },
            ],
            free: vec!["free1", "free2", "free3"],
            #[cfg(feature = "subcommand")]
            subcmd: Some("command"),
            #[cfg(feature = "stop_at_mm")]
            double_m: true,
        };
        //
        let thing = format!("{:?}", tks);
        let subcmd = {
            #[cfg(feature = "subcommand")]
            {
                " subcmd: Some(\"command\"),"
            }
            #[cfg(not(feature = "subcommand"))]
            {
                ""
            }
        };
        let expect = "Tokens { namevals: ".to_string();
        #[cfg(feature = "option_argument")]
        #[cfg(feature = "was_long")]
        let expect = expect + concat!(
            "[NameVal { opt: Opt { sho: 97, lon: \"name1\", has: Yes, num: 1 }, val: Some(\"value1\"), was_long: true },",
            " NameVal { opt: Opt { sho: 98, lon: \"name2\", has: Yes, num: 2 }, val: Some(\"value2\"), was_long: true },",
            " NameVal { opt: Opt { sho: 99, lon: \"name3\", has: No, num: 3 }, val: None, was_long: false }",
        );
        #[cfg(feature = "option_argument")]
        #[cfg(not(feature = "was_long"))]
        let expect = expect + concat!(
            "[NameVal { opt: Opt { sho: 97, lon: \"name1\", has: Yes, num: 1 }, val: Some(\"value1\") },",
            " NameVal { opt: Opt { sho: 98, lon: \"name2\", has: Yes, num: 2 }, val: Some(\"value2\") },",
            " NameVal { opt: Opt { sho: 99, lon: \"name3\", has: No, num: 3 }, val: None }",
        );
        #[cfg(not(feature = "option_argument"))]
        #[cfg(feature = "was_long")]
        let expect = expect
            + concat!(
                "[NameVal { opt: Opt { sho: 97, lon: \"name1\", num: 1 }, was_long: true },",
                " NameVal { opt: Opt { sho: 98, lon: \"name2\", num: 2 }, was_long: true },",
                " NameVal { opt: Opt { sho: 99, lon: \"name3\", num: 3 }, was_long: false }",
            );
        #[cfg(not(feature = "option_argument"))]
        #[cfg(not(feature = "was_long"))]
        let expect = expect
            + concat!(
                "[NameVal { opt: Opt { sho: 97, lon: \"name1\", num: 1 } },",
                " NameVal { opt: Opt { sho: 98, lon: \"name2\", num: 2 } },",
                " NameVal { opt: Opt { sho: 99, lon: \"name3\", num: 3 } }",
            );
        let expect = expect + "],";
        #[cfg(feature = "stop_at_mm")]
        let expect = expect + " double_m: true,";
        let expect = expect + subcmd + " free: [\"free1\", \"free2\", \"free3\"] }";
        assert_eq!(thing, expect);
    }
    #[test]
    fn lex() {
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Name = 1,
            Section = 2,
            Info = 3,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[cfg(feature = "option_argument")]
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'i', lon: "info",    has: Arg::No,    num: CmdOP::Info.to(), },
            Opt { sho: b'n', lon: "name",    has: Arg::Yes,   num: CmdOP::Name.to(), },
            Opt { sho: b's', lon: "section", has: Arg::Maybe, num: CmdOP::Section.to(), },
        ];
        #[cfg(not(feature = "option_argument"))]
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'i', lon: "info",    num: CmdOP::Info.to(), },
            Opt { sho: b'n', lon: "name",    num: CmdOP::Name.to(), },
            Opt { sho: b's', lon: "section", num: CmdOP::Section.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'i',0),(b'n',1),(b's',2)];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };
        //
        let thing = format!("{:?}", lex);
        //
        let subcmd = {
            #[cfg(feature = "subcommand")]
            {
                ", subcmds: []"
            }
            #[cfg(not(feature = "subcommand"))]
            {
                ""
            }
        };
        //
        #[cfg(feature = "option_argument")]
        let opts_s = concat!(
            "opts: ",
            "[Opt { sho: 105, lon: \"info\", has: No, num: 3 },",
            " Opt { sho: 110, lon: \"name\", has: Yes, num: 1 },",
            " Opt { sho: 115, lon: \"section\", has: Maybe, num: 2 }]",
        );
        #[cfg(not(feature = "option_argument"))]
        let opts_s = concat!(
            "opts: ",
            "[Opt { sho: 105, lon: \"info\", num: 3 },",
            " Opt { sho: 110, lon: \"name\", num: 1 },",
            " Opt { sho: 115, lon: \"section\", num: 2 }]",
        );
        let sho_idx_s = ", sho_idx: [(105, 0), (110, 1), (115, 2)]";
        let expect = "Lex { ".to_string() + opts_s + sho_idx_s + subcmd + " }";
        assert_eq!(thing, expect);
    }
    #[cfg(feature = "long_only")]
    #[test]
    fn lex_single() {
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Name = 1,
            Section = 2,
            Info = 3,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[cfg(feature = "option_argument")]
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'i', lon: "info",    has: Arg::No,    num: CmdOP::Info.to(), },
            Opt { sho: b'n', lon: "name",    has: Arg::Yes,   num: CmdOP::Name.to(), },
            Opt { sho: b's', lon: "section", has: Arg::Maybe, num: CmdOP::Section.to(), },
        ];
        #[cfg(not(feature = "option_argument"))]
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'i', lon: "info",    num: CmdOP::Info.to(), },
            Opt { sho: b'n', lon: "name",    num: CmdOP::Name.to(), },
            Opt { sho: b's', lon: "section", num: CmdOP::Section.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'i',0),(b'n',1),(b's',2)];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx)
        };
        //
        let thing = format!("{:?}", lex);
        //
        let subcmd = {
            #[cfg(feature = "subcommand")]
            {
                ", subcmds: []"
            }
            #[cfg(not(feature = "subcommand"))]
            {
                ""
            }
        };
        #[cfg(feature = "option_argument")]
        let opts_s = concat!(
            "opts: ",
            "[Opt { sho: 105, lon: \"info\", has: No, num: 3 },",
            " Opt { sho: 110, lon: \"name\", has: Yes, num: 1 },",
            " Opt { sho: 115, lon: \"section\", has: Maybe, num: 2 }]",
        );
        #[cfg(not(feature = "option_argument"))]
        let opts_s = concat!(
            "opts: ",
            "[Opt { sho: 105, lon: \"info\", num: 3 },",
            " Opt { sho: 110, lon: \"name\", num: 1 },",
            " Opt { sho: 115, lon: \"section\", num: 2 }]",
        );
        let sho_idx_s = ", sho_idx: [(105, 0), (110, 1), (115, 2)]";
        let expect = "Lex { ".to_string() + opts_s + sho_idx_s + subcmd + " }";
        assert_eq!(thing, expect);
    }
    #[cfg(feature = "subcommand")]
    #[test]
    fn lex_sub_cmd() {
        //
        #[repr(u8)]
        #[derive(Debug, PartialEq)]
        enum CmdOP {
            Name = 1,
            Section = 2,
            Info = 3,
        }
        impl CmdOP {
            pub const fn to(self) -> OptNum {
                self as OptNum
            }
        }
        //
        #[cfg(feature = "option_argument")]
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'i', lon: "info",    has: Arg::No,    num: CmdOP::Info.to(), },
            Opt { sho: b'n', lon: "name",    has: Arg::Yes,   num: CmdOP::Name.to(), },
            Opt { sho: b's', lon: "section", has: Arg::Maybe, num: CmdOP::Section.to(), },
        ];
        #[cfg(not(feature = "option_argument"))]
        #[rustfmt::skip]
        let opt_ary = [
            Opt { sho: b'i', lon: "info",    num: CmdOP::Info.to(), },
            Opt { sho: b'n', lon: "name",    num: CmdOP::Name.to(), },
            Opt { sho: b's', lon: "section", num: CmdOP::Section.to(), },
        ];
        #[rustfmt::skip]
        let opt_ary_sho_idx = [(b'i',0),(b'n',1),(b's',2)];
        //
        let lex = {
            assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
                &opt_ary,
                &opt_ary_sho_idx
            ));
            Lex::create_with(&opt_ary, &opt_ary_sho_idx).subcmd(&["sub_command"])
        };
        //
        let thing = format!("{:?}", lex);
        //
        let subcmd = ", subcmds: [\"sub_command\"]";
        //
        #[cfg(feature = "option_argument")]
        let opts_s = concat!(
            "opts: ",
            "[Opt { sho: 105, lon: \"info\", has: No, num: 3 },",
            " Opt { sho: 110, lon: \"name\", has: Yes, num: 1 },",
            " Opt { sho: 115, lon: \"section\", has: Maybe, num: 2 }]",
        );
        #[cfg(not(feature = "option_argument"))]
        let opts_s = concat!(
            "opts: ",
            "[Opt { sho: 105, lon: \"info\", num: 3 },",
            " Opt { sho: 110, lon: \"name\", num: 1 },",
            " Opt { sho: 115, lon: \"section\", num: 2 }]",
        );
        let sho_idx_s = ", sho_idx: [(105, 0), (110, 1), (115, 2)]";
        let expect = "Lex { ".to_string() + opts_s + sho_idx_s + subcmd + " }";
        assert_eq!(thing, expect);
    }
}
