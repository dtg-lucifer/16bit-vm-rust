//! The main executable for the Rusty 16-bit VM.
//!
//! This program demonstrates how to use the VM by:
//! 1. Creating a new VM instance
//! 2. Loading a simple program that adds two numbers
//! 3. Executing the program step by step
//! 4. Displaying the result

use rustyvm::{Machine, Register};

/// The main entry point for the VM demo application.
///
/// Creates a VM instance, loads a demo program that adds two numbers (10 + 8),
/// executes the program step by step, and prints the result stored in Register A.
///
/// # Returns
/// * `Ok(())` - If the program executed successfully
/// * `Err(String)` - Error message if execution failed
fn main() -> Result<(), String> {
    let mut vm = Machine::new();

    // Demo program: This simple program demonstrates basic VM functionality
    // by adding two numbers and storing the result in a register.
    //
    // The program consists of 4 instructions:
    // 1. PUSH 10    - Push the value 10 onto the stack
    // 2. PUSH 8     - Push the value 8 onto the stack
    // 3. ADDSTACK   - Pop the two values, add them, and push the result (18)
    // 4. POPREGISTER A - Pop the result and store it in register A
    //
    // Alternative 16-bit method for program loading:
    //
    // The VM supports writing instructions as 16-bit values, where:
    // - Lower 8 bits contain the opcode
    // - Upper 8 bits contain the argument
    //
    // The same program could be written as:
    // vm.memory.write2(0, 0x1 | (10 << 8)); // PUSH 10 (0x0A01)
    // vm.memory.write2(2, 0x1 | (8 << 8));  // PUSH 8 (0x0801)
    // vm.memory.write2(4, 0x3);             // ADDSTACK (0x0003)
    // vm.memory.write2(6, 0x2 | (0 << 8));  // POPREGISTER A (0x0002)

    // INSTRUCTION 1: PUSH 10
    // This instruction pushes the value 10 onto the stack
    vm.memory.write(0, 0x1); // Opcode: 0x1 = PUSH
    vm.memory.write(1, 10); // Argument: 10 (value to push)
    // 16-bit equivalent: vm.memory.write2(0, 0x0A01); // 0x01 = PUSH, 0x0A = 10

    // INSTRUCTION 2: PUSH 8
    // This instruction pushes the value 8 onto the stack
    vm.memory.write(2, 0x1); // Opcode: 0x1 = PUSH
    vm.memory.write(3, 8); // Argument: 8 (value to push)
    // 16-bit equivalent: vm.memory.write2(2, 0x0801); // 0x01 = PUSH, 0x08 = 8

    // INSTRUCTION 3: ADDSTACK
    // This instruction pops two values from the stack (8, then 10),
    // adds them (10 + 8 = 18), and pushes the result (18) back onto the stack
    vm.memory.write(4, 0x3); // Opcode: 0x3 = ADDSTACK
    vm.memory.write(5, 0); // No argument needed for ADDSTACK
    // 16-bit equivalent: vm.memory.write2(4, 0x0003); // 0x03 = ADDSTACK, 0x00 = unused

    // INSTRUCTION 4: POPREGISTER A
    // This instruction pops the value (18) from the stack and stores it in register A
    vm.memory.write(6, 0x2); // Opcode: 0x2 = POPREGISTER
    vm.memory.write(7, 0); // Argument: 0 = Register A (see Register enum)
    // 16-bit equivalent: vm.memory.write2(6, 0x0002); // 0x02 = POPREGISTER, 0x00 = Register A

    // Execute the program step by step
    vm.step()?; // Execute PUSH 10
    vm.step()?; // Execute PUSH 8
    vm.step()?; // Execute ADDSTACK
    vm.step()?; // Execute POPREGISTER A

    // Display the result stored in register A (should be 18)
    println!("A = {}", vm.get_register(Register::A));

    // Example of a more complex program (commented out for future implementation)
    //
    // This example shows how to implement a more complex program that would
    // calculate (5 + 3) * 2 = 16 and store the result in register A.
    // Note: This would require implementing a MULSTACK instruction.
    //
    // // Reset VM state
    // let mut vm = Machine::new();
    //
    // // Program to calculate (5 + 3) * 2
    // vm.memory.write2(0, 0x0501);  // PUSH 5
    // vm.memory.write2(2, 0x0301);  // PUSH 3
    // vm.memory.write2(4, 0x0003);  // ADDSTACK    -> 8 on stack
    // vm.memory.write2(6, 0x0201);  // PUSH 2
    // vm.memory.write2(8, 0x????);  // MULSTACK    -> 16 on stack (would need to implement)
    // vm.memory.write2(10, 0x0002); // POPREG A    -> A = 16
    //
    // // Execution of complex program would be:
    // for _ in 0..6 {
    //    vm.step()?;
    // }

    // Successful execution
    Ok(())
} // end of main
