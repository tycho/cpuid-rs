use getopts::Options;
use std::env;

use cpuid::cpuid::System;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "f",
        "file",
        "Parse and import dump file instead of reading from local CPUs",
        "FILE",
    );
    opts.optopt("c", "cpu", "Which CPU to decode CPUID information from", "INDEX");
    opts.optflag("h", "help", "Print this help text");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // TODO: This kinda sucks because it will silently eat bogus values. We want
    // it to eventually accept integer values (21), integer ranges (21-35),
    // integer lists (21,22,23), or the string "all" (or similar).
    let cpu_index: i32 = matches
        .opt_str("cpu")
        .unwrap_or("-1".to_string())
        .parse::<i32>()
        .unwrap_or(-1);

    let mut cpu_start: u32 = 0;
    let mut cpu_end: u32 = num_cpus::get() as u32 - 1;

    if cpu_index > cpu_end as i32 {
        panic!(
            "CPU {} does not exist (valid range: {} to {})",
            cpu_index, cpu_start, cpu_end
        );
    }

    // For now we only accept a single CPU index in the --cpu argument, and set
    // the range to only include that value.
    if cpu_index >= 0 {
        cpu_start = cpu_index as u32;
        cpu_end = cpu_index as u32;
    }

    env_logger::init();

    let system = match matches.opt_str("file") {
        Some(filename) => System::from_file(&filename).unwrap(),
        _ => System::from_local(),
    };

    for processor in system.cpus.iter() {
        if processor.index < cpu_start || processor.index > cpu_end {
            continue;
        }
        println!("CPU {}:", processor.index);
        for entry in processor.leaves.iter() {
            println!("{}", entry);
        }
    }
}
