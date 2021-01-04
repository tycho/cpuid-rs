pub struct Registers {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

impl Registers {
    pub fn new(eax: u32, ebx: u32, ecx: u32, edx: u32) -> Registers {
        Registers {
            eax: eax,
            ebx: ebx,
            ecx: ecx,
            edx: edx,
        }
    }

    /// Try to create an ASCII representation of the bytes in the registers. Uses '.' as
    /// a placeholder for invalid ASCII values.
    pub fn ascii(&self) -> String {
        let mut string = String::new();
        for register in [self.eax, self.ebx, self.ecx, self.edx].iter() {
            for byte in register.to_le_bytes().iter() {
                if *byte > 31 && *byte < 127 {
                    string.push(*byte as char)
                } else {
                    string.push('.')
                }
            }
        }
        string
    }
}

pub fn cpuid(input: &Registers) -> Registers {
    let mut eax: u32;
    let mut ebx: u32;
    let mut ecx: u32;
    let mut edx: u32;

    unsafe {
        asm!("cpuid",
            inout("eax") input.eax => eax,
            lateout("ebx") ebx,
            inout("ecx") input.ecx => ecx,
            lateout("edx") edx)
    }

    Registers::new(eax, ebx, ecx, edx)
}
