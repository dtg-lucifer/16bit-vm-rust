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
+------+---------+----------------+
```

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

        case 0x03:  # ADDSTACK
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

## Future Architecture Extensions

The VM architecture could be extended with:

1. **Conditional Jumps**: Allow control flow based on register comparisons
2. **Subroutine Support**: Implement CALL/RETURN instructions using the stack
3. **Memory-Mapped I/O**: Designate special memory regions for device I/O
4. **Interrupts**: Support for handling asynchronous events

## Implementation Considerations

When implementing the VM, consider:

1. **Bounds Checking**: Always verify memory accesses are within bounds
2. **Error Handling**: Implement graceful handling of invalid instructions
3. **Stack Overflow**: Detect when stack operations would exceed memory bounds
4. **Instruction Timing**: For more realistic emulation, model instruction timing

This document serves as a high-level guide to the VM architecture. The actual implementation may have additional details and optimizations.
