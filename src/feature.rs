use log::*;
use std::fmt;
use textwrap::indent;

use crate::cpuid::{LeafID, Processor, RegisterName, VendorMask};
use crate::internal::feature_flags::{FeatureLeaf, FeatureSpec, FEATURE_LEAVES};

#[derive(Debug, Clone)]
/// Describes a discovered CPU feature.
pub struct Feature {
    /// Leaf this feature flag was discovered in.
    pub leaf: LeafID,

    /// Register this feature flag was discovered in.
    pub register: RegisterName,

    /// Bit index for this feature in the leaf/register this feature was discovered in.
    pub bit: u8,

    /// Mask of valid vendors this feature can be detected in.
    pub vendor_mask: VendorMask,

    /// Short name of the feature. May be blank if the feature doesn't have/need
    /// a shorter name or initialism.
    pub shortname: &'static str,

    /// Longer, more descriptive name of the feature.
    pub name: &'static str,
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

    pub fn leaf_name(&self) -> &'static str {
        leaf_name(&self.leaf, self.register)
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
/// Vector of [Feature](struct.Feature.html) objects.
pub struct FeatureVec(pub Vec<Feature>);

impl FeatureVec {
    pub fn new() -> FeatureVec {
        FeatureVec(vec![])
    }
}

fn leaf_name(leaf: &LeafID, register: RegisterName) -> &'static str {
    match leaf.eax {
        0x0000_0001 | 0x8000_0001 => "Feature Identifiers",
        0x0000_0006 => "Thermal and Power Management",
        0x0000_0007 => "Structured Extended Feature Identifiers",
        0x0000_0014 => "Intel Processor Trace Enumeration",
        0x8000_0007 => match register {
            RegisterName::EBX => "RAS Capabilities",
            RegisterName::EDX => "Advanced Power Management Information",
            _ => "",
        },
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
        let mut lastleaf: LeafID = LeafID {
            eax: 0xFFFF_FFFF,
            ecx: 0xFFFF_FFFF,
        };
        let mut lastreg: RegisterName = RegisterName::Unknown;
        for v in &self.0 {
            if v.leaf != lastleaf || v.register != lastreg {
                if lastreg != RegisterName::Unknown {
                    write!(f, "\n")?;
                }
                let mut name = leaf_name(&v.leaf, v.register).to_string();
                if name.len() > 0 {
                    name = format!(" ({})", name.to_string());
                }
                write!(
                    f,
                    "  Leaf {:08x}:{:02x}{}, register {:?}\n",
                    v.leaf.eax, v.leaf.ecx, name, v.register
                )?;
                lastleaf = v.leaf.clone();
                lastreg = v.register.clone();
            }
            let formatted = format!("{}\n", v);
            write!(f, "{}", indent(&formatted, "    "))?;
        }
        Ok(())
    }
}

pub(crate) fn describe_features(cpu: &Processor, vendor_mask: VendorMask) -> FeatureVec {
    let mut output: FeatureVec = FeatureVec::new();
    for feature_leaf in FEATURE_LEAVES.iter() {
        if !vendor_mask.intersects(feature_leaf.vendor_mask) {
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
                let bit = feature_spec.bit;
                if vendor_mask.intersects(feature_spec.vendor_mask) {
                    let mask = 1 << bit;
                    if (register & mask) != 0 {
                        // Mark that we've seen and accounted for this feature
                        // bit. We can report on unaccounted for bits afterward
                        // (in debug)
                        register &= !mask;
                        let feature = Feature::from_detection(feature_leaf, feature_spec, bit as u8);
                        output.0.push(feature);
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
