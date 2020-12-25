mod size_check {
    use flood_tide::err::OptParseError;
    use flood_tide::err::OptParseErrorKind;
    use flood_tide::err::OptParseErrors;
    //
    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<OptParseErrorKind>(), 1);
        assert_eq!(std::mem::size_of::<OptParseError>(), 56);
        assert_eq!(std::mem::size_of::<OptParseErrors>(), 24);
        //
        assert_eq!(std::mem::size_of::<String>(), 24);
        assert_eq!(std::mem::size_of::<Option<String>>(), 24);
    }
}
mod fmt_debug {
    use flood_tide::err::OptParseError;
    use flood_tide::err::OptParseErrorKind;
    use flood_tide::err::OptParseErrors;
    //
    #[test]
    fn opt_parse_error_kind() {
        let operrk = OptParseErrorKind::InvalidOption;
        let thing = format!("{:?}", operrk);
        let expect = "InvalidOption";
        assert_eq!(thing, expect);
    }
    //
    #[test]
    fn opt_parse_error() {
        let operr = OptParseError::invalid_option("--abc");
        let thing = format!("{:?}", operr);
        let expect = "OptParseError { kind: InvalidOption, desc1: \"--abc\", desc2: None }";
        assert_eq!(thing, expect);
    }
    //
    #[test]
    fn opt_parse_errors() {
        let mut operrs = OptParseErrors::new();
        operrs.push(OptParseError::invalid_option("--abc"));
        let thing = format!("{:?}", operrs);
        let expect = "OptParseErrors([OptParseError { kind: InvalidOption, desc1: \"--abc\", desc2: None }])";
        assert_eq!(thing, expect);
    }
}
