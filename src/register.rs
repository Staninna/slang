#[derive(Copy, Clone)]
pub enum Register {
    Acc = 0x01, // Accumulator
    Ip = 0x02,  // Instruction pointer
    Sp = 0x03,  // Stack pointer
    Fs = 0x04,  // Frame size
    Fp = 0x05,  // Frame pointer
    R1 = 0x06,  // General purpose register
    R2 = 0x07,  // General purpose register
    R3 = 0x08,  // General purpose register
    R4 = 0x09,  // General purpose register
    R5 = 0x0A,  // General purpose register
    R6 = 0x0B,  // General purpose register
    R7 = 0x0C,  // General purpose register
    R8 = 0x0D,  // General purpose register
}

impl Register {
    pub fn all() -> Vec<Register> {
        use Register::*;
        vec![Acc, Ip, Sp, Fs, Fp, R1, R2, R3, R4, R5, R6, R7, R8]
    }
}

impl ToString for Register {
    fn to_string(&self) -> String {
        use Register::*;
        match self {
            Acc => "acc".to_string(),
            Ip => "ip".to_string(),
            Sp => "sp".to_string(),
            Fs => "fs".to_string(),
            Fp => "fp".to_string(),
            R1 => "r1".to_string(),
            R2 => "r2".to_string(),
            R3 => "r3".to_string(),
            R4 => "r4".to_string(),
            R5 => "r5".to_string(),
            R6 => "r6".to_string(),
            R7 => "r7".to_string(),
            R8 => "r8".to_string(),
        }
    }
}

impl From<&str> for Register {
    fn from(name: &str) -> Self {
        use Register::*;
        match name {
            "acc" => Acc,
            "ip" => Ip,
            "sp" => Sp,
            "fs" => Fs,
            "fp" => Fp,
            "r1" => R1,
            "r2" => R2,
            "r3" => R3,
            "r4" => R4,
            "r5" => R5,
            "r6" => R6,
            "r7" => R7,
            "r8" => R8,
            _ => panic!("Invalid register name: {}", name),
        }
    }
}
