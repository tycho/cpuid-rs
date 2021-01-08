#![allow(dead_code)]

use log::*;
use modular_bitfield::prelude::*;
use std::cmp::Ordering;
use std::fmt;
use textwrap::indent;

#[cfg(feature = "legacy-cache-descriptors")]
use crate::cache_descriptors::lookup_cache_descriptor;

use crate::cpuid::{Processor, RegisterName, System, VendorMask};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
/// Describes the type of cache or TLB (e.g. does it contain code, data, both?)
pub enum CacheType {
    Unknown = 0,
    Code = 1,
    Trace = 2,
    Data = 3,
    Unified = 4,
    DataTLB = 5,
    CodeTLB = 6,
    SharedTLB = 7,
    LoadOnlyTLB = 8,
    StoreOnlyTLB = 9,
}

impl Default for CacheType {
    fn default() -> CacheType {
        CacheType::Unknown
    }
}

impl CacheType {
    /// Coalesce types for sorting reasons -- we want caches and then TLBs together.

    fn is_tlb(&self) -> bool {
        match self {
            CacheType::DataTLB
            | CacheType::CodeTLB
            | CacheType::SharedTLB
            | CacheType::LoadOnlyTLB
            | CacheType::StoreOnlyTLB => true,
            _ => false,
        }
    }
}

impl fmt::Display for CacheType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CacheType::Unknown => "unknown cache",
                CacheType::Data => "data cache",
                CacheType::Code => "code cache",
                CacheType::Unified => "unified cache",
                CacheType::Trace => "trace cache",
                CacheType::DataTLB => "data TLB",
                CacheType::CodeTLB => "code TLB",
                CacheType::SharedTLB => "shared TLB",
                CacheType::LoadOnlyTLB => "load-only TLB",
                CacheType::StoreOnlyTLB => "store-only TLB",
            }
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
/// Describes the level of the cache, if known.
pub enum CacheLevel {
    L0 = 0,
    L1 = 1,
    Unknown = 2,
    L2 = 3,
    L3 = 4,
    L4 = 5,
}

impl Default for CacheLevel {
    fn default() -> CacheLevel {
        CacheLevel::Unknown
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
/// Describes the associativity type of the cache.
pub enum CacheAssociativityType {
    /// Cache associativity type is unknown or not specified.
    Unknown = 0,

    /// Cache has 1-way set associativity.
    DirectMapped = 1,

    /// Cache has N-way set associativity. See the
    /// [ways](struct.CacheAssociativity.html#structfield.ways) field of
    /// [CacheAssocitivity](struct.CacheAssociativity) to find out the N value.
    NWay = 2,

    /// Cache is fully associative.
    FullyAssociative = 3,
}
impl Default for CacheAssociativityType {
    fn default() -> CacheAssociativityType {
        CacheAssociativityType::Unknown
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Describes the associativity of a cache or TLB.
pub struct CacheAssociativity {
    /// Type of associativity for this cache or TLB.
    pub mapping: CacheAssociativityType,

    /// If cache is N-way set associative, contains the number of ways of
    /// associativity. Otherwise this field is invalid.
    pub ways: u16,
}

impl CacheAssociativity {
    pub fn from_identifier(id: u8) -> CacheAssociativity {
        CacheAssociativity {
            mapping: match id {
                0x00 => CacheAssociativityType::Unknown,
                0x01 => CacheAssociativityType::DirectMapped,
                0xFF => CacheAssociativityType::FullyAssociative,
                _ => CacheAssociativityType::NWay,
            },
            ways: id as u16,
        }
    }
}

impl fmt::Display for CacheAssociativity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.mapping {
            CacheAssociativityType::Unknown => write!(f, "unknown associativity"),
            CacheAssociativityType::DirectMapped => write!(f, "direct-mapped"),
            CacheAssociativityType::NWay => write!(f, "{}-way set associative", self.ways),
            CacheAssociativityType::FullyAssociative => write!(f, "fully associative"),
        }
    }
}

#[bitfield(bits = 16)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Contains the features of a cache or TLB.
pub struct CacheFlags {
    /// `true` if the cache descriptor information was discovered outside of the
    /// standard CPU manuals.
    pub undocumented: bool,

    /// `true` if this is an IA-64 (Itanium) cache type.
    pub ia64: bool,

    /// `true` if the cache has error correction.
    pub ecc: bool,

    /// `true` if the cache is sectored.
    pub sectored: bool,

    /// `true` if the TLB handles 4KB pages.
    pub pages_4k: bool,

    /// `true` if the TLB handles 2MB pages.
    pub pages_2m: bool,

    /// `true` if the TLB handles 4MB pages.
    pub pages_4m: bool,

    /// `true` if the TLB handles 1GB pages.
    pub pages_1g: bool,

    /// `true` if the cache is initialized without software intervention.
    pub self_initializing: bool,

    /// `true` if the cache uses complex indexing.
    pub complex_indexing: bool,

    /// `true` if the cache includes data from lower cache levels.
    pub inclusive: bool,

    /// `true` if WBINVD does not invalidate lower cache levels.
    pub wbinvd_not_inclusive: bool,
    #[skip]
    __: B4,
}

#[derive(Debug, Default, Eq)]
/// Describes a cache or TLB.
pub struct CacheDescription {
    /// Level of the cache.
    pub level: CacheLevel,

    /// Type of cache or TLB.
    pub cachetype: CacheType,

    /// When describing a cache, size is in KB. When describing a TLB, size is the number of entries in the TLB.
    pub size: u32,

    /// Cache line size. Not valid for TLBs.
    pub linesize: u16,

    /// Flags describing the cache or TLB properties.
    pub flags: CacheFlags,

    /// Associativity type and ways for this cache or TLB.
    pub associativity: CacheAssociativity,

    /// Number of cache partitions. This field is invalid for a TLB.
    pub partitions: u16,

    /// Maximum number of logical CPUs sharing this cache or TLB. This may be
    /// zero, if the hardware vendor or CPUID leaf do not specify the
    /// information.
    pub max_threads_sharing: u16,

    /// Inferred number of instances of this cache/TLB in the system, based on
    /// the `max_threads_sharing` field and the number of logical processors in
    /// the [System](struct.System.html)
    pub instances: usize,
}

impl Ord for CacheDescription {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut ord: Ordering = Ordering::Equal;
        if ord == Ordering::Equal {
            ord = self.cachetype.is_tlb().cmp(&other.cachetype.is_tlb());
        }
        if ord == Ordering::Equal {
            ord = self.level.cmp(&other.level);
        }
        if ord == Ordering::Equal {
            ord = self.flags.pages_1g().cmp(&other.flags.pages_1g());
        }
        if ord == Ordering::Equal {
            ord = self.flags.pages_4m().cmp(&other.flags.pages_4m());
        }
        if ord == Ordering::Equal {
            ord = self.flags.pages_2m().cmp(&other.flags.pages_2m());
        }
        if ord == Ordering::Equal {
            ord = self.flags.pages_4k().cmp(&other.flags.pages_4k());
        }
        if ord == Ordering::Equal {
            ord = self.cachetype.cmp(&other.cachetype);
        }
        if ord == Ordering::Equal {
            ord = self.size.cmp(&other.size);
        }
        ord
    }
}

impl PartialOrd for CacheDescription {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CacheDescription {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level
            && self.cachetype == other.cachetype
            && self.size == other.size
            && self.linesize == other.linesize
            && self.associativity == other.associativity
            && self.partitions == other.partitions
    }
}

#[derive(Debug)]
/// A vector of [CacheDescriptions](struct.CacheDescription.html).
pub struct CacheVec(pub Vec<CacheDescription>);

impl CacheVec {
    pub fn new() -> CacheVec {
        CacheVec(vec![])
    }
}

impl fmt::Display for CacheVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Caches:\n")?;
        let mut last_was_cache: bool = true;
        for v in &self.0 {
            let formatted = format!("{}\n", v);
            if last_was_cache && v.cachetype.is_tlb() {
                last_was_cache = false;
                write!(f, "\n")?;
            }
            write!(f, "{}", indent(&formatted, "  "))?;
        }
        Ok(())
    }
}

fn size_str(kb: u32, cachetype: CacheType) -> String {
    if cachetype == CacheType::Trace {
        return format!("{}K-µop", kb);
    }
    if kb >= 1024 {
        format!("{}MB", kb / 1024)
    } else {
        format!("{}KB", kb)
    }
}

fn pagetypes_str(flags: &CacheFlags) -> String {
    let mut names: Vec<String> = vec![];
    if flags.pages_4k() {
        names.push("4KB".to_string());
    }
    if flags.pages_2m() {
        names.push("2MB".to_string());
    }
    if flags.pages_4m() {
        names.push("4MB".to_string());
    }
    if flags.pages_1g() {
        names.push("1GB".to_string());
    }
    if names.len() < 3 {
        names.join(" or ")
    } else {
        let mut result: String = names[..names.len() - 1].join(", ");
        result.push_str(" or ");
        result.push_str(&names[names.len() - 1]);
        result
    }
}

fn first_letter_to_uppercase(s1: String) -> String {
    let mut c = s1.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

impl CacheDescription {
    fn fmt_cache(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.instances > 0 {
            // e.g. 8 x 48KB L1 data cache
            write!(
                f,
                "{: >2} x {: >7} {: <2?} {: <}",
                self.instances,
                size_str(self.size, self.cachetype),
                self.level,
                self.cachetype
            )?;
        } else {
            // e.g. 48KB L1 data cache
            write!(
                f,
                "{: >5}{: >7} {: <2?} {: <}",
                "",
                size_str(self.size, self.cachetype),
                self.level,
                self.cachetype
            )?;
        }
        // e.g. 8-way set associative
        write!(f, ", {}", self.associativity)?;
        if self.cachetype != CacheType::Trace {
            // e.g. 64 byte line size
            write!(f, ", {} byte line size", self.linesize)?;
        }
        if self.flags.ecc() {
            write!(f, "\n{: >13}ECC", "")?;
        }
        if self.flags.self_initializing() {
            write!(f, "\n{: >13}Self-initializing", "")?;
        }
        if self.flags.inclusive() {
            write!(f, "\n{: >13}Inclusive of lower cache levels", "")?;
        }
        if self.flags.complex_indexing() {
            write!(f, "\n{: >13}Complex indexing", "")?;
        }
        if self.flags.wbinvd_not_inclusive() {
            write!(f, "\n{: >13}Does not invalidate lower cache levels", "")?;
        }
        if self.flags.undocumented() {
            write!(f, "\n{: >13}Undocumented descriptor", "")?;
        }
        //write!(f, "{: >11}Shared by max {} threads\n", "", self.max_threads_sharing);
        Ok(())
    }

    fn fmt_tlb(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cachename = match self.level {
            CacheLevel::Unknown => format!("{}", self.cachetype),
            _ => format!("{:?} {}", self.level, self.cachetype),
        };
        // e.g. L1 code TLB: 4KB pages
        write!(
            f,
            "{: >17}: {} pages, ",
            first_letter_to_uppercase(cachename),
            pagetypes_str(&self.flags)
        )?;
        // e.g. 4 entries
        write!(f, "{} entries, ", self.size)?;
        // e.g. 8-way set associative
        write!(f, "{}", self.associativity)?;
        if self.max_threads_sharing > 0 {
            // e.g. Shared by max 8 threads
            write!(
                f,
                "\n{: >19}Shared by max {} threads",
                "", self.max_threads_sharing
            )?;
        }
        Ok(())
    }
}

impl fmt::Display for CacheDescription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cachetype {
            CacheType::Data | CacheType::Code | CacheType::Unified | CacheType::Trace => {
                self.fmt_cache(f)
            }
            CacheType::DataTLB
            | CacheType::CodeTLB
            | CacheType::SharedTLB
            | CacheType::LoadOnlyTLB
            | CacheType::StoreOnlyTLB => self.fmt_tlb(f),
            _ => panic!(
                "Don't know how to describe cache type {:#?}",
                self.cachetype
            ),
        }
        //write!(f, "SOME KINDA CACHE LOL\n")?;
    }
}

fn translate_amd_l2_associativity(raw: u8) -> u8 {
    match raw {
        0x0 => 0x0,
        0x1 => 0x1,
        0x2 => 0x2,
        0x3 => 0x3,
        0x4 => 0x4,
        0x5 => 0x6,
        0x6 => 0x8,
        0x8 => 0x10,
        0xA => 0x20,
        0xB => 0x30,
        0xC => 0x40,
        0xD => 0x60,
        0xE => 0x80,
        0xF => 0xFF,
        _ => 0x0,
    }
}

fn walk_amd_cache_extended(system: &System, cpu: &Processor, out: &mut CacheVec) -> bool {
    if (system.vendor & VendorMask::AMD).is_empty() {
        debug!("walk_amd_cache_extended() skipped on non-AMD CPU");
        return false;
    }

    #[bitfield(bits = 32)]
    struct EaxCache {
        cachetype: B5,
        level: B3,
        self_initializing: bool,
        fully_associative: bool,
        #[skip]
        __: B4,
        sharing: B12,
        #[skip]
        __: B6,
    }

    #[bitfield(bits = 32)]
    struct EbxCache {
        linesize: B12,
        partitions: B10,
        ways: B10,
    }

    #[bitfield(bits = 32)]
    struct EcxCache {
        sets: u32,
    }

    #[bitfield(bits = 32)]
    struct EdxCache {
        wbinvd: bool,
        inclusive: bool,
        #[skip]
        __: B30,
    }

    if !cpu.has_feature_bit(0x8000_0001, 0, RegisterName::ECX, 22) {
        return false;
    }

    let mut subleaf: u32 = 0;
    while let Some(raw) = cpu.get_subleaf(0x8000_001D, subleaf) {
        let eax = EaxCache::from_bytes(raw.output.eax.to_le_bytes());
        let ebx = EbxCache::from_bytes(raw.output.ebx.to_le_bytes());
        let ecx = EcxCache::from_bytes(raw.output.ecx.to_le_bytes());
        let edx = EdxCache::from_bytes(raw.output.edx.to_le_bytes());
        let mut desc = CacheDescription::default();

        if eax.cachetype() == 0 {
            break;
        }

        let mut size: u32 = (ebx.partitions() as u32 + 1)
            * (ebx.linesize() as u32 + 1)
            * (ebx.ways() as u32 + 1)
            * (ecx.sets() + 1);
        size /= 1024;

        desc.size = size;
        desc.linesize = ebx.linesize() + 1;
        desc.partitions = ebx.partitions() + 1;
        desc.max_threads_sharing = eax.sharing() + 1;

        desc.level = match eax.level() {
            1 => CacheLevel::L1,
            2 => CacheLevel::L2,
            3 => CacheLevel::L3,
            _ => CacheLevel::Unknown,
        };
        desc.cachetype = match eax.cachetype() {
            1 => CacheType::Data,
            2 => CacheType::Code,
            3 => CacheType::Unified,
            _ => CacheType::Unknown,
        };
        if eax.fully_associative() {
            desc.associativity.mapping = CacheAssociativityType::FullyAssociative;
        } else {
            desc.associativity.mapping = CacheAssociativityType::NWay;
            desc.associativity.ways = ebx.ways() as u16 + 1;
        }
        desc.flags.set_self_initializing(eax.self_initializing());
        desc.flags.set_wbinvd_not_inclusive(edx.wbinvd());
        desc.flags.set_inclusive(edx.inclusive());

        desc.instances = match system.cpu_count >= (eax.sharing() + 1) as usize {
            true => system.cpu_count / (eax.sharing() + 1) as usize,
            false => 1,
        };

        debug!("walk_amd_cache_extended() found cache {:?}", desc);

        out.0.push(desc);

        subleaf += 1;
    }

    true
}

fn walk_amd_cache_legacy(system: &System, cpu: &Processor, out: &mut CacheVec) {
    // Read from:
    // AMD L1 cache features (0x8000_0005)
    // AMD L2 cache features (0x8000_0006)

    if (system.vendor & VendorMask::AMD).is_empty() {
        debug!("walk_amd_cache_legacy() skipped on non-AMD CPU");
        return;
    }

    #[bitfield(bits = 32)]
    #[derive(Debug)]
    struct L1CacheDesc {
        linesize: u8,
        linespertag: u8,
        associativity: u8,
        size: u8,
    }

    if let Some(raw) = cpu.get_subleaf(0x8000_0005, 0) {
        let level = CacheLevel::L1;

        for register in vec![RegisterName::ECX, RegisterName::EDX] {
            let cachetype = match register {
                RegisterName::ECX => CacheType::Data,
                RegisterName::EDX => CacheType::Code,
                _ => panic!("Invalid register name!"),
            };

            let regbytes = raw.output.register(register).to_le_bytes();
            let cache = L1CacheDesc::from_bytes(regbytes);

            if cache.size() != 0 {
                let desc = CacheDescription {
                    level: level.clone(),
                    cachetype: cachetype.clone(),
                    associativity: CacheAssociativity::from_identifier(cache.associativity()),
                    size: cache.size() as u32,
                    linesize: cache.linesize() as u16,
                    ..Default::default()
                };
                debug!("walk_amd_cache_legacy() found L1 cache: {:?}", desc);
                out.0.push(desc);
            }
        }
    }

    #[bitfield(bits = 32)]
    #[derive(Debug)]
    struct L2CacheDesc {
        linesize: u8,
        linespertag: B4,
        associativity: B4,
        size: u16,
    }

    #[bitfield(bits = 32)]
    #[derive(Debug)]
    struct L3CacheDesc {
        linesize: B8,
        linespertag: B4,
        associativity: B4,
        #[skip]
        __: B2,
        size: B14,
    }

    if let Some(raw) = cpu.get_subleaf(0x8000_0006, 0) {
        let l2cache = L2CacheDesc::from_bytes(raw.output.ecx.to_le_bytes());
        if l2cache.size() != 0 {
            let desc = CacheDescription {
                level: CacheLevel::L2,
                cachetype: CacheType::Unified,
                associativity: CacheAssociativity::from_identifier(translate_amd_l2_associativity(
                    l2cache.associativity(),
                )),
                size: l2cache.size() as u32,
                linesize: l2cache.linesize() as u16,
                ..Default::default()
            };
            debug!("walk_amd_cache_legacy() found L2 cache: {:?}", desc);
            out.0.push(desc);
        }

        let l3cache = L3CacheDesc::from_bytes(raw.output.edx.to_le_bytes());
        if l3cache.size() != 0 {
            let mut l3size: u32 = l3cache.size() as u32 * 512;
            if l3cache.size() == 0x0003
                || (l3cache.size() >= 0x0005 && l3cache.size() <= 0x0007)
                || (l3cache.size() >= 0x0009 && l3cache.size() <= 0x000F)
                || (l3cache.size() >= 0x0011 && l3cache.size() <= 0x001F)
            {
                l3size /= 2;
            }
            let desc = CacheDescription {
                level: CacheLevel::L3,
                cachetype: CacheType::Unified,
                associativity: CacheAssociativity::from_identifier(translate_amd_l2_associativity(
                    l3cache.associativity(),
                )),
                size: l3size,
                linesize: l3cache.linesize() as u16,
                ..Default::default()
            };
            debug!("walk_amd_cache_legacy() found L3 cache: {:?}", desc);
            out.0.push(desc);
        }
    }
}

fn walk_amd_cache(system: &System, cpu: &Processor, out: &mut CacheVec) {
    // We want to prefer, in order:
    //
    // - Extended Cache Topology (0x8000_001D)
    // - L1/L2 cache features (0x8000_0005 and 0x8000_0006)
    //
    // The latter is less expressive in terms of cache details, so we should try
    // to use Extended Cache Topology wherever possible.
    if !walk_amd_cache_extended(system, cpu, out) {
        walk_amd_cache_legacy(system, cpu, out);
    }
}

fn walk_amd_tlb(system: &System, cpu: &Processor, out: &mut CacheVec) {
    // Read from:
    // AMD L1 cache features (0x8000_0005)
    // AMD L2 cache features (0x8000_0006)

    if (system.vendor & VendorMask::AMD).is_empty() {
        debug!("walk_amd_tlb() skipped on non-AMD CPU");
        return;
    }

    #[bitfield(bits = 32)]
    #[derive(Debug)]
    struct L1TlbDesc {
        itlb_entries: u8,
        itlb_associativity: u8,
        dtlb_entries: u8,
        dtlb_associativity: u8,
    }

    if let Some(raw) = cpu.get_subleaf(0x8000_0005, 0) {
        let level = CacheLevel::L1;

        for register in vec![RegisterName::EBX, RegisterName::EAX] {
            let cacheflags = match register {
                RegisterName::EBX => CacheFlags::new().with_pages_4k(true),
                RegisterName::EAX => CacheFlags::new().with_pages_2m(true).with_pages_4m(true),
                _ => panic!("Invalid register name!"),
            };

            let regbytes = raw.output.register(register).to_le_bytes();
            let tlb = L1TlbDesc::from_bytes(regbytes);

            if tlb.dtlb_entries() > 0 {
                let desc = CacheDescription {
                    level: level.clone(),
                    cachetype: CacheType::DataTLB,
                    associativity: CacheAssociativity::from_identifier(tlb.dtlb_associativity()),
                    size: tlb.dtlb_entries() as u32,
                    flags: cacheflags.clone(),
                    ..Default::default()
                };
                debug!("walk_amd_tlb() found L1 dtlb {:?}", desc);
                out.0.push(desc);
            }
            if tlb.itlb_entries() > 0 {
                let desc = CacheDescription {
                    level: level.clone(),
                    cachetype: CacheType::CodeTLB,
                    associativity: CacheAssociativity::from_identifier(tlb.itlb_associativity()),
                    size: tlb.itlb_entries() as u32,
                    flags: cacheflags.clone(),
                    ..Default::default()
                };
                debug!("walk_amd_tlb() found L1 itlb {:?}", desc);
                out.0.push(desc);
            }
        }
    }

    #[bitfield(bits = 32)]
    #[derive(Debug)]
    struct L2TlbDesc {
        itlb_entries: B12,
        itlb_associativity: B4,
        dtlb_entries: B12,
        dtlb_associativity: B4,
    }

    if let Some(raw) = cpu.get_subleaf(0x8000_0006, 0) {
        let level = CacheLevel::L2;

        for register in vec![RegisterName::EBX, RegisterName::EAX] {
            let cacheflags = match register {
                RegisterName::EBX => CacheFlags::new().with_pages_4k(true),
                RegisterName::EAX => CacheFlags::new().with_pages_2m(true).with_pages_4m(true),
                _ => panic!("Invalid register name!"),
            };

            let regbytes = raw.output.register(register).to_le_bytes();
            let tlb = L2TlbDesc::from_bytes(regbytes);

            if tlb.dtlb_entries() > 0 {
                let desc = CacheDescription {
                    level: level.clone(),
                    cachetype: CacheType::DataTLB,
                    associativity: CacheAssociativity::from_identifier(
                        translate_amd_l2_associativity(tlb.dtlb_associativity()),
                    ),
                    size: tlb.dtlb_entries() as u32,
                    flags: cacheflags.clone(),
                    ..Default::default()
                };
                debug!("walk_amd_tlb() found L2 dtlb {:?}", desc);
                out.0.push(desc);
            }
            if tlb.itlb_entries() > 0 {
                let desc = CacheDescription {
                    level: level.clone(),
                    cachetype: CacheType::CodeTLB,
                    associativity: CacheAssociativity::from_identifier(
                        translate_amd_l2_associativity(tlb.itlb_associativity()),
                    ),
                    size: tlb.itlb_entries() as u32,
                    flags: cacheflags.clone(),
                    ..Default::default()
                };
                debug!("walk_amd_tlb() found L2 itlb {:?}", desc);
                out.0.push(desc);
            }
        }
    }

    if let Some(raw) = cpu.get_subleaf(0x8000_0019, 0) {
        for register in vec![RegisterName::EBX, RegisterName::EAX] {
            let level = match register {
                RegisterName::EAX => CacheLevel::L1,
                RegisterName::EBX => CacheLevel::L2,
                _ => panic!("Invalid register name!"),
            };

            let regbytes = raw.output.register(register).to_le_bytes();
            let tlb = L2TlbDesc::from_bytes(regbytes);

            if tlb.dtlb_entries() > 0 {
                let desc = CacheDescription {
                    level: level.clone(),
                    cachetype: CacheType::DataTLB,
                    associativity: CacheAssociativity::from_identifier(
                        translate_amd_l2_associativity(tlb.dtlb_associativity()),
                    ),
                    size: tlb.dtlb_entries() as u32,
                    flags: CacheFlags::new().with_pages_1g(true),
                    ..Default::default()
                };
                debug!("walk_amd_tlb() found 1G dtlb {:?}", desc);
                out.0.push(desc);
            }
            if tlb.itlb_entries() > 0 {
                let desc = CacheDescription {
                    level: level.clone(),
                    cachetype: CacheType::CodeTLB,
                    associativity: CacheAssociativity::from_identifier(
                        translate_amd_l2_associativity(tlb.itlb_associativity()),
                    ),
                    size: tlb.itlb_entries() as u32,
                    flags: CacheFlags::new().with_pages_1g(true),
                    ..Default::default()
                };
                debug!("walk_amd_tlb() found 1G itlb {:?}", desc);
                out.0.push(desc);
            }
        }
    }
}

fn walk_amd(system: &System, cpu: &Processor, out: &mut CacheVec) {
    walk_amd_cache(system, cpu, out);
    walk_amd_tlb(system, cpu, out);
}

fn walk_intel_dcp(system: &System, cpu: &Processor, out: &mut CacheVec) -> bool {
    #[bitfield(bits = 32)]
    #[derive(Debug)]
    struct EaxCache {
        cachetype: B5,
        level: B3,
        self_initializing: bool,
        fully_associative: bool,
        #[skip]
        __: B4,
        max_threads_sharing: B12,
        apics_reserved: B6,
    }

    #[bitfield(bits = 32)]
    #[derive(Debug)]
    struct EbxCache {
        linesize: B12,
        partitions: B10,
        associativity: B10,
    }

    #[bitfield(bits = 32)]
    #[derive(Debug)]
    struct EcxCache {
        sets: u32,
    }

    #[bitfield(bits = 32)]
    #[derive(Debug)]
    struct EdxCache {
        wbinvd: bool,
        inclusive: bool,
        complex_indexing: bool,
        #[skip]
        __: B29,
    }

    let mut retval: bool = false;

    let mut subleaf: u32 = 0;
    while let Some(raw) = cpu.get_subleaf(0x0000_0004, subleaf) {
        let eax = EaxCache::from_bytes(raw.output.eax.to_le_bytes());
        let ebx = EbxCache::from_bytes(raw.output.ebx.to_le_bytes());
        let ecx = EcxCache::from_bytes(raw.output.ecx.to_le_bytes());
        let edx = EdxCache::from_bytes(raw.output.edx.to_le_bytes());

        if eax.level() == 0 {
            break;
        }

        // Found at least one valid cache description, count this as a working
        // DCP leaf.
        retval = true;

        let mut associativity_type = CacheAssociativityType::NWay;
        if eax.fully_associative() {
            associativity_type = CacheAssociativityType::FullyAssociative;
        }
        if ebx.associativity() + 1 == 1 {
            associativity_type = CacheAssociativityType::DirectMapped;
        }

        let desc = CacheDescription {
            size: ((ebx.associativity() as u32 + 1)
                * (ebx.partitions() as u32 + 1)
                * (ebx.linesize() as u32 + 1)
                * (ecx.sets() as u32 + 1))
                / 1024,

            level: match eax.level() {
                1 => CacheLevel::L1,
                2 => CacheLevel::L2,
                3 => CacheLevel::L3,
                _ => CacheLevel::default(),
            },

            cachetype: match eax.cachetype() {
                1 => CacheType::Data,
                2 => CacheType::Code,
                3 => CacheType::Unified,
                _ => CacheType::Unknown,
            },

            associativity: CacheAssociativity {
                mapping: associativity_type,
                ways: ebx.associativity() + 1,
            },

            linesize: ebx.linesize() + 1,
            partitions: ebx.partitions() + 1,
            max_threads_sharing: eax.max_threads_sharing() + 1,

            flags: CacheFlags::new()
                .with_self_initializing(eax.self_initializing())
                .with_inclusive(edx.inclusive())
                .with_complex_indexing(edx.complex_indexing())
                .with_wbinvd_not_inclusive(edx.wbinvd()),

            instances: match system.cpu_count >= (eax.max_threads_sharing() + 1) as usize {
                true => system.cpu_count / (eax.max_threads_sharing() + 1) as usize,
                false => 1,
            },

            ..Default::default()
        };

        debug!("walk_intel_dcp() found cache {:?}", desc);

        out.0.push(desc);

        subleaf += 1;
    }

    retval
}

fn walk_intel_dat(system: &System, cpu: &Processor, out: &mut CacheVec) -> bool {
    #[bitfield(bits = 32)]
    #[derive(Debug)]
    struct EaxTLB {
        #[skip]
        __: B32,
    }

    #[bitfield(bits = 32)]
    #[derive(Debug)]
    struct EbxTLB {
        has_4k_pages: bool,
        has_2m_pages: bool,
        has_4m_pages: bool,
        has_1g_pages: bool,
        #[skip]
        __: B4,
        partitions: B3,
        #[skip]
        __: B5,
        associativity: u16,
    }

    #[bitfield(bits = 32)]
    #[derive(Debug)]
    struct EcxTLB {
        sets: u32,
    }

    #[bitfield(bits = 32)]
    #[derive(Debug)]
    struct EdxTLB {
        tlbtype: B5,
        level: B3,
        fully_associative: bool,
        #[skip]
        __: B5,
        max_threads_sharing: B12,
        #[skip]
        __: B6,
    }

    let mut retval: bool = false;

    let mut subleaf: u32 = 0;
    while let Some(raw) = cpu.get_subleaf(0x0000_0018, subleaf) {
        let _eax = EaxTLB::from_bytes(raw.output.eax.to_le_bytes());
        let ebx = EbxTLB::from_bytes(raw.output.ebx.to_le_bytes());
        let ecx = EcxTLB::from_bytes(raw.output.ecx.to_le_bytes());
        let edx = EdxTLB::from_bytes(raw.output.edx.to_le_bytes());

        if edx.tlbtype() != 0 {
            // Found at least one valid cache description, count this as a working
            // DCP leaf.
            retval = true;

            let desc = CacheDescription {
                size: ecx.sets(),

                level: match edx.level() {
                    0 => CacheLevel::L0,
                    1 => CacheLevel::L1,
                    2 => CacheLevel::L2,
                    3 => CacheLevel::L3,
                    _ => CacheLevel::default(),
                },

                cachetype: match edx.tlbtype() {
                    1 => CacheType::DataTLB,
                    2 => CacheType::CodeTLB,
                    3 => CacheType::SharedTLB,
                    4 => CacheType::LoadOnlyTLB,
                    5 => CacheType::StoreOnlyTLB,
                    _ => CacheType::Unknown,
                },

                associativity: CacheAssociativity {
                    mapping: match edx.fully_associative() {
                        true => CacheAssociativityType::FullyAssociative,
                        false => CacheAssociativityType::NWay,
                    },
                    ways: match edx.fully_associative() {
                        true => 0xFF,
                        false => ebx.associativity(),
                    },
                },

                partitions: ebx.partitions() as u16 + 1,
                max_threads_sharing: edx.max_threads_sharing() + 1,

                flags: CacheFlags::new()
                    .with_pages_4k(ebx.has_4k_pages())
                    .with_pages_2m(ebx.has_2m_pages())
                    .with_pages_4m(ebx.has_4m_pages())
                    .with_pages_1g(ebx.has_1g_pages()),

                instances: match system.cpu_count >= (edx.max_threads_sharing() + 1) as usize {
                    true => system.cpu_count / (edx.max_threads_sharing() + 1) as usize,
                    false => 1,
                },

                ..Default::default()
            };
            debug!("walk_intel_dat() found TLB {:?}", desc);
            out.0.push(desc);
        }

        subleaf += 1;
    }

    retval
}

#[cfg(feature = "legacy-cache-descriptors")]
fn walk_intel_legacy_cache(
    _system: &System,
    cpu: &Processor,
    out: &mut CacheVec,
    filter: &Vec<CacheType>,
) {
    if let Some(raw) = cpu.get_subleaf(0x0000_0002, 0) {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend_from_slice(&raw.output.eax.to_le_bytes());
        bytes.extend_from_slice(&raw.output.ebx.to_le_bytes());
        bytes.extend_from_slice(&raw.output.ecx.to_le_bytes());
        bytes.extend_from_slice(&raw.output.edx.to_le_bytes());
        bytes.sort_unstable();
        bytes.dedup();
        for descriptor in bytes.iter() {
            if *descriptor == 0x00 {
                // null cache descriptor, not worth logging
                continue;
            }
            if let Some(desc) = lookup_cache_descriptor(*descriptor) {
                if filter.contains(&desc.cachetype) {
                    debug!("walk_intel_legacy_cache() found {:?}", desc);
                    out.0.push(desc);
                }
            } else {
                // Handle the weird special cases that don't map to a single
                // cache type.
                match descriptor {
                    0x63 => {
                        if filter.contains(&CacheType::DataTLB) {
                            let mut entries = CacheVec::new();
                            entries.0.push(CacheDescription {
                                cachetype: CacheType::DataTLB,
                                size: 32,
                                flags: CacheFlags::new().with_pages_2m(true).with_pages_4m(true),
                                associativity: CacheAssociativity::from_identifier(0x04),
                                ..Default::default()
                            });
                            entries.0.push(CacheDescription {
                                cachetype: CacheType::DataTLB,
                                size: 4,
                                flags: CacheFlags::new().with_pages_1g(true),
                                associativity: CacheAssociativity::from_identifier(0x04),
                                ..Default::default()
                            });
                            debug!("walk_intel_legacy_cache() found {:?}", entries);
                            out.0.append(&mut entries.0);
                        }
                    }
                    0xB1 => {
                        if filter.contains(&CacheType::CodeTLB) {
                            let mut entries = CacheVec::new();
                            entries.0.push(CacheDescription {
                                cachetype: CacheType::CodeTLB,
                                size: 8,
                                flags: CacheFlags::new().with_pages_2m(true),
                                associativity: CacheAssociativity::from_identifier(0x04),
                                ..Default::default()
                            });
                            entries.0.push(CacheDescription {
                                cachetype: CacheType::CodeTLB,
                                size: 4,
                                flags: CacheFlags::new().with_pages_4m(true),
                                associativity: CacheAssociativity::from_identifier(0x04),
                                ..Default::default()
                            });
                            debug!("walk_intel_legacy_cache() found {:?}", entries);
                            out.0.append(&mut entries.0);
                        }
                    }
                    0xC3 => {
                        if filter.contains(&CacheType::SharedTLB) {
                            let mut entries = CacheVec::new();
                            entries.0.push(CacheDescription {
                                cachetype: CacheType::SharedTLB,
                                level: CacheLevel::L2,
                                size: 1536,
                                flags: CacheFlags::new().with_pages_4k(true).with_pages_2m(true),
                                associativity: CacheAssociativity::from_identifier(0x06),
                                ..Default::default()
                            });
                            entries.0.push(CacheDescription {
                                cachetype: CacheType::SharedTLB,
                                level: CacheLevel::L2,
                                size: 16,
                                flags: CacheFlags::new().with_pages_1g(true),
                                associativity: CacheAssociativity::from_identifier(0x04),
                                ..Default::default()
                            });
                            debug!("walk_intel_legacy_cache() found {:?}", entries);
                            out.0.append(&mut entries.0);
                        }
                    }
                    _ => {
                        debug!(
                            "walk_intel_legacy_cache() found unknown cache descriptor {:0>2x}",
                            descriptor
                        );
                    }
                }
            }
        }
    }
}

fn walk_intel_cache(system: &System, cpu: &Processor, out: &mut CacheVec) {
    if !walk_intel_dcp(system, cpu, out) {
        #[cfg(feature = "legacy-cache-descriptors")]
        {
            let cache_types: Vec<CacheType> = vec![
                CacheType::Code,
                CacheType::Data,
                CacheType::Unified,
                CacheType::Trace,
            ];
            walk_intel_legacy_cache(system, cpu, out, &cache_types);
        }
    }
}

fn walk_intel_tlb(system: &System, cpu: &Processor, out: &mut CacheVec) {
    if !walk_intel_dat(system, cpu, out) {
        #[cfg(feature = "legacy-cache-descriptors")]
        {
            let cache_types: Vec<CacheType> = vec![
                CacheType::CodeTLB,
                CacheType::DataTLB,
                CacheType::SharedTLB,
                CacheType::LoadOnlyTLB,
                CacheType::StoreOnlyTLB,
            ];
            walk_intel_legacy_cache(system, cpu, out, &cache_types);
        }
    }
}

fn walk_intel(system: &System, cpu: &Processor, out: &mut CacheVec) {
    walk_intel_cache(system, cpu, out);
    walk_intel_tlb(system, cpu, out);
}

pub(crate) fn describe_caches(system: &System, cpu: &Processor) -> CacheVec {
    let mut caches: CacheVec = CacheVec(vec![]);
    walk_amd(system, cpu, &mut caches);
    walk_intel(system, cpu, &mut caches);
    caches.0.sort();
    caches
}
