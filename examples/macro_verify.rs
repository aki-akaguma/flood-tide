use flood_tide::{argparse, Arg};

argparse! {
    pub struct MyConf {
        (help, bool, b'h', "help", Arg::No, "display help", @help),
        (version, bool, b'V', "version", Arg::No, "display version", @version),
        (verbose, bool, b'v', "verbose", Arg::No, "verbose mode"),
        (count, u32, b'c', "count", Arg::Yes, "count value"),
        (name, String, b'n', "name", Arg::Yes, "name value"),
    }
}

fn main() {
    let args = ["-vv", "--count=42", "-n", "foo", "extra"];
    let conf = MyConf::parse(&args).unwrap();
    
    println!("{:?}", conf);
    assert!(conf.verbose);
    assert_eq!(conf.count, 42);
    assert_eq!(conf.name, "foo");
    assert_eq!(conf.arg_params, vec!["extra".to_string()]);
}
