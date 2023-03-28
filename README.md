<!-- markdownlint-disable MD033 -->

# Slang Virtual Machine

## Introduction

In this project, I aim to create a virtual machine named Slang in honor of myself. The Slang VM will be a 64-bit virtual machine, and my ultimate goal is to evolve it into a programming language.

## Memory

The Slang VM's memory is 64-bit, where each memory address points to 8 bits or 1 byte of data. (WIP)

## Stack

The Slang VM features a 64-bit stack that grows downwards. The stack pointer (`SP`) indicates the top of the stack and is decremented by 1 when data is pushed onto the stack, and incremented by 1 when data is popped off the stack. The stack pointer is initialized to the maximum value provided by the amount of memory allocated to the Slang VM.

## Registers

The Slang VM has 11 registers, including 8 general-purpose registers (R1-R8) and 3 special-purpose registers (ACC, IP, SP). The special-purpose registers have specific purposes:

- `ACC`: is the accumulator register, which is used to store the result some operations
- `IP`: Instruction pointer for storing the address of the next instruction to be executed
- `SP`: Stack pointer for storing the address of the top of the stack

Here's a table of the Slang VM registers:

| Register | Description         | Size   | Code   | Initial Value                |
| -------- | ------------------- | ------ | ------ | ---------------------------- |
| `ACC`    | Accumulator         | 64-bit | `0x01` | `0x00`                       |
| `IP`     | Instruction pointer | 64-bit | `0x02` | `0x00`                       |
| `SP`     | Stack pointer       | 64-bit | `0x03` | Dynamic based on memory size |
| `FP`     | Frame pointer       | 64-bit | `0x04` | `0x00`                       |
| `FS`     | Frame size          | 64-bit | `0x05` | `0x00`                       |
| `R1`     | Register 1          | 64-bit | `0x06` | `0x00`                       |
| `R2`     | Register 2          | 64-bit | `0x07` | `0x00`                       |
| `R3`     | Register 3          | 64-bit | `0x08` | `0x00`                       |
| `R4`     | Register 4          | 64-bit | `0x09` | `0x00`                       |
| `R5`     | Register 5          | 64-bit | `0x0A` | `0x00`                       |
| `R6`     | Register 6          | 64-bit | `0x0B` | `0x00`                       |
| `R7`     | Register 7          | 64-bit | `0x0C` | `0x00`                       |
| `R8`     | Register 8          | 64-bit | `0x0D` | `0x00`                       |

## Opcodes

The Slang VM supports the following opcodes:

| Opcode | Description  | Size  | Code   | Modes  |
| ------ | ------------ | ----- | ------ | ------ |
| `NOP`  | No operation | 8-bit | `0x00` | `NULL` |

| Opcode | Description | Size  | Code   | Modes                                                                     |
| ------ | ----------- | ----- | ------ | ------------------------------------------------------------------------- |
| `MOV`  | Move Data   | 8-bit | `0x01` | `IMM->REG`, `IMM->MEM`, `REG->REG`,<br>`REG->MEM`, `MEM->REG`, `MEM->MEM` |
| `LOD`  | Load Data   | 8-bit | `0x02` | `IMM->REG`, `MEM->REG`                                                    |
| `STR`  | Store Data  | 8-bit | `0x03` | `IMM->MEM`, `REG->MEM`, `MEM->MEM`                                        |

| Opcode | Description | Size  | Code   | Modes                              |
| ------ | ----------- | ----- | ------ | ---------------------------------- |
| `ADD`  | Add         | 8-bit | `0x04` | `IMM->REG`, `REG->REG`, `MEM->REG` |
| `SUB`  | Subtract    | 8-bit | `0x05` | `IMM->REG`, `REG->REG`, `MEM->REG` |
| `MUL`  | Multiply    | 8-bit | `0x06` | `IMM->REG`, `REG->REG`, `MEM->REG` |
| `DIV`  | Divide      | 8-bit | `0x07` | `IMM->REG`, `REG->REG`, `MEM->REG` |

| Opcode | Description         | Size  | Code   | Modes                  |
| ------ | ------------------- | ----- | ------ | ---------------------- |
| `AND`  | Bitwise And         | 8-bit | `0x08` | `REG->IMM`, `REG->REG` |
| `OR`   | Bitwise Or          | 8-bit | `0x09` | `REG->IMM`, `REG->REG` |
| `XOR`  | Bitwise Xor         | 8-bit | `0x0A` | `REG->IMM`, `REG->REG` |
| `NOT`  | Bitwise Not         | 8-bit | `0x0B` | `REGISTER`, `MEMORY`   |
| `SHL`  | Bitwise Shift left  | 8-bit | `0x0C` | `REG->IMM`, `REG->REG` |
| `SHR`  | Bitwise Shift right | 8-bit | `0x0D` | `REG->IMM`, `REG->REG` |

| Opcode | Description                      | Size  | Code   | Modes                 |
| ------ | -------------------------------- | ----- | ------ | --------------------- |
| `JMP`  | Jump to addr                     | 8-bit | `0x0E` | `REGISTER`, `LITERAL` |
| `JEQ`  | Jump if equal                    | 8-bit | `0x0F` | `REGISTER`, `LITERAL` |
| `JNE`  | Jump if not equal                | 8-bit | `0x10` | `REGISTER`, `LITERAL` |
| `JGT`  | Jump if greater than             | 8-bit | `0x11` | `REGISTER`, `LITERAL` |
| `JLT`  | Jump if less than                | 8-bit | `0x12` | `REGISTER`, `LITERAL` |
| `JGE`  | Jump if greater than or equal to | 8-bit | `0x13` | `REGISTER`, `LITERAL` |
| `JLE`  | Jump if less than or equal to    | 8-bit | `0x14` | `REGISTER`, `LITERAL` |
| `JNZ`  | Jump if not zero                 | 8-bit | `0x15` | `REGISTER`, `LITERAL` |
| `JZ`   | Jump if zero                     | 8-bit | `0x16` | `REGISTER`, `LITERAL` |

| Opcode | Description            | Size  | Code   | Modes                 |
| ------ | ---------------------- | ----- | ------ | --------------------- |
| `PSH`  | Push to top of stack   | 8-bit | `0x17` | `REGISTER`, `LITERAL` |
| `POP`  | Pop from top of stack  | 8-bit | `0x18` | `REGISTER`, `NULL`    |
| `DUP`  | Duplicate top of stack | 8-bit | `0x19` | `NULL`                |
| `SWP`  | Swap top of stack      | 8-bit | `0x1A` | `NULL`                |
| `CLR`  | Clear stack            | 8-bit | `0x1B` | `NULL`                |
| `RET`  | Return from subroutine | 8-bit | `0x1C` | `NULL`                |
| `CAL`  | Call subroutine        | 8-bit | `0x1D` | `NULL`                |

## Addressing Modes

The Slang VM supports the following addressing modes:

| Mode       | Description           | Size  | Code   | Operand Sizes    |
| ---------- | --------------------- | ----- | ------ | ---------------- |
| `IMM->REG` | Immediate to register | 8-bit | `0x10` | 64-bit -> 8-bit  |
| `IMM->MEM` | Immediate to memory   | 8-bit | `0x20` | 64-bit -> 64-bit |
|            |                       |       |        |                  |
| `REG->REG` | Register to register  | 8-bit | `0x30` | 8-bit -> 8-bit   |
| `REG->MEM` | Register to memory    | 8-bit | `0x40` | 8-bit -> 64-bit  |
|            |                       |       |        |                  |
| `MEM->REG` | Memory to register    | 8-bit | `0x50` | 64-bit -> 8-bit  |
| `MEM->MEM` | Memory to memory      | 8-bit | `0x60` | 64-bit -> 64-bit |
|            |                       |       |        |                  |
| `LITERAL`  | Literals              | 8-bit | `0x70` | 64-bit           |
| `REGISTER` | Registers             | 8-bit | `0x80` | 8-bit            |
| `MEMORY`   | Memory                | 8-bit | `0x90` | 64-bit           |
| `NULL`     | No operand            | 8-bit | `0xA0` | -                |

## Bytecode Format

The format of the bytecode is as follows:

| Opcode | Addressing Mode | Operand                            | Operand                            |
| ------ | --------------- | ---------------------------------- | ---------------------------------- |
| 8-bit  | 8-bit           | 64-bit (8 bytes) OR 8-bit (1 byte) | 64-bit (8 bytes) OR 8-bit (1 byte) |

## Maybe in the future

-   Call stack
-   Memory management (GC/Heap)
-   Virtual file system
