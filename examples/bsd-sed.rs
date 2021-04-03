//! example: bsd-sed

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
Stream EDitor, commands are applied to the input.
Read standard input with no <file>, or <file> is -
"#;

const OPTIONS_TEXT: &str = r#"Options:
  -E                    interpret as extended regular expressions.
  -a                    delay opening each file until a 'w' function.
  -e <command>          append the editing commands.
  -f <command_file>     append the editing commands found in the file.
  -I <extension>        edit files in-place, saving backups with extension.
  -i <extension>        edit files in-place similarly to -I, but treat
                        each file independently from other files.
  -l                    make output line buffered.
  -n                    suppresses behavior.
  -r                    same as -E for compatibility
  -u                    make output unbuffered.
  -H, --help            display this help and exit
  -V, --version         output version information and exit
"#;

const ARGUMENTS_TEXT: &str = r#"Argument:
  <file>                file path to reading or '-' to reading from stdin
"#;

const EXAMPLES_TEXT: &str = r#"Example:
  Replace `bar' with `baz' when piped from another command:
    echo "An alternate word, like bar, is sometimes." | sed 's/bar/baz/'
"#;
//}}} TEXT

#[repr(u8)]
#[derive(Debug, PartialEq)]
enum CmdOP {
    ExtRegex = 1,
    Await,
    Edit,
    File,
    Inplace,
    Independ,
    Linebuf,
    NotEach,
    Regex,
    Unbuffer,
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
const OPT_ARY: [Opt;12] = [
    Opt { sho: b'H', lon: "help",    has: Arg::No, num: CmdOP::Help.to(), },
    Opt { sho: b'V', lon: "version", has: Arg::No, num: CmdOP::Version.to(), },
    Opt { sho: b'E', lon: "", has: Arg::No,  num: CmdOP::ExtRegex.to(), },
    Opt { sho: b'I', lon: "", has: Arg::Yes, num: CmdOP::Inplace.to(), },
    Opt { sho: b'a', lon: "", has: Arg::No,  num: CmdOP::Await.to(), },
    Opt { sho: b'e', lon: "", has: Arg::Yes, num: CmdOP::Edit.to(), },
    Opt { sho: b'f', lon: "", has: Arg::Yes, num: CmdOP::File.to(), },
    Opt { sho: b'i', lon: "", has: Arg::Yes, num: CmdOP::Independ.to(), },
    Opt { sho: b'l', lon: "", has: Arg::No,  num: CmdOP::Linebuf.to(), },
    Opt { sho: b'n', lon: "", has: Arg::No,  num: CmdOP::NotEach.to(), },
    Opt { sho: b'r', lon: "", has: Arg::No,  num: CmdOP::Regex.to(), },
    Opt { sho: b'u', lon: "", has: Arg::No,  num: CmdOP::Unbuffer.to(), },
];

#[rustfmt::skip]
const OPT_ARY_SHO_IDX: [(u8,usize);10] = [
    (b'E',2), (b'I',3), (b'a',4), (b'e',5), (b'f',6),
    (b'i',7), (b'l',8), (b'n',9), (b'r',10), (b'u',11)
];

//----------------------------------------------------------------------
#[derive(Debug, Default)]
struct CmdOptConf {
    pub opt_program: String,
    //
    pub flag_ext_regex: bool,
    pub flag_a_wait: bool,
    pub flag_edit: String,
    pub flag_file: String,
    pub flag_in_place: String,
    pub flag_independ: String,
    pub flag_linu_buf: bool,
    pub flag_not_each: bool,
    pub flag_unbuf: bool,
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
        "examples/bsd-sed" )
}

#[rustfmt::skip]
fn usage_message(program: &str) -> String {
    format!("Usage:\n  {} {}\n  {} {}",
        program,
        "[-Ealnru] <command> [<file> ...]",
        program,
        "[-Ealnr] [-e command] [-f command_file] [-I extension] [-i extension] [<file> ...]")
}

#[rustfmt::skip]
fn help_message(program: &str) -> String {
    let ver = version_message(program);
    let usa = usage_message("bsd-sed");
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
fn value_to_string(nv: &NameVal<'_>) -> Result<String, OptParseError> {
    match nv.val {
        Some(x) => Ok(x.to_string()),
        None => {
            let s = (nv.opt.sho as char).to_string();
            Err(OptParseError::missing_option_argument(&s))
        }
    }
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
        CmdOP::ExtRegex => {
            conf.flag_ext_regex = true;
        }
        CmdOP::Await => {
            conf.flag_a_wait = true;
        }
        CmdOP::Edit => {
            conf.flag_edit = value_to_string(nv)?;
        }
        CmdOP::File => {
            conf.flag_file = value_to_string(nv)?;
        }
        CmdOP::Inplace => {
            conf.flag_in_place = value_to_string(nv)?;
        }
        CmdOP::Independ => {
            conf.flag_independ = value_to_string(nv)?;
        }
        CmdOP::Linebuf => {
            conf.flag_linu_buf = true;
        }
        CmdOP::NotEach => {
            conf.flag_not_each = true;
        }
        CmdOP::Regex => {
            conf.flag_ext_regex = true;
        }
        CmdOP::Unbuffer => {
            conf.flag_unbuf = true;
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
#[cfg(not(feature = "abbreviate"))]
#[cfg(not(feature = "subcommand"))]
#[cfg(feature = "single_error")]
mod example {
    #[test]
    fn test_bsd_sed_1() {
        let program = "test-bsd-sed";
        #[rustfmt::skip]
        let args = vec!["-e", "s/abc/ABC/", "f1", "-", "f2"];
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
            " opt_program: \"test-bsd-sed\",",
            " flag_ext_regex: false,",
            " flag_a_wait: false,",
            " flag_edit: \"s/abc/ABC/\",",
            " flag_file: \"\",",
            " flag_in_place: \"\",",
            " flag_independ: \"\",",
            " flag_linu_buf: false,",
            " flag_not_each: false,",
            " flag_unbuf: false,",
            " flag_help: false,",
            " flag_version: false,",
            " arg_params: [\"f1\", \"-\", \"f2\"] }"
        );
        assert_eq!(thing, expect);
    }
    //
    #[test]
    fn test_bsd_sed_2() {
        let program = "test-bsd-sed";
        #[rustfmt::skip]
        let args = vec!["-x", "f1", "-", "f2"];
        //
        match super::parse_cmdopts(program, args) {
            Ok(conf) => {
                assert_eq!(format!("{:?}", conf), "");
                unreachable!();
            }
            Err(err) => {
                let thing = format!("{}", err);
                let expect = "Invalid option: x";
                assert_eq!(thing, expect);
            }
        };
    }
    #[test]
    fn test_bsd_sed_3() {
        let program = "test-bsd-sed";
        #[rustfmt::skip]
        let args = vec!["-Eal", "-e", "s/abc/ABC/", "f1", "-", "f2"];
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
            " opt_program: \"test-bsd-sed\",",
            " flag_ext_regex: true,",
            " flag_a_wait: true,",
            " flag_edit: \"s/abc/ABC/\",",
            " flag_file: \"\",",
            " flag_in_place: \"\",",
            " flag_independ: \"\",",
            " flag_linu_buf: true,",
            " flag_not_each: false,",
            " flag_unbuf: false,",
            " flag_help: false,",
            " flag_version: false,",
            " arg_params: [\"f1\", \"-\", \"f2\"] }"
        );
        assert_eq!(thing, expect);
    }
}
