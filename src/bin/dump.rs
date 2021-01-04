extern crate affinity;
extern crate cpuid;
extern crate num_cpus;

use cpuid::cpuid::CPUID;

fn print_cpuid(id: &CPUID) {
    println!("{}", id);
}

fn call_leaf(leaf: u32, subleaf: u32) {
    let id = CPUID::invoke(leaf, subleaf);
    print_cpuid(&id)
}

fn call_leaf_04() {
    let mut state = CPUID::invoke(0x0000_0004, 0);
    loop {
        print_cpuid(&state);
        if state.output.eax & 0xF == 0 {
            break;
        }
        state.next_subleaf();
    }
}

fn call_leaf_x2apic(leaf: u32) {
    let mut state = CPUID::invoke(leaf, 0);
    loop {
        if state.input.ecx > 0 && !(state.output.eax != 0 || state.output.ebx != 0) {
            break;
        }
        print_cpuid(&state);
        state.next_subleaf();
    }
}

fn call_leaf_0d() {
    let mut state = CPUID::invoke(0x0000_000D, 0);
    loop {
        if state.input.ecx > 0
            && !(state.output.eax != 0 || state.output.ebx != 0 || state.output.ecx != 0 || state.output.edx != 0)
        {
            break;
        }
        print_cpuid(&state);
        if state.input.ecx == 0 && state.output.eax == 0 {
            break;
        }
        state.next_subleaf();
    }
}

fn call_leaf_0f() {
    let mut state = CPUID::invoke(0x0000_000F, 0);
    let mut max_ecx = 0;
    if (state.output.edx & 0x2) != 0 {
        max_ecx = 1
    }
    loop {
        print_cpuid(&state);
        state.next_subleaf();
        if state.input.ecx > max_ecx {
            break;
        }
    }
}

fn call_leaf_10() {
    let mut state = CPUID::invoke(0x0000_0010, 0);
    let mut max_ecx = 0;
    if (state.output.ebx & 0x2) != 0 {
        max_ecx = 1
    }
    loop {
        print_cpuid(&state);
        state.next_subleaf();
        if state.input.ecx > max_ecx {
            break;
        }
    }
}

fn call_leaf_12() {
    let feature_check = CPUID::invoke(0x0000_0007, 0);
    let sgx_supported = (feature_check.output.ebx & 0x4) != 0;
    let mut state = CPUID::invoke(0x0000_0012, 0);
    loop {
        if state.input.ecx > 1 && (state.output.eax & 0xf) == 0 {
            break;
        }
        print_cpuid(&state);
        if !sgx_supported {
            break;
        }
        state.next_subleaf();
    }
}

fn call_leaf_1b() {
    let feature_check = CPUID::invoke(0x0000_0007, 0);
    let pconfig_supported = (feature_check.output.edx & 0x0004_0000) != 0;
    let mut state = CPUID::invoke(0x0000_001B, 0);
    loop {
        if state.input.ecx > 0 && (state.output.eax & 0xfff) == 0 {
            break;
        }
        print_cpuid(&state);
        if !pconfig_supported {
            break;
        }
        state.next_subleaf();
    }
}

fn call_leaf_max_ecx(leaf: u32, max_subleaf: u32) {
    let mut state = CPUID::invoke(leaf, 0);
    loop {
        print_cpuid(&state);
        state.next_subleaf();
        if state.input.ecx > max_subleaf {
            break;
        }
    }
}

fn call_leaf_ext_1d() {
    let feature_check = CPUID::invoke(0x8000_0001, 0);
    let ext_topology_supported = (feature_check.output.ecx & 0x0040_0000) != 0;
    let mut state = CPUID::invoke(0x8000_001D, 0);
    loop {
        if state.input.ecx > 0 && state.output.eax == 0 {
            break;
        }
        print_cpuid(&state);
        if !ext_topology_supported {
            break;
        }
        state.next_subleaf();
    }
}

fn call_leaf_indexed(leaf: u32) {
    let mut state = CPUID::invoke(leaf, 0);
    let max_ecx = state.output.eax;
    loop {
        print_cpuid(&state);
        state.next_subleaf();
        if state.input.ecx > max_ecx {
            break;
        }
    }
}

fn enumerate_leaves(base: u32) {
    let state = CPUID::invoke(base, 0);

    // All valid bases use eax to indicate the maximum supported leaf within that range.
    if state.output.eax < base || state.output.eax > base + 0xFFFF {
        // Even if this base isn't valid, print it so that our dump is comprehensive.
        print_cpuid(&state);
        return;
    }

    for leaf in state.input.eax..(state.output.eax + 1) {
        // Some leaves are indexed (i.e. passing different values for ecx will generate different
        // results). Unfortunately how they're indexed varies significantly. We need to call
        // a handler for each of the special leaves so they can be dumped fully.
        match leaf {
            0x0000_0004 => call_leaf_04(),
            0x0000_0007 => call_leaf_indexed(leaf),
            0x0000_000B => call_leaf_x2apic(leaf),
            0x0000_000D => call_leaf_0d(),
            0x0000_000F => call_leaf_0f(),
            0x0000_0010 => call_leaf_10(),
            0x0000_0012 => call_leaf_12(),
            0x0000_0014 => call_leaf_indexed(leaf),
            0x0000_0017 => call_leaf_indexed(leaf),
            0x0000_0018 => call_leaf_indexed(leaf),
            0x0000_001B => call_leaf_1b(),
            0x0000_001D => call_leaf_indexed(leaf),
            0x0000_001F => call_leaf_x2apic(leaf),
            0x0000_0020 => call_leaf_indexed(leaf),
            0x8000_001D => call_leaf_ext_1d(),
            0x8000_0020 => call_leaf_max_ecx(leaf, 1),
            _ => call_leaf(leaf, 0),
        }
    }
}

fn enumerate_bases() {
    let bases = vec![
        // Standard base.
        0x0000_0000,
        // Hypervisor base.
        0x4000_0000,
        // Extended base (mostly AMD things here)
        0x8000_0000,
        // Transmeta base
        0x8086_0000,
        // Centaur base
        0xc000_0000,
    ];
    for base in bases.iter() {
        enumerate_leaves(*base);
    }
}

fn main() {
    let cpus = num_cpus::get();
    for cpu in 0..cpus {
        println!("CPU {}:", cpu);
        let mask = vec![cpu];
        affinity::set_thread_affinity(mask).unwrap();
        enumerate_bases();
    }
}
