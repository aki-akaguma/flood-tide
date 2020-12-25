use optpa_util_5::OptParseError;
#[cfg(not(feature = "single_error"))]
use optpa_util_5::OptParseErrors;

use optpa_util_5::Arg;
use optpa_util_5::Lex;
use optpa_util_5::NameVal;
pub use optpa_util_5::OPErr;
use optpa_util_5::Opt;
use optpa_util_5::OptNum;

use optpa_util_5::check_sorted_opt_ary_and_sho_idx_ary_with;

//----------------------------------------------------------------------
const HELP_TEXT: &str = r#"
Options:
    -h, --help          Print this help menu
    -V, --version       Print version information
    -d, --debug         Activate debug mode
    -v, --verbose       Verbose mode. -vv is more verbose
    -s, --speed <speed> Set speed (default: 42.0)
    --color <when>      Use markers to highlight (default: auto)
                        <when> is 'always', 'never', or 'auto'
    -c, --config <path> Give a path string argument
Args:
    <input>             Input file name
    [<output>]          Output file name, stdout if not present
"#;

#[derive(Debug, Default, PartialEq)]
pub struct CmdOptConf {
    pub opt_program: String,
    //
    pub flag_debug: bool,
    pub cnt_verbose: usize,
    pub opt_speed: f32,
    //opt_color: OptColorWhen,
    pub opt_config: Option<String>,
    //
    pub arg_input: String,
    pub arg_output: Option<String>,
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
enum CmdOP {
    Help = 1,
    Version,
    Debug,
    Verbose,
    Speed,
    //Color,
    Config,
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

#[rustfmt::skip]
const OPT_ARY: [Opt; 6] = [
    Opt { sho: b'c', lon: "config",  has: Arg::Yes, num: CmdOP::Config.to(), },
    Opt { sho: b'd', lon: "debug",   has: Arg::No,  num: CmdOP::Debug.to(), },
    Opt { sho: b'h', lon: "help",    has: Arg::No,  num: CmdOP::Help.to(), },
    Opt { sho: b's', lon: "speed",   has: Arg::Yes, num: CmdOP::Speed.to(), },
    Opt { sho: b'v', lon: "verbose", has: Arg::No,  num: CmdOP::Verbose.to(), },
    Opt { sho: b'V', lon: "version", has: Arg::No,  num: CmdOP::Version.to(), },
];

#[rustfmt::skip]
const OPT_ARY_SHO_IDX: [(u8,usize);6] = [
    (b'V',5), (b'c',0), (b'd',1), (b'h',2), (b's',3), (b'v',4)
];

//----------------------------------------------------------------------
fn full_usage(program: &str) -> String {
    let usage = format!("Usage: {} [options] <input> [<output>]", program);
    let opts = HELP_TEXT;
    format!("{}\n{}", usage, opts)
}

#[inline(never)]
fn print_help_and_exit(conf: &CmdOptConf) {
    let s = full_usage(&conf.opt_program);
    println!("{}", s);
    std::process::exit(0);
}

#[inline(never)]
fn print_version_and_exit(_conf: &CmdOptConf) {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    std::process::exit(0);
}

#[inline(never)]
fn mk_invalid_option_argument(nv: &NameVal, err_s: &str) -> OptParseError {
    #[cfg(feature = "was_long")]
    let name = if nv.was_long {
        nv.opt.lon.to_string()
    } else {
        (nv.opt.sho as char).to_string()
    };
    #[cfg(not(feature = "was_long"))]
    let name = nv.opt.lon.to_string();
    OptParseError::invalid_option_argument(&name, err_s)
}

#[inline(never)]
fn value_to_f32(nv: &NameVal<'_>) -> Result<f32, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<f32>() {
            Ok(d) => Ok(d),
            Err(err) => Err(mk_invalid_option_argument(nv, &err.to_string())),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}

//#[inline(never)]
fn parse_match(conf: &mut CmdOptConf, nv: &NameVal<'_>) -> Result<(), OptParseError> {
    match CmdOP::from(nv.opt.num) {
        CmdOP::Help => {
            print_help_and_exit(conf);
        }
        CmdOP::Version => {
            print_version_and_exit(conf);
        }
        CmdOP::Debug => {
            conf.flag_debug = true;
        }
        CmdOP::Verbose => {
            conf.cnt_verbose += 1;
        }
        CmdOP::Speed => {
            conf.opt_speed = value_to_f32(nv)?;
        }
        CmdOP::Config => {
            conf.opt_config = match nv.val {
                Some(s) => Some(s.to_string()),
                None => None,
            };
        }
    }
    Ok(())
}

pub fn check_sorted_opt_ary_and_sho_idx_ary() -> bool {
    check_sorted_opt_ary_and_sho_idx_ary_with(&OPT_ARY, &OPT_ARY_SHO_IDX)
}

#[cfg(feature = "single_error")]
#[inline(never)]
pub fn parse_cmdopts(program: &str, env_args: &[&str]) -> Result<CmdOptConf, OptParseError> {
    //
    let mut conf = CmdOptConf {
        opt_program: program.to_string(),
        opt_speed: 42.0,
        ..Default::default()
    };
    //
    let lex = Lex::create_with(&OPT_ARY, &OPT_ARY_SHO_IDX);
    let tokens = match lex.tokens_from(&env_args) {
        Ok(t) => t,
        Err(err) => return Err(err),
    };
    //
    for nv in tokens.namevals.iter() {
        match parse_match(&mut conf, &nv) {
            Ok(_) => {}
            Err(err) => return Err(err),
        }
    }
    let free = tokens.free;
    if !free.is_empty() {
        conf.arg_input = free[0].to_string();
        conf.arg_output = if free.len() > 1 {
            Some(free[1].to_string())
        } else {
            None
        };
    } else {
        return Err(OptParseError::missing_argument("<input>"));
    }
    //
    Ok(conf)
}
/*
#[cfg(feature = "single_error")]
pub fn create_conf() -> Result<CmdOptConf, OptParseError> {
    let mut env_args: Vec<String> = env::args().collect();
    let program = env_args.remove(0);
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, env_args)
}
*/
#[cfg(not(feature = "single_error"))]
//#[inline(never)]
pub fn parse_cmdopts(program: &str, env_args: &[&str]) -> Result<CmdOptConf, OptParseErrors> {
    //
    let mut conf = CmdOptConf {
        opt_program: program.to_string(),
        opt_speed: 42.0,
        ..Default::default()
    };
    //
    let lex = Lex::create_with(&OPT_ARY, &OPT_ARY_SHO_IDX);
    let tokens = match lex.tokens_from(&env_args) {
        Ok(t) => t,
        Err(errs) => {
            return Err(errs);
        }
    };
    //
    let mut errs = OptParseErrors::new();
    for nv in tokens.namevals.iter() {
        match parse_match(&mut conf, &nv) {
            Ok(_) => {}
            Err(e) => errs.push(e),
        }
    }
    let free = tokens.free;
    if !free.is_empty() {
        conf.arg_input = free[0].to_string();
        conf.arg_output = if free.len() > 1 {
            Some(free[1].to_string())
        } else {
            None
        }
    } else {
        errs.push(OptParseError::missing_argument("<input>"));
    }
    if !errs.is_empty() {
        return Err(errs);
    }
    //
    Ok(conf)
}
/*
#[cfg(not(feature = "single_error"))]
pub fn create_conf() -> Result<CmdOptConf, OptParseErrors> {
    let mut env_args: Vec<String> = env::args().collect();
    let program = env_args.remove(0);
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, env_args)
}
*/
