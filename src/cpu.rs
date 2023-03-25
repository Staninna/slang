use crate::{
    devices::{
        device::{Device64Bit, Device8Bit},
        ram::Ram,
        registers::Registers,
    },
    opcodes::{AddrMode, Instruction, Opcode, Operand},
    register::Register,
};
use hashbrown::HashMap;

pub struct Cpu {
    ram: Ram, // TODO: Make this an device mapper
    regs: Registers,
    regs_names: Vec<Register>,
    regs_addr_map: HashMap<String, u64>,
}

// public methods
impl Cpu {
    pub fn new(mem_size: usize) -> Self {
        // Get all the registers
        let regs_names = Register::all();

        // Make a register memory buffer
        let regs = Registers::new(regs_names.len() * std::mem::size_of::<u64>());

        // Make a register map
        let mut regs_addr_map = HashMap::new();
        for (i, name) in regs_names.iter().enumerate() {
            regs_addr_map.insert(name.to_string(), (i * std::mem::size_of::<u64>()) as u64);
        }

        // Return the CPU
        Self {
            regs,
            regs_names,
            regs_addr_map,
            ram: Ram::new(mem_size),
        }
    }

    // Run the CPU
    pub fn run(&mut self) {
        loop {
            // Fetch the instruction
            let instr = self.fetch();

            // Execute the instruction
            self.execute(instr);
        }
    }
}

// private methods
impl Cpu {
    // Read a register
    fn read_reg(&self, reg: Register) -> u64 {
        let addr = self.get_reg_addr(reg);
        self.regs.read(addr)
    }

    // Write a register
    fn write_reg(&mut self, reg: Register, data: u64) {
        let addr = self.get_reg_addr(reg);
        self.regs.write(addr, data);
    }

    // Get reg address
    fn get_reg_addr(&self, reg: Register) -> u64 {
        *self.regs_addr_map.get(&reg.to_string()).unwrap()
    }

    // Index an register
    fn index_reg(&mut self, index: u8) -> Register {
        self.regs_names[index as usize]
    }

    // Fetch 8 bits of data from the instruction pointer
    fn fetch8(&mut self) -> u8 {
        let ip = self.read_reg(Register::Ip);
        let data = self.ram.read(ip);
        self.write_reg(Register::Ip, ip + 1);
        data
    }

    // Fetch 64 bits of data from the instruction pointer
    fn fetch64(&mut self) -> u64 {
        let ip = self.read_reg(Register::Ip);
        let data = self.ram.read64(ip);
        self.write_reg(Register::Ip, ip + 8);
        data
    }

    // Fetch instruction
    fn fetch(&mut self) -> Instruction {
        // Fetch the opcode and address mode
        let opcode = Opcode::from(self.fetch8());
        let addr_mode = AddrMode::from(self.fetch8());

        // Match the address mode and get the operands
        let operands = self.fetch_operands(&addr_mode);

        // Return the fetched instruction
        Instruction::new(opcode, addr_mode, operands)
    }

    // Fetch operands
    fn fetch_operands(&mut self, addr_mode: &AddrMode) -> (Operand, Operand) {
        use AddrMode::*;
        use Operand::*;
        match addr_mode {
            AddrMode::None => (Operand::None, Operand::None),
            RegToReg => (Reg(self.fetch8()), Reg(self.fetch8())),
            RegToMem => (Reg(self.fetch8()), Mem(self.fetch64())),
            ImmToReg => (Imm(self.fetch64()), Reg(self.fetch8())),
            ImmToMem => (Imm(self.fetch64()), Mem(self.fetch64())),
            MemToReg => (Mem(self.fetch64()), Reg(self.fetch8())),
            MemToMem => (Mem(self.fetch64()), Mem(self.fetch64())),
        }
    }

    // Execute an instruction
    fn execute(&mut self, instr: Instruction) {
        let (opcode, _, operands) = instr.unpack();

        use Opcode::*;
        match opcode {
            Nop => {}
            Mov => self.mov(operands),
            Lod => self.lod(operands),
            Str => self.str(operands),
            Add => self.add(operands),
            Sub => todo!(),
            Mul => todo!(),
            Div => todo!(),
        }
    }

    // TODO: Order the `match operands` in this order: IMM->REG, IMM->MEM, REG->REG, REG->MEM, MEM->REG, MEM->MEM

    // Move data
    fn mov(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        match operands {
            // Imm -> Reg
            (Imm(imm), Reg(reg)) => {
                let reg = self.index_reg(reg);
                self.write_reg(reg, imm);
            }
            // Imm -> Mem
            (Imm(imm), Mem(mem)) => {
                self.ram.write64(mem, imm);
            }
            // Reg -> Reg
            (Reg(reg), Reg(reg2)) => {
                let reg1 = self.index_reg(reg);
                let reg2 = self.index_reg(reg2);
                let data = self.read_reg(reg1);
                self.write_reg(reg2, data);
            }
            // Reg -> Mem
            (Reg(reg), Mem(mem)) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.ram.write64(mem, data);
            }
            // Mem -> Reg
            (Mem(mem), Reg(reg)) => {
                let reg = self.index_reg(reg);
                let data = self.ram.read64(mem);
                self.write_reg(reg, data);
            }
            // Mem -> Mem
            (Mem(mem), Mem(mem2)) => {
                let data = self.ram.read64(mem);
                self.ram.write64(mem2, data);
            }
            _ => panic!("Invalid operands for mov instruction"),
        }
    }

    // Load data
    fn lod(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        match operands {
            // Mem -> Reg
            (Mem(mem), Reg(reg)) => {
                let reg = self.index_reg(reg);
                let data = self.ram.read64(mem);
                self.write_reg(reg, data);
            }
            // Imm -> Reg
            (Imm(imm), Reg(reg)) => {
                let reg = self.index_reg(reg);
                self.write_reg(reg, imm);
            }
            _ => panic!("Invalid operands for lod instruction"),
        }
    }

    // Store data
    fn str(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        match operands {
            // Reg -> Mem
            (Reg(reg), Mem(mem)) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.ram.write64(mem, data);
            }
            // Imm -> Mem
            (Imm(imm), Mem(mem)) => {
                self.ram.write64(mem, imm);
            }
            // Mem -> Mem
            (Mem(mem), Mem(mem2)) => {
                let data = self.ram.read64(mem);
                self.ram.write64(mem2, data);
            }
            _ => panic!("Invalid operands for str instruction"),
        }
    }

    // Add data
    fn add(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Imm -> Reg
            (Imm(imm), Reg(reg)) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.write_reg(Acc, data + imm);
            }
            // Reg -> Reg
            (Reg(reg), Reg(reg2)) => {
                let reg1 = self.index_reg(reg);
                let reg2 = self.index_reg(reg2);
                let data = self.read_reg(reg1);
                let data2 = self.read_reg(reg2);
                self.write_reg(Acc, data + data2);
            }
            // Mem -> Reg
            (Mem(mem), Reg(reg)) => {
                let reg = self.index_reg(reg);
                let data = self.ram.read64(mem);
                let data2 = self.read_reg(reg);
                self.write_reg(Acc, data + data2);
            }
            _ => panic!("Invalid operands for add instruction"),
        }
    }
}
