use devices::rom::Rom;

mod cpu;
mod dev_map;
mod devices;
mod opcodes;
mod register;

const MEM_SIZE: usize = 1024; // 1 KiB

fn main() {
    // Create CPU
    let mut cpu = cpu::Cpu::new(MEM_SIZE);

    // Attach ROM
    let rom = Box::new(rom());
    cpu.attach(rom, String::from("rom"), 0);

    println!("{:#?}", cpu.dev_mapper.dump());

    // Run CPU
    cpu.run();
}

fn rom() -> Rom {
    use opcodes::Opcode::*;
    use register::Register::*;

    #[rustfmt::skip]
    let program = vec![
        // Init:

        // Move 1 to R1
        Mov as u8, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, Reg1 as u8, // MOV 0x0000000000000001, R1 (0x0000 - 0x000A)
        // Move 0 to R2
        Mov as u8, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, Reg2 as u8, // MOV 0x0000000000000000, R2 (0x000B - 0x0015)
        // Move 10 to R3
        Mov as u8, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, Reg3 as u8, // MOV 0x000000000000000A, R3 (0x0016 - 0x0020)

        // Loop:

        // Add R1 to R2
        Add as u8, 0x30, Reg1 as u8, Reg2 as u8,                                     // ADD R1, R2 (0x0021 - 0x0027)
        // Move R2 to R1
        Mov as u8, 0x30, Reg2 as u8, Reg1 as u8,                                     // MOV R2, R1 (0x0028 - 0x002E)
        // Move 1 to R3
        Sub as u8, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, Reg3 as u8, // SUB 0x0000000000000001, R3 (0x002F - 0x0039)
        // Move to loop if R3 is not 0
        Jnz as u8, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21,             // JNZ loop (0x003A - 0x0044)

        // Crash the CPU
        0x69
    ];

    let mut rom = Rom::new(program.len());

    // Print rom per instruction (for debugging)
    for (i, byte) in program.iter().enumerate() {
        println!("0x{:04X}: 0x{:02X}", i, byte);
    }

    rom.flash(&program);

    rom.dump();

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
