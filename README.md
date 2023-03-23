# Slang VM

## Introduction

I'd like to make another attempt at writing a virtual machine. This time, I'll try to create a 64-bit virtual machine and call it Slang, named after myself. My ultimate goal is to turn Slang into a programming language.

## Memory

The Slang VM has 64-bit memory. Every memory address points to 8-bits (1 byte) of data. There will be a stack in the future, but for now, there is only a heap.

## Registers

The Slang VM has ten registers, including eight general-purpose registers, one accumulator register, and one program counter register. I may add more registers in the future.

Here's a table of the Slang VM registers:

| Register | Description         | Size   | Code | Initial Value |
| -------- | ------------------- | ------ | ---- | ------------- |
| ACC      | Accumulator         | 64-bit | 0x00 | 0x00          |
| IP       | Instruction pointer | 64-bit | 0x01 | 0x00          |
| R1       | Register 1          | 64-bit | 0x02 | 0x00          |
| R2       | Register 2          | 64-bit | 0x03 | 0x00          |
| R3       | Register 3          | 64-bit | 0x04 | 0x00          |
| R4       | Register 4          | 64-bit | 0x05 | 0x00          |
| R5       | Register 5          | 64-bit | 0x06 | 0x00          |
| R6       | Register 6          | 64-bit | 0x07 | 0x00          |
| R7       | Register 7          | 64-bit | 0x08 | 0x00          |
| R8       | Register 8          | 64-bit | 0x09 | 0x00          |

## Opcodes

The Slang VM supports the following opcodes (for now):

| Opcode | Description  | Size  | Code | Modes         |
| ------ | ------------ | ----- | ---- | ------------- |
| NOP    | No operation | 8-bit | 0x00 | -             |
| MOV    | Move         | 8-bit | 0x01 | REG, MEM, IMM |
| LOD    | Load         | 8-bit | 0x02 | REG, MEM, IMM |
| STR    | Store        | 8-bit | 0x03 | REG, MEM, IMM |

## Addressing Modes

The Slang VM supports the following addressing modes:

| Mode | Description | Size  | Code |
| ---- | ----------- | ----- | ---- |
| REG  | Register    | 8-bit | 0x00 |
| MEM  | Memory      | 8-bit | 0x01 |
| IMM  | Immediate   | 8-bit | 0x02 |

## Bytecode Format

The format of the bytecode is as follows:

| Opcode | Addressing Mode | Operand          |
| ------ | --------------- | ---------------- |
| 8-bit  | 8-bit           | 64-bit (8 bytes) |

Some examples of bytecode:

```asm
; MOV R1, 0x123456789ABCDEF0
0x01 0x00 0x123456789ABCDEF0

; LOD R1, [R2]
0x02 0x01 0x02 ; 0x02 is padded to 8 bytes

; STR R1, [R2]
0x03 0x01 0x02 ; 0x02 is padded to 8 bytes

; STR R1, 0x123456789ABCDEF0
0x03 0x02 0x123456789ABCDEF0
```
