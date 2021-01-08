#![allow(dead_code)]

use log::*;
use modular_bitfield::prelude::*;
use std::fmt;

use crate::cpuid::{Processor, System};

#[derive(Debug, Clone)]
pub struct TopologyProp {
    pub mask: u32,
    pub shift: u8,
    pub total: u16,
    pub reported: bool,
}

impl TopologyProp {
    pub fn new() -> TopologyProp {
        TopologyProp {
            mask: 0,
            shift: 0,
            total: 0,
            reported: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TopologyProps {
    pub socket: TopologyProp,
    pub core: TopologyProp,
    pub thread: TopologyProp,
}

impl TopologyProps {
    pub fn new() -> TopologyProps {
        TopologyProps {
            socket: TopologyProp::new(),
            core: TopologyProp::new(),
            thread: TopologyProp::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TopologyInferred {
    pub sockets: u32,
    pub cores_per_socket: u16,
    pub threads_per_core: u8,
}

impl TopologyInferred {
    pub fn new() -> TopologyInferred {
        TopologyInferred {
            sockets: 0,
            cores_per_socket: 0,
            threads_per_core: 0,
        }
    }

    pub fn valid(&self) -> bool {
        self.sockets != 0 && self.cores_per_socket != 0 && self.threads_per_core != 0
    }
}

impl fmt::Display for TopologyInferred {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} logical CPUs ({} sockets, {} cores per socket, {} threads per core)",
            self.sockets * self.cores_per_socket as u32 * self.threads_per_core as u32,
            self.sockets,
            self.cores_per_socket,
            self.threads_per_core
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct TopologyID {
    pub socket: u32,
    pub core: u32,
    pub thread: u32
}
impl TopologyID {
    pub fn new() -> TopologyID {
        TopologyID {
            socket: 0,
            core: 0,
            thread: 0,
        }
    }
}
impl fmt::Display for TopologyID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!( f, "socket {}, core {}, thread {}", self.socket, self.core, self.thread)
    }
}

fn describe_topology_cpu(state: &System, cpu: &Processor) -> Option<(TopologyProps, TopologyInferred)> {
    #[bitfield(bits = 32)]
    struct EaxX2Apic {
        shift: B5,
        #[skip]
        __: B27,
    }

    #[bitfield(bits = 32)]
    struct EbxX2Apic {
        count: B16,
        #[skip]
        __: B16,
    }

    #[bitfield(bits = 32)]
    struct EcxX2Apic {
        level: B8,
        leveltype: B8,
        #[skip]
        __: B16,
    }

    #[bitfield(bits = 32)]
    struct EdxX2Apic {
        x2apic_id: u32,
    }

    if let Some(feature_check) = cpu.get_subleaf(0x0000_000B, 0) {
        if feature_check.output.eax == 0 && feature_check.output.ebx == 0 {
            return None;
        }
    } else {
        return None;
    }

    let mut x2apic: TopologyProps = TopologyProps::new();

    x2apic.socket.reported = true;
    x2apic.socket.mask = 0xFFFF_FFFF;

    for leaf in cpu.get(0x0000_000B).iter() {
        debug!("Leaf {:x?}", leaf);
        if leaf.output.eax == 0 && leaf.output.ebx == 0 {
            continue;
        }
        let eax = EaxX2Apic::from_bytes(leaf.output.eax.to_le_bytes());
        let ebx = EbxX2Apic::from_bytes(leaf.output.ebx.to_le_bytes());
        let ecx = EcxX2Apic::from_bytes(leaf.output.ecx.to_le_bytes());
        let _edx = EdxX2Apic::from_bytes(leaf.output.edx.to_le_bytes());

        match ecx.leveltype() {
            // Thread level
            1 => {
                x2apic.thread.total = ebx.count();
                x2apic.thread.shift = eax.shift();
                x2apic.thread.mask = !(0xFFFF_FFFF << eax.shift());
                x2apic.thread.reported = true;
            }

            // Core level
            2 => {
                x2apic.core.total = ebx.count();
                x2apic.core.shift = eax.shift();
                x2apic.core.mask = !(0xFFFF_FFFF << eax.shift());
                x2apic.core.reported = true;

                x2apic.socket.shift = x2apic.core.shift;
                x2apic.socket.mask = 0xFFFF_FFFF ^ x2apic.core.mask;
            }

            _ => {
                break;
            }
        }
    }

    if x2apic.thread.reported && x2apic.core.reported {
        x2apic.core.mask ^= x2apic.thread.mask;
    } else if !x2apic.core.reported && x2apic.thread.reported {
        x2apic.core.mask = 0;
        x2apic.core.total = 1;
        x2apic.socket.shift = x2apic.thread.shift;
        x2apic.socket.mask = 0xFFFF_FFFF ^ x2apic.thread.mask;
    }

    x2apic.socket.shift = x2apic.socket.mask.trailing_zeros() as u8;
    x2apic.core.shift = x2apic.core.mask.trailing_zeros() as u8;
    x2apic.thread.shift = x2apic.thread.mask.trailing_zeros() as u8;

    debug!("Socket {:x?}", x2apic.socket);
    debug!("Core {:x?}", x2apic.core);
    debug!("Thread {:x?}", x2apic.thread);

    if x2apic.core.total == 0 || x2apic.thread.total == 0 {
        return None;
    }

    if x2apic.core.total > x2apic.thread.total {
        x2apic.core.total /= x2apic.thread.total;
    }

    let mut inferred: TopologyInferred = TopologyInferred::new();
    inferred.sockets = state.cpu_count as u32 / (x2apic.core.total as u32 * x2apic.thread.total as u32);
    inferred.cores_per_socket = x2apic.core.total;
    inferred.threads_per_core = x2apic.thread.total as u8;

    Some((x2apic, inferred))
}

pub(crate) fn describe_topology(system: &mut System) {
    if let Some((topo_props, topo)) = describe_topology_cpu(system, &system.cpus[0]) {
        system.topology = topo;
        system.topology_props = topo_props;
    }
}
