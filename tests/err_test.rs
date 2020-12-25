// OptParseError
mod test_0 {
    use flood_tide::err::OptParseError;
    #[cfg(feature = "option_argument")]
    use flood_tide::err::OptParseErrorKind;
    //
    #[test]
    fn test_invalid_option() {
        let err = OptParseError::invalid_option("--abc");
        let thing = format!("{}", err);
        let expect = "Invalid option: --abc";
        assert_eq!(thing, expect);
    }
    #[test]
    fn test_missing_option() {
        let err = OptParseError::missing_option("--abc");
        let thing = format!("{}", err);
        let expect = "Missing option: --abc";
        assert_eq!(thing, expect);
    }
    //
    #[cfg(feature = "option_argument")]
    #[test]
    fn test_attr() {
        let err = OptParseError::unexpected_option_argument("--abc", "defg");
        assert_eq!(err.kind(), OptParseErrorKind::UnexpectedOptionArgument);
        assert_eq!(err.desc1_str(), "--abc");
        assert_eq!(err.to_string(), "Unexpected option argument: --abc: defg");
    }
    #[cfg(feature = "option_argument")]
    #[test]
    fn test_invalid_option_argument() {
        let err = OptParseError::invalid_option_argument("--abc", "invalid float literal");
        let thing = format!("{}", err);
        let expect = "Invalid option argument: --abc: invalid float literal";
        assert_eq!(thing, expect);
    }
    #[cfg(feature = "option_argument")]
    #[test]
    fn test_unexpected_option_argument() {
        let err = OptParseError::unexpected_option_argument("--abc", "defg");
        let thing = format!("{}", err);
        let expect = "Unexpected option argument: --abc: defg";
        assert_eq!(thing, expect);
    }
    #[cfg(feature = "option_argument")]
    #[test]
    fn test_missing_option_argument() {
        let err = OptParseError::missing_option_argument("--abc");
        let thing = format!("{}", err);
        let expect = "Missing option argument: --abc";
        assert_eq!(thing, expect);
    }
    //
    #[cfg(feature = "argument")]
    #[test]
    fn test_missing_argument() {
        let err = OptParseError::missing_argument("<input>");
        let thing = format!("{}", err);
        let expect = "Missing argument: <input>";
        assert_eq!(thing, expect);
    }
    #[cfg(feature = "argument")]
    #[test]
    fn test_unexpected_argument() {
        let err = OptParseError::unexpected_argument("xyz");
        let thing = format!("{}", err);
        let expect = "Unexpected argument: xyz";
        assert_eq!(thing, expect);
    }
    //
    #[cfg(feature = "subcommand")]
    #[test]
    fn test_invalid_subcommand() {
        let err = OptParseError::invalid_subcommand("new");
        let thing = format!("{}", err);
        let expect = "Invalid subcommand: new";
        assert_eq!(thing, expect);
    }
    #[cfg(feature = "subcommand")]
    #[test]
    fn test_missing_subcommand() {
        let err = OptParseError::missing_subcommand("<command>");
        let thing = format!("{}", err);
        let expect = "Missing subcommand: <command>";
        assert_eq!(thing, expect);
    }
    //
    #[test]
    #[cfg(feature = "abbreviate")]
    fn test_ambiguous_option() {
        let err = OptParseError::ambiguous_option("--abc", "abcd, abce");
        let thing = format!("{}", err);
        let expect = "Ambiguous option: --abc: abcd, abce";
        assert_eq!(thing, expect);
    }
    #[test]
    #[cfg(all(feature = "abbreviate", feature = "subcommand"))]
    fn test_ambiguous_subcommand() {
        let err = OptParseError::ambiguous_subcommand("new", "new-first, new-second");
        let thing = format!("{}", err);
        let expect = "Ambiguous subcommand: new: new-first, new-second";
        assert_eq!(thing, expect);
    }
}

// OptParseErrors
mod test_1 {
    use flood_tide::err::OptParseError;
    use flood_tide::err::OptParseErrors;
    //
    #[test]
    fn test_errors() {
        let mut errs = OptParseErrors::new();
        assert_eq!(errs.is_empty(), true);
        assert_eq!(errs.len(), 0);
        //
        errs.push(OptParseError::invalid_option("--abc"));
        errs.push(OptParseError::missing_option("--abc"));
        #[cfg(feature = "option_argument")]
        {
            errs.push(OptParseError::invalid_option_argument(
                "--abc",
                "invalid float literal",
            ));
            errs.push(OptParseError::unexpected_option_argument("--abc", "defg"));
            errs.push(OptParseError::missing_option_argument("--abc"));
        }
        #[cfg(feature = "argument")]
        {
            errs.push(OptParseError::missing_argument("<input>"));
            errs.push(OptParseError::unexpected_argument("xyz"));
        }
        #[cfg(feature = "subcommand")]
        {
            errs.push(OptParseError::invalid_subcommand("new"));
            errs.push(OptParseError::missing_subcommand("<command>"));
        }
        #[cfg(feature = "abbreviate")]
        errs.push(OptParseError::ambiguous_option("--abc", "abcd, abce"));
        #[cfg(all(feature = "abbreviate", feature = "subcommand"))]
        errs.push(OptParseError::ambiguous_subcommand(
            "new",
            "new-first, new-second",
        ));
        //
        assert_eq!(errs.is_empty(), false);
        //
        let len = 2;
        #[cfg(feature = "option_argument")]
        let len = len + 3;
        #[cfg(feature = "argument")]
        let len = len + 2;
        #[cfg(feature = "abbreviate")]
        let len = len + 1;
        #[cfg(all(feature = "abbreviate", feature = "subcommand"))]
        let len = len + 1;
        #[cfg(feature = "subcommand")]
        let len = len + 2;
        assert_eq!(errs.len(), len);
        //
        let thing = format!("{}", errs);
        let expect = concat!("Invalid option: --abc\n", "Missing option: --abc",);
        #[cfg(feature = "option_argument")]
        let expect = expect.to_string()
            + concat!(
                "\n",
                "Invalid option argument: --abc: invalid float literal\n",
                "Unexpected option argument: --abc: defg\n",
                "Missing option argument: --abc",
            );
        #[cfg(feature = "argument")]
        let expect = expect.to_string()
            + concat!(
                "\n",
                "Missing argument: <input>\n",
                "Unexpected argument: xyz",
            );
        #[cfg(feature = "subcommand")]
        let expect = expect.to_string()
            + concat!(
                "\n",
                "Invalid subcommand: new\n",
                "Missing subcommand: <command>",
            );
        #[cfg(feature = "abbreviate")]
        let expect = expect.to_string() + concat!("\n", "Ambiguous option: --abc: abcd, abce",);
        #[cfg(all(feature = "abbreviate", feature = "subcommand"))]
        let expect =
            expect.to_string() + concat!("\n", "Ambiguous subcommand: new: new-first, new-second",);
        assert_eq!(thing, expect);
    }
    #[test]
    fn test_errors_append() {
        let mut errs = OptParseErrors::new();
        errs.push(OptParseError::invalid_option("--abc"));
        errs.push(OptParseError::missing_option("--abc"));
        #[cfg(feature = "option_argument")]
        {
            errs.push(OptParseError::invalid_option_argument(
                "--abc",
                "invalid float literal",
            ));
            errs.push(OptParseError::unexpected_option_argument("--abc", "defg"));
        }
        #[cfg(feature = "abbreviate")]
        errs.push(OptParseError::ambiguous_option("--abc", "abcd, abce"));
        //
        let mut errs2 = OptParseErrors::new();
        errs2.push(OptParseError::invalid_option("--abcd"));
        errs2.push(OptParseError::missing_option("--abcd"));
        #[cfg(feature = "option_argument")]
        errs2.push(OptParseError::missing_option_argument("--abc"));
        #[cfg(feature = "argument")]
        {
            errs2.push(OptParseError::missing_argument("<input>"));
            errs2.push(OptParseError::unexpected_argument("xyz"));
        }
        #[cfg(feature = "subcommand")]
        {
            errs2.push(OptParseError::invalid_subcommand("new"));
            errs2.push(OptParseError::missing_subcommand("<command>"));
        }
        #[cfg(all(feature = "abbreviate", feature = "subcommand"))]
        errs2.push(OptParseError::ambiguous_subcommand(
            "new",
            "new-first, new-second",
        ));
        //
        errs.append(errs2);
        //
        let thing = format!("{}", errs);
        let expect1 = concat!("Invalid option: --abc\n", "Missing option: --abc\n",);
        #[cfg(feature = "option_argument")]
        let expect1 = expect1.to_string()
            + concat!(
                "Invalid option argument: --abc: invalid float literal\n",
                "Unexpected option argument: --abc: defg\n",
            );
        #[cfg(feature = "abbreviate")]
        let expect1 = expect1.to_string() + concat!("Ambiguous option: --abc: abcd, abce\n",);
        let expect2 = concat!("Invalid option: --abcd\n", "Missing option: --abcd\n",);
        #[cfg(feature = "option_argument")]
        let expect2 = expect2.to_string() + "Missing option argument: --abc\n";
        #[cfg(feature = "argument")]
        let expect2 = expect2.to_string()
            + concat!("Missing argument: <input>\n", "Unexpected argument: xyz\n",);
        #[cfg(feature = "subcommand")]
        let expect2 = expect2.to_string()
            + concat!(
                "Invalid subcommand: new\n",
                "Missing subcommand: <command>\n",
            );
        #[cfg(all(feature = "abbreviate", feature = "subcommand"))]
        let expect2 =
            expect2.to_string() + concat!("Ambiguous subcommand: new: new-first, new-second\n",);
        let expect = expect1.to_string() + &expect2;
        assert_eq!(thing + "\n", expect);
    }
    #[test]
    fn test_errors_iter() {
        let mut errs = OptParseErrors::new();
        errs.push(OptParseError::invalid_option("--abc"));
        errs.push(OptParseError::missing_option("--abc"));
        #[cfg(feature = "option_argument")]
        {
            errs.push(OptParseError::invalid_option_argument(
                "--abc",
                "invalid float literal",
            ));
            errs.push(OptParseError::unexpected_option_argument("--abc", "defg"));
            errs.push(OptParseError::missing_option_argument("--abc"));
        }
        #[cfg(feature = "argument")]
        {
            errs.push(OptParseError::missing_argument("<input>"));
            errs.push(OptParseError::unexpected_argument("xyz"));
        }
        #[cfg(feature = "subcommand")]
        {
            errs.push(OptParseError::invalid_subcommand("new"));
            errs.push(OptParseError::missing_subcommand("<command>"));
        }
        #[cfg(feature = "abbreviate")]
        errs.push(OptParseError::ambiguous_option("--abc", "abcd, abce"));
        #[cfg(all(feature = "abbreviate", feature = "subcommand"))]
        errs.push(OptParseError::ambiguous_subcommand(
            "new",
            "new-first, new-second",
        ));
        //
        let thing = {
            let mut s = String::new();
            let mut it = errs.iter();
            while let Some(err) = it.next() {
                let ss = format!("{}\n", err);
                s.push_str(ss.as_str());
            }
            s
        };
        let expect = concat!("Invalid option: --abc\n", "Missing option: --abc\n",);
        #[cfg(feature = "option_argument")]
        let expect = expect.to_string()
            + concat!(
                "Invalid option argument: --abc: invalid float literal\n",
                "Unexpected option argument: --abc: defg\n",
                "Missing option argument: --abc\n",
            );
        #[cfg(feature = "argument")]
        let expect = expect.to_string()
            + concat!("Missing argument: <input>\n", "Unexpected argument: xyz\n",);
        #[cfg(feature = "subcommand")]
        let expect = expect.to_string()
            + concat!(
                "Invalid subcommand: new\n",
                "Missing subcommand: <command>\n",
            );
        #[cfg(feature = "abbreviate")]
        let expect = expect.to_string() + concat!("Ambiguous option: --abc: abcd, abce\n",);
        #[cfg(all(feature = "abbreviate", feature = "subcommand"))]
        let expect =
            expect.to_string() + concat!("Ambiguous subcommand: new: new-first, new-second\n",);
        assert_eq!(thing, expect);
    }
}
