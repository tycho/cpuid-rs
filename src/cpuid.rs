use bitflags::*;
use log::*;
use scan_fmt::*;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::cache::{describe_caches, CacheVec};
use crate::feature::{describe_features, FeatureVec};

#[derive(Debug, Clone, PartialEq)]
pub struct LeafID {
    /// Input `eax` value
    pub eax: u32,

    /// Input `ecx` value
    pub ecx: u32,
}

impl LeafID {
    pub fn new(eax: u32, ecx: u32) -> LeafID {
        LeafID { eax: eax, ecx: ecx }
    }
}

#[derive(Debug, Clone)]
pub struct Registers {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum RegisterName {
    EAX,
    EBX,
    ECX,
    EDX,
    Unknown,
}

bitflags! {
    pub struct VendorMask: u32 {
        const UNKNOWN = 0x0000_0000;

        // Common helper masks
        const ANY_CPU = 0x0000_00FF;
        const ANY_HYPERVISOR = 0x0000_FF00;

        // One-hot identifiers for CPU vendors
        const INTEL = 0x0000_0001;
        const AMD = 0x0000_0002;
        const CENTAUR = 0x0000_0004;
        const CYRIX = 0x0000_0008;
        const TRANSMETA = 0x0000_0010;
        const HYGON = 0x0000_0020;

        // Common vendor masks
        const INTELAMD = 0x0000_0003;

        // One-hot identifiers for hypervisor vendors
        const HYPERV = 0x0000_0100;
        const KVM = 0x0000_0200;
        const TCG = 0x0000_0400;
        const XEN = 0x0000_0800;
        const PARALLELS = 0x0000_1000;
        const VMWARE = 0x0000_2000;
        const BHYVE = 0x0000_4000;
    }
}

impl VendorMask {
    fn from_string(input: String) -> VendorMask {
        match input.as_str() {
            "GenuineIntel" => VendorMask::INTEL,
            "GenuineIotel" => VendorMask::INTEL,
            "AuthenticAMD" => VendorMask::AMD,
            "CentaurHauls" => VendorMask::CENTAUR,
            "CyrixInstead" => VendorMask::CYRIX,
            "GenuineTMx86" => VendorMask::TRANSMETA,
            "HygonGenuine" => VendorMask::HYGON,

            "Microsoft Hv" => VendorMask::HYPERV,
            "KVMKVMKVM" => VendorMask::KVM,
            "TCGTCGTCGTCG" => VendorMask::TCG,
            "XenVMMXenVMM" => VendorMask::XEN,
            " lrpepyh  vr" => VendorMask::PARALLELS,
            "VMwareVMware" => VendorMask::VMWARE,
            "bhyve bhyve " => VendorMask::BHYVE,
            _ => VendorMask::UNKNOWN,
        }
    }
}

fn bytes_to_ascii_dump(bytes: Vec<u8>) -> String {
    let mut string = String::new();
    for byte in bytes.iter() {
        if *byte > 31 && *byte < 127 {
            string.push(*byte as char)
        } else {
            string.push('.')
        }
    }
    string
}

fn bytes_to_ascii(bytes: Vec<u8>) -> String {
    let mut string = String::new();
    for byte in bytes.iter() {
        let chr = *byte as char;
        if chr.is_ascii() {
            string.push(chr);
        }
    }
    string
}

fn squeeze_str(input: String) -> String {
    let mut output = String::new();
    let mut last_was_space = false;
    for inchar in input.trim().chars() {
        if inchar == '\0' {
            break;
        }
        if inchar.is_whitespace() {
            if !last_was_space {
                output.push(inchar);
                last_was_space = true;
            }
        } else if !inchar.is_control() {
            output.push(inchar);
            last_was_space = false;
        }
    }

    // We may have pushed a single space as the last character, truncate that away.
    output.truncate(output.trim_end().len());

    output
}

impl Registers {
    /// Creates a new [Registers](struct.Registers.html) structure from register
    /// values.
    pub fn new(eax: u32, ebx: u32, ecx: u32, edx: u32) -> Registers {
        Registers {
            eax: eax,
            ebx: ebx,
            ecx: ecx,
            edx: edx,
        }
    }

    /// Read a specific register by name.
    pub fn register(&self, name: RegisterName) -> u32 {
        match name {
            RegisterName::EAX => self.eax,
            RegisterName::EBX => self.ebx,
            RegisterName::ECX => self.ecx,
            RegisterName::EDX => self.edx,
            _ => panic!("Invalid register"),
        }
    }

    /// Try to create an ASCII representation of the bytes in the registers,
    /// ordered as `[eax, ebx, ecx, edx]`. Uses `.` as a placeholder for bytes
    /// that cannot be represented as ASCII values.
    pub fn ascii(&self) -> String {
        let mut bytes: Vec<u8> = vec![];
        for register in [self.eax, self.ebx, self.ecx, self.edx].iter() {
            for byte in register.to_le_bytes().iter() {
                bytes.push(*byte);
            }
        }
        bytes_to_ascii_dump(bytes)
    }
}

#[cfg(target_arch = "x86")]
use core::arch::x86::__cpuid_count;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::__cpuid_count;

/// Directly query via the CPUID instruction on the current processor. You
/// probably don't want to use this function, unless you want to discover
/// information that this library does not expose an interface for. It is
/// recommended that you use the higher level interfaces available in the
/// [System](struct.System.html) structure.
pub fn cpuid(input: &LeafID, output: &mut Registers) {
    unsafe {
        let result = __cpuid_count(input.eax, input.ecx);
        output.eax = result.eax;
        output.ebx = result.ebx;
        output.ecx = result.ecx;
        output.edx = result.edx;
    }
}

#[derive(Debug, Clone)]
pub struct RawCPUIDResponse {
    pub input: LeafID,
    pub output: Registers,
}

impl RawCPUIDResponse {
    /// Creates an empty `RawCPUIDResponse` structure.
    pub fn new() -> RawCPUIDResponse {
        RawCPUIDResponse {
            input: LeafID::new(0, 0),
            output: Registers::new(0, 0, 0, 0),
        }
    }

    /// Executes the CPUID instruction with the input register values specified
    /// by `eax` and `ecx` and creates a new `RawCPUIDResponse` containing the
    /// response.
    pub fn invoke(eax: u32, ecx: u32) -> RawCPUIDResponse {
        let input = LeafID::new(eax, ecx);
        let mut output = Registers::new(0, 0, 0, 0);
        cpuid(&input, &mut output);
        RawCPUIDResponse {
            input: input,
            output: output,
        }
    }

    /// Executes the CPUID instruction with the input values specified in
    /// [input](#structfield.input) and fills [output](#structfield.output) with
    /// the response register values.
    pub fn call(&mut self) {
        cpuid(&self.input, &mut self.output);
    }

    /// Increments `ecx` in [input](#structfield.input) and executes the CPUID
    /// instruction, replacing the values in [output](#structfield.output) with
    /// the response register values.
    pub fn next_subleaf(&mut self) {
        self.input.ecx += 1;
        cpuid(&self.input, &mut self.output);
    }
}

#[derive(Debug, Clone)]
pub struct Processor {
    /// Logical index of this CPU on the system.
    pub index: u32,

    /// Vector of all the raw [responses](struct.RawCPUIDResponse.html) for known
    /// CPUID leaves.
    pub leaves: Vec<RawCPUIDResponse>,
}

impl Processor {
    /// Creates an empty `Processor` object.
    pub fn new() -> Processor {
        Processor {
            index: 0,
            leaves: vec![],
        }
    }

    /// Walk all known CPUID leaves on the current processor. Note that you should
    /// set your process or thread affinity to prevent the OS from moving the
    /// process/thread around causing you to query other CPUs inadvertently.
    pub fn from_local() -> Processor {
        let mut processor: Processor = Processor::new();
        walk_bases(&mut processor.leaves);
        processor.fill();
        processor
    }

    /// Gets a single [RawCPUIDResponse](struct.RawCPUIDResponse.html) object
    /// matching the specified input `eax` and `ecx` values. Returns None if no
    /// match was found for this processor.
    pub fn get_subleaf(&self, eax: u32, ecx: u32) -> Option<&RawCPUIDResponse> {
        for result in self.leaves.iter() {
            if result.input.eax == eax && result.input.ecx == ecx {
                return Some(&result);
            }
        }
        None
    }

    /// Gets all [RawCPUIDResponse](struct.RawCPUIDResponse.html) objects with matching input `eax` values.
    pub fn get(&self, eax: u32) -> Vec<&RawCPUIDResponse> {
        let mut out: Vec<&RawCPUIDResponse> = vec![];
        for result in self.leaves.iter() {
            if result.input.eax == eax {
                out.push(&result);
            }
        }
        out
    }

    /// Finds the matching hardware vendor as a
    /// [VendorMask](struct.VendorMask.html) for the current processor, based on
    /// the contents of leaf `0x0000_0000`.
    pub fn decode_vendor(&self, base: u32) -> VendorMask {
        if let Some(leaf) = self.get_subleaf(base, 0x0) {
            let mut bytes: Vec<u8> = vec![];
            for register in [leaf.output.ebx, leaf.output.edx, leaf.output.ecx].iter() {
                for byte in register.to_le_bytes().iter() {
                    bytes.push(*byte);
                }
            }
            VendorMask::from_string(bytes_to_ascii(bytes))
        } else {
            VendorMask::UNKNOWN
        }
    }

    /// Tests if the specified `bit` is set in the specified `register` from a
    /// particular leaf/subleaf.
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
                    _ => panic!("Invalid register"),
                };
                bits & (1 << bit) != 0
            }
        }
    }

    fn fill(&mut self) {}
}

#[derive(Debug)]
pub struct System {
    /// Vector of processors in the system.
    pub cpus: Vec<Processor>,

    /// Matching vendor IDs discovered in the various CPUID leaves. May contain
    /// more than one vendor, e.g. if a hypervisor is present.
    pub vendor: VendorMask,

    /// Normalized processor name string.
    pub name_string: String,

    /// Vector of all the discovered caches and TLBs in the first processor.
    pub caches: CacheVec,

    /// Vector of all the discovered features in the first processor.
    pub features: FeatureVec,
}

impl System {
    fn new() -> System {
        System {
            cpus: vec![],
            vendor: VendorMask::UNKNOWN,
            name_string: String::new(),
            caches: CacheVec::new(),
            features: FeatureVec::new(),
        }
    }

    /// Walk all known CPUID leaves for each CPU on the local system and store
    /// the results in a new [System](struct.System.html) object.
    pub fn from_local() -> System {
        let mut system: System = System::new();
        let cpu_start: u32 = 0;
        let cpu_end: u32 = num_cpus::get() as u32 - 1;

        let old_affinity = affinity::get_thread_affinity().unwrap();

        for cpu in cpu_start..(cpu_end + 1) {
            let mask = vec![cpu as usize];

            // TODO: This can fail, and we should be noisy about it when it does.
            // Though if we're on macOS we can't do anything about it since there
            // isn't any thread affinity API there.
            affinity::set_thread_affinity(mask).unwrap();

            let mut processor = Processor::from_local();
            processor.index = cpu;
            system.cpus.push(processor);
        }

        affinity::set_thread_affinity(old_affinity).unwrap();

        system.fill();

        system
    }

    /// Import a CPUID dump file instead of querying processors on the local
    /// machine.
    pub fn from_file(filename: &str) -> std::io::Result<System> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut system: System = System::new();
        let mut processor: Processor = Processor::new();
        let mut cpu_index: i32 = -1;

        for line in reader.lines() {
            let line = line?;
            if let Ok((in_eax, in_ecx, out_eax, out_ebx, out_ecx, out_edx)) = scan_fmt!(&line, "CPUID {x}:{x} = {x} {x} {x} {x}", [hex u32], [hex u32], [hex u32], [hex u32], [hex u32], [hex u32])
            {
                processor.leaves.push(RawCPUIDResponse {
                    input: LeafID {
                        eax: in_eax,
                        ecx: in_ecx,
                    },
                    output: Registers {
                        eax: out_eax,
                        ebx: out_ebx,
                        ecx: out_ecx,
                        edx: out_edx,
                    },
                })
            } else if let Ok(sc_index) = scan_fmt!(&line, "CPU {}:", i32) {
                if cpu_index >= 0 {
                    processor.fill();
                    processor.index = cpu_index as u32;
                    system.cpus.push(processor);
                    processor = Processor::new();
                }
                cpu_index = sc_index;
            }
        }

        if cpu_index >= 0 {
            processor.fill();
            processor.index = cpu_index as u32;
            system.cpus.push(processor);
        }

        system.fill();

        Ok(system)
    }

    fn fill(&mut self) {
        // Order is important. Feature/cache decoding depends a lot on the vendor string.
        self.fill_vendor();
        self.fill_processor_name();
        self.fill_caches();
        self.fill_features();
    }

    fn fill_caches(&mut self) {
        self.caches = describe_caches(self, &self.cpus[0])
    }

    fn fill_features(&mut self) {
        self.features = describe_features(&self.cpus[0], self.vendor);
    }

    fn fill_vendor(&mut self) {
        if let Some(leaf) = self.cpus[0].get_subleaf(0x0000_0000, 0x0) {
            let mut bytes: Vec<u8> = vec![];
            for register in [leaf.output.ebx, leaf.output.edx, leaf.output.ecx].iter() {
                for byte in register.to_le_bytes().iter() {
                    bytes.push(*byte);
                }
            }
            let vendor_id = VendorMask::from_string(bytes_to_ascii(bytes));
            debug!("decoded processor vendor: {:#?}", vendor_id);
            self.vendor |= vendor_id;
        }
        if let Some(leaf) = self.cpus[0].get_subleaf(0x4000_0000, 0x0) {
            let mut bytes: Vec<u8> = vec![];
            for register in [leaf.output.ebx, leaf.output.ecx, leaf.output.edx].iter() {
                for byte in register.to_le_bytes().iter() {
                    bytes.push(*byte);
                }
            }
            let vendor_id = VendorMask::from_string(bytes_to_ascii(bytes));
            debug!("decoded hypervisor vendor: {:#?}", vendor_id);
            self.vendor |= vendor_id;
        }
        debug!("final vendor mask: {:#?}", self.vendor);
    }

    fn fill_processor_name(&mut self) {
        let mut bytes: Vec<u8> = vec![];
        for leaf_id in [0x8000_0002, 0x8000_0003, 0x8000_0004].iter() {
            if let Some(leaf) = self.cpus[0].get_subleaf(*leaf_id, 0x0) {
                for register in [
                    leaf.output.eax,
                    leaf.output.ebx,
                    leaf.output.ecx,
                    leaf.output.edx,
                ]
                .iter()
                {
                    for byte in register.to_le_bytes().iter() {
                        bytes.push(*byte);
                    }
                }
            }
        }
        if bytes.len() == 3 * 4 * 4 {
            self.name_string = squeeze_str(bytes_to_ascii(bytes));
            debug!("decoded name string: {:#?}", self.name_string);
        }
    }
}

impl fmt::Display for RawCPUIDResponse {
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

fn call_leaf(out: &mut Vec<RawCPUIDResponse>, leaf: u32, subleaf: u32) {
    let state = RawCPUIDResponse::invoke(leaf, subleaf);
    out.push(state);
}

fn call_leaf_04(out: &mut Vec<RawCPUIDResponse>) {
    let mut state = RawCPUIDResponse::invoke(0x0000_0004, 0);
    loop {
        out.push(state.clone());
        if state.output.eax & 0xF == 0 {
            break;
        }
        state.next_subleaf();
    }
}

fn call_leaf_x2apic(out: &mut Vec<RawCPUIDResponse>, leaf: u32) {
    let mut state = RawCPUIDResponse::invoke(leaf, 0);
    loop {
        if state.input.ecx > 0 && !(state.output.eax != 0 || state.output.ebx != 0) {
            break;
        }
        out.push(state.clone());
        state.next_subleaf();
    }
}

fn call_leaf_0d(out: &mut Vec<RawCPUIDResponse>) {
    let mut state = RawCPUIDResponse::invoke(0x0000_000D, 0);
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

fn call_leaf_0f(out: &mut Vec<RawCPUIDResponse>) {
    let mut state = RawCPUIDResponse::invoke(0x0000_000F, 0);
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

fn call_leaf_10(out: &mut Vec<RawCPUIDResponse>) {
    let mut state = RawCPUIDResponse::invoke(0x0000_0010, 0);
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

fn call_leaf_12(out: &mut Vec<RawCPUIDResponse>) {
    let feature_check = RawCPUIDResponse::invoke(0x0000_0007, 0);
    let sgx_supported = (feature_check.output.ebx & 0x4) != 0;
    let mut state = RawCPUIDResponse::invoke(0x0000_0012, 0);
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

fn call_leaf_1b(out: &mut Vec<RawCPUIDResponse>) {
    let feature_check = RawCPUIDResponse::invoke(0x0000_0007, 0);
    let pconfig_supported = (feature_check.output.edx & 0x0004_0000) != 0;
    let mut state = RawCPUIDResponse::invoke(0x0000_001B, 0);
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

fn call_leaf_max_ecx(out: &mut Vec<RawCPUIDResponse>, leaf: u32, max_subleaf: u32) {
    let mut state = RawCPUIDResponse::invoke(leaf, 0);
    loop {
        out.push(state.clone());
        state.next_subleaf();
        if state.input.ecx > max_subleaf {
            break;
        }
    }
}

fn call_leaf_ext_1d(out: &mut Vec<RawCPUIDResponse>) {
    let feature_check = RawCPUIDResponse::invoke(0x8000_0001, 0);
    let ext_topology_supported = (feature_check.output.ecx & 0x0040_0000) != 0;
    let mut state = RawCPUIDResponse::invoke(0x8000_001D, 0);
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

fn call_leaf_indexed(out: &mut Vec<RawCPUIDResponse>, leaf: u32) {
    let mut state = RawCPUIDResponse::invoke(leaf, 0);
    let max_ecx = state.output.eax;
    loop {
        out.push(state.clone());
        state.next_subleaf();
        if state.input.ecx > max_ecx {
            break;
        }
    }
}

fn walk_leaves(out: &mut Vec<RawCPUIDResponse>, base: u32) {
    let state = RawCPUIDResponse::invoke(base, 0);

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

fn walk_bases(out: &mut Vec<RawCPUIDResponse>) {
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
        // Mystery leaves, found as easter eggs on some CPUs
        0x8FFF_FFFE,
        0x8FFF_FFFF,
    ];

    for base in bases.iter() {
        walk_leaves(out, *base);
    }
}
