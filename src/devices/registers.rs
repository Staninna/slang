use super::device::{Buffer, Device64Bit};

#[derive(Copy, Clone)]
pub enum Register {
    Acc = 0x01, // Accumulator
    Ip = 0x02,  // Instruction pointer
    R1 = 0x03,  // General purpose register
    R2 = 0x04,  // General purpose register
    R3 = 0x05,  // General purpose register
    R4 = 0x06,  // General purpose register
    R5 = 0x07,  // General purpose register
    R6 = 0x08,  // General purpose register
    R7 = 0x09,  // General purpose register
    R8 = 0x0A,  // General purpose register
}

impl Register {
    // return a vector of all the registers dynamically so if we add more registers we don't have to change this
    pub fn all() -> Vec<Register> {
        vec![
            Register::Acc,
            Register::Ip,
            Register::R1,
            Register::R2,
            Register::R3,
            Register::R4,
            Register::R5,
            Register::R6,
            Register::R7,
            Register::R8,
        ]
    }
}

impl ToString for Register {
    fn to_string(&self) -> String {
        match self {
            Register::Acc => "acc".to_string(),
            Register::Ip => "ip".to_string(),
            Register::R1 => "r1".to_string(),
            Register::R2 => "r2".to_string(),
            Register::R3 => "r3".to_string(),
            Register::R4 => "r4".to_string(),
            Register::R5 => "r5".to_string(),
            Register::R6 => "r6".to_string(),
            Register::R7 => "r7".to_string(),
            Register::R8 => "r8".to_string(),
        }
    }
}

impl From<&str> for Register {
    fn from(name: &str) -> Self {
        match name {
            "acc" => Register::Acc,
            "ip" => Register::Ip,
            "r1" => Register::R1,
            "r2" => Register::R2,
            "r3" => Register::R3,
            "r4" => Register::R4,
            "r5" => Register::R5,
            "r6" => Register::R6,
            "r7" => Register::R7,
            "r8" => Register::R8,
            _ => panic!("Invalid register name: {}", name),
        }
    }
}

pub struct Registers {
    buffer: Buffer,
}

impl Registers {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: Buffer::new(size),
        }
    }
}

impl Device64Bit for Registers {
    fn read(&self, addr: u64) -> u64 {
        if !self.check_addr(addr) {
            panic!("Invalid address: {0:#x}", addr)
        }

        let mut data: u64 = 0;
        for i in 0..std::mem::size_of::<u64>() {
            // Add the data from the next 8 bits to the data by shifting to the right place and oring it
            data |= (self.buffer.data[(addr + i as u64) as usize] as u64) << (i * 8);
        }
        data
    }

    fn write(&mut self, addr: u64, data: u64) {
        if !self.check_addr(addr) {
            panic!("Invalid address: {0:#x}", addr)
        }

        for i in 0..std::mem::size_of::<u64>() {
            // Write the data to the next 8 bits by shifting to the right place and replacing the data
            self.buffer.data[(addr + i as u64) as usize] = (data >> (i * 8)) as u8;
        }
    }

    fn size(&self) -> usize {
        self.buffer.data.len()
    }
}
