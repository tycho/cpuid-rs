// Depends on rust nightly
//#![feature(asm)]
#[macro_use]

pub mod cpuid;
pub mod cache;
pub mod feature;
pub mod topology;

#[cfg(feature = "legacy-cache-descriptors")]
mod cache_descriptors;
