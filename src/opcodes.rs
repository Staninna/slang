pub enum Opcode {
    // Misc
    Nop = 0x00,

    // Load and store
    Mov = 0x01,
    Lod = 0x02,
    Str = 0x03,

    // Arithmetic
    Add = 0x04,
    Sub = 0x05,
    Mul = 0x06,
    Div = 0x07,

    // Bitwise
    And = 0x08,
    Or = 0x09,
    Xor = 0x0A,
    Not = 0x0B,
    Shl = 0x0C,
    Shr = 0x0D,

    // Branching
    Jmp = 0x0E,
    Jeq = 0x0F,
    Jne = 0x10,
    Jgt = 0x11,
    Jlt = 0x12,
    Jge = 0x13,
    Jle = 0x14,
    Jnz = 0x15,
    Jz = 0x16,

    // Stack
    Psh = 0x17,
    Pop = 0x18,
    Dup = 0x19,
    Swp = 0x1A,
    Clr = 0x1B,
    Ret = 0x1C,
    Cal = 0x1D,
}

impl From<u8> for Opcode {
    fn from(opcode: u8) -> Self {
        use Opcode::*;
        match opcode {
            // Misc
            0x00 => Nop,

            // Load and store
            0x01 => Mov,
            0x02 => Lod,
            0x03 => Str,

            // Arithmetic
            0x04 => Add,
            0x05 => Sub,
            0x06 => Mul,
            0x07 => Div,

            // Bitwise
            0x08 => And,
            0x09 => Or,
            0x0A => Xor,
            0x0B => Not,
            0x0C => Shl,
            0x0D => Shr,

            // Branching
            0x0E => Jmp,
            0x0F => Jeq,
            0x10 => Jne,
            0x11 => Jgt,
            0x12 => Jlt,
            0x13 => Jge,
            0x14 => Jle,
            0x15 => Jnz,
            0x16 => Jz,

            // Stack
            0x17 => Psh,
            0x18 => Pop,
            0x19 => Dup,
            0x1A => Swp,
            0x1B => Clr,
            0x1C => Ret,
            0x1D => Cal,

            _ => panic!("Invalid opcode: {0:#x}", opcode),
        }
    }
}

pub enum AddrMode {
    Null = 0x00,
    RegToReg = 0x10,
    RegToMem = 0x20,
    ImmToReg = 0x30,
    ImmToMem = 0x40,
    MemToReg = 0x50,
    MemToMem = 0x60,
    Literal = 0x70,
    Register = 0x80,
    Memory = 0x90,
}

impl From<u8> for AddrMode {
    fn from(addr_mode: u8) -> Self {
        use AddrMode::*;
        match addr_mode {
            0x00 => Null,
            0x10 => RegToReg,
            0x20 => RegToMem,
            0x30 => ImmToReg,
            0x40 => ImmToMem,
            0x50 => MemToReg,
            0x60 => MemToMem,
            0x70 => Literal,
            0x80 => Register,
            0x90 => Memory,

            _ => panic!("Invalid address mode: {0:#x}", addr_mode),
        }
    }
}

pub enum Operand {
    Null,
    Reg(u8),
    Imm(u64),
    Mem(u64),
}

pub struct Instruction {
    pub opcode: Opcode,
    pub addr_mode: AddrMode,
    pub operands: (Operand, Operand),
}

impl Instruction {
    pub fn new(opcode: Opcode, addr_mode: AddrMode, operands: (Operand, Operand)) -> Self {
        Self {
            opcode,
            addr_mode,
            operands,
        }
    }

    pub fn unpack(self) -> (Opcode, AddrMode, (Operand, Operand)) {
        (self.opcode, self.addr_mode, self.operands)
    }
}
