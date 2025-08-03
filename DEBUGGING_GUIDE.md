# Rusty 16-bit VM Debugging Guide

This guide covers the debugging features of the Rusty 16-bit VM and provides tips for effective program inspection and troubleshooting.

## Running Programs

There are several ways to run programs in the Rusty 16-bit VM:

### Using the Command Line

To run a program with the VM directly:

```bash
# Compile an assembly file to hex bytecode
cargo run --bin asm -- path/to/program.asm > output.hex

# Run the VM with the compiled hex file
cargo run --bin vm -- output.hex
```

### Using the Makefile

For convenience, you can use the provided Makefile:

```bash
# Compile the assembly file in prog/add.asm to hex bytecode
make gen-hex

# Run the VM with the generated binary
make run
```

## Debugging Mode

The VM includes a step-by-step debugging mode that allows you to inspect the state of the VM after each instruction.

### Activating Manual Mode

To run the VM in manual (debugging) mode, add the `--manual` flag:

```bash
cargo run --bin vm -- prog.hex --manual
```

### Manual Mode Controls

When in manual mode, you'll see a prompt after each instruction:

```
Press Enter to step, enter 's' to print state, or type 'exit' to quit...
```

Available commands:
- **Enter**: Execute the next instruction
- **s**: Show the current VM state (registers, stack, next instruction)
- **exit**: Terminate the VM and exit

### Understanding State Output

When you enter 's' in manual mode, you'll see output like this:

```
[State] PC=0x0004 | SP=0x1004 | FLAGS=0b00000000
Regs: A=0x0000(0) B=0x0000(0) C=0x0000(0) M=0x0000(0)
     R0=0x0000(0) R1=0x0000(0) R2=0x0000(0) R3=0x0000(0) R4=0x0000(0)
Stack: [0x1002]=0x0018(24) [0x1000]=0x000A(10)
Next: 0x0004 | AddStack
```

This display shows:

1. **Header**: Program Counter (PC), Stack Pointer (SP), and FLAGS values
2. **Registers**: All register values in both hexadecimal and decimal formats
3. **Stack**: Up to 3 items from the top of the stack with their memory addresses
4. **Next Instruction**: The instruction that will be executed next

## Debugging Tips

### 1. Watch the Stack

The stack grows upward from address 0x1000. When values are pushed onto the stack, SP increases. When values are popped, SP decreases.

Stack errors to watch for:
- **Stack underflow**: Attempting to pop from an empty stack (SP < 0x1000)
- **Stack overflow**: Pushing beyond available memory

### 2. Track Register Values

Pay attention to how registers change after each instruction. The state output makes it easy to see which registers were modified.

### 3. Inspect the Next Instruction

The "Next" line in the state output shows what instruction will be executed next. This helps you anticipate what changes to expect.

### 4. Common Issues

Some common issues to look for:

- **Incorrect values in registers**: May indicate a bug in your assembly code logic
- **Unexpected stack contents**: Could suggest push/pop imbalance or incorrect calculation
- **SP decreasing below 0x1000**: Indicates a stack underflow (popping too many items)
- **PC jumping to unexpected locations**: May indicate issues with control flow

## Advanced Debugging

For more complex programs, consider:

1. **Adding comments**: Comment your assembly code thoroughly to make debugging easier
2. **Breaking programs into sections**: Use labels and comments to divide your program logically
3. **Using registers consistently**: Establish conventions for register usage (e.g., A for results, B and C for operands)
4. **Testing incrementally**: Debug small sections before combining them into larger programs

## Modifying the VM

If you need to extend the VM's debugging capabilities:

1. The debugging code is primarily in `src/machine.rs`
2. The manual mode handling is in `src/bin/vm/main.rs`
3. You can enhance state output by modifying the `print_intermediate_state()` method

## Example Debugging Session

Here's a walkthrough of debugging a simple addition program:

1. Start the VM in manual mode:
   ```bash
   cargo run --bin vm -- prog.hex --manual
   ```

2. Press Enter to execute the first instruction:
   ```
   Instruction: opcode=0x01, arg=0x0A @ PC=0 => Push(10), SP=0x1000
   ```

3. Enter 's' to see the VM state:
   ```
   [State] PC=0x0002 | SP=0x1002 | FLAGS=0b00000000
   Regs: A=0x0000(0) B=0x0000(0) C=0x0000(0) M=0x0000(0)
        R0=0x0000(0) R1=0x0000(0) R2=0x0000(0) R3=0x0000(0) R4=0x0000(0)
   Stack: [0x1000]=0x000A(10)
   Next: 0x0002 | Push(24)
   ```

4. Continue stepping through instructions, checking the state as needed to understand program execution.

This interactive approach allows you to observe exactly how your program executes and helps identify any issues in your assembly code.
