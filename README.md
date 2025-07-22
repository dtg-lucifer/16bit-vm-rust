# Rusty 16-bit VM

A simple 16-bit virtual machine implementation in Rust. This project provides a stack-based VM with a small instruction set to execute basic programs.

## Architecture Overview

The Rusty 16-bit VM is a stack-based virtual machine designed with simplicity in mind. It executes instructions sequentially, using a stack for data operations and registers for state management.

The VM is designed with the following components:

### Memory

- 8 KB (8192 bytes) of linear memory
- Each memory location stores a single byte (8 bits)
- Memory addresses range from `0x0000` to `0x1FFF`

### Registers

The VM includes 8 16-bit registers:

| Register | Index | Purpose           |
| -------- | ----- | ----------------- |
| A        | 0     | General purpose   |
| B        | 1     | General purpose   |
| C        | 2     | General purpose   |
| M        | 3     | Memory operations |
| SP       | 4     | Stack Pointer     |
| PC       | 5     | Program Counter   |
| BP       | 6     | Base Pointer      |
| FLAGS    | 7     | Status flags      |

### Stack

- The stack grows upward in memory (from lower to higher addresses)
- SP (Stack Pointer) points to the next available memory location
- Each stack entry is 16 bits (2 bytes) wide
- Stack operations:
    - Push: Write value at SP, then increment SP by 2
    - Pop: Decrement SP by 2, then read value at SP

Stack Visualization:

```
Memory Address    Content        Description
+-------------+  +---------+
| 0x1004      |  |         |    Next available position (current SP)
+-------------+  +---------+
| 0x1002      |  |  0x0012 |    Last pushed value (18)
+-------------+  +---------+
| 0x1000      |  |  0x000A |    Earlier pushed value (10)
+-------------+  +---------+
```

## Instruction Format

Each instruction occupies 2 consecutive bytes in memory:

```
+------------+------------+
| Byte 0     | Byte 1     |
+------------+------------+
| OPCODE     | ARGUMENT   |
+------------+------------+
```

Memory layout example showing instructions:

```
Memory:
+-------+-------+-------+-------+-------+-------+-------+-------+
| 0x00  | 0x01  | 0x02  | 0x03  | 0x04  | 0x05  | 0x06  | 0x07  |
+-------+-------+-------+-------+-------+-------+-------+-------+
| 0x01  | 0x0A  | 0x01  | 0x08  | 0x03  | 0x00  | 0x02  | 0x00  |
+-------+-------+-------+-------+-------+-------+-------+-------+
  PUSH    10      PUSH    8     ADDSTACK  --    POPREG   REG_A
   |___________|    |___________|  |___________|  |___________|
   Instruction 1    Instruction 2  Instruction 3  Instruction 4
```

- **OPCODE** (1 byte): Specifies the operation to perform
- **ARGUMENT** (1 byte): Parameter for the operation

## Instruction Set

| Opcode | Mnemonic    | Argument          | Description                                |
| ------ | ----------- | ----------------- | ------------------------------------------ |
| 0x00   | NOP         | (none)            | No operation                               |
| 0x01   | PUSH        | 8-bit value       | Push value onto stack                      |
| 0x02   | POPREGISTER | Register index    | Pop value from stack into register         |
| 0x03   | ADDSTACK    | (none)            | Pop two values, add them, push result      |
| 0x04   | ADDREGISTER | Two 4-bit indices | Add two registers, store in first register |
| 0x05   | SIGNAL      | 8-bit signal code | Signal the VM with a specific code         |

## Programming the VM

### Loading a Program

Programs are loaded directly into memory starting at address 0. Each instruction and its argument must be written to consecutive bytes.

Example using 8-bit memory operations (preferred method):

```rust
// Push 10 onto stack
vm.memory.write(0, 0x01);  // PUSH opcode
vm.memory.write(1, 10);    // Value 10

// Push 8 onto stack
vm.memory.write(2, 0x01);  // PUSH opcode
vm.memory.write(3, 8);     // Value 8

// Add the two values on stack
vm.memory.write(4, 0x03);  // ADDSTACK opcode
vm.memory.write(5, 0);     // Not used

// Pop result into register A
vm.memory.write(6, 0x02);  // POPREGISTER opcode
vm.memory.write(7, 0);     // Register A (index 0)
```

Alternative example using 16-bit memory operations:

```rust
// Each instruction is written as a single 16-bit value
// Format: opcode in lower 8 bits, argument in upper 8 bits

// Push 10 onto stack
vm.memory.write2(0, 0x0A01);  // 0x01 = PUSH, 0x0A = 10

// Push 8 onto stack
vm.memory.write2(2, 0x0801);  // 0x01 = PUSH, 0x08 = 8

// Add the two values on stack
vm.memory.write2(4, 0x0003);  // 0x03 = ADDSTACK, 0x00 = unused

// Pop result into register A
vm.memory.write2(6, 0x0002);  // 0x02 = POPREGISTER, 0x00 = Register A
```

Both approaches produce the same program in memory, but the 8-bit method is often easier to understand.

### Program Execution

1. PC starts at address 0
2. VM reads the opcode and argument at PC and PC+1
3. PC is incremented by 2 (to the next instruction)
4. The instruction is executed
5. Process repeats until program completion or error

## Program Examples

### Adding Two Numbers

This program adds two numbers (10 and 8) and stores the result in register A:

```
Address | Value | Description
--------|-------|------------
0x0000  | 0x01  | PUSH
0x0001  | 0x0A  | Value 10
0x0002  | 0x01  | PUSH
0x0003  | 0x08  | Value 8
0x0004  | 0x03  | ADDSTACK
0x0005  | 0x00  | (unused)
0x0006  | 0x02  | POPREGISTER
0x0007  | 0x00  | Register A
```

#### Execution Flow

Here's what happens when this program executes:

1. **Instruction 1: PUSH 10**

    ```
    PC = 0, SP = 0x1000
    Read opcode 0x01 (PUSH) and argument 0x0A (10)
    Action: Push 10 onto the stack
    Result: Memory[0x1000] = 10, SP = 0x1002, PC = 2
    ```

2. **Instruction 2: PUSH 8**

    ```
    PC = 2, SP = 0x1002
    Read opcode 0x01 (PUSH) and argument 0x08 (8)
    Action: Push 8 onto the stack
    Result: Memory[0x1002] = 8, SP = 0x1004, PC = 4
    ```

3. **Instruction 3: ADDSTACK**

    ```
    PC = 4, SP = 0x1004
    Read opcode 0x03 (ADDSTACK) and unused argument 0x00
    Action: Pop two values from stack, add them, push result
           Pop 8, then pop 10, compute 10 + 8 = 18, push 18
    Result: Memory[0x1002] = 18, SP = 0x1004, PC = 6
    ```

4. **Instruction 4: POPREGISTER A**
    ```
    PC = 6, SP = 0x1004
    Read opcode 0x02 (POPREGISTER) and argument 0x00 (Register A)
    Action: Pop value from stack, store in Register A
           Pop 18 and store in Register A
    Result: Register A = 18, SP = 0x1002, PC = 8
    ```

Final state: Register A contains 18

## Implementation Details

### Memory

Memory is implemented as a vector of bytes with bounds checking:

```rust
pub struct LinearMemory {
    bytes: Vec<u8>,
    size: usize,
}
```

The VM provides two ways to interact with memory:

1. **8-bit Operations**:
    - `read(addr)`: Read a single byte from memory
    - `write(addr, value)`: Write a single byte to memory

2. **16-bit Operations**:
    - `read2(addr)`: Read a 16-bit value from memory (2 consecutive bytes)
    - `write2(addr, value)`: Write a 16-bit value to memory

16-bit values are stored in memory using little-endian format:

```
Address N:   Lower 8 bits
Address N+1: Upper 8 bits
```

### Machine

The Machine struct ties everything together:

```rust
pub struct Machine {
    pub registers: [u16; 8],
    pub memory: Box<dyn Addressable>,
}
```

It provides methods for:

- Executing instructions: `step()`
- Stack manipulation: `push()` and `pop()`
- Register access: `get_register()`

## Building and Running

Clone the repository and use Cargo to build and run:

```bash
# Build the VM
cargo build

# Run the default example
cargo run
```

## Future Enhancements

Potential improvements for the VM:

- Additional instructions (subtraction, multiplication, division)
- Jump/branch instructions for control flow
- Memory-mapped I/O operations
- Assembler for easier program creation
- Support for functions and subroutines
- Extended memory addressing (beyond 8KB)
- Virtual I/O devices (terminal, disk, etc.)
- Interactive debugger with step-through execution

## Programming Techniques

### Using 8-bit vs 16-bit Operations

The VM supports both 8-bit and 16-bit memory operations:

**8-bit Operations (Recommended for Beginners)**:

```rust
// Writing a PUSH 10 instruction
vm.memory.write(0, 0x01);  // PUSH opcode at address 0
vm.memory.write(1, 10);    // Value 10 at address 1
```

**16-bit Operations (More Compact)**:

```rust
// Writing a PUSH 10 instruction (combining opcode and argument)
vm.memory.write2(0, 0x0A01);  // 0x01 = opcode, 0x0A = 10
```

In both cases, the resulting memory contains the same values:

- Memory[0] = 0x01 (PUSH opcode)
- Memory[1] = 0x0A (value 10)

### When to Use Each

- **8-bit Operations**: Better for understanding the VM's byte-by-byte operation
- **16-bit Operations**: More compact code when you're comfortable with the VM

## Debugging Tips

When writing programs for the VM:

1. **Track the Stack**: Monitor SP and the values on the stack after each operation
2. **Check Register Values**: Print register values at key points in your program
3. **Step Through Execution**: Execute one instruction at a time to identify issues
4. **Verify Memory Layout**: Ensure instructions are placed correctly in memory

Example debugging output:

```
Instruction: opcode=0x01, arg=0x0A @ PC=0 => Push(10)
SP: 0x1000
Instruction: opcode=0x01, arg=0x08 @ PC=2 => Push(8)
SP: 0x1002
Instruction: opcode=0x03, arg=0x00 @ PC=4 => AddStack
SP: 0x1004
AddStack: 10 + 8 = 18
Instruction: opcode=0x02, arg=0x00 @ PC=6 => PopRegister(A)
SP: 0x1002
Instruction: opcode=0x05, arg=0x09 @ PC=8 => Signal(9)
SP: 0x1000
```

## Assembly Language Support

The VM includes a basic assembler that can translate assembly language instructions into bytecode. The assembler supports the following syntax:

### Assembly Instructions

| Assembly  | Description                           | Example    |
| --------- | ------------------------------------- | ---------- |
| `PUSH #n` | Push decimal value n onto stack       | `PUSH #10` |
| `PUSH $n` | Push hexadecimal value n onto stack   | `PUSH $0A` |
| `POP reg` | Pop value from stack into register    | `POP A`    |
| `ADDS`    | Pop two values, add them, push result | `ADDS`     |
| `SIG $n`  | Signal the VM with hex code n         | `SIG $09`  |

### Assembly Example

```
PUSH #10    ; Push decimal 10 onto the stack
PUSH #24    ; Push decimal 24 onto the stack
ADDS        ; Add the two values (10+24=34)
POP B       ; Store result (34) in register B
PUSH #5     ; Push decimal 5 onto the stack
PUSH #22    ; Push decimal 22 onto the stack
ADDS        ; Add the two values (5+22=27)
POP C       ; Store result (27) in register C
PUSH #100   ; Push decimal 100 onto the stack
POP A       ; Store value (100) in register A
SIG $09     ; Signal to halt the VM
```

### Using the Assembler

```bash
# Compile assembly file to hexadecimal bytecode
cargo run --bin asm -- prog/your_program_asm > output.hex

# Run the VM with your program
cargo run --bin vm -- output.bin
```

### Makefile Commands

For convenience, you can use the provided Makefile:

```bash
# Generate binary from an assembly file
make gen-hex

# Run the VM with the generated binary
make run
```

## License

This project is open source and available under the [MIT License](LICENSE).
