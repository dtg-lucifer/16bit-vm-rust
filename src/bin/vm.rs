//! The main executable for the Rusty 16-bit VM.
//!
//! This program demonstrates how to use the VM by:
//! 1. Creating a new VM instance
//! 2. Loading a simple program that adds two numbers
//! 3. Executing the program step by step
//! 4. Displaying the result

use std::{
    env,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

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

    // ----------------------------------------------------------------
    // Load program from .bin file

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

    print!("Program: loaded program (hex) = [");
    for b in &buffer {
        print!("{b:02X}, ");
    }
    print!("]\n");

    let res = vm.memory.read_from_vec(&mut buffer);
    match res {
        Some((_, op)) => {
            println!("Program: running loaded program...");
            // Execute the program step by step
            for _ in 0..(op as u32) {
                vm.step()?;
            }
        }
        None => (),
    }

    // Display the result stored in register A (should be 18)
    println!("A = {}", vm.get_register(Register::A));

    // Successful execution
    Ok(())
} // end of main
