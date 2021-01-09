use cpuid::cache::{CacheAssociativityType, CacheFlags, CacheLevel, CacheType};
use cpuid::cpuid::{Signature, System, VendorMask};
use cpuid::topology::TopologyInferred;
use std::path::PathBuf;

fn dump_path(name: &str) -> String {
    let mut pathbuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    pathbuf.push("resources/test/dumps");
    pathbuf.push(name);
    pathbuf.as_path().to_str().unwrap().to_string()
}

#[test]
fn import_dump_cyrix() {
    {
        let import =
            System::from_file(&dump_path("CyrixInstead/CyrixInstead0000520_6x86_CPUID.txt")).unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 1);
        assert_eq!(import.vendor, VendorMask::CYRIX);
        assert_eq!(import.name_string, "");
        assert_eq!(import.features.0.len(), 3);
    }
    {
        let import = System::from_file(&dump_path("CyrixInstead/CyrixInstead0000601_MII_CPUID.txt")).unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 1);
        assert_eq!(import.vendor, VendorMask::CYRIX);
        assert_eq!(import.name_string, "");
        assert_eq!(import.features.0.len(), 8);
    }
}

#[test]
fn import_dump_transmeta() {
    {
        let import =
            System::from_file(&dump_path("GenuineTMx86/GenuineTMx860000543_Crusoe_CPUID.txt")).unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 1);
        assert_eq!(import.vendor, VendorMask::TRANSMETA);
        assert_eq!(import.name_string, "Transmeta(tm) Crusoe(tm) Processor TM5800");
        assert_eq!(import.caches.0.len(), 0);
        assert_eq!(import.features.0.len(), 11);
    }
    {
        let import =
            System::from_file(&dump_path("GenuineTMx86/GenuineTMx860000F24_Efficeon_CPUID.txt")).unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 1);
        assert_eq!(import.vendor, VendorMask::TRANSMETA);
        assert_eq!(import.name_string, "Transmeta Efficeon(tm) Processor TM8000");
        assert_eq!(import.caches.0.len(), 1);
        assert_eq!(import.features.0.len(), 25);
    }
}

#[test]
fn import_dump_sis() {
    {
        let import =
            System::from_file(&dump_path("SiS_SiS_SiS_/SiS SiS SiS 0000505_SiS550_CPUID.txt")).unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 1);
        assert_eq!(import.vendor, VendorMask::SIS);
        assert_eq!(import.name_string, "");
        assert_eq!(import.caches.0.len(), 0);
        assert_eq!(import.features.0.len(), 4);
        assert_eq!(
            import.cpus[0].signature,
            Signature {
                family: 0x5,
                model: 0x0,
                stepping: 0x5,
            }
        );
    }
}

#[test]
fn import_dump_rise() {
    {
        let import =
            System::from_file(&dump_path("RiseRiseRise/RiseRiseRise0000580_mP6II_CPUID.txt")).unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 1);
        assert_eq!(import.vendor, VendorMask::RISE);
        assert_eq!(import.name_string, "");
        assert_eq!(import.caches.0.len(), 0);
        assert_eq!(import.features.0.len(), 3);
        assert_eq!(
            import.cpus[0].signature,
            Signature {
                family: 0x5,
                model: 0x8,
                stepping: 0x0,
            }
        );
    }
}

#[test]
fn import_dump_virtualcpu() {
    {
        let import =
            System::from_file(&dump_path("Virtual_CPU_/Virtual CPU 0000F4A_Snap850_CPUID.txt")).unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 8);
        assert_eq!(import.vendor, VendorMask::VIRTUAL_CPU);
        assert_eq!(import.name_string, "Virtual CPU @ 2.74GHz");
        assert_eq!(import.caches.0.len(), 6);
        assert_eq!(import.features.0.len(), 17);
        assert_eq!(
            import.cpus[0].signature,
            Signature {
                family: 0xf,
                model: 0x4,
                stepping: 0xa,
            }
        );
    }
}

#[test]
fn import_dump_hygon() {
    {
        let import =
            System::from_file(&dump_path("HygonGenuine/HygonGenuine0900F02_Hygon_CPUID.txt")).unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 16);
        assert!(import.vendor.contains(VendorMask::AMD));
        assert_eq!(import.vendor, VendorMask::HYGON);
        assert_eq!(import.name_string, "Hygon C86 3185 8-core Processor");
        assert_eq!(import.caches.0.len(), 14);
        assert_eq!(import.features.0.len(), 113);
        assert_eq!(
            import.cpus[0].signature,
            Signature {
                family: 0x18,
                model: 0x0,
                stepping: 0x2,
            }
        );
    }
}

#[test]
fn import_dump_centaur() {
    {
        let import = System::from_file(&dump_path(
            "CentaurHauls/CentaurHauls0000663_C5A_Samuel_CPUID.txt",
        ))
        .unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 1);
        assert_eq!(import.vendor, VendorMask::CENTAUR);
        assert_eq!(import.name_string, "VIA Samuel");
        assert_eq!(import.features.0.len(), 9);
    }
    {
        let import =
            System::from_file(&dump_path("CentaurHauls/CentaurHauls000067A_C5C_Ezra_CPUID.txt")).unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 1);
        assert_eq!(import.vendor, VendorMask::CENTAUR);
        assert_eq!(import.name_string, "VIA Ezra");
        assert_eq!(import.features.0.len(), 8);
    }
    {
        let import = System::from_file(&dump_path(
            "CentaurHauls/CentaurHauls00006FE_CNR_Isaiah_CPUID3.txt",
        ))
        .unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 4);
        assert_eq!(import.vendor, VendorMask::CENTAUR);
        assert_eq!(import.name_string, "ZHAOXIN KaiXian ZX-C+ C4580@1.83GHz");
        assert_eq!(
            import.topology,
            TopologyInferred {
                sockets: 1,
                cores_per_socket: 4,
                threads_per_core: 1
            }
        );
        assert_eq!(import.caches.0.len(), 4);
        for cache in import.caches.0.iter() {
            match cache.cachetype {
                CacheType::Code | CacheType::Data => {
                    assert_eq!(cache.level, CacheLevel::L1);
                    assert_eq!(cache.instances, 4);
                }
                CacheType::Unified => {
                    assert_eq!(cache.level, CacheLevel::L2);
                    assert_eq!(cache.instances, 1);
                }
                CacheType::CodeTLB => {
                    assert_eq!(cache.flags, CacheFlags::new().with_pages_4k(true));
                }
                _ => panic!("unexpected cache type"),
            }
        }
    }
}

#[test]
fn import_dump_amd() {
    {
        let import = System::from_file(&dump_path("AuthenticAMD/AuthenticAMD0000500_K5_CPUID.txt")).unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 1);
        assert_eq!(import.vendor, VendorMask::AMD);
        assert_eq!(import.name_string, "");
        assert_eq!(import.features.0.len(), 9);
    }
    {
        let import = System::from_file(&dump_path("AuthenticAMD/AuthenticAMD0000570_K6_CPUID.txt")).unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 1);
        assert_eq!(import.vendor, VendorMask::AMD);
        assert_eq!(import.name_string, "AMD-K6tm w/ multimedia extensions");
        assert_eq!(import.features.0.len(), 9);
    }
    {
        let import = System::from_file(&dump_path(
            "AuthenticAMD/AuthenticAMD0000644_K7_Thunderbird_CPUID.txt",
        ))
        .unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 1);
        assert_eq!(import.vendor, VendorMask::AMD);
        assert_eq!(import.name_string, "AMD Athlon(tm) Processor");
        assert_eq!(import.features.0.len(), 20);
        assert_eq!(import.caches.0.len(), 9);

        // This isn't a comprehensive test of the cache data, just a spot check to make sure
        // relevant fields are properly populated.
        for cache in import.caches.0.iter() {
            match cache.cachetype {
                CacheType::Code | CacheType::Data => {
                    assert_eq!(cache.level, CacheLevel::L1);
                    assert_eq!(cache.size, 64);
                    // AMD doesn't expose this information in older processors.
                    assert_eq!(cache.instances, 0);
                }
                CacheType::Unified => {
                    assert_eq!(cache.level, CacheLevel::L2);
                    assert_eq!(cache.size, 256);
                    // AMD doesn't expose this information in older processors.
                    assert_eq!(cache.instances, 0);
                }
                CacheType::DataTLB => {
                    // Should always be 0 for TLBs
                    assert_eq!(cache.linesize, 0);
                    assert_eq!(cache.partitions, 0);

                    match cache.level {
                        CacheLevel::L1 => match cache.flags.pages_4k() {
                            true => {
                                assert_eq!(cache.size, 24);
                                assert_eq!(
                                    cache.associativity.mapping,
                                    CacheAssociativityType::FullyAssociative
                                );
                                assert_eq!(cache.associativity.ways, 0xFF);
                            }
                            false => {
                                assert_eq!(cache.size, 8);
                                assert_eq!(cache.associativity.mapping, CacheAssociativityType::NWay);
                                assert_eq!(cache.associativity.ways, 0x04);
                            }
                        },
                        CacheLevel::L2 => {}
                        _ => panic!("unexpected cache level"),
                    }
                }
                CacheType::CodeTLB => {}
                _ => panic!("unexpected cache type"),
            }
        }
    }
    {
        let import =
            System::from_file(&dump_path("AuthenticAMD/AuthenticAMD0830F10_K17_Rome_CPUID.txt")).unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 256);
        assert_eq!(import.caches.0.len(), 14);
        assert_eq!(import.vendor, VendorMask::AMD);
        assert_eq!(import.name_string, "AMD EPYC 7742 64-Core Processor");
        assert_eq!(
            import.cpus[0].signature,
            Signature {
                family: 0x17,
                model: 0x31,
                stepping: 0
            }
        );
        assert_eq!(
            import.topology,
            TopologyInferred {
                sockets: 2,
                cores_per_socket: 64,
                threads_per_core: 2
            }
        );
    }
}

#[test]
fn import_dump_intel() {
    {
        let import = System::from_file(&dump_path("GenuineIntel/GenuineIntel0000480_486_CPUID.txt")).unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 1);
        assert_eq!(import.vendor, VendorMask::INTEL);
        assert_eq!(import.name_string, "");
        assert_eq!(import.caches.0.len(), 0);
        assert_eq!(import.features.0.len(), 2);
        assert_eq!(
            import.cpus[0].signature,
            Signature {
                family: 0x4,
                model: 0x8,
                stepping: 0x0,
            }
        );
    }
    {
        let import = System::from_file(&dump_path("GenuineIntel/GenuineIntel0000517_P5_CPUID.txt")).unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 1);
        assert_eq!(import.vendor, VendorMask::INTEL);
        assert_eq!(import.name_string, "");
        assert_eq!(import.caches.0.len(), 0);
        assert_eq!(import.features.0.len(), 8);
    }
    {
        let import = System::from_file(&dump_path(
            "GenuineIntel/GenuineIntel0000633_P2_Klamath_CPUID.txt",
        ))
        .unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 1);
        assert_eq!(import.vendor, VendorMask::INTEL);
        assert_eq!(import.name_string, "");
        assert_eq!(import.caches.0.len(), 7);
        assert_eq!(import.features.0.len(), 16);
    }
    {
        let import = System::from_file(&dump_path(
            "GenuineIntel/GenuineIntel0000683_P3_Coppermine_CPUID.txt",
        ))
        .unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 2);
        assert_eq!(import.vendor, VendorMask::INTEL);
        assert_eq!(import.name_string, "");
        assert_eq!(import.caches.0.len(), 7);
        assert_eq!(import.features.0.len(), 21);
        assert_eq!(
            import.cpus[0].signature,
            Signature {
                family: 0x6,
                model: 0x8,
                stepping: 0x3,
            }
        );
        // TODO: Topology for multi-socket/multi-core without x2APIC?
    }
    {
        let import =
            System::from_file(&dump_path("GenuineIntel/GenuineIntel0000695_PM_Banias_CPUID.txt")).unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 1);
        assert_eq!(import.vendor, VendorMask::INTEL);
        assert_eq!(import.name_string, "Intel(R) Celeron(R) M processor 1300MHz");
        assert_eq!(import.caches.0.len(), 8);
        assert_eq!(import.features.0.len(), 24);
    }
    {
        let import = System::from_file(&dump_path(
            "GenuineIntel/GenuineIntel0000F0A_P4_Willamette_CPUID.txt",
        ))
        .unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 1);
        assert_eq!(import.vendor, VendorMask::INTEL);
        assert_eq!(import.name_string, "Intel(R) Pentium(R) 4 CPU 1700MHz");
        assert_eq!(import.caches.0.len(), 6);
        assert_eq!(import.features.0.len(), 27);
    }
    {
        let import = System::from_file(&dump_path(
            "GenuineIntel/GenuineIntel00106A2_Nehalem-EP_CPUID_2.txt",
        ))
        .unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 16);
        assert_eq!(import.vendor, VendorMask::INTEL);
        assert_eq!(import.name_string, "Genuine Intel(R) CPU @ 0000 @ 2.67GHz");
        assert_eq!(import.caches.0.len(), 10);
        assert_eq!(import.features.0.len(), 50);
        assert_eq!(
            import.cpus[0].signature,
            Signature {
                family: 0x6,
                model: 0x1a,
                stepping: 0x2,
            }
        );
        assert_eq!(
            import.topology,
            TopologyInferred {
                sockets: 2,
                cores_per_socket: 4,
                threads_per_core: 2
            }
        );
    }
    {
        let import = System::from_file(&dump_path(
            "GenuineIntel/GenuineIntel00806C1_TigerLake_CPUID3.txt",
        ))
        .unwrap();
        assert_eq!(import.cpu_count, import.cpus.len());
        assert_eq!(import.cpu_count, 8);
        assert_eq!(import.vendor, VendorMask::INTEL);
        assert_eq!(
            import.name_string,
            "11th Gen Intel(R) Core(TM) i7-1165G7 @ 2.80GHz"
        );
        assert_eq!(import.caches.0.len(), 12);
        assert!(import.features.0.len() >= 138);
    }
}

#[test]
fn import_dump_localsystem() {
    let import = System::from_local();
    assert_eq!(import.cpu_count, import.cpus.len());
    // TODO: more tests here, probably by getting information from other sources and
    // cross-referencing.
}
