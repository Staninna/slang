use devices::{rom::Rom, stdout::Stdout};

mod cpu;
mod dev_map;
mod devices;
mod opcodes;
mod register;

const MEM_SIZE: usize = 1024 * 1024 * 1024 * 4; // 4GB
const ROM_SIZE: usize = 1024 * 1024; // 1MB
const STDOUT_ADDR: u64 = 0x0000_0000_FFFF_0000;

fn main() {
    // Create CPU
    let mut cpu = cpu::Cpu::new(MEM_SIZE);

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
    use opcodes::AddrMode::*;
    use opcodes::Opcode::*;

    // Stdout_addr as u8 list to easily flash to ROM
    let stdout = STDOUT_ADDR.to_be_bytes();

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
    ];

    let mut rom = Rom::new(ROM_SIZE);

    rom.flash(&program);

    rom.dump(program.len());

    rom
}

// ; Initialize value to be incremented
// MOV 0x0000000000000001, R1 ; Move immediate value to register R1

// ; Loop 10 times and increment the value in R1 each iteration
// MOV 0x0000000000000000, R2 ; Move immediate value to register R2
// MOV 0x000000000000000A, R3 ; Move immediate value to register R3
// loop:
//     ADD R1, R2              ; Add the value in R1 to R2
//     MOV R2, R1              ; Move the result back to R1
//     SUB 0x0000000000000001, R3 ; Decrement the loop counter
//     JNZ loop               ; Jump to "loop" if the zero flag is not set
