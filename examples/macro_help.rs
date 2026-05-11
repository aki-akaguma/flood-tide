use flood_tide::{argparse, Arg, HelpVersion};

argparse! {
    #[allow(non_camel_case_types)]
    pub struct MyConf {
        (help, bool, b'h', "help", Arg::No, "display help", @help),
        (version, bool, b'V', "version", Arg::No, "display version", @version),
        (verbose, bool, b'v', "verbose", Arg::No, "verbose mode"),
    }
}

fn main() {
    let args = ["--help"];
    let conf = MyConf::parse(&args).unwrap();
    
    println!("is_help: {}", conf.is_help());
    assert!(conf.is_help());
    
    let args = ["-V"];
    let conf = MyConf::parse(&args).unwrap();
    println!("is_version: {}", conf.is_version());
    assert!(conf.is_version());
}
