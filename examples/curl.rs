//! example: curl

use flood_tide::parse_simple_gnu_style;
use flood_tide::Arg;
use flood_tide::NameVal;
use flood_tide::OPErr;
use flood_tide::Opt;
use flood_tide::OptNum;
use flood_tide::OptParseError;
#[cfg(not(feature = "single_error"))]
use flood_tide::OptParseErrors;

//----------------------------------------------------------------------
include!("curl.cmd.help.rs.txt");

//{{{ TEXT
const DESCRIPTIONS_TEXT: &str = r#"
transfer a URL, another name is the multiprotocol getter
"#;

const ARGUMENTS_TEXT: &str = r#"Argument:
  <url>                     url to getting, protocol is http or ftp
"#;

const EXAMPLES_TEXT: &str = r#"Examples:
  You  can specify multiple URLs or parts of URLs by writing part sets within braces as in:
    curl "http://site.{one,two,three}.comn"
  you can get sequences of alphanumeric series by using [] as in:
    curl "ftp://ftp.example.com/file[1-100].txt"
"#;
//}}} TEXT

//----------------------------------------------------------------------
#[rustfmt::skip]
fn version_message(_program: &str) -> String {
    format!( "{} {} ({})",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"),
        "examples/curl" )
}

#[rustfmt::skip]
fn usage_message(program: &str) -> String {
    format!("Usage:\n  {} {}", program, "[options] <url>")
}

#[rustfmt::skip]
fn help_message(program: &str) -> String {
    let ver = version_message(program);
    let usa = usage_message("curl");
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
    println!("{}", version_message(&conf.opt_program));
    std::process::exit(0);
}

#[inline(never)]
fn value_to_string(nv: &NameVal<'_>) -> Result<String, OptParseError> {
    match nv.val {
        Some(x) => Ok(x.to_string()),
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}

#[inline(never)]
fn value_to_u32(nv: &NameVal<'_>) -> Result<u32, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<u32>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}

#[inline(never)]
fn value_to_u64(nv: &NameVal<'_>) -> Result<u64, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<u64>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}

#[inline(never)]
fn parse_match(conf: &mut CmdOptConf, nv: &NameVal<'_>) -> Result<(), OptParseError> {
    include!("curl.cmd.match.rs.txt");
    Ok(())
}

fn parse_cmdopts(program: &str, args: Vec<&str>) -> Result<CmdOptConf, OPErr> {
    //
    let mut conf = CmdOptConf {
        opt_program: program.to_string(),
        ..Default::default()
    };
    let (opt_free, r_errs) =
        parse_simple_gnu_style(&mut conf, &OPT_ARY, &OPT_ARY_SHO_IDX, &args, parse_match);
    //
    #[cfg(feature = "single_error")]
    {
        if let Err(err) = r_errs {
            return Err(err);
        }
        if let Some(free) = opt_free {
            if !free.is_empty() {
                conf.arg_params = free;
            } else {
                return Err(OptParseError::missing_argument("<url>"));
            }
        }
    }
    #[cfg(not(feature = "single_error"))]
    {
        let mut errs = if let Err(errs) = r_errs {
            errs
        } else {
            OptParseErrors::new()
        };
        let missing_arg = if let Some(free) = opt_free {
            if !free.is_empty() {
                conf.arg_params = free;
                false
            } else {
                true
            }
        } else {
            true
        };
        if missing_arg {
            errs.push(OptParseError::missing_argument("<url>"));
        }
        if !errs.is_empty() {
            return Err(errs);
        }
    }
    //
    Ok(conf)
}

//----------------------------------------------------------------------
fn create_conf() -> Result<CmdOptConf, OPErr> {
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
    fn test_curl_0() {
        assert_eq!(std::mem::size_of::<super::CmdOptConf>(), 2712);
    }
    #[test]
    fn test_curl_1() {
        let program = "test-curl";
        #[rustfmt::skip]
        let args = vec!["url1"];
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
        let expect_st = r#"CmdOptConf { opt_program: "test-curl", opt_abstract_unix_socket: "", opt_alt_svc: "", flg_anyauth: false, flg_append: false, flg_basic: false, opt_cacert: "", opt_capath: "", opt_cert: "", flg_cert_status: false, opt_cert_type: "", opt_ciphers: "", flg_compressed: false, flg_compressed_ssh: false, opt_config: "", opt_connect_timeout: 0, opt_connect_to: "", opt_continue_at: 0,"#;
        let expect_ed = r#"flg_tr_encoding: false, opt_trace: "", opt_trace_ascii: "", flg_trace_time: false, opt_unix_socket: "", opt_upload_file: "", opt_url: "", flg_use_ascii: false, opt_user: "", opt_user_agent: "", flg_verbose: false, opt_write_out: "", flg_xattr: false, flg_help: false, flg_version: false, arg_params: ["url1"] }"#;
        assert_eq!(&thing[..expect_st.len()], expect_st);
        assert_eq!(&thing[(thing.len() - expect_ed.len())..], expect_ed);
    }
    #[test]
    fn test_curl_2() {
        let program = "test-curl";
        #[rustfmt::skip]
        let args = vec!["-a", "url1"];
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
        let expect_st = r#"CmdOptConf { opt_program: "test-curl", opt_abstract_unix_socket: "", opt_alt_svc: "", flg_anyauth: false, flg_append: true, flg_basic: false, opt_cacert: "", opt_capath: "", opt_cert: "", flg_cert_status: false, opt_cert_type: "", opt_ciphers: "", flg_compressed: false, flg_compressed_ssh: false, opt_config: "", opt_connect_timeout: 0, opt_connect_to: "", opt_continue_at: 0,"#;
        let expect_ed = r#"flg_tr_encoding: false, opt_trace: "", opt_trace_ascii: "", flg_trace_time: false, opt_unix_socket: "", opt_upload_file: "", opt_url: "", flg_use_ascii: false, opt_user: "", opt_user_agent: "", flg_verbose: false, opt_write_out: "", flg_xattr: false, flg_help: false, flg_version: false, arg_params: ["url1"] }"#;
        assert_eq!(&thing[..expect_st.len()], expect_st);
        assert_eq!(&thing[(thing.len() - expect_ed.len())..], expect_ed);
    }
    #[test]
    fn test_curl_3() {
        let program = "test-curl";
        #[rustfmt::skip]
        let args = vec!["-v", "url1"];
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
        let expect_st = r#"CmdOptConf { opt_program: "test-curl", opt_abstract_unix_socket: "", opt_alt_svc: "", flg_anyauth: false, flg_append: false, flg_basic: false, opt_cacert: "", opt_capath: "", opt_cert: "", flg_cert_status: false, opt_cert_type: "", opt_ciphers: "", flg_compressed: false, flg_compressed_ssh: false, opt_config: "", opt_connect_timeout: 0, opt_connect_to: "", opt_continue_at: 0,"#;
        let expect_ed = r#"flg_tr_encoding: false, opt_trace: "", opt_trace_ascii: "", flg_trace_time: false, opt_unix_socket: "", opt_upload_file: "", opt_url: "", flg_use_ascii: false, opt_user: "", opt_user_agent: "", flg_verbose: true, opt_write_out: "", flg_xattr: false, flg_help: false, flg_version: false, arg_params: ["url1"] }"#;
        assert_eq!(&thing[..expect_st.len()], expect_st);
        assert_eq!(&thing[(thing.len() - expect_ed.len())..], expect_ed);
    }
    #[test]
    fn test_curl_4() {
        let program = "test-curl";
        #[rustfmt::skip]
        let args = vec!["--unix-socket", "path1", "url1"];
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
        let expect_st = r#"CmdOptConf { opt_program: "test-curl", opt_abstract_unix_socket: "", opt_alt_svc: "", flg_anyauth: false, flg_append: false, flg_basic: false, opt_cacert: "", opt_capath: "", opt_cert: "", flg_cert_status: false, opt_cert_type: "", opt_ciphers: "", flg_compressed: false, flg_compressed_ssh: false, opt_config: "", opt_connect_timeout: 0, opt_connect_to: "", opt_continue_at: 0,"#;
        let expect_ed = r#"flg_tr_encoding: false, opt_trace: "", opt_trace_ascii: "", flg_trace_time: false, opt_unix_socket: "path1", opt_upload_file: "", opt_url: "", flg_use_ascii: false, opt_user: "", opt_user_agent: "", flg_verbose: false, opt_write_out: "", flg_xattr: false, flg_help: false, flg_version: false, arg_params: ["url1"] }"#;
        assert_eq!(&thing[..expect_st.len()], expect_st);
        assert_eq!(&thing[(thing.len() - expect_ed.len())..], expect_ed);
    }
}
