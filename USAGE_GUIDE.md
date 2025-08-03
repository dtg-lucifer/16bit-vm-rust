# Rusty 16-bit VM Usage Guide

This guide covers how to use the Rusty 16-bit VM, including writing programs, assembling code, and running the VM.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/rusty-16bit-vm.git
cd rusty-16bit-vm

# Compile an assembly file and run it
cargo run --bin asm -- prog/add.asm > prog.hex
cargo run --bin vm -- prog.hex
```

## Writing Assembly Programs

The Rusty 16-bit VM uses a simple assembly language. Programs are stored in files with the `.asm` extension.

### Basic Syntax

- One instruction per line
- Comments start with `;`
- Labels end with `:`
- Instructions are case-insensitive (both `PUSH` and `push` work)
- Register names are case-insensitive (both `A` and `a` work)

### Numeric Values

- Decimal numbers are prefixed with `%` (e.g., `%10`)
- Hexadecimal numbers are prefixed with `$` (e.g., `$0A`)

### Example Program

```asm
; Add two numbers and store result in register A
push %10            ; Push 10 onto the stack
push %20            ; Push 20 onto the stack
adds                ; Add the top two values on the stack
pop A               ; Pop the result into register A

; Signal to halt the VM
sig $09             ; Send signal 9 (halt)
```

## Assembling Programs

The VM includes an assembler to convert assembly code into bytecode.

```bash
# Basic usage
cargo run --bin asm -- input.asm > output.hex

# Example
cargo run --bin asm -- prog/add.asm > prog.hex
```

## Running Programs

After assembling your program, you can run it in the VM.

### Normal Execution

```bash
# Run the VM with the hex file
cargo run --bin vm -- prog.hex
```

### Manual/Debug Mode

```bash
# Run in manual mode for step-by-step execution
cargo run --bin vm -- prog.hex --manual
```

In manual mode:
- Press **Enter** to execute the next instruction
- Enter **s** to display the VM state
- Enter **exit** to quit

See [DEBUGGING_GUIDE.md](DEBUGGING_GUIDE.md) for more detailed debugging instructions.

## Using the Makefile

The VM includes a Makefile with common operations:

```bash
# Generate hex file from prog/add.asm
make gen-hex

# Run the VM with the generated hex file
make run

# Run the VM in manual mode
make step
```

## Working with the VM

### Memory Structure

The VM has 8 KB of memory organized as:
- Program area: Starts at address 0x0000
- Stack area: Starts at address 0x1000 (grows upward)

### Registers

The VM has 13 registers:
- General purpose: `A`, `B`, `C`
- Memory operations: `M`
- System registers: `SP` (Stack Pointer), `PC` (Program Counter), `BP` (Base Pointer), `FLAGS`
- Extended registers: `R0`, `R1`, `R2`, `R3`, `R4`

## Common Instructions

| Instruction | Description | Example |
|-------------|-------------|---------|
| `PUSH %n`   | Push decimal value onto stack | `PUSH %10` |
| `PUSH $n`   | Push hex value onto stack | `PUSH $0A` |
| `POP reg`   | Pop value into register | `POP A` |
| `PUSHR reg` | Push register value onto stack | `PUSHR B` |
| `ADDS`      | Add top two stack values | `ADDS` |
| `ADDR r1 r2`| Add registers, result in r1 | `ADDR A B` |
| `NOP`       | No operation | `NOP` |
| `SIG $n`    | Signal the VM | `SIG $09` |

For a full instruction reference, see [ASSEMBLY_REFERENCE.md](ASSEMBLY_REFERENCE.md).

## Example Workflow

1. Write your assembly program in a `.asm` file
2. Assemble the program to generate bytecode:
   ```bash
   cargo run --bin asm -- your_program.asm > your_program.hex
   ```
3. Run the program in the VM:
   ```bash
   cargo run --bin vm -- your_program.hex
   ```
4. If needed, debug using manual mode:
   ```bash
   cargo run --bin vm -- your_program.hex --manual
   ```

## Next Steps

- Experiment with the example programs in the `prog/` directory
- Write your own assembly programs
- Refer to the [ASSEMBLY_REFERENCE.md](ASSEMBLY_REFERENCE.md) for detailed instruction information
- See [DEBUGGING_GUIDE.md](DEBUGGING_GUIDE.md) for debugging techniques
