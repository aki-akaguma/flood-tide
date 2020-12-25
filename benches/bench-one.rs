use criterion::{criterion_group, criterion_main, Criterion};

mod one;

#[rustfmt::skip]
const ENV_ARGS: [&str;6] = ["-d", "-vv", "-s", "123", "inp", "oup"];

fn process_one(env_args: &[&str]) -> Result<one::CmdOptConf, one::OPErr> {
    one::parse_cmdopts("prog", env_args)
}

fn criterion_test(_c: &mut Criterion) {
    let result_conf = one::CmdOptConf {
        opt_program: "prog".to_string(),
        flag_debug: true,
        cnt_verbose: 2,
        opt_speed: 123.0,
        opt_config: None,
        arg_input: "inp".to_string(),
        arg_output: Some("oup".to_string()),
        ..Default::default()
    };
    match process_one(criterion::black_box(&ENV_ARGS)) {
        Ok(conf) => {
            assert_eq!(conf, result_conf);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    assert!(one::check_sorted_opt_ary_and_sho_idx_ary());
    //
    //_c.bench_function("test::", |b| { b.iter(|| {}) });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("one::", |b| {
        b.iter(|| {
            let _r = process_one(criterion::black_box(&ENV_ARGS));
        })
    });
}

criterion_group!(tests, criterion_test);
criterion_group!(benches, criterion_benchmark);
criterion_main!(tests, benches);
//
