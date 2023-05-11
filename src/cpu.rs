use crate::{
    devices::{device::Device, device_mapper::DeviceMapper, registers::Registers},
    opcodes::{AddrMode, Instruction, Opcode, Operand},
    register::Register,
};
use hashbrown::HashMap;

pub struct Cpu {
    dev_mapper: DeviceMapper<u64>,
    regs: Registers,
    regs_names: Vec<Register>,
    regs_addr_map: HashMap<Register, u64>,
}

// public methods
impl Cpu {
    pub fn new(mem_size: usize) -> Self {
        // Get all the registers
        let regs_names = Register::all();

        // Make a register memory buffer
        let mut regs = Registers::new(regs_names.len() * std::mem::size_of::<u64>());

        // Make a register map
        let mut regs_addr_map = HashMap::new();
        for (i, reg) in regs_names.iter().enumerate() {
            regs_addr_map.insert(reg.to_owned(), (i * std::mem::size_of::<u64>()) as u64);
        }

        // Set the stack pointer to the end of the memory
        regs.write(
            *regs_addr_map.get(&Register::StackPointer).unwrap(),
            mem_size as u64,
        );

        // Return the CPU
        Self {
            regs,
            regs_names,
            regs_addr_map,
            dev_mapper: DeviceMapper::new(),
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
        *self.regs_addr_map.get(&reg).unwrap()
    }

    // Index an register
    fn index_reg(&mut self, index: u8) -> Register {
        self.regs_names[index as usize]
    }

    // Fetch 8 bits of data from the instruction pointer
    fn fetch8(&mut self) -> u8 {
        let ip = self.read_reg(Register::InstructionPointer);
        let data = self.dev_mapper.read(ip);
        self.write_reg(Register::InstructionPointer, ip + 1);
        data as u8
    }

    // Fetch 64 bits of data from the instruction pointer
    fn fetch64(&mut self) -> u64 {
        let ip = self.read_reg(Register::InstructionPointer);
        let data = self.dev_mapper.read(ip);
        self.write_reg(Register::InstructionPointer, ip + 8);
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
            AddrMode::Null => (Operand::Null, Operand::Null),
            RegToReg => (Reg(self.fetch8()), Reg(self.fetch8())),
            RegToMem => (Reg(self.fetch8()), Mem(self.fetch64())),
            ImmToReg => (Imm(self.fetch64()), Reg(self.fetch8())),
            ImmToMem => (Imm(self.fetch64()), Mem(self.fetch64())),
            MemToReg => (Mem(self.fetch64()), Reg(self.fetch8())),
            MemToMem => (Mem(self.fetch64()), Mem(self.fetch64())),
            Literal => (Imm(self.fetch64()), Operand::Null),
            Register => (Reg(self.fetch8()), Operand::Null),
            Memory => (Mem(self.fetch64()), Operand::Null),
        }
    }

    // Push the state of the CPU to the stack
    fn push_state(&mut self) {
        use Operand::*;
        use Register::*;

        // Push the registers
        self.psh((Reg(InstructionPointer as u8), Null));
        self.psh((Reg(Reg1 as u8), Null));
        self.psh((Reg(Reg2 as u8), Null));
        self.psh((Reg(Reg3 as u8), Null));
        self.psh((Reg(Reg4 as u8), Null));
        self.psh((Reg(Reg5 as u8), Null));
        self.psh((Reg(Reg6 as u8), Null));
        self.psh((Reg(Reg7 as u8), Null));
        self.psh((Reg(Reg8 as u8), Null));

        // Push the stack frame size
        self.psh((Reg(FrameSize as u8), Null));
        self.write_reg(FrameSize, 0);

        // Write the new frame pointer
        let sp = self.read_reg(StackPointer);
        self.write_reg(FramePointer, sp);
    }

    // Pop the state of the CPU from the stack
    fn pop_state(&mut self) {
        use Operand::*;
        use Register::*;

        // Pop the stack frame size
        self.pop((Reg(FrameSize as u8), Null));

        // Pop the registers
        self.pop((Reg(Reg8 as u8), Null));
        self.pop((Reg(Reg7 as u8), Null));
        self.pop((Reg(Reg6 as u8), Null));
        self.pop((Reg(Reg5 as u8), Null));
        self.pop((Reg(Reg4 as u8), Null));
        self.pop((Reg(Reg3 as u8), Null));
        self.pop((Reg(Reg2 as u8), Null));
        self.pop((Reg(Reg1 as u8), Null));
        self.pop((Reg(InstructionPointer as u8), Null));

        // Remove arguments from the stack
        let num_args = self.read_reg(ArgCount);
        for _ in 0..num_args {
            self.pop((Null, Null));
        }

        // Reset frame pointer
        let fs = self.read_reg(FrameSize);
        self.write_reg(FramePointer, fs + self.read_reg(FramePointer));
    }

    // Execute an instruction
    fn execute(&mut self, instr: Instruction) {
        let (opcode, _, operands) = instr.unpack();

        use Opcode::*;
        match opcode {
            // Misc
            Nop => {} // No operation

            // Load and store
            Mov => self.mov(operands),
            Lod => self.lod(operands),
            Str => self.str(operands),

            // Arithmetic
            Add => self.add(operands),
            Sub => self.sub(operands),
            Mul => self.mul(operands),
            Div => self.div(operands),
            Inc => self.inc(operands),
            Dec => self.dec(operands),

            // Bitwise
            And => self.and(operands),
            Or => self.or(operands),
            Xor => self.xor(operands),
            Not => self.not(operands),
            Shl => self.shl(operands),
            Shr => self.shr(operands),

            // Branching
            Jmp => self.jmp(operands),
            Jeq => self.jeq(operands),
            Jne => self.jne(operands),
            Jgt => self.jgt(operands),
            Jlt => self.jlt(operands),
            Jge => self.jge(operands),
            Jle => self.jle(operands),
            Jnz => self.jnz(operands),
            Jz => self.jz(operands),

            // Stack
            Psh => self.psh(operands),
            Pop => self.pop(operands),
            Dup => self.dup(operands),
            Swp => self.swp(operands),
            Clr => self.clr(operands),
            Ret => self.ret(operands),
            Cal => self.cal(operands),
        }
    }

    // Move
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
                self.dev_mapper.write(mem, imm);
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
                self.dev_mapper.write(mem, data);
            }
            // Mem -> Reg
            (Mem(mem), Reg(reg)) => {
                let reg = self.index_reg(reg);
                let data = self.dev_mapper.read(mem);
                self.write_reg(reg, data);
            }
            // Mem -> Mem
            (Mem(mem), Mem(mem2)) => {
                let data = self.dev_mapper.read(mem);
                self.dev_mapper.write(mem2, data);
            }
            _ => panic!("Invalid operands for mov instruction"),
        }
    }

    // Load
    fn lod(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        match operands {
            // Imm -> Reg
            (Imm(imm), Reg(reg)) => {
                let reg = self.index_reg(reg);
                self.write_reg(reg, imm);
            }
            // Mem -> Reg
            (Mem(mem), Reg(reg)) => {
                let reg = self.index_reg(reg);
                let data = self.dev_mapper.read(mem);
                self.write_reg(reg, data);
            }
            _ => panic!("Invalid operands for lod instruction"),
        }
    }

    // Store
    fn str(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        match operands {
            // Imm -> Mem
            (Imm(imm), Mem(mem)) => {
                self.dev_mapper.write(mem, imm);
            }
            // Reg -> Mem
            (Reg(reg), Mem(mem)) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.dev_mapper.write(mem, data);
            }
            // Mem -> Mem
            (Mem(mem), Mem(mem2)) => {
                let data = self.dev_mapper.read(mem);
                self.dev_mapper.write(mem2, data);
            }
            _ => panic!("Invalid operands for str instruction"),
        }
    }

    // Add
    fn add(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Imm -> Reg
            (Imm(imm), Reg(reg)) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.write_reg(Accumulator, data + imm);
            }
            // Reg -> Reg
            (Reg(reg), Reg(reg2)) => {
                let reg1 = self.index_reg(reg);
                let reg2 = self.index_reg(reg2);
                let data = self.read_reg(reg1);
                let data2 = self.read_reg(reg2);
                self.write_reg(Accumulator, data + data2);
            }
            // Mem -> Reg
            (Mem(mem), Reg(reg)) => {
                let reg = self.index_reg(reg);
                let data = self.dev_mapper.read(mem);
                let data2 = self.read_reg(reg);
                self.write_reg(Accumulator, data + data2);
            }
            _ => panic!("Invalid operands for add instruction"),
        }
    }

    // Subtract
    fn sub(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Imm -> Reg
            (Imm(imm), Reg(reg)) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.write_reg(Accumulator, data - imm);
            }
            // Reg -> Reg
            (Reg(reg), Reg(reg2)) => {
                let reg1 = self.index_reg(reg);
                let reg2 = self.index_reg(reg2);
                let data = self.read_reg(reg1);
                let data2 = self.read_reg(reg2);
                self.write_reg(Accumulator, data - data2);
            }
            // Mem -> Reg
            (Mem(mem), Reg(reg)) => {
                let reg = self.index_reg(reg);
                let data = self.dev_mapper.read(mem);
                let data2 = self.read_reg(reg);
                self.write_reg(Accumulator, data - data2);
            }
            _ => panic!("Invalid operands for sub instruction"),
        }
    }

    // Multiply
    fn mul(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Imm -> Reg
            (Imm(imm), Reg(reg)) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.write_reg(Accumulator, data * imm);
            }
            // Reg -> Reg
            (Reg(reg), Reg(reg2)) => {
                let reg1 = self.index_reg(reg);
                let reg2 = self.index_reg(reg2);
                let data = self.read_reg(reg1);
                let data2 = self.read_reg(reg2);
                self.write_reg(Accumulator, data * data2);
            }
            // Mem -> Reg
            (Mem(mem), Reg(reg)) => {
                let reg = self.index_reg(reg);
                let data = self.dev_mapper.read(mem);
                let data2 = self.read_reg(reg);
                self.write_reg(Accumulator, data * data2);
            }
            _ => panic!("Invalid operands for mul instruction"),
        }
    }

    // Divide
    fn div(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Imm -> Reg
            (Imm(imm), Reg(reg)) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.write_reg(Accumulator, data / imm);
            }
            // Reg -> Reg
            (Reg(reg), Reg(reg2)) => {
                let reg1 = self.index_reg(reg);
                let reg2 = self.index_reg(reg2);
                let data = self.read_reg(reg1);
                let data2 = self.read_reg(reg2);
                self.write_reg(Accumulator, data / data2);
            }
            // Mem -> Reg
            (Mem(mem), Reg(reg)) => {
                let reg = self.index_reg(reg);
                let data = self.dev_mapper.read(mem);
                let data2 = self.read_reg(reg);
                self.write_reg(Accumulator, data / data2);
            }
            _ => panic!("Invalid operands for div instruction"),
        }
    }

    // Increment
    fn inc(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Reg
            (Reg(reg), Null) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.write_reg(Accumulator, data + 1);
            }
            // Mem
            (Mem(mem), Null) => {
                let data = self.dev_mapper.read(mem);
                self.dev_mapper.write(mem, data + 1);
            }
            _ => panic!("Invalid operands for inc instruction"),
        }
    }

    // Decrement
    fn dec(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Reg
            (Reg(reg), Null) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.write_reg(Accumulator, data - 1);
            }
            // Mem
            (Mem(mem), Null) => {
                let data = self.dev_mapper.read(mem);
                self.dev_mapper.write(mem, data - 1);
            }
            _ => panic!("Invalid operands for dec instruction"),
        }
    }

    // Bitwise AND
    fn and(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Reg -> Imm
            (Reg(reg), Imm(imm)) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.write_reg(Accumulator, data & imm);
            }
            // Reg -> Reg
            (Reg(reg), Reg(reg2)) => {
                let reg1 = self.index_reg(reg);
                let reg2 = self.index_reg(reg2);
                let data = self.read_reg(reg1);
                let data2 = self.read_reg(reg2);
                self.write_reg(Accumulator, data & data2);
            }
            _ => panic!("Invalid operands for and instruction"),
        }
    }

    // Bitwise OR
    fn or(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Reg -> Imm
            (Reg(reg), Imm(imm)) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.write_reg(Accumulator, data | imm);
            }
            // Reg -> Reg
            (Reg(reg), Reg(reg2)) => {
                let reg1 = self.index_reg(reg);
                let reg2 = self.index_reg(reg2);
                let data = self.read_reg(reg1);
                let data2 = self.read_reg(reg2);
                self.write_reg(Accumulator, data | data2);
            }
            _ => panic!("Invalid operands for or instruction"),
        }
    }

    // Bitwise XOR
    fn xor(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Reg -> Imm
            (Reg(reg), Imm(imm)) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.write_reg(Accumulator, data ^ imm);
            }
            // Reg -> Reg
            (Reg(reg), Reg(reg2)) => {
                let reg1 = self.index_reg(reg);
                let reg2 = self.index_reg(reg2);
                let data = self.read_reg(reg1);
                let data2 = self.read_reg(reg2);
                self.write_reg(Accumulator, data ^ data2);
            }
            _ => panic!("Invalid operands for xor instruction"),
        }
    }

    // Bitwise NOT
    fn not(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Reg
            (Reg(reg), Null) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.write_reg(Accumulator, !data);
            }
            // Mem
            (Mem(mem), Null) => {
                let data = self.dev_mapper.read(mem);
                self.write_reg(Accumulator, !data);
            }
            _ => panic!("Invalid operands for not instruction"),
        }
    }

    // Bitwise Shift Left
    fn shl(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Reg -> Imm
            (Reg(reg), Imm(imm)) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.write_reg(Accumulator, data << imm);
            }
            // Reg -> Reg
            (Reg(reg), Reg(reg2)) => {
                let reg1 = self.index_reg(reg);
                let reg2 = self.index_reg(reg2);
                let data = self.read_reg(reg1);
                let data2 = self.read_reg(reg2);
                self.write_reg(Accumulator, data << data2);
            }
            _ => panic!("Invalid operands for shl instruction"),
        }
    }

    // Bitwise Shift Right
    fn shr(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Reg -> Imm
            (Reg(reg), Imm(imm)) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.write_reg(Accumulator, data >> imm);
            }
            // Reg -> Reg
            (Reg(reg), Reg(reg2)) => {
                let reg1 = self.index_reg(reg);
                let reg2 = self.index_reg(reg2);
                let data = self.read_reg(reg1);
                let data2 = self.read_reg(reg2);
                self.write_reg(Accumulator, data >> data2);
            }
            _ => panic!("Invalid operands for shr instruction"),
        }
    }

    // Jump
    fn jmp(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Reg
            (Reg(reg), Null) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.write_reg(InstructionPointer, data);
            }
            // Imm
            (Imm(imm), Null) => {
                self.write_reg(InstructionPointer, imm);
            }
            _ => panic!("Invalid operands for jmp instruction"),
        }
    }

    // Jump if equal
    fn jeq(&mut self, operands: (Operand, Operand)) {
        let acc = self.read_reg(Register::Accumulator);

        use Operand::*;
        use Register::*;
        match operands {
            // Reg
            (Reg(reg), Null) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                if acc == data {
                    self.write_reg(InstructionPointer, data);
                }
            }
            // Imm
            (Imm(imm), Null) => {
                if acc == imm {
                    self.write_reg(InstructionPointer, imm);
                }
            }
            _ => panic!("Invalid operands for jeq instruction"),
        }
    }

    // Jump if not equal
    fn jne(&mut self, operands: (Operand, Operand)) {
        let acc = self.read_reg(Register::Accumulator);

        use Operand::*;
        use Register::*;
        match operands {
            // Reg
            (Reg(reg), Null) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                if acc != data {
                    self.write_reg(InstructionPointer, data);
                }
            }
            // Imm
            (Imm(imm), Null) => {
                if acc != imm {
                    self.write_reg(InstructionPointer, imm);
                }
            }
            _ => panic!("Invalid operands for jne instruction"),
        }
    }

    // Jump if greater than
    fn jgt(&mut self, operands: (Operand, Operand)) {
        let acc = self.read_reg(Register::Accumulator);

        use Operand::*;
        use Register::*;
        match operands {
            // Reg
            (Reg(reg), Null) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                if acc > data {
                    self.write_reg(InstructionPointer, data);
                }
            }
            // Imm
            (Imm(imm), Null) => {
                if acc > imm {
                    self.write_reg(InstructionPointer, imm);
                }
            }
            _ => panic!("Invalid operands for jgt instruction"),
        }
    }

    // Jump if less than
    fn jlt(&mut self, operands: (Operand, Operand)) {
        let acc = self.read_reg(Register::Accumulator);

        use Operand::*;
        use Register::*;
        match operands {
            // Reg
            (Reg(reg), Null) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                if acc < data {
                    self.write_reg(InstructionPointer, data);
                }
            }
            // Imm
            (Imm(imm), Null) => {
                if acc < imm {
                    self.write_reg(InstructionPointer, imm);
                }
            }
            _ => panic!("Invalid operands for jlt instruction"),
        }
    }

    // Jump if greater than or equal to
    fn jge(&mut self, operands: (Operand, Operand)) {
        let acc = self.read_reg(Register::Accumulator);

        use Operand::*;
        use Register::*;
        match operands {
            // Reg
            (Reg(reg), Null) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                if acc >= data {
                    self.write_reg(InstructionPointer, data);
                }
            }
            // Imm
            (Imm(imm), Null) => {
                if acc >= imm {
                    self.write_reg(InstructionPointer, imm);
                }
            }
            _ => panic!("Invalid operands for jge instruction"),
        }
    }

    // Jump if less than or equal to
    fn jle(&mut self, operands: (Operand, Operand)) {
        let acc = self.read_reg(Register::Accumulator);

        use Operand::*;
        use Register::*;
        match operands {
            // Reg
            (Reg(reg), Null) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                if acc <= data {
                    self.write_reg(InstructionPointer, data);
                }
            }
            // Imm
            (Imm(imm), Null) => {
                if acc <= imm {
                    self.write_reg(InstructionPointer, imm);
                }
            }
            _ => panic!("Invalid operands for jle instruction"),
        }
    }

    // Jump not zero
    fn jnz(&mut self, operands: (Operand, Operand)) {
        let acc = self.read_reg(Register::Accumulator);

        use Operand::*;
        use Register::*;
        match operands {
            // Reg
            (Reg(reg), Null) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                if acc != 0 {
                    self.write_reg(InstructionPointer, data);
                }
            }
            // Imm
            (Imm(imm), Null) => {
                if acc != 0 {
                    self.write_reg(InstructionPointer, imm);
                }
            }
            _ => panic!("Invalid operands for jnz instruction"),
        }
    }

    // Jump zero
    fn jz(&mut self, operands: (Operand, Operand)) {
        let acc = self.read_reg(Register::Accumulator);

        use Operand::*;
        use Register::*;
        match operands {
            // Reg
            (Reg(reg), Null) => {
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                if acc == 0 {
                    self.write_reg(InstructionPointer, data);
                }
            }
            // Imm
            (Imm(imm), Null) => {
                if acc == 0 {
                    self.write_reg(InstructionPointer, imm);
                }
            }
            _ => panic!("Invalid operands for jz instruction"),
        }
    }

    // Push to stack
    fn psh(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Imm -> Stack
            (Imm(imm), Null) => {
                let sp = self.read_reg(StackPointer);
                self.dev_mapper.write(sp, imm);
                self.write_reg(StackPointer, sp - std::mem::size_of::<u64>() as u64);
                self.write_reg(
                    FrameSize,
                    self.read_reg(FrameSize) + std::mem::size_of::<u64>() as u64,
                );
            }
            // Reg -> Stack
            (Reg(reg), _) => {
                let sp = self.read_reg(StackPointer);
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.dev_mapper.write(sp, data);
                self.write_reg(StackPointer, sp - std::mem::size_of::<u64>() as u64);
                self.write_reg(
                    FrameSize,
                    self.read_reg(FrameSize) + std::mem::size_of::<u64>() as u64,
                );
            }
            _ => panic!("Invalid operands for psh instruction"),
        }
    }

    // Pop from stack
    fn pop(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Stack -> Reg
            (Reg(reg), Null) => {
                let sp = self.read_reg(StackPointer);
                let reg = self.index_reg(reg);
                let data = self.dev_mapper.read(sp);
                self.write_reg(reg, data);
                self.write_reg(StackPointer, sp + std::mem::size_of::<u64>() as u64);
                self.write_reg(
                    FrameSize,
                    self.read_reg(FrameSize) - std::mem::size_of::<u64>() as u64,
                );
            }
            // Null
            (Null, Null) => {
                let sp = self.read_reg(StackPointer);
                self.write_reg(StackPointer, sp + std::mem::size_of::<u64>() as u64);
                self.write_reg(
                    FrameSize,
                    self.read_reg(FrameSize) - std::mem::size_of::<u64>() as u64,
                );
            }
            _ => panic!("Invalid operands for pop instruction"),
        }
    }

    // Duplicate top of stack
    fn dup(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Stack -> Stack
            (Null, Null) => {
                let sp = self.read_reg(StackPointer);
                let data = self.dev_mapper.read(sp);
                self.dev_mapper
                    .write(sp - std::mem::size_of::<u64>() as u64, data);
                self.write_reg(StackPointer, sp - std::mem::size_of::<u64>() as u64);
            }
            _ => panic!("Invalid operands for dup instruction"),
        }
    }

    // Swap top two elements of stack
    fn swp(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Stack -> Stack
            (Null, Null) => {
                let sp = self.read_reg(StackPointer);
                let data1 = self.dev_mapper.read(sp);
                let data2 = self.dev_mapper.read(sp + std::mem::size_of::<u64>() as u64);
                self.dev_mapper.write(sp, data2);
                self.dev_mapper
                    .write(sp + std::mem::size_of::<u64>() as u64, data1);
            }
            _ => panic!("Invalid operands for swp instruction"),
        }
    }

    // Clear the whole stack
    fn clr(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        // use Register::*;
        match operands {
            // Stack -> Stack
            (Null, Null) => {
                todo!("Fix the .size() method");
                // self.write_reg(StackPointer, self.dev_mapper.size() as u64);
            }
            _ => panic!("Invalid operands for clr instruction"),
        }
    }

    // Return from subroutine
    fn ret(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        match operands {
            // Stack -> Stack
            (Null, Null) => {
                self.pop_state();
            }
            _ => panic!("Invalid operands for ret instruction"),
        }
    }

    // Call a subroutine
    fn cal(&mut self, operands: (Operand, Operand)) {
        use Operand::*;
        use Register::*;
        match operands {
            // Imm -> Stack
            (Imm(imm), Null) => {
                self.push_state();
                self.write_reg(InstructionPointer, imm);
            }
            // Reg -> Stack
            (Reg(reg), _) => {
                self.push_state();
                let reg = self.index_reg(reg);
                let data = self.read_reg(reg);
                self.write_reg(InstructionPointer, data);
            }
            _ => panic!("Invalid operands for cal instruction"),
        }
    }
}
