//! The main executable for the Rusty 16-bit VM.
//!
//! This program demonstrates how to use the VM by:
//! 1. Creating a new VM instance
//! 2. Loading a program from a file
//! 3. Executing the program until completion
//! 4. Displaying the final state of registers and memory
//!
//! # Usage
//!
//! ```
//! cargo run --bin vm -- path/to/program.hex
//! ```
//!
//! # Program Format
//!
//! The input file should contain bytecode for the VM. This can be generated
//! using either:
//!
//! - The assembler: `cargo run --bin asm -- path/to/assembly > program.hex`

use std::{
    env,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use rustyvm::Machine;

/// Signal handler for the halt operation (signal code 0x09).
///
/// This function is called when the VM executes a SIGNAL instruction
/// with code 0x09, indicating the program should halt execution.
///
/// # Parameters
///
/// * `vm` - Mutable reference to the VM instance
///
/// # Returns
///
/// * `Ok(())` - Always succeeds, setting the VM's halt flag
fn signal_halt(vm: &mut Machine) -> Result<(), String> {
    // This function is called when the VM halts
    // It can be used to perform any cleanup or final operations
    vm.halt = true;
    Ok(())
}

/// The main entry point for the VM runner application.
///
/// This function:
/// 1. Creates a new VM instance
/// 2. Registers the halt signal handler
/// 3. Loads a program from the specified file
/// 4. Executes the program until completion or error
/// 5. Displays the final state of the VM
///
/// # Command Line Arguments
///
/// * First argument: Path to the binary or hex program file to execute
///
/// # Returns
///
/// * `Ok(())` - If the program executed successfully
/// * `Err(String)` - Error message if execution failed
fn main() -> Result<(), String> {
    let mut vm = Machine::new();
    // Register the halt signal handler for signal code 0x09
    vm.define_handler(0x09, signal_halt);

    // ----------------------------------------------------------------
    // Load program from the specified file

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        return Err(format!("usage: {} <input>", args[0]));
    }

    let file: File = match File::open(Path::new(&args[1])) {
        Err(e) => {
            return Err(format!("failed to open the file, err - {}", e));
        }
        Ok(f) => f,
    };

    let mut buffer: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(file);

    let r = reader.read_to_end(&mut buffer);
    match r {
        Ok(s) => println!("Program: read {s} bytes from the file"),
        Err(e) => panic!("Error: cannot read, err = {e}"),
    }

    println!("Program: loaded program = {:?}", buffer);

    // Print the loaded program in hexadecimal format for better readability
    print!("Program: loaded program (hex) = [");
    for b in &buffer {
        print!("{b:02X}, ");
    }
    print!("]\n");

    // Load the program into memory at address 0
    // Each instruction occupies 2 consecutive bytes in memory:
    // - First byte (at even address): OPCODE (like 0x01 for PUSH)
    // - Second byte (at odd address): ARGUMENT (like 0x0A for value 10)
    //
    // When VM reads these bytes as a 16-bit value using little-endian format,
    // they appear as: ARGUMENT (upper 8 bits) | OPCODE (lower 8 bits)
    if let Some((bytes, instructions)) = vm.memory.load_from_vec(&buffer, 0) {
        println!(
            "Program: loaded {} bytes ({} instructions)",
            bytes, instructions
        );
        println!("Program: running loaded program...");
    }

    // Execute the program instruction by instruction until halted
    // This loop will continue until:
    // 1. A SIGNAL instruction with code 0x09 is executed (halt)
    // 2. An error occurs during execution
    // 3. The PC goes out of bounds
    // Execute instructions until halted or error occurs
    // Each instruction cycle:
    // 1. Read the opcode and argument from memory at PC (Program Counter)
    // 2. Increment PC by 2 (to point to the next instruction)
    // 3. Parse and execute the instruction
    // 4. Update VM state accordingly (registers, stack, etc.)
    while !vm.halt {
        match vm.step() {
            Ok(_) => continue, // Continue executing until halt
            Err(e) => {
                println!("Error during execution: {}", e);
                return Err(e);
            }
        }
    }

    // Print the final state
    vm.print_state();

    // Successful execution
    Ok(())
} // end of main
