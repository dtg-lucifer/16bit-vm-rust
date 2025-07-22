# Rusty 16-bit VM Assembly Language Reference

This document serves as a comprehensive reference for the assembly language used with the Rusty 16-bit VM.

## Overview

The Rusty 16-bit VM assembly language provides a human-readable way to write programs for the VM. It features a minimal syntax with instructions for stack manipulation, register operations, arithmetic, and system control.

## Syntax Rules

- One instruction per line
- Instructions are case-sensitive (must be uppercase)
- Comments start with `;` and continue to the end of the line
- Decimal numbers are prefixed with `#` (e.g., `#10`)
- Hexadecimal numbers are prefixed with `$` (e.g., `$0A`)
- Register names are written directly (e.g., `A`, `B`, `C`)

## Registers

The VM has 8 16-bit registers:

| Register | Index | Purpose                                |
|----------|-------|----------------------------------------|
| A        | 0     | General purpose                        |
| B        | 1     | General purpose                        |
| C        | 2     | General purpose                        |
| M        | 3     | Memory operations                      |
| SP       | 4     | Stack Pointer (points to next free slot)|
| PC       | 5     | Program Counter                        |
| BP       | 6     | Base Pointer                           |
| FLAGS    | 7     | Status flags                           |

## Instructions

### Stack Operations

#### PUSH - Push value onto stack

Push an 8-bit value onto the stack.

**Syntax:**
- `PUSH #n` - Push decimal value n
- `PUSH $n` - Push hexadecimal value n

**Examples:**
```assembly
PUSH #10    ; Push decimal 10
PUSH $0A    ; Push hexadecimal 0A (also 10)
PUSH #255   ; Push maximum 8-bit value
```

**Encoding:**
- Opcode: `0x01`
- Argument: The value to push (8-bit)

#### POP - Pop value from stack into register

Pop the top value from the stack and store it in the specified register.

**Syntax:**
- `POP reg` - Pop into register (reg is A, B, C, etc.)

**Examples:**
```assembly
POP A       ; Pop value into register A
POP B       ; Pop value into register B
POP C       ; Pop value into register C
```

**Encoding:**
- Opcode: `0x02`
- Argument: Register index (0 for A, 1 for B, 2 for C, etc.)

### Arithmetic Operations

#### ADDS - Add Stack

Pop two values from the stack, add them, and push the result back onto the stack.

**Syntax:**
- `ADDS`

**Example:**
```assembly
PUSH #5     ; Push 5
PUSH #7     ; Push 7
ADDS        ; Pop 7, pop 5, push 12 (5+7)
```

**Encoding:**
- Opcode: `0x03`
- Argument: `0x00` (unused)

### System Operations

#### SIG - Signal

Send a signal to the VM. Signals can trigger special behavior like halting execution.

**Syntax:**
- `SIG $n` - Signal with hexadecimal code n

**Examples:**
```assembly
SIG $09     ; Halt signal
```

**Encoding:**
- Opcode: `0x05`
- Argument: Signal code (8-bit)

## Full Program Example

Here's a complete example program that:
1. Adds two numbers (10 + 24)
2. Stores the result in register B
3. Adds two more numbers (5 + 22)
4. Stores that result in register C
5. Stores a constant in register A
6. Halts the VM with a signal

```assembly
; Example program demonstrating register usage
PUSH #10    ; First operand
PUSH #24    ; Second operand
ADDS        ; Add: 10 + 24 = 34
POP B       ; Store sum in register B
PUSH #5     ; Third operand
PUSH #22    ; Fourth operand
ADDS        ; Add: 5 + 22 = 27
POP C       ; Store sum in register C
PUSH #100   ; Constant value
POP A       ; Store in register A
SIG $09     ; Halt VM
```

## Assembler Usage

### Compiling Assembly Code

Use the assembler to convert assembly language to bytecode:

```
cargo run --bin asm -- path/to/program_asm > output.hex
```

### Running the Program

Run the compiled program:

```
cargo run --bin vm -- output.hex
```

### Using the Makefile

For convenience, you can use the provided Makefile:

```bash
# Compile the assembly file in prog/add_asm to hex bytecode
make gen-hex

# Run the VM with the generated binary
make run
```

## Extending the Assembly Language

To extend the assembly language with new instructions:

1. Add a new operation in the VM's `Op` enum
2. Update the parser in the assembler to recognize the new syntax
3. Update the VM's instruction decoder to handle the new opcode

## Best Practices

1. Use comments liberally to explain your code
2. Use meaningful register assignments (A, B, C for different purposes)
3. Structure your code with empty lines between logical sections
4. Keep instructions organized in a top-down flow
5. Use consistent indentation for readability

## Limitations

- Currently, there is no direct instruction for pushing register values onto the stack
- No conditional branching or jump instructions
- No direct memory addressing operations
- Limited to 8-bit immediate values in instructions
