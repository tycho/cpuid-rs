use std::fmt;

#[derive(Debug, Clone)]
pub struct LeafID {
    pub eax: u32,
    pub ecx: u32,
}

#[derive(Debug, Clone)]
pub struct Registers {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

pub enum RegisterName {
    EAX,
    EBX,
    ECX,
    EDX,
}

impl LeafID {
    pub fn new(eax: u32, ecx: u32) -> LeafID {
        LeafID { eax: eax, ecx: ecx }
    }
}

impl Registers {
    pub fn new(eax: u32, ebx: u32, ecx: u32, edx: u32) -> Registers {
        Registers {
            eax: eax,
            ebx: ebx,
            ecx: ecx,
            edx: edx,
        }
    }

    pub fn register(&self, name: RegisterName) -> u32 {
        match name {
            RegisterName::EAX => self.eax,
            RegisterName::EBX => self.ebx,
            RegisterName::ECX => self.ecx,
            RegisterName::EDX => self.edx,
        }
    }

    /// Try to create an ASCII representation of the bytes in the registers. Uses '.' as
    /// a placeholder for invalid ASCII values.
    pub fn ascii(&self) -> String {
        let mut string = String::new();
        for register in [self.eax, self.ebx, self.ecx, self.edx].iter() {
            for byte in register.to_le_bytes().iter() {
                if *byte > 31 && *byte < 127 {
                    string.push(*byte as char)
                } else {
                    string.push('.')
                }
            }
        }
        string
    }
}

pub fn cpuid(input: &LeafID, output: &mut Registers) {
    unsafe {
        asm!("cpuid",
            inout("eax") input.eax => output.eax,
            lateout("ebx") output.ebx,
            inout("ecx") input.ecx => output.ecx,
            lateout("edx") output.edx)
    }
}

#[derive(Debug, Clone)]
pub struct CPUID {
    pub input: LeafID,
    pub output: Registers,
}

impl CPUID {
    pub fn new() -> CPUID {
        CPUID {
            input: LeafID::new(0, 0),
            output: Registers::new(0, 0, 0, 0),
        }
    }
    pub fn invoke(eax: u32, ecx: u32) -> CPUID {
        let input = LeafID::new(eax, ecx);
        let mut output = Registers::new(0, 0, 0, 0);
        cpuid(&input, &mut output);
        CPUID {
            input: input,
            output: output,
        }
    }
    pub fn call(&mut self) {
        cpuid(&self.input, &mut self.output);
    }
    pub fn next_subleaf(&mut self) {
        self.input.ecx += 1;
        cpuid(&self.input, &mut self.output);
    }
}

#[derive(Debug)]
pub struct CPUIDSnapshot {
    pub leaves: Vec<CPUID>,
}

impl CPUIDSnapshot {
    pub fn new() -> CPUIDSnapshot {
        CPUIDSnapshot { leaves: vec![] }
    }

    pub fn get_subleaf(&self, leaf: u32, subleaf: u32) -> Option<&CPUID> {
        for result in self.leaves.iter() {
            if result.input.eax == leaf && result.input.ecx == subleaf {
                return Some(&result);
            }
        }
        None
    }

    pub fn get(&self, leaf: u32) -> Vec<&CPUID> {
        let mut out: Vec<&CPUID> = vec![];
        for result in self.leaves.iter() {
            if result.input.eax == leaf {
                out.push(&result);
            }
        }
        out
    }

    pub fn has_feature_bit(
        &self,
        leaf: u32,
        subleaf: u32,
        register: RegisterName,
        bit: u32,
    ) -> bool {
        match self.get_subleaf(leaf, subleaf) {
            None => false,
            Some(leafdata) => {
                let bits = match register {
                    RegisterName::EAX => leafdata.output.eax,
                    RegisterName::EBX => leafdata.output.ebx,
                    RegisterName::ECX => leafdata.output.ecx,
                    RegisterName::EDX => leafdata.output.edx,
                };
                bits & (1 << bit) != 0
            }
        }
    }
}

impl fmt::Display for CPUID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CPUID {:08x}:{:02x} = {:08x} {:08x} {:08x} {:08x} | {}",
            self.input.eax,
            self.input.ecx,
            self.output.eax,
            self.output.ebx,
            self.output.ecx,
            self.output.edx,
            self.output.ascii()
        )
    }
}

fn call_leaf(out: &mut Vec<CPUID>, leaf: u32, subleaf: u32) {
    let state = CPUID::invoke(leaf, subleaf);
    out.push(state);
}

fn call_leaf_04(out: &mut Vec<CPUID>) {
    let mut state = CPUID::invoke(0x0000_0004, 0);
    loop {
        out.push(state.clone());
        if state.output.eax & 0xF == 0 {
            break;
        }
        state.next_subleaf();
    }
}

fn call_leaf_x2apic(out: &mut Vec<CPUID>, leaf: u32) {
    let mut state = CPUID::invoke(leaf, 0);
    loop {
        if state.input.ecx > 0 && !(state.output.eax != 0 || state.output.ebx != 0) {
            break;
        }
        out.push(state.clone());
        state.next_subleaf();
    }
}

fn call_leaf_0d(out: &mut Vec<CPUID>) {
    let mut state = CPUID::invoke(0x0000_000D, 0);
    loop {
        if state.input.ecx > 0
            && !(state.output.eax != 0
                || state.output.ebx != 0
                || state.output.ecx != 0
                || state.output.edx != 0)
        {
            break;
        }
        out.push(state.clone());
        if state.input.ecx == 0 && state.output.eax == 0 {
            break;
        }
        state.next_subleaf();
    }
}

fn call_leaf_0f(out: &mut Vec<CPUID>) {
    let mut state = CPUID::invoke(0x0000_000F, 0);
    let mut max_ecx = 0;
    if (state.output.edx & 0x2) != 0 {
        max_ecx = 1
    }
    loop {
        out.push(state.clone());
        state.next_subleaf();
        if state.input.ecx > max_ecx {
            break;
        }
    }
}

fn call_leaf_10(out: &mut Vec<CPUID>) {
    let mut state = CPUID::invoke(0x0000_0010, 0);
    let mut max_ecx = 0;
    if (state.output.ebx & 0x2) != 0 {
        max_ecx = 1
    }
    loop {
        out.push(state.clone());
        state.next_subleaf();
        if state.input.ecx > max_ecx {
            break;
        }
    }
}

fn call_leaf_12(out: &mut Vec<CPUID>) {
    let feature_check = CPUID::invoke(0x0000_0007, 0);
    let sgx_supported = (feature_check.output.ebx & 0x4) != 0;
    let mut state = CPUID::invoke(0x0000_0012, 0);
    loop {
        if state.input.ecx > 1 && (state.output.eax & 0xf) == 0 {
            break;
        }
        out.push(state.clone());
        if !sgx_supported {
            break;
        }
        state.next_subleaf();
    }
}

fn call_leaf_1b(out: &mut Vec<CPUID>) {
    let feature_check = CPUID::invoke(0x0000_0007, 0);
    let pconfig_supported = (feature_check.output.edx & 0x0004_0000) != 0;
    let mut state = CPUID::invoke(0x0000_001B, 0);
    loop {
        if state.input.ecx > 0 && (state.output.eax & 0xfff) == 0 {
            break;
        }
        out.push(state.clone());
        if !pconfig_supported {
            break;
        }
        state.next_subleaf();
    }
}

fn call_leaf_max_ecx(out: &mut Vec<CPUID>, leaf: u32, max_subleaf: u32) {
    let mut state = CPUID::invoke(leaf, 0);
    loop {
        out.push(state.clone());
        state.next_subleaf();
        if state.input.ecx > max_subleaf {
            break;
        }
    }
}

fn call_leaf_ext_1d(out: &mut Vec<CPUID>) {
    let feature_check = CPUID::invoke(0x8000_0001, 0);
    let ext_topology_supported = (feature_check.output.ecx & 0x0040_0000) != 0;
    let mut state = CPUID::invoke(0x8000_001D, 0);
    loop {
        if state.input.ecx > 0 && state.output.eax == 0 {
            break;
        }
        out.push(state.clone());
        if !ext_topology_supported {
            break;
        }
        state.next_subleaf();
    }
}

fn call_leaf_indexed(out: &mut Vec<CPUID>, leaf: u32) {
    let mut state = CPUID::invoke(leaf, 0);
    let max_ecx = state.output.eax;
    loop {
        out.push(state.clone());
        state.next_subleaf();
        if state.input.ecx > max_ecx {
            break;
        }
    }
}

fn walk_leaves(out: &mut Vec<CPUID>, base: u32) {
    let state = CPUID::invoke(base, 0);

    // All valid bases use eax to indicate the maximum supported leaf within that range.
    if state.output.eax < base || state.output.eax > base + 0xFFFF {
        // Even if this base isn't valid, print it so that our dump is comprehensive.
        out.push(state);
        return;
    }

    for leaf in state.input.eax..(state.output.eax + 1) {
        // Some leaves are indexed (i.e. passing different values for ecx will generate different
        // results). Unfortunately how they're indexed varies significantly. We need to call
        // a handler for each of the special leaves so they can be dumped fully.
        match leaf {
            0x0000_0004 => call_leaf_04(out),
            0x0000_0007 => call_leaf_indexed(out, leaf),
            0x0000_000B => call_leaf_x2apic(out, leaf),
            0x0000_000D => call_leaf_0d(out),
            0x0000_000F => call_leaf_0f(out),
            0x0000_0010 => call_leaf_10(out),
            0x0000_0012 => call_leaf_12(out),
            0x0000_0014 => call_leaf_indexed(out, leaf),
            0x0000_0017 => call_leaf_indexed(out, leaf),
            0x0000_0018 => call_leaf_indexed(out, leaf),
            0x0000_001B => call_leaf_1b(out),
            0x0000_001D => call_leaf_indexed(out, leaf),
            0x0000_001F => call_leaf_x2apic(out, leaf),
            0x0000_0020 => call_leaf_indexed(out, leaf),
            0x8000_001D => call_leaf_ext_1d(out),
            0x8000_0020 => call_leaf_max_ecx(out, leaf, 1),
            _ => call_leaf(out, leaf, 0),
        }
    }
}

fn walk_bases(out: &mut Vec<CPUID>) {
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
        walk_leaves(out, *base);
    }
}

/// Walk all known CPUID leaves on the current processor. Note that you should
/// set your process or thread affinity to prevent the OS from moving the
/// process/thread around causing you to query other CPUs inadvertently.
pub fn walk() -> CPUIDSnapshot {
    let mut out: CPUIDSnapshot = CPUIDSnapshot::new();
    walk_bases(&mut out.leaves);
    out
}
