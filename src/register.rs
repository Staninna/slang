#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Register {
    Accumulator = 0x01,        // Accumulator
    InstructionPointer = 0x02, // Instruction pointer
    StackPointer = 0x03,       // Stack pointer
    FramePointer = 0x04,       // Frame pointer
    FrameSize = 0x05,          // Frame size
    ArgCount = 0x06,           // Argument count
    Reg1 = 0x07,               // General purpose register
    Reg2 = 0x08,               // General purpose register
    Reg3 = 0x09,               // General purpose register
    Reg4 = 0x0A,               // General purpose register
    Reg5 = 0x0B,               // General purpose register
    Reg6 = 0x0C,               // General purpose register
    Reg7 = 0x0D,               // General purpose register
    Reg8 = 0x0E,               // General purpose register
}

impl Register {
    pub fn all() -> Vec<Register> {
        use Register::*;
        vec![
            Accumulator,
            InstructionPointer,
            StackPointer,
            FramePointer,
            FrameSize,
            ArgCount,
            Reg1,
            Reg2,
            Reg3,
            Reg4,
            Reg5,
            Reg6,
            Reg7,
            Reg8,
        ]
    }
}

impl ToString for Register {
    fn to_string(&self) -> String {
        use Register::*;
        match self {
            Accumulator => "accumulator".to_string(),
            InstructionPointer => "instruction_pointer".to_string(),
            StackPointer => "stack_pointer".to_string(),
            FramePointer => "frame_pointer".to_string(),
            FrameSize => "frame_size".to_string(),
            ArgCount => "arg_count".to_string(),
            Reg1 => "reg_1".to_string(),
            Reg2 => "reg_2".to_string(),
            Reg3 => "reg_3".to_string(),
            Reg4 => "reg_4".to_string(),
            Reg5 => "reg_5".to_string(),
            Reg6 => "reg_6".to_string(),
            Reg7 => "reg_7".to_string(),
            Reg8 => "reg_8".to_string(),
        }
    }
}

impl From<&str> for Register {
    fn from(name: &str) -> Self {
        use Register::*;
        match name {
            "accumulator" => Accumulator,
            "instruction_pointer" => InstructionPointer,
            "stack_pointer" => StackPointer,
            "frame_pointer" => FramePointer,
            "frame_size" => FrameSize,
            "arg_count" => ArgCount,
            "reg_1" => Reg1,
            "reg_2" => Reg2,
            "reg_3" => Reg3,
            "reg_4" => Reg4,
            "reg_5" => Reg5,
            "reg_6" => Reg6,
            "reg_7" => Reg7,
            "reg_8" => Reg8,
            _ => panic!("Invalid register name: {}", name),
        }
    }
}
