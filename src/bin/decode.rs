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
    opts.optflag("v", "verbose", "Print more details");
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

    env_logger::init();

    let system = match matches.opt_str("file") {
        Some(filename) => System::from_file(&filename).unwrap(),
        _ => System::from_local(),
    }
    .with_decoded();

    println!("{: >16}: {:?}", "Vendor(s)", system.vendor);
    println!("{: >16}: {}", "Processor Name", system.name_string);
    println!("{: >16}: {}", "Signature", system.cpus[0].signature);
    if system.topology.valid() {
        println!("{: >16}: {}", "Topology", system.topology);
    } else {
        println!("{: >16}: {}", "Logical CPUs", system.cpu_count);
    }
    if matches.opt_present("v") {
        println!("\nLogical CPU topology IDs:");
        for cpu in system.cpus.iter() {
            if let Some(topology) = cpu.topology() {
                println!("  CPU {}: {}", cpu.index, topology);
            }
        }
    }
    println!("\n{}", system.caches);
    println!("{}", system.features);
}
