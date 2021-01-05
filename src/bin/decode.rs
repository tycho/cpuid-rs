use clap::{value_t, App, Arg};

use cpuid::cpuid::System;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("CPUID decoding tool")
        .version(VERSION)
        .author("Steven Noonan <steven@uplinklabs.net>")
        .about("Decodes known valid CPUID leaves to stdout")
        .arg(
            Arg::with_name("cpu")
                .short("c")
                .long("cpu")
                .value_name("INDEX")
                .help("Which CPU to decode CPUID information from")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Parse and import dump file instead of reading from local CPUs")
                .takes_value(true),
        )
        .get_matches();

    // TODO: This kinda sucks because it will silently eat bogus values. We want
    // it to eventually accept integer values (21), integer ranges (21-35),
    // integer lists (21,22,23), or the string "all" (or similar).
    let cpu_index = value_t!(matches, "cpu", i32).unwrap_or(0);

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

    let system = match matches.value_of("file") {
        Some(filename) => System::from_file(filename).unwrap(),
        _ => System::from_local(),
    };

    for (cpu, _snapshot) in system.cpus.iter() {
        if *cpu < cpu_start || *cpu > cpu_end {
            continue;
        }
        println!("CPU {}:", cpu);
    }
    //println!("{:#?}", system.caches());
    println!("{}", system.caches());
}
