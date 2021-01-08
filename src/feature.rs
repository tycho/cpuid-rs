use log::*;
use std::fmt;
use textwrap::indent;

use crate::cpuid::{LeafID, Processor, RegisterName, VendorMask};

#[derive(Debug, Clone)]
pub struct Feature {
    leaf: LeafID,
    register: RegisterName,
    bit: u8,
    vendor_mask: VendorMask,
    shortname: &'static str,
    name: &'static str,
}

impl Feature {
    fn from_detection(leaf: &FeatureLeaf, spec: &FeatureSpec, bit: u8) -> Feature {
        Feature {
            leaf: leaf.leaf.clone(),
            register: leaf.register,
            bit: bit,
            vendor_mask: spec.vendor_mask,
            shortname: spec.shortname,
            name: spec.name,
        }
    }
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.shortname.len() != 0 && self.shortname != self.name {
            write!(f, "{} ({})", self.name, self.shortname)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

#[derive(Debug)]
pub struct FeatureVec(pub Vec<Feature>);

impl FeatureVec {
    pub fn new() -> FeatureVec {
        FeatureVec(vec![])
    }
}

fn leaf_name(leaf: &LeafID, register: &RegisterName) -> &'static str {
    match leaf.eax {
        0x0000_0001 | 0x8000_0001 => "Feature Identifiers",
        0x0000_0006 => "Thermal and Power Management",
        0x0000_0007 => "Structured Extended Feature Identifiers",
        0x0000_0014 => "Intel Processor Trace Enumeration",
        0x8000_0007 => {
            match register {
                RegisterName::EBX => "RAS Capabilities",
                RegisterName::EDX => "Advanced Power Management Information",
                _ => "",
            }
        }
        0x8000_0008 => "Extended Feature Extensions ID",
        0x8000_000A => "SVM Feature Identifiers",
        0x8000_001A => "Performance Optimization Identifiers",
        0x8000_001B => "Instruction Based Sampling Identifiers",
        0xC000_0001 => "Centaur Feature Identifiers",
        _ => "",
    }
}

impl fmt::Display for FeatureVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Features:\n")?;
        let mut lastleaf: LeafID = LeafID { eax: 0xFFFF_FFFF, ecx: 0xFFFF_FFFF };
        let mut lastreg: RegisterName = RegisterName::Unknown;
        for v in &self.0 {
            if v.leaf != lastleaf || v.register != lastreg {
                if lastreg != RegisterName::Unknown {
                    write!(f, "\n")?;
                }
                let mut name = leaf_name(&v.leaf, &v.register).to_string();
                if name.len() > 0 {
                    name = format!(" ({})", name.to_string());
                }
                write!(f, "  Leaf {:08x}:{:02x}{}, register {:?}\n", v.leaf.eax, v.leaf.ecx, name, v.register)?;
                lastleaf = v.leaf.clone();
                lastreg = v.register.clone();
            }
            let formatted = format!("{}\n", v);
            write!(f, "{}", indent(&formatted, "    "))?;
        }
        Ok(())
    }
}

struct FeatureLeaf {
    leaf: LeafID,
    vendor_mask: VendorMask,
    register: RegisterName,
    bits: &'static [FeatureSpec; 40],
}

struct FeatureSpec {
    bit: u8,
    vendor_mask: VendorMask,
    shortname: &'static str,
    name: &'static str,
}

static FEATURES_0000_0001_EDX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::ANY_CPU,  shortname: "FPU", name: "x87 FPU on chip", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::ANY_CPU,  shortname: "VME", name: "Virtual-8086 Mode Enhancement", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::ANY_CPU,  shortname: "DE", name: "Debugging Extensions", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::ANY_CPU,  shortname: "PSE", name: "Page Size Extensions", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::ANY_CPU,  shortname: "TSC", name: "Time Stamp Counter", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::ANY_CPU,  shortname: "MSR", name: "RDMSR and WRMSR support", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::ANY_CPU,  shortname: "PAE", name: "Physical Address Extensions", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::ANY_CPU,  shortname: "MCE", name: "Machine Check Exception", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::ANY_CPU,  shortname: "CX8", name: "CMPXCHG8B instruction", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::ANY_CPU,  shortname: "APIC", name: "APIC on chip", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::ANY_CPU,  shortname: "SEP", name: "SYSENTER and SYSEXIT instructions", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::ANY_CPU,  shortname: "MTRR", name: "Memory Type Range Registers", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::ANY_CPU,  shortname: "PGE", name: "PTE Global Bit", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::ANY_CPU,  shortname: "MCA", name: "Machine Check Architecture", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::ANY_CPU,  shortname: "CMOV", name: "Conditional Move/Compare Instruction", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::ANY_CPU,  shortname: "PAT", name: "Page Attribute Table", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::ANY_CPU,  shortname: "PSE-36", name: "Page Size Extension", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::ANY_CPU,  shortname: "PSN", name: "Processor Serial Number", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::ANY_CPU,  shortname: "CLFSH", name: "CLFLUSH instruction", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::ANY_CPU,  shortname: "DS", name: "Debug Store", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::ANY_CPU,  shortname: "ACPI", name: "Thermal Monitor and Clock Control", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::ANY_CPU,  shortname: "MMX", name: "MMX instruction set", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::ANY_CPU,  shortname: "FXSR", name: "FXSAVE/FXRSTOR instructions", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::ANY_CPU,  shortname: "SSE", name: "SSE instructions", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::ANY_CPU,  shortname: "SSE2", name: "SSE2 instructions", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::ANY_CPU,  shortname: "SS", name: "Self Snoop", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::ANY_CPU,  shortname: "HTT", name: "Hyperthreading", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::ANY_CPU,  shortname: "TM", name: "Thermal Monitor", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::ANY_CPU,  shortname: "PBE", name: "Pending Break Enable", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_0000_0001_ECX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::ANY_CPU,  shortname: "SSE3", name: "SSE3 instructions", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::ANY_CPU,  shortname: "PCLMULQDQ", name: "PCLMULQDQ instruction", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::ANY_CPU,  shortname: "DTES64", name: "64-bit DS area", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::ANY_CPU,  shortname: "MONITOR", name: "MONITOR/MWAIT instructions", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::ANY_CPU,  shortname: "DS-CPL", name: "CPL qualified debug store", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::ANY_CPU,  shortname: "VMX", name: "Virtual Machine Extensions", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::ANY_CPU,  shortname: "SMX", name: "Safer Mode Extensions", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::ANY_CPU,  shortname: "EIST", name: "Enhanced Intel SpeedStep Technology", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::ANY_CPU,  shortname: "TM2", name: "Thermal Monitor 2", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::ANY_CPU,  shortname: "SSSE3", name: "SSSE3 instructions", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::ANY_CPU,  shortname: "CNXT-ID", name: "L1 context ID", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::ANY_CPU,  shortname: "SDBG", name: "Silicon debug via IA32_DEBUG_INTERFACE MSR", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::ANY_CPU,  shortname: "FMA", name: "Fused Multiply-Add AVX instructions", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::ANY_CPU,  shortname: "CMPXCHG16B", name: "CMPXCHG16B instruction available", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::ANY_CPU,  shortname: "xTPR", name: "xTPR Update Control", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::ANY_CPU,  shortname: "PDCM", name: "Perfmon and Debug Capability", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::ANY_CPU,  shortname: "PCID", name: "Process-context identifiers", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::ANY_CPU,  shortname: "DCA", name: "Prefetch from memory-mapped device, direct cache access", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::ANY_CPU,  shortname: "SSE4.1", name: "SSE4.1 instructions", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::ANY_CPU,  shortname: "SSE4.2", name: "SSE4.2 instructions", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::ANY_CPU,  shortname: "x2APIC", name: "x2APIC", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::ANY_CPU,  shortname: "MOVBE", name: "MOVBE instruction", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::ANY_CPU,  shortname: "POPCNT", name: "POPCNT instruction", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::ANY_CPU,  shortname: "TSC-Deadline", name: "APIC supports one-shot using TSC deadline", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::ANY_CPU,  shortname: "AES-NI", name: "AES-NI instruction set", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::ANY_CPU,  shortname: "XSAVE", name: "XSAVE/XRSTOR extended state instructions", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::ANY_CPU,  shortname: "OSXSAVE", name: "OS enabled XSAVE support", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::ANY_CPU,  shortname: "AVX", name: "AVX instructions", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::ANY_CPU,  shortname: "F16C", name: "16-bit floating-point conversion instructions", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::ANY_CPU,  shortname: "RDRAND", name: "RDRAND instruction", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::ANY_CPU,  shortname: "RAZ", name: "Hypervisor", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

// Thermal and Power Management Feature Flags (0000_0006)
static FEATURES_0000_0006_EAX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "Digital temperature sensor", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "Intel Turbo Boost Technology", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::ANY_CPU,  shortname: "ARAT", name: "Always running APIC timer", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "Power limit notification controls", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "Clock modulation duty cycle extensions", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "Package thermal management", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::ANY_CPU,  shortname: "HWP", name: "Hardware-managed P-state base support", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "HWP notification interrupt enable MSR", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "HWP activity window MSR", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "HWP energy/performance preference MSR", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "HWP package level request MSR", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::ANY_CPU,  shortname: "HDC", name: "Hardware duty cycle programming", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "Intel Turbo Boost Max Technology 3.0", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "HWP Capabilities, Highest Performance change", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "HWP PECI override", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "Flexible HWP", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "Fast access mode for IA32_HWP_REQUEST MSR", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "Hardware feedback MSRs", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "Ignoring idle logical processor HWP request", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "Enhanced hardware feedback MSRs", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "IP payloads are LIP", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_0000_0006_ECX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "Hardware-coordination feedback capability, IA32_APERF and IA32_MPERF MSRs", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::ANY_CPU,  shortname: "", name: "Performance-energy bias preference", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_0000_0007_0_EBX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::INTELAMD, shortname: "FSGSBASE", name: "FSGSBASE instructions", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::INTEL,    shortname: "TSC_ADJUST", name: "IA32_TSC_ADJUST MSR is supported", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::INTEL,    shortname: "SGX", name: "Software Guard Extensions", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::INTELAMD, shortname: "BMI1", name: "Bit Manipulation Instructions", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::INTEL,    shortname: "HLE", name: "Hardware Lock Elision", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::INTELAMD, shortname: "AVX2", name: "Advanced Vector Extensions 2.0", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::INTEL,    shortname: "FDP_EXCPTN_ONLY", name: "x87 FPU data pointer updated only on x87 exception", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::INTELAMD, shortname: "SMEP", name: "Supervisor Mode Execution Protection", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::INTELAMD, shortname: "BMI2", name: "Bit Manipulation Instructions 2", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::INTELAMD, shortname: "", name: "Enhanced REP MOVSB/STOSB", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::INTELAMD, shortname: "INVPCID", name: "INVPCID instruction", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::INTEL,    shortname: "RTM", name: "Restricted Transactional Memory", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::INTELAMD, shortname: "PQM", name: "Platform QoS Monitoring", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::INTEL,    shortname: "", name: "x87 FPU CS and DS deprecated", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::INTEL,    shortname: "MPX", name: "Memory Protection Extensions", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::INTELAMD, shortname: "PQE", name: "Platform QoS Enforcement", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::INTEL,    shortname: "AVX512F", name: "AVX512 foundation", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::INTEL,    shortname: "AVX512DQ", name: "AVX512 double/quadword instructions", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::INTELAMD, shortname: "RDSEED", name: "RDSEED instruction", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::INTELAMD, shortname: "ADX", name: "Multi-Precision Add-Carry Instructions", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::INTELAMD, shortname: "SMAP", name: "Supervisor Mode Access Prevention", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::INTEL,    shortname: "AVX512IFMA", name: "AVX512 integer FMA instructions", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::INTEL,    shortname: "PCOMMIT", name: "Persistent commit instruction", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::AMD,      shortname: "", name: "RDPID instruction and TSC_AUX MSR support", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::INTELAMD, shortname: "CLFLUSHOPT", name: "CLFLUSHOPT instruction", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::INTELAMD, shortname: "CLWB", name: "Cache line write-back instruction", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::INTEL,    shortname: "", name: "Intel Processor Trace", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::INTEL,    shortname: "AVX512PF", name: "AVX512 prefetch instructions", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::INTEL,    shortname: "AVX512ER", name: "AVX512 exponent/reciprocal instructions", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::INTEL,    shortname: "AVX512CD", name: "AVX512 conflicte detection instructions", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::INTELAMD, shortname: "SHA", name: "SHA-1/SHA-256 instructions", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::INTEL,    shortname: "AVX512BW", name: "AVX512 byte/word instructions", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::INTEL,    shortname: "AVX512VL", name: "AVX512 vector length instructions", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_0000_0007_0_ECX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::INTEL,    shortname: "PREFETCHWT1", name: "PREFETCHWT1 instruction", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::INTEL,    shortname: "AVX512_VBMI", name: "AVX512 vector byte manipulation instructions", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::INTELAMD, shortname: "UMIP", name: "User Mode Instruction Prevention", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::INTELAMD, shortname: "PKU", name: "Protection Keys for User-mode pages", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::INTELAMD, shortname: "OSPKE", name: "OS-enabled protection keys", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::INTEL,    shortname: "WAITPKG", name: "Wait and Pause Enhancements", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::INTEL,    shortname: "AVX512_VBMI2", name: "AVX512 vector byte manipulation instructions 2", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::INTELAMD, shortname: "CET_SS", name: "CET shadow stack", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::INTEL,    shortname: "GFNI", name: "Galois Field NI / Galois Field Affine Transformation", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::INTELAMD, shortname: "VAES", name: "VEX-encoded AES-NI", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::INTELAMD, shortname: "VPCL", name: "VEX-encoded PCLMUL", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::INTEL,    shortname: "AVX512_VNNI", name: "AVX512 Vector Neural Network instructions", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::INTEL,    shortname: "AVX512_BITALG", name: "AVX512 Bitwise Algorithms", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::INTEL,    shortname: "", name: "", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::INTEL,    shortname: "AVX512_VPOPCNTDQ", name: "AVX512 VPOPCNTDQ instruction", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::INTEL,    shortname: "", name: "", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::INTEL,    shortname: "VA57", name: "5-level paging", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::INTEL,    shortname: "", name: "", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::INTEL,    shortname: "", name: "", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::INTEL,    shortname: "", name: "", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::INTEL,    shortname: "", name: "", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::INTEL,    shortname: "", name: "", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::INTELAMD, shortname: "RDPID", name: "Read Processor ID", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::INTEL,    shortname: "KL", name: "Key Locker", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::INTEL,    shortname: "", name: "", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::INTEL,    shortname: "CLDEMOTE", name: "Cache Line Demote", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::INTEL,    shortname: "", name: "", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::INTEL,    shortname: "MOVDIRI", name: "32-bit Direct Stores", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::INTEL,    shortname: "MOVDIRI64B", name: "64-bit Direct Stores", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::INTEL,    shortname: "ENQCMD", name: "Enqueue Stores", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::INTEL,    shortname: "SGX_LC", name: "SGX Launch Configuration", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::INTEL,    shortname: "PKS", name: "Protection keys for supervisor-mode pages", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_0000_0007_0_EDX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::INTEL,    shortname: "AVX512_4VNNIW", name: "AVX512 Neural Network Instructions", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::INTEL,    shortname: "AVX512_4FMAPS", name: "AVX512 Multiply Accumulation single precision", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::INTELAMD, shortname: "", name: "Fast Short REP MOV", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::INTEL,    shortname: "AVX512_VP2INTERSECT", name: "AVX512 Vector Intersection instructions", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::INTEL,    shortname: "", name: "MD_CLEAR", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::INTEL,    shortname: "", name: "TSX Force Abort MSR", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::INTEL,    shortname: "", name: "SERIALIZE", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::INTEL,    shortname: "", name: "Hybrid", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::INTEL,    shortname: "", name: "TSX suspend load address tracking", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::INTEL,    shortname: "", name: "PCONFIG", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::INTEL,    shortname: "CET_IBT", name: "CET indirect branch tracking", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::INTEL,    shortname: "AMX-BF16", name: "Tile computation on bfloat16", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::INTEL,    shortname: "AVX512-FP16", name: "AVX512 16-bit FP support", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::INTEL,    shortname: "AMX-TILE", name: "Tile architecture", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::INTEL,    shortname: "AMX-INT8", name: "Tile computation on 8-bit integers", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::INTEL,    shortname: "SPEC_CTRL", name: "IBRS and IBPB speculation control instructions", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::INTEL,    shortname: "STIBP", name: "Single Thread Indirect Branch Predictors", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::INTEL,    shortname: "L1D_FLUSH", name: "L1 Data Cache Flush", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::INTEL,    shortname: "", name: "IA32_ARCH_CAPABILITIES MSR support", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::INTEL,    shortname: "", name: "IA32_CORE_CAPABILITIES MSR support", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::INTEL,    shortname: "SSBD", name: "Speculative Store Bypass Disable", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_0000_0007_1_EAX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::INTEL,    shortname: "AVX_VNNI", name: "AVX Vector Neural Network Instructions", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::INTEL,    shortname: "AVX512_BF16", name: "AVX512 Vector Neural Network BFLOAT16", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::INTEL,    shortname: "", name: "Fast zero-length MOVSB", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::INTEL,    shortname: "", name: "Fast short STOSB", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::INTEL,    shortname: "", name: "Fast short CMPSB, SCASB", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::INTEL,    shortname: "HRESET", name: "History Reset", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::INTEL,    shortname: "LAM", name: "Linear Address Masking", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_0000_0014_0_EBX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::INTEL,   shortname: "", name: "CR3 filtering", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::INTEL,   shortname: "", name: "Configurable PSB, Cycle-Accurate Mode", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::INTEL,   shortname: "", name: "Filtering preserved across warm reset", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::INTEL,   shortname: "", name: "MTC timing packet, suppression of COFI-based packets", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::INTEL,   shortname: "", name: "PTWRITE", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::INTEL,   shortname: "", name: "Power Event Trace", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::INTEL,   shortname: "", name: "PSB and PMI preservation MSRs", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_0000_0014_0_ECX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::INTEL,   shortname: "", name: "ToPA output scheme", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::INTEL,   shortname: "", name: "ToPA tables hold multiple output entries", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::INTEL,   shortname: "", name: "Single-range output scheme", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::INTEL,   shortname: "", name: "Trace Transport output support", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::UNKNOWN, shortname: "", name: "", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::INTEL,   shortname: "", name: "IP payloads are LIP", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_4000_0001_EAX_KVM: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::KVM,      shortname: "", name: "Clocksource", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::KVM,      shortname: "", name: "NOP IO Delay", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::KVM,      shortname: "", name: "MMU Op", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::KVM,      shortname: "", name: "Clocksource 2", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::KVM,      shortname: "", name: "Async PF", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::KVM,      shortname: "", name: "Steal Time", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::KVM,      shortname: "", name: "PV EOI", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::KVM,      shortname: "", name: "PV UNHALT", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::KVM,      shortname: "", name: "PV TLB FLUSH", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::KVM,      shortname: "", name: "PV ASYNC PF VMEXIT", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::KVM,      shortname: "", name: "PV SEND IPI", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::KVM,      shortname: "", name: "PV POLL CONTROL", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::KVM,      shortname: "", name: "PV SCHED YIELD", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::KVM,      shortname: "", name: "Clocksource stable", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_8000_0001_EDX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::AMD,      shortname: "FPU", name: "x87 FPU on chip", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::AMD,      shortname: "VME", name: "Virtual-8086 Mode Enhancement", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::AMD,      shortname: "DE", name: "Debugging Extensions", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::AMD,      shortname: "PSE", name: "Page Size Extensions", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::AMD,      shortname: "TSC", name: "Time Stamp Counter", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::AMD,      shortname: "MSR", name: "RDMSR and WRMSR support", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::AMD,      shortname: "PAE", name: "Physical Address Extensions", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::AMD,      shortname: "MCE", name: "Machine Check Exception", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::AMD,      shortname: "CX8", name: "CMPXCHG8B instruction", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::AMD,      shortname: "APIC", name: "APIC on chip", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::ANY_CPU,  shortname: "SYSCALL", name: "SYSCALL and SYSRET instructions", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::AMD,      shortname: "MTRR", name: "Memory Type Range Registers", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::AMD,      shortname: "PGE", name: "PTE Global Bit", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::AMD,      shortname: "MCA", name: "Machine Check Architecture", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::AMD,      shortname: "CMOV", name: "Conditional Move/Compare Instruction", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::AMD,      shortname: "PAT", name: "Page Attribute Table", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::AMD,      shortname: "PSE-36", name: "Page Size Extension", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::INTEL,    shortname: "XD", name: "eXecute Disable page attribute bit", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::AMD,      shortname: "NX", name: "No eXecute page attribute bit", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::AMD,      shortname: "MMXExt", name: "AMD extensions to MMX instructions", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::AMD,      shortname: "MMX", name: "MMX instruction set", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::AMD,      shortname: "FXSR", name: "FXSAVE/FXRSTOR instructions", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::AMD,      shortname: "FFXSR", name: "FXSAVE/FXRSTOR instruction optimizations", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::ANY_CPU,  shortname: "Page1GB", name: "1GB page support", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::ANY_CPU,  shortname: "RDTSCP", name: "RDTSCP instruction and IA32_TSC_AUX MSR", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::ANY_CPU,  shortname: "LM", name: "Long Mode, EM64T", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::UNKNOWN,  shortname: "3DNowExt", name: "AMD extensions to 3DNow! instructions", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::AMD,      shortname: "3DNow", name: "3DNow! instructions", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_8000_0001_ECX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::ANY_CPU,  shortname: "LahfSahf", name: "LAHF/SAHF instruction support in 64-bit mode", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::AMD,      shortname: "CmpLegacy", name: "Core multi-processing legacy mode", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::AMD,      shortname: "SVM", name: "Secure Virtual Machine", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::AMD,      shortname: "ExtApicSpace", name: "extended APIC space", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::AMD,      shortname: "AltMovCr8", name: "LOCK MOV CR0 means MOV CR8", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::ANY_CPU,  shortname: "LZCNT", name: "LZCNT instruction", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::AMD,      shortname: "SSE4A", name: "SSE4A instructions", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::AMD,      shortname: "MisAlignSse", name: "misaligned SSE support", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::ANY_CPU,  shortname: "3DNowPrefetch", name: "PREFETCH and PREFETCHW instruction support", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::AMD,      shortname: "OSVW", name: "OS-visible workaround support", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::AMD,      shortname: "IBS", name: "Instruction based sampling", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::AMD,      shortname: "XOP", name: "Extended operation support", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::AMD,      shortname: "SKINIT", name: "SKINIT/STGI instructions", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::AMD,      shortname: "WDT", name: "Watchdog timer", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::AMD,      shortname: "LWP", name: "Lightweight profiling", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::AMD,      shortname: "FMA4", name: "4-operand FMA instructions", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::AMD,      shortname: "TCE", name: "Translation cache extension", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::AMD,      shortname: "", name: "node ID support", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::AMD,      shortname: "", name: "trailing bit manipulation instructions", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::AMD,      shortname: "", name: "topology extensions", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::AMD,      shortname: "PerfCtrExtCore", name: "core performance counter extensions", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::AMD,      shortname: "PerfCtrExtDF", name: "data fabricperformance counter extensions", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::AMD,      shortname: "", name: "streaming performance monitor architecture", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::AMD,      shortname: "DataBreakpointExtension", name: "data access breakpoint extensions", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::AMD,      shortname: "PerfTsc", name: "performance timestamp counter", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::AMD,      shortname: "PerfCtrExtLLC", name: "Last Level Cache performance counter extensions", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::AMD,      shortname: "MwaitExtended", name: "MONITORX/MWAITX instructions", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::AMD,      shortname: "AdMskExtn", name: "address mask extension for instruction breakpoint", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_8000_0007_EBX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::AMD,  shortname: "McaOverflowRecov", name: "MCA overflow recovery support", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::AMD,  shortname: "SUCCOR", name: "Software uncorrectable error containment and recovery", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::AMD,  shortname: "HWA", name: "Hardware assert", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::AMD,  shortname: "ScalableMca", name: "Scalable machine check architecture", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::AMD,  shortname: "PFEH", name: "Platform first error handling", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_8000_0007_EDX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::AMD,      shortname: "TS", name: "Temperature sensor", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::AMD,      shortname: "FID", name: "Frequency ID control", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::AMD,      shortname: "VID", name: "Voltage ID control", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::AMD,      shortname: "TTP", name: "THERMTRIP", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::AMD,      shortname: "HTC", name: "Hardware thermal control", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::AMD,      shortname: "", name: "100 MHz multiplier control", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::AMD,      shortname: "TscInvariant", name: "TSC rate is invariant", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::AMD,      shortname: "CPB", name: "Core performance boost", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::AMD,      shortname: "EffFreqRO", name: "Read-only effective frequency interface, APERF/MPERF", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::AMD,      shortname: "", name: "Processor feedback interface", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::AMD,      shortname: "", name: "Core power reporting", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::AMD,      shortname: "", name: "Connected standby", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::AMD,      shortname: "RAPL", name: "Running average power limit", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_8000_0008_EBX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::AMD,      shortname: "CLZERO", name: "Clear zero instruction", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::AMD,      shortname: "InstRetCntMsr", name: "Instructions retired count support", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::AMD,      shortname: "RstrFpErrPtrs", name: "XSAVE always saves/restores error pointers", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::AMD,      shortname: "", name: "INVLPGB and TLBSYNC instruction", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::AMD,      shortname: "RDPRU", name: "RDPRU instruction", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::AMD,      shortname: "MBE", name: "Memory bandwidth enforcement", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::AMD,      shortname: "MCOMMIT", name: "Memory commit instruction", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::ANY_CPU,  shortname: "WBNOINVD", name: "Write back and invalidate cache", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::AMD,      shortname: "LBR", name: "Last branch extensions", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::AMD,      shortname: "IBPB", name: "Indirect Branch Prediction Barrier", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::AMD,      shortname: "INT_WBINVD", name: "Interruptible WBINVD,WBNOINVD", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::AMD,      shortname: "IBRS", name: "Indirect Branch Restricted Speculation", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::AMD,      shortname: "STIBP", name: "Single Thread Indirect Branch Prediction", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::AMD,      shortname: "StibpAlwaysOn", name: "STIBP always enabled", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::AMD,      shortname: "IbrsPreferred", name: "IBRS preferred over software solution", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::AMD,      shortname: "IbrsSameMode", name: "IBRS provides Same Mode Protection", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::AMD,      shortname: "", name: "EFER.LMLSE is unsupported", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::AMD,      shortname: "", name: "INVLPGB for guest nested translations", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::AMD,      shortname: "PPIN", name: "Protected Processor Inventory Number", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::AMD,      shortname: "SSBD", name: "Speculative Store Bypass Disable", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::AMD,      shortname: "VIRT_SPEC_CTL", name: "Speculation control for virtual machines", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::AMD,      shortname: "SsbdNotNeeded", name: "SSBD no longer needed", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_8000_000A_EDX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::AMD,      shortname: "NP", name: "Nested paging", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::AMD,      shortname: "LbrVit", name: "LBR virtualization", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::AMD,      shortname: "SVML", name: "SVM lock", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::AMD,      shortname: "NRIPS", name: "NRIP save", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::AMD,      shortname: "TscRateMsr", name: "MSR-based TSC rate control", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::AMD,      shortname: "", name: "VMCB clean bits", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::AMD,      shortname: "", name: "Flush by ASID", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::AMD,      shortname: "", name: "Pause intercept filter", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::AMD,      shortname: "", name: "Encrypted µcode patch", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::AMD,      shortname: "", name: "Pause filter threshold", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::AMD,      shortname: "AVIC", name: "AMD virtual interrupt controller", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::AMD,      shortname: "", name: "Virtualized VMLOAD/VMSAVE", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::AMD,      shortname: "", name: "Virtualized GIF", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::AMD,      shortname: "GMET", name: "Guest mode execution trap", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::AMD,      shortname: "", name: "SVM supervisor shadow stack restrictions", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::AMD,      shortname: "GuesSpecCtl", name: "SPEC_TRL virtualization", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::AMD,      shortname: "", name: "INVLPGB/TLBSYNC hypervisor enable", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_8000_001A_EAX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::AMD,      shortname: "FP128", name: "128-bit SSE full-width pipelines", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::AMD,      shortname: "MOVU", name: "Efficient MOVU SSE instructions", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::AMD,      shortname: "FP256", name: "256-bit AVX full-width pipelines", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_8000_001B_EAX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::AMD,      shortname: "IBSFFV", name: "IBS feature flags valid", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::AMD,      shortname: "FetchSam", name: "IBS fetch sampling", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::AMD,      shortname: "OpSam", name: "IBS execution sampling", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::AMD,      shortname: "RdWrOpCnt", name: "Read/write of op counter", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::AMD,      shortname: "OpCnt", name: "Op counting mode", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::AMD,      shortname: "BrnTrgt", name: "Branch target address reporting", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::AMD,      shortname: "OpCntExt", name: "IBS op cur/max count extended by 7 bits", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::AMD,      shortname: "RipInvalidChk", name: "IBS RIP invalid indication", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::AMD,      shortname: "OpBrnFuse", name: "IBS fused branch micro-op indication", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::AMD,      shortname: "IbsFetchCtlExtd", name: "IBS fetch control extended MSR", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::AMD,      shortname: "IbsOpData4", name: "IBS op data 4 MSR", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

static FEATURES_C000_0001_EDX: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::CENTAUR,  shortname: "", name: "Alternate Instruction Set available", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::CENTAUR,  shortname: "", name: "Alternate Instruction Set enabled", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::CENTAUR,  shortname: "", name: "Random Number Generator available", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::CENTAUR,  shortname: "", name: "Random Number Generator enabled", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::CENTAUR,  shortname: "", name: "LongHaul MSR 0000_110Ah", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::CENTAUR,  shortname: "", name: "FEMMS", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::CENTAUR,  shortname: "", name: "Advanced Cryptography Engien (ACE) available", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::CENTAUR,  shortname: "", name: "Advanced Cryptography Engien (ACE) enabled", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::CENTAUR,  shortname: "", name: "Montgomery Multiplier and Hash Engine (ACE2) available", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::CENTAUR,  shortname: "", name: "Montgomery Multiplier and Hash Engine (ACE2) enabled", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::CENTAUR,  shortname: "", name: "Padlock hash engine (PHE) available", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::CENTAUR,  shortname: "", name: "Padlock hash engine (PHE) enabled", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::CENTAUR,  shortname: "", name: "Padlock montgomery multiplier (PMM) available", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::CENTAUR,  shortname: "", name: "Padlock montgomery multiplier (PMM) enabled", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];

/*
static FEATURES_0000_0000_REG: [FeatureSpec; 40] = [
    FeatureSpec { bit: 1,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 2,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 3,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 4,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 5,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 6,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 7,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 8,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 9,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 10, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 11, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 12, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 13, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 14, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 15, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 16, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 17, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 18, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 19, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 20, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 21, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 22, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 23, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 24, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 25, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 26, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 27, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 28, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 29, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 30, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 31, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 32, vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },

    // Slack fill. Some bits have multiple uses depending on vendor, so we have
    // to fill the end of each vector with a bunch of junk to keep the arrays
    // all the same size.
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
    FeatureSpec { bit: 0,  vendor_mask: VendorMask::UNKNOWN,  shortname: "", name: "", },
];
*/

static FEATURE_LEAVES: [FeatureLeaf; 20] = [
    FeatureLeaf { leaf: LeafID { eax: 0x0000_0001, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::EDX, bits: &FEATURES_0000_0001_EDX, },
    FeatureLeaf { leaf: LeafID { eax: 0x0000_0001, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::ECX, bits: &FEATURES_0000_0001_ECX, },
    FeatureLeaf { leaf: LeafID { eax: 0x0000_0006, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::EAX, bits: &FEATURES_0000_0006_EAX, },
    FeatureLeaf { leaf: LeafID { eax: 0x0000_0006, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::ECX, bits: &FEATURES_0000_0006_ECX, },
    FeatureLeaf { leaf: LeafID { eax: 0x0000_0007, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::EBX, bits: &FEATURES_0000_0007_0_EBX, },
    FeatureLeaf { leaf: LeafID { eax: 0x0000_0007, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::ECX, bits: &FEATURES_0000_0007_0_ECX, },
    FeatureLeaf { leaf: LeafID { eax: 0x0000_0007, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::EDX, bits: &FEATURES_0000_0007_0_EDX, },
    FeatureLeaf { leaf: LeafID { eax: 0x0000_0007, ecx: 1, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::EAX, bits: &FEATURES_0000_0007_1_EAX, },
    FeatureLeaf { leaf: LeafID { eax: 0x0000_0014, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::EBX, bits: &FEATURES_0000_0014_0_EBX, },
    FeatureLeaf { leaf: LeafID { eax: 0x0000_0014, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::ECX, bits: &FEATURES_0000_0014_0_ECX, },
    FeatureLeaf { leaf: LeafID { eax: 0x4000_0001, ecx: 0, }, vendor_mask: VendorMask::KVM,     register: RegisterName::EAX, bits: &FEATURES_4000_0001_EAX_KVM, },
    FeatureLeaf { leaf: LeafID { eax: 0x8000_0001, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::EDX, bits: &FEATURES_8000_0001_EDX, },
    FeatureLeaf { leaf: LeafID { eax: 0x8000_0001, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::ECX, bits: &FEATURES_8000_0001_ECX, },
    FeatureLeaf { leaf: LeafID { eax: 0x8000_0007, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::EBX, bits: &FEATURES_8000_0007_EBX, },
    FeatureLeaf { leaf: LeafID { eax: 0x8000_0007, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::EDX, bits: &FEATURES_8000_0007_EDX, },
    FeatureLeaf { leaf: LeafID { eax: 0x8000_0008, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::EBX, bits: &FEATURES_8000_0008_EBX, },
    FeatureLeaf { leaf: LeafID { eax: 0x8000_000A, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::EDX, bits: &FEATURES_8000_000A_EDX, },
    FeatureLeaf { leaf: LeafID { eax: 0x8000_001A, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::EAX, bits: &FEATURES_8000_001A_EAX, },
    FeatureLeaf { leaf: LeafID { eax: 0x8000_001B, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::EAX, bits: &FEATURES_8000_001B_EAX, },
    FeatureLeaf { leaf: LeafID { eax: 0xC000_0001, ecx: 0, }, vendor_mask: VendorMask::ANY_CPU, register: RegisterName::EDX, bits: &FEATURES_C000_0001_EDX, },
];

pub fn collect(cpu: &Processor, vendor_mask: VendorMask) -> FeatureVec {
    let mut output: FeatureVec = FeatureVec::new();
    for feature_leaf in FEATURE_LEAVES.iter() {
        if (vendor_mask & feature_leaf.vendor_mask).is_empty() {
            continue;
        }
        if let Some(raw) = cpu.get_subleaf(feature_leaf.leaf.eax, feature_leaf.leaf.ecx) {
            debug!(
                "Leaf {:08x}:{:02x}:{:?} beginning decode",
                feature_leaf.leaf.eax, feature_leaf.leaf.ecx, feature_leaf.register
            );
            let mut register: u32 = raw.output.register(feature_leaf.register);
            if feature_leaf.leaf.eax == 0x8000_0001 && feature_leaf.register == RegisterName::EDX {
                // These are features covered in leaf 0x0000_0001, and we don't
                // want to repeat them here.
                register &= !0x0183ffff;
            }
            for feature_spec in feature_leaf.bits.iter() {
                if feature_spec.bit < 1 {
                    continue;
                }
                let bit = feature_spec.bit - 1;
                if !(vendor_mask & feature_spec.vendor_mask).is_empty() {
                    let mask = 1 << bit;
                    if (register & mask) != 0 {
                        // Mark that we've seen and accounted for this feature
                        // bit. We can report on unaccounted for bits afterward
                        // (in debug)
                        register &= !mask;
                        output.0.push(Feature::from_detection(
                            feature_leaf,
                            feature_spec,
                            bit as u8,
                        ));
                    }
                }
            }
            if register != 0 {
                debug!(
                    "Leaf {:08x}:{:02x}:{:?} unaccounted for bits: {:08x}",
                    feature_leaf.leaf.eax, feature_leaf.leaf.ecx, feature_leaf.register, register
                );
            }
        }
    }
    output
}
