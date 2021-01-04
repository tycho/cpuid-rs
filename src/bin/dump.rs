extern crate affinity;
extern crate cpuid;
extern crate num_cpus;

use cpuid::cpuid::{Registers,cpuid};

fn print_regs(input: &Registers, output: &Registers) {
    let ascii = output.ascii();
    println!(
        "CPUID {:08x}:{:02x} = {:08x} {:08x} {:08x} {:08x} | {}",
        input.eax, input.ecx, output.eax, output.ebx, output.ecx, output.edx, ascii
    );
}

fn call_leaf(leaf: u32, subleaf: u32) {
    let input = Registers::new(leaf, 0, subleaf, 0);
    let output = cpuid(&input);
    print_regs(&input, &output);
}

fn call_leaf_04() {
    let mut input = Registers::new(0x0000_0004, 0, 0, 0);
    loop {
        let output = cpuid(&input);
        print_regs(&input, &output);
        if output.eax & 0xF == 0 {
            break;
        }
        input.ecx += 1;
    }
}

fn call_leaf_x2apic(leaf: u32) {
    let mut input = Registers::new(leaf, 0, 0, 0);
    loop {
        let output = cpuid(&input);
        if input.ecx > 0 && !(output.eax != 0 || output.ebx != 0) {
            break;
        }
        print_regs(&input, &output);
        input.ecx += 1;
    }
}

fn call_leaf_0d() {
    let mut input = Registers::new(0x0000_000D, 0, 0, 0);
    loop {
        let output = cpuid(&input);
        if input.ecx > 0
            && !(output.eax != 0 || output.ebx != 0 || output.ecx != 0 || output.edx != 0)
        {
            break;
        }
        print_regs(&input, &output);
        if input.ecx == 0 && output.eax == 0 {
            break;
        }
        input.ecx += 1;
    }
}

fn call_leaf_0f() {
    let mut input = Registers::new(0x0000_000F, 0, 0, 0);
    let output = cpuid(&input);
    let mut max_ecx = 0;
    if (output.edx & 0x2) != 0 {
        max_ecx = 1
    }
    loop {
        let output = cpuid(&input);
        print_regs(&input, &output);
        input.ecx += 1;
        if input.ecx > max_ecx {
            break;
        }
    }
}

fn call_leaf_10() {
    let mut input = Registers::new(0x0000_0010, 0, 0, 0);
    let output = cpuid(&input);
    let mut max_ecx = 0;
    if (output.ebx & 0x2) != 0 {
        max_ecx = 1
    }
    loop {
        let output = cpuid(&input);
        print_regs(&input, &output);
        input.ecx += 1;
        if input.ecx > max_ecx {
            break;
        }
    }
}

fn call_leaf_12() {
    let mut input = Registers::new(0x0000_0007, 0, 0, 0);
    let output = cpuid(&input);
    let sgx_supported = (output.ebx & 0x4) != 0;
    input.eax = 0x0000_0012;
    loop {
        let output = cpuid(&input);
        if input.ecx > 1 && (output.eax & 0xf) == 0 {
            break;
        }
        print_regs(&input, &output);
        if !sgx_supported {
            break;
        }
        input.ecx += 1;
    }
}

fn call_leaf_1b() {
    let mut input = Registers::new(0x0000_0007, 0, 0, 0);
    let output = cpuid(&input);
    let pconfig_supported = (output.edx & 0x0004_0000) != 0;
    input.eax = 0x0000_001b;
    loop {
        let output = cpuid(&input);
        if input.ecx > 0 && (output.eax & 0xfff) == 0 {
            break;
        }
        print_regs(&input, &output);
        if !pconfig_supported {
            break;
        }
        input.ecx += 1;
    }
}

fn call_leaf_max_ecx(leaf: u32, max_subleaf: u32) {
    let mut input = Registers::new(leaf, 0, 0, 0);
    loop {
        let output = cpuid(&input);
        print_regs(&input, &output);
        input.ecx += 1;
        if input.ecx > max_subleaf {
            break;
        }
    }
}

fn call_leaf_ext_1d() {
    let mut input = Registers::new(0x8000_0001, 0, 0, 0);
    let output = cpuid(&input);
    let ext_topology_supported = (output.ecx & 0x0040_0000) != 0;
    input.eax = 0x8000_001d;
    loop {
        let output = cpuid(&input);
        if input.ecx > 0 && output.eax == 0 {
            break;
        }
        print_regs(&input, &output);
        if !ext_topology_supported {
            break;
        }
        input.ecx += 1;
    }
}

fn call_leaf_indexed(leaf: u32) {
    let mut input = Registers::new(leaf, 0, 0, 0);
    let output = cpuid(&input);
    let max_ecx = output.eax;
    loop {
        print_regs(&input, &output);
        input.ecx += 1;
        if input.ecx > max_ecx {
            break;
        }
    }
}

fn enumerate_leaves(base: u32) {
    let input = Registers::new(base, 0, 0, 0);
    let output = cpuid(&input);

    // All valid bases use eax to indicate the maximum supported leaf within that range.
    if output.eax < base || output.eax > base + 0xFFFF {
        // Even if this base isn't valid, print it so that our dump is comprehensive.
        print_regs(&input, &output);
        return;
    }

    for leaf in input.eax..(output.eax + 1) {
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
