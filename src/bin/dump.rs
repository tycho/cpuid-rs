extern crate affinity;
extern crate cpuid;
extern crate num_cpus;

use cpuid::cpuid::{walk, CPUID};

fn print_cpuid(id: &CPUID) {
    println!("{}", id);
}

fn main() {
    let cpus = num_cpus::get();
    for cpu in 0..cpus {
        println!("CPU {}:", cpu);
        let mask = vec![cpu];
        affinity::set_thread_affinity(mask).unwrap();
        for entry in walk().iter() {
            print_cpuid(&entry);
        }
    }
}
