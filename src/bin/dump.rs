use clap::{value_t, App, Arg};

use cpuid::cpuid::walk as cpuid_walk;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("CPUID dump tool")
        .version(VERSION)
        .author("Steven Noonan <steven@uplinklabs.net>")
        .about("Dumps all known valid CPUID leaves to stdout")
        .arg(
            Arg::with_name("cpu")
                .short("c")
                .long("cpu")
                .value_name("INDEX")
                .help("Which CPU to dump CPUID information from")
                .takes_value(true),
        )
        .get_matches();
    let mut cpu_start: usize = 0;
    let mut cpu_end: usize = num_cpus::get() - 1;

    // TODO: This kinda sucks because it will silently eat bogus values. We want
    // it to eventually accept integer values (21), integer ranges (21-35),
    // integer lists (21,22,23), or the string "all" (or similar).
    let cpu_index = value_t!(matches, "cpu", i32).unwrap_or(-1);

    if cpu_index > cpu_end as i32 {
        panic!(
            "CPU {} does not exist (valid range: {} to {})",
            cpu_index, cpu_start, cpu_end
        );
    }

    // For now we only accept a single CPU index in the --cpu argument, and set
    // the range to only include that value.
    if cpu_index >= 0 {
        cpu_start = cpu_index as usize;
        cpu_end = cpu_index as usize;
    }

    for cpu in cpu_start..(cpu_end + 1) {
        println!("CPU {}:", cpu);
        let mask = vec![cpu];

        // TODO: This can fail, and we should be noisy about it when it does.
        // Though if we're on macOS we can't do anything about it since there
        // isn't any thread affinity API there.
        affinity::set_thread_affinity(mask).unwrap();

        let snapshot = cpuid_walk();
        for entry in snapshot.leaves.iter() {
            println!("{}", entry);
        }
    }
}
