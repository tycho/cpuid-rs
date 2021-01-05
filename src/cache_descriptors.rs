use crate::cache::{CacheAssociativity, CacheDescription, CacheFlags, CacheLevel, CacheType};

pub fn lookup_cache_descriptor(descriptor: u8) -> Option<CacheDescription> {
    match descriptor {
        0x01 => Some(CacheDescription {
            cachetype: CacheType::CodeTLB,
            size: 32,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x02 => Some(CacheDescription {
            cachetype: CacheType::CodeTLB,
            size: 2,
            flags: CacheFlags::new().with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0xFF),
            ..Default::default()
        }),
        0x03 => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            size: 64,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x04 => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            size: 8,
            flags: CacheFlags::new().with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x05 => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            size: 32,
            flags: CacheFlags::new().with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x06 => Some(CacheDescription {
            cachetype: CacheType::Code,
            level: CacheLevel::L1,
            size: 8,
            linesize: 32,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x08 => Some(CacheDescription {
            cachetype: CacheType::Code,
            level: CacheLevel::L1,
            size: 16,
            linesize: 32,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x09 => Some(CacheDescription {
            cachetype: CacheType::Code,
            level: CacheLevel::L1,
            size: 32,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x0A => Some(CacheDescription {
            cachetype: CacheType::Data,
            level: CacheLevel::L1,
            size: 8,
            linesize: 32,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x02),
            ..Default::default()
        }),
        0x0B => Some(CacheDescription {
            cachetype: CacheType::CodeTLB,
            level: CacheLevel::L1,
            size: 4,
            flags: CacheFlags::new().with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x0C => Some(CacheDescription {
            cachetype: CacheType::Data,
            level: CacheLevel::L1,
            size: 16,
            linesize: 32,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x0D => Some(CacheDescription {
            cachetype: CacheType::Data,
            level: CacheLevel::L1,
            size: 16,
            linesize: 64,
            flags: CacheFlags::new().with_ecc(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x0E => Some(CacheDescription {
            cachetype: CacheType::Data,
            level: CacheLevel::L1,
            size: 24,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x06),
            ..Default::default()
        }),
        0x10 => Some(CacheDescription {
            cachetype: CacheType::Data,
            level: CacheLevel::L1,
            size: 16,
            linesize: 32,
            flags: CacheFlags::new().with_ia64(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x15 => Some(CacheDescription {
            cachetype: CacheType::Code,
            level: CacheLevel::L1,
            size: 16,
            linesize: 32,
            flags: CacheFlags::new().with_ia64(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x1A => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 96,
            linesize: 64,
            flags: CacheFlags::new().with_ia64(true),
            associativity: CacheAssociativity::from_identifier(0x06),
            ..Default::default()
        }),
        0x1D => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 128,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x02),
            ..Default::default()
        }),
        0x21 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 256,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x22 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 512,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x23 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 1024,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x24 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 1024,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x10),
            ..Default::default()
        }),
        0x25 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 2048,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x29 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 4096,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x2C => Some(CacheDescription {
            cachetype: CacheType::Data,
            level: CacheLevel::L1,
            size: 32,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x30 => Some(CacheDescription {
            cachetype: CacheType::Code,
            level: CacheLevel::L1,
            size: 32,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x39 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 128,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x3A => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 192,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x06),
            ..Default::default()
        }),
        0x3B => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 128,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x02),
            ..Default::default()
        }),
        0x3C => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 256,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x3D => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 384,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x06),
            ..Default::default()
        }),
        0x3E => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 512,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        // 0x40 -> handled in parent function, special case
        0x41 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 128,
            linesize: 32,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x42 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 256,
            linesize: 32,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x43 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 512,
            linesize: 32,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x44 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 1024,
            linesize: 32,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x45 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 2048,
            linesize: 32,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x46 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 4096,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x47 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 8192,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x48 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 3072,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x0C),
            ..Default::default()
        }),
        0x4A => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 6144,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x0C),
            ..Default::default()
        }),
        0x4B => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 8192,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x10),
            ..Default::default()
        }),
        0x4C => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 12288,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x0C),
            ..Default::default()
        }),
        0x4D => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 16384,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x10),
            ..Default::default()
        }),
        0x4E => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 6144,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x18),
            ..Default::default()
        }),
        0x4F => Some(CacheDescription {
            cachetype: CacheType::CodeTLB,
            size: 32,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0x00),
            ..Default::default()
        }),
        0x50 => Some(CacheDescription {
            cachetype: CacheType::CodeTLB,
            size: 64,
            flags: CacheFlags::new()
                .with_pages_4k(true)
                .with_pages_2m(true)
                .with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0x00),
            ..Default::default()
        }),
        0x51 => Some(CacheDescription {
            cachetype: CacheType::CodeTLB,
            size: 128,
            flags: CacheFlags::new()
                .with_pages_4k(true)
                .with_pages_2m(true)
                .with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0x00),
            ..Default::default()
        }),
        0x52 => Some(CacheDescription {
            cachetype: CacheType::CodeTLB,
            size: 256,
            flags: CacheFlags::new()
                .with_pages_4k(true)
                .with_pages_2m(true)
                .with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0x00),
            ..Default::default()
        }),
        0x55 => Some(CacheDescription {
            cachetype: CacheType::CodeTLB,
            size: 256,
            flags: CacheFlags::new().with_pages_2m(true).with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0xFF),
            ..Default::default()
        }),
        0x56 => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            level: CacheLevel::L0,
            size: 16,
            flags: CacheFlags::new().with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x57 => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            level: CacheLevel::L0,
            size: 16,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x59 => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            level: CacheLevel::L0,
            size: 16,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0xFF),
            ..Default::default()
        }),
        0x5A => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            size: 32,
            flags: CacheFlags::new().with_pages_2m(true).with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x5B => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            size: 64,
            flags: CacheFlags::new().with_pages_4k(true).with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0xFF),
            ..Default::default()
        }),
        0x5C => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            size: 128,
            flags: CacheFlags::new().with_pages_4k(true).with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0xFF),
            ..Default::default()
        }),
        0x5D => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            size: 256,
            flags: CacheFlags::new().with_pages_4k(true).with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0xFF),
            ..Default::default()
        }),
        0x60 => Some(CacheDescription {
            cachetype: CacheType::Data,
            level: CacheLevel::L1,
            size: 16,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x61 => Some(CacheDescription {
            cachetype: CacheType::CodeTLB,
            size: 48,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0xFF),
            ..Default::default()
        }),
        // 0x63 -> two different entries simultaneously, handled in parent function
        0x64 => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            size: 512,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x66 => Some(CacheDescription {
            cachetype: CacheType::Data,
            level: CacheLevel::L1,
            size: 8,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x67 => Some(CacheDescription {
            cachetype: CacheType::Data,
            level: CacheLevel::L1,
            size: 16,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x68 => Some(CacheDescription {
            cachetype: CacheType::Data,
            level: CacheLevel::L1,
            size: 32,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x6A => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            level: CacheLevel::L0,
            size: 64,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x6B => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            size: 256,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x6C => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            size: 128,
            flags: CacheFlags::new().with_pages_2m(true).with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x6D => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            size: 16,
            flags: CacheFlags::new().with_pages_1g(true),
            associativity: CacheAssociativity::from_identifier(0xFF),
            ..Default::default()
        }),
        0x70 => Some(CacheDescription {
            cachetype: CacheType::Trace,
            level: CacheLevel::L1,
            size: 12,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x71 => Some(CacheDescription {
            cachetype: CacheType::Trace,
            level: CacheLevel::L1,
            size: 16,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x72 => Some(CacheDescription {
            cachetype: CacheType::Trace,
            level: CacheLevel::L1,
            size: 32,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x73 => Some(CacheDescription {
            cachetype: CacheType::Trace,
            level: CacheLevel::L1,
            size: 64,
            flags: CacheFlags::new().with_undocumented(true),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x76 => Some(CacheDescription {
            cachetype: CacheType::CodeTLB,
            size: 8,
            flags: CacheFlags::new().with_pages_2m(true).with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0xFF),
            ..Default::default()
        }),
        0x77 => Some(CacheDescription {
            cachetype: CacheType::Code,
            level: CacheLevel::L1,
            size: 16,
            linesize: 64,
            flags: CacheFlags::new().with_ia64(true).with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x78 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 1024,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x79 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 128,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x7A => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 256,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x7B => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 512,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x7C => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 1024,
            linesize: 64,
            flags: CacheFlags::new().with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x7D => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 2048,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x7E => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 256,
            linesize: 128,
            flags: CacheFlags::new().with_ia64(true).with_sectored(true),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x7F => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 512,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x02),
            ..Default::default()
        }),
        0x80 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 512,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x81 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 128,
            linesize: 32,
            flags: CacheFlags::new().with_undocumented(true),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x82 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 256,
            linesize: 32,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x83 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 512,
            linesize: 32,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x84 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 1024,
            linesize: 32,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x85 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 2048,
            linesize: 32,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x86 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 512,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x87 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L2,
            size: 1024,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0x88 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 2048,
            linesize: 64,
            flags: CacheFlags::new().with_ia64(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x89 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 4096,
            linesize: 64,
            flags: CacheFlags::new().with_ia64(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x8A => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 8192,
            linesize: 64,
            flags: CacheFlags::new().with_ia64(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0x8D => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 3072,
            linesize: 128,
            flags: CacheFlags::new().with_ia64(true),
            associativity: CacheAssociativity::from_identifier(0x0C),
            ..Default::default()
        }),
        0xA0 => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            size: 32,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0xFF),
            ..Default::default()
        }),
        0xB0 => Some(CacheDescription {
            cachetype: CacheType::CodeTLB,
            size: 128,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        // 0xB1 -> two entries, special case. handled in parent function
        0xB2 => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            size: 64,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0xB3 => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            size: 128,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0xB4 => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            level: CacheLevel::L1,
            size: 256,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0xB5 => Some(CacheDescription {
            cachetype: CacheType::CodeTLB,
            size: 64,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0xB6 => Some(CacheDescription {
            cachetype: CacheType::CodeTLB,
            size: 128,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0xBA => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            level: CacheLevel::L1,
            size: 64,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0xC0 => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            size: 8,
            flags: CacheFlags::new().with_pages_4k(true).with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0xC1 => Some(CacheDescription {
            cachetype: CacheType::SharedTLB,
            level: CacheLevel::L2,
            size: 1024,
            flags: CacheFlags::new().with_pages_4k(true).with_pages_2m(true),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0xC2 => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            size: 16,
            flags: CacheFlags::new().with_pages_2m(true).with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        // 0xC3 -> two entries. special case handled by parent function.
        0xC4 => Some(CacheDescription {
            cachetype: CacheType::DataTLB,
            size: 32,
            flags: CacheFlags::new().with_pages_2m(true).with_pages_4m(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0xCA => Some(CacheDescription {
            cachetype: CacheType::SharedTLB,
            level: CacheLevel::L2,
            size: 512,
            flags: CacheFlags::new().with_pages_4k(true),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0xD0 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 512,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0xD1 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 1024,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0xD2 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 2048,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x04),
            ..Default::default()
        }),
        0xD6 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 1024,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0xD7 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 2048,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0xD8 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 4096,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x08),
            ..Default::default()
        }),
        0xDC => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 1536,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x0C),
            ..Default::default()
        }),
        0xDD => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 3072,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x0C),
            ..Default::default()
        }),
        0xDE => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 6144,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x0C),
            ..Default::default()
        }),
        0xE2 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 2048,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x10),
            ..Default::default()
        }),
        0xE3 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 4096,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x10),
            ..Default::default()
        }),
        0xE4 => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 8192,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x10),
            ..Default::default()
        }),
        0xEA => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 12288,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x18),
            ..Default::default()
        }),
        0xEB => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 18432,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x18),
            ..Default::default()
        }),
        0xEC => Some(CacheDescription {
            cachetype: CacheType::Unified,
            level: CacheLevel::L3,
            size: 24576,
            linesize: 64,
            flags: CacheFlags::new(),
            associativity: CacheAssociativity::from_identifier(0x18),
            ..Default::default()
        }),
        _ => None,
    }
}
