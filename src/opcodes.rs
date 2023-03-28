pub enum Opcode {
    // Misc
    Nop = 0xFF,

    // Load and store
    Mov = 0x01,
    Lod = 0x02,
    Str = 0x03,

    // Arithmetic
    Add = 0x11,
    Sub = 0x12,
    Mul = 0x13,
    Div = 0x14,
    Inc = 0x15,
    Dec = 0x16,

    // Bitwise
    And = 0x21,
    Or = 0x22,
    Xor = 0x23,
    Not = 0x24,
    Shl = 0x25,
    Shr = 0x26,

    // Branching
    Jmp = 0x31,
    Jeq = 0x32,
    Jne = 0x33,
    Jgt = 0x34,
    Jlt = 0x35,
    Jge = 0x36,
    Jle = 0x37,
    Jnz = 0x38,
    Jz = 0x39,

    // Stack
    Psh = 0x41,
    Pop = 0x42,
    Dup = 0x43,
    Swp = 0x44,
    Clr = 0x45,
    Ret = 0x46,
    Cal = 0x47,
}

impl From<u8> for Opcode {
    fn from(opcode: u8) -> Self {
        use Opcode::*;
        match opcode {
            // Misc
            0xFF => Nop,

            // Load and store
            0x01 => Mov,
            0x02 => Lod,
            0x03 => Str,

            // Arithmetic
            0x11 => Add,
            0x12 => Sub,
            0x13 => Mul,
            0x14 => Div,
            0x15 => Inc,
            0x16 => Dec,

            // Bitwise
            0x21 => And,
            0x22 => Or,
            0x23 => Xor,
            0x24 => Not,
            0x25 => Shl,
            0x26 => Shr,

            // Branching
            0x31 => Jmp,
            0x32 => Jeq,
            0x33 => Jne,
            0x34 => Jgt,
            0x35 => Jlt,
            0x36 => Jge,
            0x37 => Jle,
            0x38 => Jnz,
            0x39 => Jz,

            // Stack
            0x41 => Psh,
            0x42 => Pop,
            0x43 => Dup,
            0x44 => Swp,
            0x45 => Clr,
            0x46 => Ret,
            0x47 => Cal,

            _ => panic!("Invalid opcode: {0:#x}", opcode),
        }
    }
}

pub enum AddrMode {
    ImmToReg = 0x10,
    ImmToMem = 0x20,
    RegToReg = 0x30,
    RegToMem = 0x40,
    MemToReg = 0x50,
    MemToMem = 0x60,
    Literal = 0x70,
    Register = 0x80,
    Memory = 0x90,
    Null = 0xA0,
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
