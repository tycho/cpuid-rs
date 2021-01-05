#![allow(dead_code, unused_attributes)]

use modular_bitfield::prelude::*;

use crate::cpuid::{CPUIDSnapshot, RegisterName};

#[derive(Debug)]
#[repr(u8)]
pub enum CacheType {
    Unknown = 0,
    Data,
    Code,
    Unified,
    Trace,
    DataTLB,
    CodeTLB,
    LoadOnlyTLB,
    StoreOnlyTLB,
}
impl Default for CacheType {
    fn default() -> CacheType {
        CacheType::Unknown
    }
}

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum CacheLevel {
    Unknown = 0,
    L0,
    L1,
    L2,
    L3,
    L4,
}
impl Default for CacheLevel {
    fn default() -> CacheLevel {
        CacheLevel::Unknown
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum CacheAssociativityType {
    Unknown = 0,
    DirectMapped = 1,
    NWay = 2,
    FullyAssociative = 3,
}
impl Default for CacheAssociativityType {
    fn default() -> CacheAssociativityType {
        CacheAssociativityType::Unknown
    }
}

#[derive(Debug, Default)]
pub struct CacheAssociativity {
    pub mapping: CacheAssociativityType,
    pub ways: u16,
}

impl CacheAssociativity {
    fn from_identifier(id: u8) -> CacheAssociativity {
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

#[bitfield(bits = 16)]
#[derive(Debug, Clone, Default)]
pub struct CacheFlags {
    pub undocumented: bool,
    pub ia64: bool,
    pub ecc: bool,
    pub sectored: bool,
    pub pages_4k: bool,
    pub pages_2m: bool,
    pub pages_4m: bool,
    pub pages_1g: bool,
    pub self_initializing: bool,
    pub complex_indexing: bool,
    pub inclusive: bool,
    pub wbinvd_not_inclusive: bool,
    #[skip]
    __: B4,
}

#[derive(Debug, Default)]
pub struct CacheDescription {
    pub level: CacheLevel,
    pub cachetype: CacheType,
    pub size: u32,
    pub linesize: u16,
    pub flags: CacheFlags,
    pub associativity: CacheAssociativity,
    pub partitions: u16,
    pub max_threads_sharing: u16,
}

fn walk_amd_cache_extended(cpuid: &CPUIDSnapshot, out: &mut Vec<CacheDescription>) -> bool {
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

    if !cpuid.has_feature_bit(0x8000_0001, 0, RegisterName::ECX, 22) {
        return false;
    }

    let mut subleaf: u32 = 0;
    while let Some(raw) = cpuid.get_subleaf(0x8000_001D, subleaf) {
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

        out.push(desc);

        subleaf += 1;
    }

    true
}

fn walk_amd_cache_legacy(_cpuid: &CPUIDSnapshot, _out: &mut Vec<CacheDescription>) {
    // TODO
}

fn walk_amd_cache(cpuid: &CPUIDSnapshot, out: &mut Vec<CacheDescription>) {
    // We want to prefer, in order:
    //
    // - Extended Cache Topology (0x8000_001D)
    // - L1/L2 cache features (0x8000_0005 and 0x8000_0006)
    //
    // The latter is less expressive in terms of cache details, so we should try
    // to use Extended Cache Topology wherever possible.
    if !walk_amd_cache_extended(cpuid, out) {
        walk_amd_cache_legacy(cpuid, out);
    }
}

fn walk_amd_tlb(cpuid: &CPUIDSnapshot, out: &mut Vec<CacheDescription>) {
    // Read from:
    // AMD L1 cache features (0x8000_0005)
    // AMD L2 cache features (0x8000_0006)

    #[bitfield(bits = 32)]
    #[derive(Debug)]
    struct L1TlbDesc {
        itlb_entries: u8,
        itlb_associativity: u8,
        dtlb_entries: u8,
        dtlb_associativity: u8,
    }

    if let Some(raw) = cpuid.get_subleaf(0x8000_0005, 0) {
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
                out.push(CacheDescription {
                    level: level.clone(),
                    cachetype: CacheType::DataTLB,
                    associativity: CacheAssociativity::from_identifier(tlb.dtlb_associativity()),
                    size: tlb.dtlb_entries() as u32,
                    flags: cacheflags.clone(),
                    ..Default::default()
                });
            }
            if tlb.itlb_entries() > 0 {
                out.push(CacheDescription {
                    level: level.clone(),
                    cachetype: CacheType::CodeTLB,
                    associativity: CacheAssociativity::from_identifier(tlb.itlb_associativity()),
                    size: tlb.itlb_entries() as u32,
                    flags: cacheflags.clone(),
                    ..Default::default()
                });
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

    if let Some(raw) = cpuid.get_subleaf(0x8000_0006, 0) {
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
                out.push(CacheDescription {
                    level: level.clone(),
                    cachetype: CacheType::DataTLB,
                    associativity: match tlb.dtlb_associativity() {
                        0x4 => CacheAssociativity {
                            mapping: CacheAssociativityType::NWay,
                            ways: 4,
                        },
                        0x6 => CacheAssociativity {
                            mapping: CacheAssociativityType::NWay,
                            ways: 8,
                        },
                        _ => CacheAssociativity::default(),
                    },
                    size: tlb.dtlb_entries() as u32,
                    flags: cacheflags.clone(),
                    ..Default::default()
                });
            }
            if tlb.itlb_entries() > 0 {
                out.push(CacheDescription {
                    level: level.clone(),
                    cachetype: CacheType::CodeTLB,
                    associativity: match tlb.dtlb_associativity() {
                        0x4 => CacheAssociativity {
                            mapping: CacheAssociativityType::NWay,
                            ways: 4,
                        },
                        0x6 => CacheAssociativity {
                            mapping: CacheAssociativityType::NWay,
                            ways: 8,
                        },
                        _ => CacheAssociativity::default(),
                    },
                    size: tlb.itlb_entries() as u32,
                    flags: cacheflags.clone(),
                    ..Default::default()
                });
            }
        }
    }
}

fn walk_amd(cpuid: &CPUIDSnapshot, out: &mut Vec<CacheDescription>) {
    walk_amd_cache(cpuid, out);
    walk_amd_tlb(cpuid, out);
}

fn walk_dcp() {
    // TODO
}

pub fn walk(cpuid: &CPUIDSnapshot) -> Vec<CacheDescription> {
    let mut caches: Vec<CacheDescription> = vec![];
    walk_amd(cpuid, &mut caches);
    caches
}
