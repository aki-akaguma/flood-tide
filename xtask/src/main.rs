//
// ref)
//   https://github.com/matklad/cargo-xtask
//
use anyhow::Context;
use std::io::Read;
use std::io::Write;

mod gen_features_combination;
mod gen_src_example_curl_cmd;

fn main() -> anyhow::Result<()> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let program = env_args.remove(0);
    if env_args.is_empty() {
        print_help_and_exit(&program);
    }
    let cmd = env_args[0].as_str();
    match cmd {
        "all-gen" => all_gen()?,
        "gen-src-example-curl-cmd" => gen_src_example_curl_cmd::do_gen_src()?,
        "gen-features-combination" => gen_features_combination::do_gen_src()?,
        "--help" | "-h" | "-H" | "help" => print_help_and_exit(&program),
        "--version" | "-V" | "-v" => print_version_and_exit(&program),
        "x" => do_x()?,
        _ => {
            eprintln!("Not fount command: {}", cmd);
            unreachable!()
        }
    }
    //
    Ok(())
}

fn print_version_and_exit(_program: &str) {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    std::process::exit(0);
}

fn print_help_and_exit(program: &str) {
    println!(
        "[usage] {} {{{}}}",
        program,
        concat!(
            "all-gen",
            "|",
            "gen-src-example-curl-cmd",
            "|",
            "gen-features-combination"
        )
    );
    std::process::exit(0);
}

fn all_gen() -> anyhow::Result<()> {
    gen_src_example_curl_cmd::do_gen_src()?;
    gen_features_combination::do_gen_src()?;
    Ok(())
}

pub fn update_file(sss: &String, file_path: &str) -> anyhow::Result<()> {
    let contents = {
        let mut contents = String::new();
        if let Ok(mut file) = std::fs::File::open(file_path) {
            file.read_to_string(&mut contents)
                .with_context(|| format!("could not read file `{}`", file_path))?;
        }
        contents
    };
    if contents != *sss {
        println!("update: {}", file_path);
        let mut file = std::fs::File::create(file_path)
            .with_context(|| format!("could not create file `{}`", file_path))?;
        write!(file, "{}", sss).with_context(|| format!("could not write file `{}`", file_path))?;
    }
    //
    Ok(())
}

fn do_x() -> anyhow::Result<()> {
    Ok(())
}
