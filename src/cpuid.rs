#![allow(dead_code)]

use bitflags::*;
use log::*;
use modular_bitfield::prelude::*;
use scan_fmt::*;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::cache::{describe_caches, CacheVec};
use crate::feature::{describe_features, FeatureVec};

#[derive(Debug, Clone, PartialEq)]
/// Input `eax` and `ecx` values for a single CPUID invocation.
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
/// Output registers for a single CPUID invocation.
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
    /// Bitmask for Vendor IDs, used to identify both physical CPU vendors and
    /// hypervisor vendors.
    pub struct VendorMask: u32 {
        /// This mask contains no vendor flags.
        const UNKNOWN = 0x0000_0000;

        //
        // Common helper masks
        //

        /// Mask covering any physical CPU vendor IDs
        const ANY_CPU = 0x0000_00FF;

        /// Mask covering any hypervisor vendor IDs
        const ANY_HYPERVISOR = 0x0000_FF00;

        //
        // One-hot identifiers for CPU vendors
        //

        /// Vendor flag for `GenuineIntel` CPUs
        const INTEL = 0x0000_0001;

        /// Vendor flag for `AuthenticAMD` CPUs
        const AMD = 0x0000_0002;

        /// Vendor flag for `CentaurHauls` CPUs
        const CENTAUR = 0x0000_0004;

        /// Vendor flag for `CyrixInstead` CPUs
        const CYRIX = 0x0000_0008;

        /// Vendor flag for `GenuineTMx86` CPUs
        const TRANSMETA = 0x0000_0010;

        /// Vendor flag for `HygonGenuine` CPUs
        const HYGON = 0x0000_0020;

        //
        // Common vendor masks
        //

        /// Mask covering both Intel and AMD CPUs.
        const INTELAMD = 0x0000_0003;

        //
        // One-hot identifiers for hypervisor vendors
        //

        /// Vendor flag for Microsoft Hyper-V hypervisor
        const HYPERV = 0x0000_0100;

        /// Vendor flag for Linux KVM hypervisor
        const KVM = 0x0000_0200;

        /// Vendor flag for QEMU TCG hypervisor
        const TCG = 0x0000_0400;

        /// Vendor flag for Xen hypervisor
        const XEN = 0x0000_0800;

        /// Vendor flag for Parallels Desktop hypervisor
        const PARALLELS = 0x0000_1000;

        /// Vendor flag for VMware hypervisors
        const VMWARE = 0x0000_2000;

        /// Vendor flag for FreeBSD's byve hypervisor
        const BHYVE = 0x0000_4000;
    }
}

impl VendorMask {
    fn from_string(input: String) -> VendorMask {
        debug!("attempting to match vendor string {:?}", input);
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
    let mut string = String::with_capacity(bytes.len());
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
    let mut string = String::with_capacity(bytes.len());
    for byte in bytes.iter() {
        let chr = *byte as char;
        if chr == '\0' {
            break;
        }
        if chr.is_ascii() && !chr.is_control() {
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
/// Structure containing a CPUID leaf ID and the output register values for a
/// single CPUID invocation.
pub struct RawCPUIDResponse {
    /// Input leaf ID
    pub input: LeafID,

    /// Output registers.
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

#[bitfield(bits = 32)]
#[derive(Debug)]
struct SignatureRaw {
    stepping: B4,
    model: B4,
    family: B4,
    #[skip]
    __: B4,
    extmodel: B4,
    extfamily: B8,
    #[skip]
    __: B4,
}

#[derive(Debug, Clone)]
/// Describes the processor signature (family, model, stepping).
pub struct Signature {
    /// Family ID, including extended family.
    pub family: u16,

    /// Model ID, including extended model.
    pub model: u16,

    /// Stepping ID.
    pub stepping: u8,
}
impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Family {:X}h, Model {:X}h, Stepping {:X}h",
            self.family, self.model, self.stepping
        )
    }
}

impl Signature {
    pub fn new() -> Signature {
        Signature {
            family: 0,
            model: 0,
            stepping: 0,
        }
    }
}

#[derive(Debug, Clone)]
/// Structure containing CPUID data for a single logical CPU.
///
/// Contains the logical CPU index, raw CPUID request/response data, vendor mask,
/// signature, etc.
///
/// More of the decoded data is available in the [System](struct.System.html)
/// structure.
pub struct Processor {
    /// Logical index of this CPU on the system.
    pub index: u32,

    /// Vector of all the raw [responses](struct.RawCPUIDResponse.html) for known
    /// CPUID leaves.
    pub leaves: Vec<RawCPUIDResponse>,

    /// Matching vendor IDs discovered in the various CPUID leaves. May contain
    /// more than one vendor, e.g. if a hypervisor is present.
    pub vendor: VendorMask,

    /// Describes the processor signature.
    pub signature: Signature,
}

impl Processor {
    /// Creates an empty `Processor` object.
    pub fn new() -> Processor {
        Processor {
            index: 0,
            leaves: vec![],
            vendor: VendorMask::UNKNOWN,
            signature: Signature::new(),
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
    pub fn has_feature_bit(&self, leaf: u32, subleaf: u32, register: RegisterName, bit: u32) -> bool {
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

    fn fill(&mut self) {
        self.fill_vendor();
        self.fill_signature();
    }

    fn fill_signature(&mut self) {
        if let Some(leaf) = self.get_subleaf(0x0000_0001, 0) {
            let rawsignature: SignatureRaw = SignatureRaw::from_bytes(leaf.output.eax.to_le_bytes());
            self.signature.family = rawsignature.family() as u16 + rawsignature.extfamily() as u16;
            self.signature.model = rawsignature.model() as u16;
            self.signature.stepping = rawsignature.stepping();
            if rawsignature.family() == 0xf
                || (self.vendor.contains(VendorMask::INTEL) && rawsignature.family() == 0x6)
            {
                self.signature.model += (rawsignature.extmodel() as u16) << 4;
            }
        }
    }

    fn fill_vendor(&mut self) {
        if let Some(leaf) = self.get_subleaf(0x0000_0000, 0x0) {
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
        if let Some(leaf) = self.get_subleaf(0x4000_0000, 0x0) {
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
}

#[derive(Debug)]
/// Structure containing a snapshot of one or more logical CPUs.
///
/// It usually contains CPUID data for all logical CPUs, except on macOS, where
/// lack of any thread/process affinity APIs makes it impossible to query
/// anything other than one CPU.
///
/// Aside from the raw CPUID data, this structure also contains the decoded
/// vendor IDs, name string, cache descriptions, feature descriptions, etc.
pub struct System {
    /// Vector of processors in the system. May only contain one instance if the
    /// platform does not support thread affinity APIs (*COUGH, COUGH* macOS
    /// *COUGH, COUGH*)
    pub cpus: Vec<Processor>,

    /// Number of CPUs in the system. May not match the length of the `cpus`
    /// vector on platforms without thread affinity APIs.
    pub cpu_count: usize,

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
            cpu_count: 0,
            vendor: VendorMask::UNKNOWN,
            name_string: String::new(),
            caches: CacheVec::new(),
            features: FeatureVec::new(),
        }
    }

    /// Walk all known CPUID leaves for each CPU on the local system and store
    /// the results in a new [System](struct.System.html) object.
    pub fn from_local() -> System {
        System::from_local_impl()
    }

    #[cfg(not(target_os = "macos"))]
    fn from_local_impl() -> System {
        let mut system: System = System::new();
        let cpu_start: u32 = 0;
        let cpu_end: u32 = num_cpus::get() as u32 - 1;

        let old_affinity = affinity::get_thread_affinity().unwrap();

        for cpu in cpu_start..(cpu_end + 1) {
            debug!("collecting leaves for CPU {:?}", cpu);
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

        system.cpu_count = num_cpus::get();
        system.fill();

        system
    }

    #[cfg(target_os = "macos")]
    fn from_local_impl() -> System {
        let mut system: System = System::new();
        let mut processor = Processor::from_local();
        processor.index = 0;
        debug!("collecting leaves for one CPU");
        system.cpus.push(processor);
        system.cpu_count = num_cpus::get();
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

        system.cpu_count = system.cpus.len();
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
        self.vendor = self.cpus[0].vendor;
    }

    fn fill_processor_name(&mut self) {
        let mut bytes: Vec<u8> = vec![];
        for leaf_id in [0x8000_0002, 0x8000_0003, 0x8000_0004].iter() {
            if let Some(leaf) = self.cpus[0].get_subleaf(*leaf_id, 0x0) {
                for register in [leaf.output.eax, leaf.output.ebx, leaf.output.ecx, leaf.output.edx].iter() {
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

fn call_leaf_04(out: &mut Vec<RawCPUIDResponse>, state: &mut RawCPUIDResponse) {
    loop {
        out.push(state.clone());
        if state.output.eax & 0xF == 0 {
            break;
        }
        state.next_subleaf();
    }
}

fn call_leaf_x2apic(out: &mut Vec<RawCPUIDResponse>, state: &mut RawCPUIDResponse) {
    loop {
        if state.input.ecx > 0 && !(state.output.eax != 0 || state.output.ebx != 0) {
            break;
        }
        out.push(state.clone());
        state.next_subleaf();
    }
}

fn call_leaf_0d(out: &mut Vec<RawCPUIDResponse>, state: &mut RawCPUIDResponse) {
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

fn call_leaf_0f(out: &mut Vec<RawCPUIDResponse>, state: &mut RawCPUIDResponse) {
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

fn call_leaf_10(out: &mut Vec<RawCPUIDResponse>, state: &mut RawCPUIDResponse) {
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

fn call_leaf_12(out: &mut Vec<RawCPUIDResponse>, state: &mut RawCPUIDResponse) {
    let feature_check = RawCPUIDResponse::invoke(0x0000_0007, 0);
    let sgx_supported = (feature_check.output.ebx & 0x4) != 0;
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

fn call_leaf_1b(out: &mut Vec<RawCPUIDResponse>, state: &mut RawCPUIDResponse) {
    let feature_check = RawCPUIDResponse::invoke(0x0000_0007, 0);
    let pconfig_supported = (feature_check.output.edx & 0x0004_0000) != 0;
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

fn call_leaf_max_ecx(out: &mut Vec<RawCPUIDResponse>, state: &mut RawCPUIDResponse, max_subleaf: u32) {
    loop {
        out.push(state.clone());
        state.next_subleaf();
        if state.input.ecx > max_subleaf {
            break;
        }
    }
}

fn call_leaf_ext_1d(out: &mut Vec<RawCPUIDResponse>, state: &mut RawCPUIDResponse) {
    let feature_check = RawCPUIDResponse::invoke(0x8000_0001, 0);
    let ext_topology_supported = (feature_check.output.ecx & 0x0040_0000) != 0;
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

fn call_leaf_indexed(out: &mut Vec<RawCPUIDResponse>, state: &mut RawCPUIDResponse) {
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
    let mut state = RawCPUIDResponse::invoke(base, 0);

    // All valid bases use eax to indicate the maximum supported leaf within that range.
    if state.output.eax < base || state.output.eax > base + 0xFFFF {
        // Even if this base isn't valid, print it so that our dump is comprehensive.
        out.push(state);
        return;
    }

    let begin: usize = state.input.eax as usize;
    let end: usize = state.output.eax as usize + 1;

    out.reserve(end - begin);

    for leaf in begin..end {
        state.input.eax = leaf as u32;
        state.input.ecx = 0;
        state.call();

        // Some leaves are indexed (i.e. passing different values for ecx will generate different
        // results). Unfortunately how they're indexed varies significantly. We need to call
        // a handler for each of the special leaves so they can be dumped fully.
        match leaf {
            0x0000_0004 => call_leaf_04(out, &mut state),
            0x0000_0007 => call_leaf_indexed(out, &mut state),
            0x0000_000B => call_leaf_x2apic(out, &mut state),
            0x0000_000D => call_leaf_0d(out, &mut state),
            0x0000_000F => call_leaf_0f(out, &mut state),
            0x0000_0010 => call_leaf_10(out, &mut state),
            0x0000_0012 => call_leaf_12(out, &mut state),
            0x0000_0014 => call_leaf_indexed(out, &mut state),
            0x0000_0017 => call_leaf_indexed(out, &mut state),
            0x0000_0018 => call_leaf_indexed(out, &mut state),
            0x0000_001B => call_leaf_1b(out, &mut state),
            0x0000_001D => call_leaf_indexed(out, &mut state),
            0x0000_001F => call_leaf_x2apic(out, &mut state),
            0x0000_0020 => call_leaf_indexed(out, &mut state),
            0x8000_001D => call_leaf_ext_1d(out, &mut state),
            0x8000_0020 => call_leaf_max_ecx(out, &mut state, 1),
            _ => out.push(state.clone()),
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
