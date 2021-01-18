use std::cmp::Ordering;
use std::io::BufRead;

pub fn do_gen_src() -> anyhow::Result<()> {
    let (vec_optstr, vec_line) = parse_input_file("examples/curl.cmd.txt")?;
    //
    let sss = do_gen_src_help(&vec_optstr, &vec_line)?;
    crate::update_file(&sss, "examples/curl.cmd.help.rs.txt")?;
    crate::update_file(&sss, "benches/curl.cmd.help.rs.txt")?;
    //
    let sss = do_gen_src_match(&vec_optstr)?;
    crate::update_file(&sss, "examples/curl.cmd.match.rs.txt")?;
    crate::update_file(&sss, "benches/curl.cmd.match.rs.txt")?;
    //
    Ok(())
}

fn do_gen_src_help(vec_optstr: &[OptStr], vec_line: &[String]) -> anyhow::Result<String> {
    let mut sss = String::with_capacity(4 * 1024);
    //
    let s = r"// WARN: This file is auto generated by";
    sss += &format!("{} {}", s, env!("CARGO_PKG_NAME"));
    sss += r#"
const OPTIONS_TEXT: &str = r""#;
    for line in vec_line {
        sss += &format!("{}\n", line);
    }
    sss += "\";\n";
    //
    sss += r#"
#[repr(u8)]
#[derive(Debug, PartialEq)]
enum CmdOP {
"#;
    for rec in vec_optstr.iter() {
        sss += &format!("    {},\n", rec.enum_s);
    }
    sss += "}\n";
    sss += r#"
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
"#;
    //
    let vec_optstr_sorted = {
        let mut target: Vec<&OptStr> = vec_optstr.iter().map(|o| o).collect();
        target.sort_by(|&a, &b| match a.lon.cmp(&b.lon) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match a.sho.cmp(&b.sho) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => a.num.cmp(&b.num),
            },
        });
        target
    };
    let s = r#"
#[rustfmt::skip]
const OPT_ARY: [Opt;"#;
    sss += &format!("{}{}] = [\n", s, vec_optstr_sorted.len());
    for rec in vec_optstr_sorted.iter() {
        sss += "    Opt { ";
        if rec.sho.is_empty() {
            sss += "sho: 0u8,  ";
        } else {
            sss += &format!("sho: b'{}', ", rec.sho);
        }
        let s = "\"".to_string() + &rec.lon + "\",";
        sss += &format!("lon: {:-17}", s);
        sss += if rec.meta.is_empty() {
            "has: Arg::No,  "
        } else {
            "has: Arg::Yes, "
        };
        sss += &format!("num: CmdOP::{}.to(), ", rec.enum_s);
        sss += "},\n";
    }
    sss += "];\n";
    //
    let mut vec_optstr_sho_idx: Vec<(_, usize)> = vec_optstr_sorted
        .iter()
        .enumerate()
        .filter(|(_, &o)| !o.sho.is_empty())
        .map(|(i, &o)| (&o.sho, i))
        .collect();
    vec_optstr_sho_idx.sort_by(|a, b| a.0.cmp(&b.0));
    //
    let s = r#"
#[rustfmt::skip]
const OPT_ARY_SHO_IDX: [(u8,usize);"#;
    sss += &format!("{}{}] = [\n", s, vec_optstr_sho_idx.len());
    for elm in vec_optstr_sho_idx.iter() {
        sss += &format!("(b'{}',{}),", elm.0, elm.1);
    }
    sss += "];\n";
    //
    sss += r#"
#[derive(Debug, Default, PartialEq)]
pub struct CmdOptConf {
    pub opt_program: String,
    //
"#;
    for rec in vec_optstr.iter() {
        sss += &format!("    pub {}: {},\n", rec.field_s, rec.type_s);
    }
    sss += r#"    //
    pub arg_params: Vec<String>,
}
"#;
    sss += r#"
impl flood_tide::HelpVersion for CmdOptConf {
    fn is_help(&self) -> bool {
        self.flg_help
    }
    fn is_version(&self) -> bool {
        self.flg_version
    }
}
"#;
    //
    Ok(sss)
}

fn do_gen_src_match(vec_optstr: &[OptStr]) -> anyhow::Result<String> {
    let mut sss = String::with_capacity(4 * 1024);
    //
    let s = r"// WARN: This file is auto generated by";
    sss += &format!("{} {}", s, env!("CARGO_PKG_NAME"));
    //
    sss += r#"
match CmdOP::from(nv.opt.num) {
"#;
    for rec in vec_optstr.iter() {
        sss += &format!("    CmdOP::{} => {{\n", rec.enum_s);
        match rec.type_s.as_str() {
            "bool" => match rec.enum_s.as_str() {
                "Help" => {
                    sss += "        print_help_and_exit(conf);\n";
                }
                "Version" => {
                    sss += "        print_version_and_exit(conf);\n";
                }
                _ => {
                    sss += &format!("        conf.{} = true;\n", rec.field_s);
                }
            },
            "String" => {
                sss += &format!("        conf.{} = value_to_string(nv)?;\n", rec.field_s);
            }
            "u32" => {
                sss += &format!("        conf.{} = value_to_u32(nv)?;\n", rec.field_s);
            }
            "u64" => {
                sss += &format!("        conf.{} = value_to_u64(nv)?;\n", rec.field_s);
            }
            _ => unreachable!(),
        }
        sss += "    }\n";
    }
    sss += r#"}
"#;
    //
    Ok(sss)
}

#[rustfmt::skip]
#[derive(Default, Clone)]
struct OptStr {
    num: i32,           // number
    sho: String,        // short option
    lon: String,        // long option
    meta: String,       // option's meta
    _comment: String,   // option comment
    type_s: String,     // type string
    enum_s: String,     // enume field string
    field_s: String,    // struct field string
}

impl OptStr {
    fn to_enum(&self) -> String {
        let r = &self.lon;
        let v: Vec<_> = r
            .split('-')
            .map(|w| {
                let mut cs: Vec<char> = w.chars().collect();
                cs[0] = cs[0].to_ascii_uppercase();
                let mut s = String::new();
                for c in cs {
                    s.push(if c == '.' { '_' } else { c });
                }
                s
            })
            .collect();
        v.join("")
    }
    fn to_field(&self) -> String {
        let mut s = String::with_capacity(self.lon.len());
        for c in self.lon.chars() {
            #[rustfmt::skip]
            let c = match c { '-' => '_', '.' => '_', _ => c, };
            s.push(c);
        }
        let prefix = if self.meta.is_empty() { "flg_" } else { "opt_" };
        prefix.to_string() + &s
    }
}

fn parse_input_file(in_file: &str) -> anyhow::Result<(Vec<OptStr>, Vec<String>)> {
    let mut vec_line: Vec<String> = Vec::new();
    let mut vec_optstr: Vec<OptStr> = Vec::new();
    //
    let re_1 = regex::Regex::new(r"^ *-([^ ]), +--([^ ]+) +(<[^>]+>) +([^ ].*)$").unwrap();
    let re_2 = regex::Regex::new(r"^ *-([^ ]), +--([^ ]+) +([^ ].*)$").unwrap();
    let re_3 = regex::Regex::new(r"^ +--([^ ]+) +(<[^>]+>) +([^ ].*)$").unwrap();
    let re_4 = regex::Regex::new(r"^ +--([^ ]+) +([^ ].*)$").unwrap();
    //
    let mut v_num = 0;
    let reader = std::io::BufReader::new(std::fs::File::open(in_file)?);
    for line in reader.lines() {
        let line = line?;
        if line == "Options:" {
            // nothing todo
        } else if let Some(caps) = re_1.captures(&line) {
            //  -C  --continue-at <offset>        Resumed transfer offset
            v_num += 1;
            vec_optstr.push(OptStr {
                num: v_num,
                sho: caps[1].to_string(),
                lon: caps[2].to_string(),
                meta: caps[3].to_string(),
                _comment: caps[4].to_string(),
                ..OptStr::default()
            });
        } else if let Some(caps) = re_2.captures(&line) {
            //  -q  --disable             Disable .curlrc
            v_num += 1;
            vec_optstr.push(OptStr {
                num: v_num,
                sho: caps[1].to_string(),
                lon: caps[2].to_string(),
                meta: "".to_string(),
                _comment: caps[3].to_string(),
                ..OptStr::default()
            });
        } else if let Some(caps) = re_3.captures(&line) {
            //      --data-binary <data>  HTTP POST binary data
            v_num += 1;
            vec_optstr.push(OptStr {
                num: v_num,
                sho: "".to_string(),
                lon: caps[1].to_string(),
                meta: caps[2].to_string(),
                _comment: caps[3].to_string(),
                ..OptStr::default()
            });
        } else if let Some(caps) = re_4.captures(&line) {
            //      --digest              Use HTTP Digest Authentication
            v_num += 1;
            vec_optstr.push(OptStr {
                num: v_num,
                sho: "".to_string(),
                lon: caps[1].to_string(),
                meta: "".to_string(),
                _comment: caps[2].to_string(),
                ..OptStr::default()
            });
        } else {
            eprintln!("LINE ERROR: {}", line);
            unreachable!();
        }
        vec_line.push(line);
    }
    //
    for v in &mut vec_optstr {
        let v_type = if v.meta.is_empty() { "bool" } else { "String" };
        let v_type = match v.lon.as_str() {
            "connect-timeout" => "u32",
            "continue-at" => "u64",
            "expect100-timeout" => "u32",
            "happy-eyeballs-timeout-ms" => "u64",
            "keepalive-time" => "u32",
            "limit-rate" => "u64",
            "max-filesize" => "u64",
            "max-redirs" => "u32",
            "max-time" => "u32",
            "retry" => "u32",
            "retry-delay" => "u32",
            "retry-max-time" => "u32",
            "speed-limit" => "u64",
            "speed-time" => "u32",
            "tftp-blksize" => "u32",
            _ => v_type,
        };
        //
        v.type_s = v_type.to_string();
        v.enum_s = v.to_enum();
        v.field_s = v.to_field();
    }
    //
    Ok((vec_optstr, vec_line))
}
