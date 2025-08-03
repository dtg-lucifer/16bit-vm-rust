# Rusty 16-bit VM Architecture

This document explains the core architecture of the Rusty 16-bit VM in pseudocode and diagrams to help you understand how it works.

## Memory Layout

The VM has 8 KB (8192 bytes) of memory laid out as follows:

```
+------------------------+ 0x0000
|                        |
|    Program Memory      |
|                        |
+------------------------+ ~0x1000
|                        |
|    Stack Memory        |
|  (grows upward)        |
|                        |
+------------------------+ 0x1FFF
```

## Register Set

Each register is 16 bits (2 bytes) wide:

```
+----------------+----------------+
| Name | Purpose | Initial Value  |
+------+---------+----------------+
| A    | General | 0              |
| B    | General | 0              |
| C    | General | 0              |
| M    | Memory  | 0              |
| SP   | Stack   | 0x1000         |
| PC   | Program | 0              |
| BP   | Base    | 0              |
| FLAGS| Status  | 0              |
| R0   | General | 0              |
| R1   | General | 0              |
| R2   | General | 0              |
| R3   | General | 0              |
| R4   | General | 0              |
+------+---------+----------------+
```

Register categories:
- **A, B, C**: Dual-purpose registers used for general storage but also as implicit operands
- **M, SP, PC, BP, FLAGS**: System registers with dedicated functions
- **R0-R4**: Pure general-purpose registers for data storage only

## Core VM Loop

Here's the core VM loop in pseudocode:

```
function run_vm():
    initialize_registers()

    while not halted:
        # Fetch instruction
        opcode = memory[PC]
        argument = memory[PC + 1]

        # Advance PC
        PC = PC + 2

        # Decode and execute
        execute_instruction(opcode, argument)

        # Check for termination conditions
        if PC >= memory_size or error_occurred:
            halt()

function execute_instruction(opcode, argument):
    match opcode:
        case 0x00:  # NOP
            # Do nothing

        case 0x01:  # PUSH
            # Push argument onto stack
            memory[SP] = argument
            SP = SP + 2

        case 0x02:  # POPREGISTER
            # Pop value from stack into register
            SP = SP - 2
            registers[argument] = memory[SP]

        case 0x03:  # PUSHREGISTER
            # Push register value onto stack
            value = registers[argument]
            memory[SP] = value
            SP = SP + 2

        case 0x0F:  # ADDSTACK
            # Add top two values on stack
            SP = SP - 2
            value1 = memory[SP]
            SP = SP - 2
            value2 = memory[SP]
            result = value1 + value2
            memory[SP] = result
            SP = SP + 2

        case 0x04:  # ADDREGISTER
            # Add two registers (high and low 4 bits of argument)
            reg1 = argument & 0x0F
            reg2 = (argument >> 4) & 0x0F
            registers[reg1] = registers[reg1] + registers[reg2]

        case 0x04:  # ADDREGISTER
            # Add two registers (high and low 4 bits of argument)
            reg1 = (argument >> 4) & 0x0F
            reg2 = argument & 0x0F
            registers[reg1] = registers[reg1] + registers[reg2]

        case 0x09:  # SIGNAL
            # Signal with a specific code (can be used for halting or I/O)
            execute_signal(argument)

        case _:
            raise_error("Invalid opcode")
```

## Stack Operations

The stack operations work as follows:

### PUSH Operation

```
function push(value):
    memory[SP] = value & 0xFF        # Lower byte
    memory[SP + 1] = (value >> 8) & 0xFF  # Upper byte
    SP = SP + 2
```

### POP Operation

```
function pop():
    SP = SP - 2
    lower_byte = memory[SP]
    upper_byte = memory[SP + 1]
    return (upper_byte << 8) | lower_byte
```

## Instruction Format

Each instruction is encoded as 2 consecutive bytes:

```
+-----------------------------+-----------------------------+
|          First Byte         |         Second Byte         |
+-----------------------------+-----------------------------+
|           OPCODE            |          ARGUMENT           |
+-----------------------------+-----------------------------+
```

### Programming Instructions

There are two ways to write instructions to memory:

#### 1. Using 8-bit Operations (Recommended for Clarity)

```
// Write a PUSH 10 instruction
memory.write(0, 0x01);  // PUSH opcode
memory.write(1, 0x0A);  // Value 10
```

#### 2. Using 16-bit Operations (More Compact)

```
// Write a PUSH 10 instruction in one operation
memory.write2(0, 0x0A01);  // 0x01 = opcode, 0x0A = value 10
```

Both approaches produce identical results in memory, but the 8-bit operations can be easier to understand for beginners.

## Example Execution Trace

Here's an execution trace of a simple program that adds two numbers:

```
Initial state:
  PC = 0x0000, SP = 0x1000
  Memory[0x0000] = 0x01 (PUSH)
  Memory[0x0001] = 0x0A (Value 10)
  Memory[0x0002] = 0x01 (PUSH)
  Memory[0x0003] = 0x08 (Value 8)
  Memory[0x0004] = 0x03 (ADDSTACK)
  Memory[0x0005] = 0x00 (Not used)
  Memory[0x0006] = 0x02 (POPREGISTER)
  Memory[0x0007] = 0x00 (Register A)

Execution steps:
1. Fetch: opcode = 0x01, argument = 0x0A
   PC = 0x0002
   Execute: PUSH 10
   Stack: [10]
   SP = 0x1002

2. Fetch: opcode = 0x01, argument = 0x08
   PC = 0x0004
   Execute: PUSH 8
   Stack: [10, 8]
   SP = 0x1004

3. Fetch: opcode = 0x03, argument = 0x00
   PC = 0x0006
   Execute: ADDSTACK
   Pop: 8, Pop: 10, Push: 18
   Stack: [18]
   SP = 0x1002

4. Fetch: opcode = 0x02, argument = 0x00
   PC = 0x0008
   Execute: POPREGISTER 0 (Register A)
   Pop: 18, Register A = 18
   Stack: []
   SP = 0x1000

Final state:
  PC = 0x0008, SP = 0x1000
  Register A = 18
```

## Memory Access Patterns

### 16-bit Value Storage

The VM stores 16-bit values in memory using little-endian format:

```
+-------------+-------------+
| Address N   | Address N+1 |
+-------------+-------------+
| Lower Byte  | Upper Byte  |
+-------------+-------------+
```

For example, to store the value 0x1234:

- Memory[N] = 0x34 (lower byte)
- Memory[N+1] = 0x12 (upper byte)

#### Memory Access Methods

The VM provides both 8-bit and 16-bit memory access methods:

```
// 8-bit operations
byte = read(address)
write(address, byte)

// 16-bit operations
word = read2(address)
write2(address, word)
```

The 16-bit operations automatically handle the byte order conversion:

```
// Reading 16-bit value
function read2(address):
    low_byte = read(address)
    high_byte = read(address + 1)
    return (high_byte << 8) | low_byte

// Writing 16-bit value
function write2(address, value):
    write(address, value & 0xFF)         // low byte
    write(address + 1, (value >> 8) & 0xFF)  // high byte
```

## Future Architecture Extensions

The VM architecture could be extended with:

1. **Conditional Jumps**: Allow control flow based on register comparisons
2. **Subroutine Support**: Implement CALL/RETURN instructions using the stack
3. **Memory-Mapped I/O**: Designate special memory regions for device I/O
4. **Interrupts**: Support for handling asynchronous events
5. **Additional Operations**: More complex instructions like multiplication, division, or bitwise operations

### Example Extension: Multiplication

A multiplication instruction could be added as follows:

```
// MULSTACK (0x05) - Pop two values, multiply them, push result
function mulstack():
    value1 = pop()
    value2 = pop()
    result = value1 * value2
    push(result)

// Another example extension: MULREG (multiply registers)
function mulregister(reg1, reg2):
    registers[reg1] = registers[reg1] * registers[reg2]

// Sample Program (Calculating 8 * 3 = 24)
Memory[0] = 0x01  // PUSH
Memory[1] = 0x08  // Value 8
Memory[2] = 0x01  // PUSH
Memory[3] = 0x03  // Value 3
Memory[4] = 0x05  // MULSTACK
Memory[5] = 0x00  // No argument
Memory[6] = 0x02  // POPREGISTER
Memory[7] = 0x00  // Register A

// In 16-bit format:
memory.write2(0, 0x0801)  // PUSH 8
memory.write2(2, 0x0301)  // PUSH 3
memory.write2(4, 0x0005)  // MULSTACK
memory.write2(6, 0x0002)  // POPREGISTER A
```

## Implementation Considerations

When implementing the VM, consider:

1. **Bounds Checking**: Always verify memory accesses are within bounds
2. **Error Handling**: Implement graceful handling of invalid instructions
3. **Stack Overflow**: Detect when stack operations would exceed memory bounds
4. **Instruction Timing**: For more realistic emulation, model instruction timing

This document serves as a high-level guide to the VM architecture. The actual implementation may have additional details and optimizations.

## Assembly Language Support

The VM includes an assembler that translates human-readable assembly code into machine code bytecode. This makes it easier to write programs for the VM without having to manually encode the binary instructions.

### Assembly Syntax

The assembler supports the following instruction formats:

```
PUSH #10    ; Push decimal value 10 onto the stack
PUSH $0A    ; Push hexadecimal value 0A (10) onto the stack
POP A       ; Pop value from stack into register A
POP B       ; Pop value from stack into register B
POP C       ; Pop value from stack into register C
ADDS        ; Add top two stack values, push result
SIG $09     ; Signal with hexadecimal code 09
```

### Assembler Usage

The assembler reads assembly instructions from a file and outputs the corresponding bytecode:

```
cargo run --bin asm -- your_program_asm > program.hex
```

### Example Assembly Program

Here's a complete example program that adds two numbers and stores the result in Register B:

```
; Add two numbers and store in Register B
PUSH #10    ; Push 10 onto stack
POP A       ; Store in Register A
PUSH #20    ; Push 20 onto stack
POP B       ; Store in Register B
ADDR A B    ; Add: A = A + B (10 + 20 = 30)
PUSHR A     ; Push result (30) onto stack
POP R0      ; Store result in R0
SIG $09     ; Signal to halt the VM
```

This assembly code would be translated into the following bytecode:

```
01 0A 01 14 03 00 02 01 05 09
```

Where:

- `01 0A` = PUSH 10
- `01 14` = PUSH 20
- `03 00` = ADDSTACK
- `02 01` = POP to Register B
- `05 09` = SIGNAL 09 (halt)
