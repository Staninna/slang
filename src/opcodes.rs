pub enum Opcode {
    Nop = 0x00,
    Mov = 0x01,
    Lod = 0x02,
    Str = 0x03,
    Add = 0x04,
    Sub = 0x05,
    Mul = 0x06,
    Div = 0x07,
}

impl From<u8> for Opcode {
    fn from(opcode: u8) -> Self {
        use Opcode::*;
        match opcode {
            0x00 => Nop,
            0x01 => Mov,
            0x02 => Lod,
            0x03 => Str,
            0x04 => Add,
            0x05 => Sub,
            0x06 => Mul,
            0x07 => Div,
            _ => panic!("Invalid opcode: {0:#x}", opcode),
        }
    }
}

pub enum AddrMode {
    None = 0x00,
    RegToReg = 0x10,
    RegToMem = 0x20,
    ImmToReg = 0x30,
    ImmToMem = 0x40,
    MemToReg = 0x50,
    MemToMem = 0x60,
}

impl From<u8> for AddrMode {
    fn from(addr_mode: u8) -> Self {
        match addr_mode {
            0x00 => AddrMode::None,
            0x10 => AddrMode::RegToReg,
            0x20 => AddrMode::RegToMem,
            0x30 => AddrMode::ImmToReg,
            0x40 => AddrMode::ImmToMem,
            0x50 => AddrMode::MemToReg,
            0x60 => AddrMode::MemToMem,
            _ => panic!("Invalid address mode: {0:#x}", addr_mode),
        }
    }
}

pub enum Operand {
    None,
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
