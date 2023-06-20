use vm::{
    cpu::Cpu,
    devices::{
        rom::Rom,
        stdout::{Stdout, STDOUT_NEWLINE},
    },
    opcodes::AddrMode::*,
    opcodes::Opcode::*,
};

mod vm;

const MEM_SIZE: usize = 1024 * 1024 * 1024 * 4; // 4GB
const ROM_SIZE: usize = 1024 * 1024; // 1MB
const STDOUT_ADDR: u64 = 0x0000_0000_FFFF_0000;

fn main() {
    // Create CPU
    let mut cpu = Cpu::new(MEM_SIZE);

    // Attach ROM
    let rom = Box::new(rom());
    cpu.attach(rom, String::from("rom"), 0);

    // Attach stdout
    let stdout = Box::new(Stdout::new());
    cpu.attach(stdout, String::from("stdout"), STDOUT_ADDR);

    // Run CPU
    cpu.run();
}

fn rom() -> Rom {
    // Stdout constants as bytes
    let stdout = STDOUT_ADDR.to_be_bytes();
    let new_line = (STDOUT_NEWLINE as u64).to_be_bytes();

    #[rustfmt::skip]
    let program = vec![
        // Print "Hello, World!"
        Mov as u8, ImmToMem as u8, 0x00, 0x00, 0x00, 0x00 ,0x00, 0x00, 0x00, 'H' as u8, stdout[0], stdout[1], stdout[2], stdout[3], stdout[4], stdout[5], stdout[6], stdout[7],
        Mov as u8, ImmToMem as u8, 0x00, 0x00, 0x00, 0x00 ,0x00, 0x00, 0x00, 'e' as u8, stdout[0], stdout[1], stdout[2], stdout[3], stdout[4], stdout[5], stdout[6], stdout[7],
        Mov as u8, ImmToMem as u8, 0x00, 0x00, 0x00, 0x00 ,0x00, 0x00, 0x00, 'l' as u8, stdout[0], stdout[1], stdout[2], stdout[3], stdout[4], stdout[5], stdout[6], stdout[7],
        Mov as u8, ImmToMem as u8, 0x00, 0x00, 0x00, 0x00 ,0x00, 0x00, 0x00, 'l' as u8, stdout[0], stdout[1], stdout[2], stdout[3], stdout[4], stdout[5], stdout[6], stdout[7],
        Mov as u8, ImmToMem as u8, 0x00, 0x00, 0x00, 0x00 ,0x00, 0x00, 0x00, 'o' as u8, stdout[0], stdout[1], stdout[2], stdout[3], stdout[4], stdout[5], stdout[6], stdout[7],
        Mov as u8, ImmToMem as u8, 0x00, 0x00, 0x00, 0x00 ,0x00, 0x00, 0x00, ',' as u8, stdout[0], stdout[1], stdout[2], stdout[3], stdout[4], stdout[5], stdout[6], stdout[7],
        Mov as u8, ImmToMem as u8, 0x00, 0x00, 0x00, 0x00 ,0x00, 0x00, 0x00, ' ' as u8, stdout[0], stdout[1], stdout[2], stdout[3], stdout[4], stdout[5], stdout[6], stdout[7],
        Mov as u8, ImmToMem as u8, 0x00, 0x00, 0x00, 0x00 ,0x00, 0x00, 0x00, 'W' as u8, stdout[0], stdout[1], stdout[2], stdout[3], stdout[4], stdout[5], stdout[6], stdout[7],
        Mov as u8, ImmToMem as u8, 0x00, 0x00, 0x00, 0x00 ,0x00, 0x00, 0x00, 'o' as u8, stdout[0], stdout[1], stdout[2], stdout[3], stdout[4], stdout[5], stdout[6], stdout[7],
        Mov as u8, ImmToMem as u8, 0x00, 0x00, 0x00, 0x00 ,0x00, 0x00, 0x00, 'r' as u8, stdout[0], stdout[1], stdout[2], stdout[3], stdout[4], stdout[5], stdout[6], stdout[7],
        Mov as u8, ImmToMem as u8, 0x00, 0x00, 0x00, 0x00 ,0x00, 0x00, 0x00, 'l' as u8, stdout[0], stdout[1], stdout[2], stdout[3], stdout[4], stdout[5], stdout[6], stdout[7],
        Mov as u8, ImmToMem as u8, 0x00, 0x00, 0x00, 0x00 ,0x00, 0x00, 0x00, 'd' as u8, stdout[0], stdout[1], stdout[2], stdout[3], stdout[4], stdout[5], stdout[6], stdout[7],
        Mov as u8, ImmToMem as u8, 0x00, 0x00, 0x00, 0x00 ,0x00, 0x00, 0x00, '!' as u8, stdout[0], stdout[1], stdout[2], stdout[3], stdout[4], stdout[5], stdout[6], stdout[7],
        Mov as u8, ImmToMem as u8, new_line[0], new_line[1], new_line[2], new_line[3], new_line[4], new_line[5], new_line[6], new_line[7], stdout[0], stdout[1], stdout[2], stdout[3], stdout[4], stdout[5], stdout[6], stdout[7],
        Hlt as u8, Null as u8,
    ];

    let mut rom = Rom::new(ROM_SIZE);

    rom.flash(&program);

    rom
}
