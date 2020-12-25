use itertools::Itertools;

pub fn do_gen_src() -> anyhow::Result<()> {
    // test has 9 atomeed 50 minutes.
    let v = vec![
        //"no_std",
        //
        "argument",
        "option_argument",
        "stop_at_mm",
        "subcommand",
        //
        "long_only",
        "single_error",
        "stop_at_free",
        //
        "abbreviate",
        //"optnum_u16",
        //"was_long",
    ];
    //
    let sss = do_gen_src_make(&v)?;
    crate::update_file(&sss, "features_comb.mk")?;
    //
    Ok(())
}

fn do_gen_src_make(v: &[&str]) -> anyhow::Result<String> {
    let mut sss = String::with_capacity(4 * 1024);
    sss += r#"features_comb = "#;
    sss += &v.join(" ");
    //
    for i in 2..v.len() {
        let comb = v.iter().combinations(i);
        for item in comb {
            sss += " ";
            sss += &item.iter().copied().join("+");
        }
    }
    sss += "\n";
    //
    Ok(sss)
}
