use crate::devices::{
    device::{Device64Bit, Device8Bit},
    ram::Ram,
    registers::Registers,
};
use hashbrown::HashMap;

pub struct Cpu {
    regs: Registers,
    regs_maps: HashMap<String, u64>,
    reg_names: Vec<String>,
    ram: Ram,
}

// public methods
impl Cpu {
    pub fn new(mem_size: usize) -> Self {
        let reg_names = vec![
            "acc".to_string(), // Accumulator
            "ip".to_string(),  // Instruction Pointer
            "r1".to_string(),  // Register 1
            "r2".to_string(),  // Register 2
            "r3".to_string(),  // Register 3
            "r4".to_string(),  // Register 4
            "r5".to_string(),  // Register 5
            "r6".to_string(),  // Register 6
            "r7".to_string(),  // Register 7
            "r8".to_string(),  // Register 8
        ];

        // Make a register memory buffer
        let regs = Registers::new(reg_names.len() * std::mem::size_of::<u64>());

        // Make a register map
        let mut regs_maps: HashMap<String, u64> = HashMap::new();
        for (i, name) in reg_names.iter().enumerate() {
            regs_maps.insert(name.to_string(), (i * std::mem::size_of::<u64>()) as u64);
        }

        // Return the CPU
        Self {
            regs,
            regs_maps,
            reg_names,
            ram: Ram::new(mem_size),
        }
    }

    pub fn run(&mut self) {
        self.fetch8();
    }
}

// private methods
impl Cpu {
    // Read a register
    fn read_reg(&self, name: &str) -> u64 {
        let addr = match self.regs_maps.get(name) {
            Some(addr) => addr,
            None => panic!("Register {} not found", name),
        };
        self.regs.read(*addr)
    }

    // Write a register
    fn write_reg(&mut self, name: &str, data: u64) {
        let addr = match self.regs_maps.get(name) {
            Some(addr) => addr,
            None => panic!("Register {} not found", name),
        };
        self.regs.write(*addr, data);
    }

    // Index an register
    fn index_reg(&mut self, index: u8) -> &str {
        &self.reg_names[index as usize]
    }

    // Fetch 8 bits of data from the instruction pointer
    fn fetch8(&mut self) -> u8 {
        let ip = self.read_reg("ip");
        let data = self.ram.read(ip);
        self.write_reg("ip", ip + 1);
        data
    }

    // Fetch 64 bits of data from the instruction pointer
    fn fetch64(&mut self) -> u64 {
        let ip = self.read_reg("ip");
        let data = self.ram.read64(ip);
        self.write_reg("ip", ip + 8);
        data
    }
}
