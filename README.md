## About
I want to eventually move this beyond "proof of concept", but my primary goal
of this whole project is just to learn how to write Rust code. I tried writing
another project before and found it was a bit too ambitious given my lack of
knowledge about Rust data structures, argument passing conventions, ownership,
etc. Porting my CPUID tooling to Rust feels a lot more achievable.

## Current state
As of this writing, this is only a standalone tool called "dump" which will
print out all known CPUID leaves on each CPU (including the weird indexed
leaves).

## Future work
* Create a library and maintainable API for calling CPUID, most likely with tiered
  complexity depending on level of information needed
	- at the lowest level, just an exported function to execute the CPUID
    instruction directly with specific EAX/ECX
	- at higher levels, exported APIs for:
      + enumerating/walking all CPUID leaves (especially for use by "dump" tool)
      + identifying specific CPU feature support
      + structured data about the CPU and its caches, TLBs, features,
      topology, etc.
      
* There are numerous leaves which are redundant, and some are better sources of
  information than others. It would be nice to account for these in any
  exported API. E.g. on Intel, leaf 0x2 cache descriptors are obsoleted by
  leaf 0x4 deterministic cache parameters and leaf 0x18 deterministic address
  translation parameters. On AMD, leaves for L1/L2/L3 cache data are
  obsoleted by the more expressive AMD Extended Cache Topology leaf.

* Make "dump" use the new library, walk all the leaves and print the results.

* Make a new "decode" tool which prints decoded CPUID leaves in
  human-readable and machine-readable formats. Should have features similar to
  my C version of CPUID (ability to dump/decode, reading/decoding dump files,
  dumping in multiple formats, etc)
