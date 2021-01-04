use std::fmt;

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

pub fn cpuid(input: &Registers, output: &mut Registers) {
    unsafe {
        asm!("cpuid",
            inout("eax") input.eax => output.eax,
            lateout("ebx") output.ebx,
            inout("ecx") input.ecx => output.ecx,
            lateout("edx") output.edx)
    }
}

pub struct CPUID {
    pub input: Registers,
    pub output: Registers,
}

impl CPUID {
    pub fn new() -> CPUID {
        CPUID {
            input: Registers::new(0, 0, 0, 0),
            output: Registers::new(0, 0, 0, 0),
        }
    }
    pub fn invoke(eax: u32, ecx: u32) -> CPUID {
        let input = Registers::new(eax, 0, ecx, 0);
        let mut output = Registers::new(0, 0, 0, 0);
        cpuid(&input, &mut output);
        CPUID {
            input: input,
            output: output,
        }
    }
    pub fn call(&mut self) {
        cpuid(&self.input, &mut self.output);
    }
    pub fn next_subleaf(&mut self) {
        self.input.ecx += 1;
        cpuid(&self.input, &mut self.output);
    }
}

impl fmt::Display for CPUID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "CPUID {:08x}:{:02x} = {:08x} {:08x} {:08x} {:08x} | {}",
            self.input.eax, self.input.ecx, self.output.eax, self.output.ebx, self.output.ecx, self.output.edx, self.output.ascii()
        )
    }
}
