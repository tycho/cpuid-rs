[![License: ISC](https://img.shields.io/badge/License-ISC-blue.svg)](https://opensource.org/licenses/ISC)

## About
This is an incomplete Rust port of my [CPUID tool written in
C](https://github.com/tycho/cpuid). My primary goal of this project is just to
learn to write Rust and to come up with a decent library for decoding CPUID
information.

The project is currently has three components:

- `cpuid` library/crate, with ability to import CPUID data from a text file or
  from CPUs on the local system
- `decode` binary, which prints human-readable decoding of the more
  interesting features exposed in CPUID
- `dump` binary, which can create text-based dump files of all known CPUID
  leaves on the local system. Dumps can be imported with the library or with
  the `decode` binary using the `-f` argument.

## Building
Install rust (ideally via `rustup`, which is what I use), and then do a `cargo
build` in the project root and test out the `target/debug/dump` and
`target/debug/decode` binaries.

## Current State
The library (and `dump` binary) can dump all known valid CPUID leaves, even
those with weird subleaf indexing (i.e. nonzero input `ecx` values).

The library and decode tools can decode some of the more interesting features,
including:

- Feature flags from various leaves:
  + Leaf `0x0000_0001`, feature identifiers
  + Leaf `0x8000_0001`, extended feature identifiers
  + Leaf `0x0000_0006`, thermal and power management
  + Leaf `0x0000_0007`, structured extended feature identifiers
  + Leaf `0x0000_0014`, Intel processor trace enumeration
  + Leaf `0x8000_0007`, RAS and advanced power management information
  + Leaf `0x8000_0008`, extended feature extensions ID
  + Leaf `0x8000_000A`, SVM feature identifiers
  + Leaf `0x8000_001A`, performance optimization identifiers
  + Leaf `0x8000_001B`, instruction based sampling identifiers
  + Leaf `0xC000_0001`, Centaur feature identifiers

- Cache descriptors from multiple different leaves, with preference for the
  most detailed sources:
  + Leaf `0x0000_0004`, Deterministic Cache Parameters. Intel-specific leaf.
  + Leaf `0x0000_0018`, Deterministic Address Translation Parameters.
	Intel-specific leaf.
  + Leaf `0x8000_001D`, Cache Topology Information. AMD-specific leaf, similar
	in detail to the Intel-specific "deterministic" leaves above.
  + Leaf `0x8000_0005`, L1 TLB and L1 cache information. AMD-specific leaf.
  + Leaf `0x8000_0006`, L2 TLB and L2/L3 cache information. AMD-specific leaf.
  + Leaf `0x8000_0019`, 1GB page TLB information. AMD-specific leaf.
  + Leaf `0x0000_0002`, Cache and TLB information. This leaf has the lowest
	priority over the others, because it's the least detailed and Intel has
	deprecated its use in favor of the "Deterministic" leaves above. It also
	requires maintaining an ugly mapping of magic byte identifiers to
	structured information about the cache features.

- CPU topology from leaf `0x0000_000B` (x2APIC)

## Future Work
* Implement more CPUID leaves in a library-friendly way. I don't want the
  interface to become clunky, disorganized, or filled with redundant
  information.
* Consider different API designs. I cobbled this together and I'm not quite
  happy with the API the crate currently exposes.
* Consider how to implement feature detection slightly differently --
  I currently just generate a big vector of all the features it discovers, but
  it's not exactly searchable. The design really reflects how I use it right
  now, which is to just iterate over all of the features and print them out.
* Investigate and fix any bad practices I've used in my Rust code. I am
  a complete Rust newbie, so I have no doubt done some stupid things.
* Release a crate on crates.io at some point, once the design isn't shifting
  as much.
* Implement support for reading multiple different formats from CPUID dump
  files. For example, Todd Allen's CPUID tool, the dumps from InstLatx64, etc.
* Keep dependencies as minimal as possible. Early on I tried using a crate
  called `clap` for command-line parsing. It's a wonderful and powerful tool,
  but it also added like 500KB to the binary sizes. I didn't really need all
  the flexibility it provided, so I ended up migrating to the much smaller and
  less capable `getopt` crate.
* Implement a test/benchmark suite
* Keep the `rustdoc` information up to date and readable.
