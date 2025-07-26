//! The main executable for the Rusty 16-bit VM.

use std::{
    env,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use rustyvm::Machine;

/// Signal handler for the halt operation (signal code 0x09).
/// Sets the VM's halt flag when executed.
fn signal_halt(vm: &mut Machine) -> Result<(), String> {
    // This function is called when the VM halts
    // It can be used to perform any cleanup or final operations
    vm.halt = true;
    Ok(())
}

/// The main entry point for the VM runner application.
/// Creates VM, loads program, executes until completion, and displays state.
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
        Ok(_) => println!("Program: read successfully!"),
        Err(e) => panic!("Error: cannot read, err = {e}"),
    }

    // Load the program into memory at address 0
    if let Some((bytes, instructions)) = vm.memory.load_from_vec(&buffer, 0) {
        println!(
            "Program: loaded {} bytes ({} instructions)",
            bytes, instructions
        );
        println!("Program: running loaded program...");
    }

    // Execute instructions until halted or error occurs
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
