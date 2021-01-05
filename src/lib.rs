#![feature(asm)]
#[macro_use]

pub mod cpuid;
pub mod cache;

#[cfg(feature="legacy-cache-descriptors")]
mod cache_descriptors;
