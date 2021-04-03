//! example: gnu-cat

use flood_tide::parse_simple_gnu_style;
use flood_tide::Arg;
use flood_tide::HelpVersion;
use flood_tide::NameVal;
use flood_tide::Opt;
use flood_tide::OptNum;
use flood_tide::OptParseError;

//----------------------------------------------------------------------
//{{{ TEXT
const DESCRIPTIONS_TEXT: &str = r#"
Concatenate <file(s)> to standard output.
Read standard input with no <file>, or <file> is -
"#;

const OPTIONS_TEXT: &str = r#"Options:
  -A, --show-all            equivalent to -vET
  -b, --number-nonblank     number nonempty output lines, overrides -n
  -e                        equivalent to -vE
  -E, --show-ends           display $ at end of each line
  -n, --number              number all output lines
  -s, --squeeze-blank       suppress repeated empty output lines
  -t                        equivalent to -vT
  -T, --show-tabs           display TAB characters as ^I
  -v, --show-nonprinting    use ^ and M- notation, except for LFD and TAB
  -H, --help            display this help and exit
  -V, --version         output version information and exit
"#;

const ARGUMENTS_TEXT: &str = r#"Argument:
  <file>                file path to reading or '-' to reading from stdin
"#;

const EXAMPLES_TEXT: &str = r#"Examples:
  cat f - g     Output f's contents, then standard input, then g's contents.
  cat           Copy standard input to standard output.
"#;
//}}} TEXT

#[repr(u8)]
#[derive(Debug, PartialEq)]
enum CmdOP {
    ShowAll = 1,
    NumberNB,
    E,
    Ends,
    Number,
    Squeeze,
    T,
    Tab,
    Visual,
    //
    Help,
    Version,
}
impl std::convert::From<OptNum> for CmdOP {
    fn from(value: OptNum) -> Self {
        unsafe { std::mem::transmute_copy(&value) }
    }
}
impl CmdOP {
    pub const fn to(self) -> OptNum {
        self as OptNum
    }
}

//
#[rustfmt::skip]
const OPT_ARY: [Opt;11] = [
    Opt { sho: b'e', lon: "",                 has: Arg::No, num: CmdOP::E.to(), },
    Opt { sho: b't', lon: "",                 has: Arg::No, num: CmdOP::T.to(), },
    Opt { sho: b'H', lon: "help",    has: Arg::No, num: CmdOP::Help.to(), },
    Opt { sho: b'n', lon: "number",           has: Arg::No, num: CmdOP::Number.to(), },
    Opt { sho: b'b', lon: "number-nonblank",  has: Arg::No, num: CmdOP::NumberNB.to(), },
    Opt { sho: b'A', lon: "show-all",         has: Arg::No, num: CmdOP::ShowAll.to(), },
    Opt { sho: b'E', lon: "show-ends",        has: Arg::No, num: CmdOP::Ends.to(), },
    Opt { sho: b'v', lon: "show-nonprinting", has: Arg::No, num: CmdOP::Visual.to(), },
    Opt { sho: b'T', lon: "show-tabs",        has: Arg::No, num: CmdOP::Tab.to(), },
    Opt { sho: b's', lon: "squeeze-blank",    has: Arg::No, num: CmdOP::Squeeze.to(), },
    Opt { sho: b'V', lon: "version", has: Arg::No, num: CmdOP::Version.to(), },
];

#[rustfmt::skip]
const OPT_ARY_SHO_IDX: [(u8,usize);11] = [
    (b'A',5), (b'E',6), (b'H',2), (b'T',8), (b'V',10),
    (b'b',4), (b'e',0), (b'n',3), (b's',9), (b't',1), (b'v',7)
];

//----------------------------------------------------------------------
#[derive(Debug, Default)]
struct CmdOptConf {
    pub opt_program: String,
    //
    pub flag_tab: bool,
    pub flag_ends: bool,
    pub flag_visual: bool,
    pub flag_number_nb: bool,
    pub flag_number: bool,
    pub flag_squeeze: bool,
    //
    pub flag_help: bool,
    pub flag_version: bool,
    //
    pub arg_params: Vec<String>,
}
impl HelpVersion for CmdOptConf {
    fn is_help(&self) -> bool {
        self.flag_help
    }
    fn is_version(&self) -> bool {
        self.flag_version
    }
}

//----------------------------------------------------------------------
#[rustfmt::skip]
fn version_message(_program: &str) -> String {
    format!( "{} {} ({})",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"),
        "examples/gnu-cat" )
}

#[rustfmt::skip]
fn usage_message(program: &str) -> String {
    format!("Usage:\n  {} {}", program, "[options] <file(s)>")
}

#[rustfmt::skip]
fn help_message(program: &str) -> String {
    let ver = version_message(program);
    let usa = usage_message("gnu-cat");
    [ &ver, "", &usa, DESCRIPTIONS_TEXT, OPTIONS_TEXT,
        ARGUMENTS_TEXT, EXAMPLES_TEXT].join("\n")
}

#[inline(never)]
fn print_help_and_exit(conf: &CmdOptConf) {
    print!("{}", help_message(&conf.opt_program));
    std::process::exit(0);
}

#[inline(never)]
fn print_version_and_exit(conf: &CmdOptConf) {
    print!("{}\n", version_message(&conf.opt_program));
    std::process::exit(0);
}

#[inline(never)]
fn parse_match(conf: &mut CmdOptConf, nv: &NameVal<'_>) -> Result<(), OptParseError> {
    match CmdOP::from(nv.opt.num) {
        CmdOP::Help => {
            print_help_and_exit(conf);
        }
        CmdOP::Version => {
            print_version_and_exit(conf);
        }
        //
        CmdOP::ShowAll => {
            conf.flag_tab = true;
            conf.flag_ends = true;
            conf.flag_visual = true;
        }
        CmdOP::NumberNB => {
            conf.flag_number_nb = true;
            conf.flag_number = false;
        }
        CmdOP::E => {
            conf.flag_visual = true;
            conf.flag_ends = true;
        }
        CmdOP::Ends => {
            conf.flag_ends = true;
        }
        CmdOP::Number => {
            conf.flag_number = true;
        }
        CmdOP::Squeeze => {
            conf.flag_squeeze = true;
        }
        CmdOP::T => {
            conf.flag_tab = true;
            conf.flag_visual = true;
        }
        CmdOP::Tab => {
            conf.flag_tab = true;
        }
        CmdOP::Visual => {
            conf.flag_visual = true;
        }
    }
    Ok(())
}

fn parse_cmdopts(program: &str, args: Vec<&str>) -> Result<CmdOptConf, OptParseError> {
    //
    let mut conf = CmdOptConf {
        opt_program: program.to_string(),
        ..Default::default()
    };
    let (free_opt, err_r) =
        parse_simple_gnu_style(&mut conf, &OPT_ARY, &OPT_ARY_SHO_IDX, &args, parse_match);
    //
    if let Err(err) = err_r {
        return Err(err);
    }
    if let Some(free) = free_opt {
        if !free.is_empty() {
            conf.arg_params = free;
        }
    }
    //
    Ok(conf)
}

//----------------------------------------------------------------------
fn create_conf() -> Result<CmdOptConf, OptParseError> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let program = env_args.remove(0);
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, env_args)
}

fn run(conf: &CmdOptConf) {
    eprintln!("{:?}", conf);
}

//----------------------------------------------------------------------
fn main() {
    //
    let conf = match create_conf() {
        Ok(conf) => conf,
        Err(err) => {
            const TRY_HELP_MSG: &str = "Try --help for help.";
            eprintln!("{}\n{}", err, TRY_HELP_MSG);
            std::process::exit(1);
        }
    };
    //
    run(&conf);
    //
    std::process::exit(0);
}

#[cfg(not(feature = "long_only"))]
#[cfg(all(feature = "single_error", feature = "abbreviate"))]
mod example {
    #[test]
    fn test_gnu_cat_1() {
        let program = "test-gnu-cat";
        #[rustfmt::skip]
        let args = vec!["f1", "-", "f2"];
        //
        let conf = match super::parse_cmdopts(program, args) {
            Ok(conf) => conf,
            Err(err) => {
                assert_eq!(format!("{}", err), "");
                unreachable!();
            }
        };
        //
        let thing = format!("{:?}", conf);
        let expect = concat!(
            "CmdOptConf {",
            " opt_program: \"test-gnu-cat\",",
            " flag_tab: false,",
            " flag_ends: false,",
            " flag_visual: false,",
            " flag_number_nb: false,",
            " flag_number: false,",
            " flag_squeeze: false,",
            " flag_help: false,",
            " flag_version: false,",
            " arg_params: [\"f1\", \"-\", \"f2\"] }"
        );
        assert_eq!(thing, expect);
    }
    //
    #[test]
    fn test_gnu_cat_2() {
        let program = "test-gnu-cat";
        #[rustfmt::skip]
        let args = vec!["-A", "f1", "-", "f2"];
        //
        let conf = match super::parse_cmdopts(program, args) {
            Ok(conf) => conf,
            Err(err) => {
                assert_eq!(format!("{}", err), "");
                unreachable!();
            }
        };
        //
        let thing = format!("{:?}", conf);
        let expect = concat!(
            "CmdOptConf {",
            " opt_program: \"test-gnu-cat\",",
            " flag_tab: true,",
            " flag_ends: true,",
            " flag_visual: true,",
            " flag_number_nb: false,",
            " flag_number: false,",
            " flag_squeeze: false,",
            " flag_help: false,",
            " flag_version: false,",
            " arg_params: [\"f1\", \"-\", \"f2\"] }"
        );
        assert_eq!(thing, expect);
    }
    //
    #[test]
    fn test_gnu_cat_3() {
        let program = "test-gnu-cat";
        #[rustfmt::skip]
        let args = vec!["-a", "f1", "-", "f2"];
        //
        match super::parse_cmdopts(program, args) {
            Ok(conf) => {
                assert_eq!(format!("{:?}", conf), "");
                unreachable!();
            }
            Err(err) => {
                let thing = format!("{}", err);
                let expect = "Invalid option: a";
                assert_eq!(thing, expect);
            }
        };
    }
    //
    #[test]
    fn test_gnu_cat_5() {
        let program = "test-gnu-cat";
        #[rustfmt::skip]
        let args = vec!["--show-a", "f1", "-", "f2"];
        //
        let conf = match super::parse_cmdopts(program, args) {
            Ok(conf) => conf,
            Err(err) => {
                assert_eq!(format!("{}", err), "");
                unreachable!();
            }
        };
        //
        let thing = format!("{:?}", conf);
        let expect = concat!(
            "CmdOptConf {",
            " opt_program: \"test-gnu-cat\",",
            " flag_tab: true,",
            " flag_ends: true,",
            " flag_visual: true,",
            " flag_number_nb: false,",
            " flag_number: false,",
            " flag_squeeze: false,",
            " flag_help: false,",
            " flag_version: false,",
            " arg_params: [\"f1\", \"-\", \"f2\"] }"
        );
        assert_eq!(thing, expect);
    }
    //
    #[test]
    fn test_gnu_cat_6() {
        let program = "test-gnu-cat";
        #[rustfmt::skip]
        let args = vec!["--show", "f1", "-", "f2"];
        //
        match super::parse_cmdopts(program, args) {
            Ok(conf) => {
                assert_eq!(format!("{:?}", conf), "");
                unreachable!();
            }
            Err(err) => {
                let thing = format!("{}", err);
                let expect = concat!(
                    "Ambiguous option: show: possibilities:",
                    " \'--show-all\'",
                    " \'--show-ends\'",
                    " \'--show-nonprinting\'",
                    " \'--show-tabs\'"
                );
                assert_eq!(thing, expect);
            }
        };
    }
}
