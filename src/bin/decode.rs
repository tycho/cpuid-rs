use clap::{value_t, App, Arg};

use cpuid::cpuid::{walk as cpuid_walk, snapshots_from_file, CPUIDSnapshot};
use cpuid::cache::walk as cache_walk;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn collect_file(cpu_start: u32, cpu_end: u32, filename: &str) -> Vec<(u32, CPUIDSnapshot)> {
    let snapshot_result = snapshots_from_file(filename);
    let mut collected: Vec<(u32, CPUIDSnapshot)> = vec![];
    if let Ok(snapshots) = snapshot_result {
        for (idx, snapshot) in snapshots.iter() {
            if *idx >= cpu_start as u32 && *idx <= cpu_end as u32 {
                collected.push((*idx, snapshot.clone()))
            }
        }
    }
    collected
}

fn collect_local(cpu_start: u32, cpu_end: u32) -> Vec<(u32, CPUIDSnapshot)> {
    let mut collected: Vec<(u32, CPUIDSnapshot)> = vec![];

    for cpu in cpu_start..(cpu_end + 1) {
        let mask = vec![cpu as usize];

        // TODO: This can fail, and we should be noisy about it when it does.
        // Though if we're on macOS we can't do anything about it since there
        // isn't any thread affinity API there.
        affinity::set_thread_affinity(mask).unwrap();

        collected.push((cpu, cpuid_walk()));
    }

    collected
}

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
        ).arg(
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

    let snapshots: Vec<(u32, CPUIDSnapshot)> =
        match matches.value_of("file") {
            Some(filename) => collect_file(cpu_start, cpu_end, filename),
            _ => collect_local(cpu_start, cpu_end),
        };

    for (cpu, snapshot) in snapshots.iter() {
        println!("CPU {}:", cpu);
        println!("Caches:");
        println!("{:#?}", cache_walk(&snapshot));
    }
}
