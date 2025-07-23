//! Binary converter for the Rusty 16-bit VM.
//!
//! This program reads a text file containing hexadecimal values and converts them
//! into a binary format that can be executed by the VM.
//!
//! # Usage
//!
//! ```
//! cargo run --bin bin -- path/to/hexfile > output.bin
//! ```
//!
//! # Input Format
//!
//! The input file should contain space-separated hexadecimal values (00-FF).
//! Each value represents one byte of the program.
//!
//! Example:
//! ```
//! 01 0A 01 08 03 00 02 00 05 09
//! ```
//! This represents:
//! - PUSH 10
//! - PUSH 8
//! - ADDSTACK
//! - POP to register A
//! - SIGNAL 9 (halt)

use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

/// Main entry point for the binary converter.
///
/// This function:
/// 1. Reads a text file containing hexadecimal values
/// 2. Converts each hex value to a binary byte
/// 3. Outputs the binary data to stdout
///
/// # Arguments
///
/// * First argument: Path to the input file with hex values
///
/// # Returns
///
/// * `Ok(())` - If conversion was successful
/// * `Err(String)` - Error message if any step failed
fn main() -> Result<(), String> {
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

    // Process:
    // 1. Read the file line by line
    //
    // 2. For each line break that into multiple parts
    // divided by spaces
    //
    // 3. Each part should be a hexadecimal value (00-FF)
    //
    // 4. Parse them into binary bytes

    let lines: Vec<String> = match BufReader::new(file).lines().collect() {
        Ok(lines) => lines,
        Err(e) => {
            return Err(format!("cannot read the file due to - {}", e));
        }
    };

    // Parse the hexadecimal values to binary bytes
    let mut outputs: Vec<u8> = Vec::new();

    for l in lines {
        // Skip empty lines or handle comments (lines that start with semicolon)
        if l.trim().is_empty() || l.trim_start().starts_with(';') {
            continue;
        }

        // Split by spaces and convert each hex value to a byte
        for t in l.split(" ").filter(|x| !x.is_empty()) {
            // Parse the hex string to a u8 value
            let b = u8::from_str_radix(t, 16)
                .map_err(|e| format!("Failed to parse hex value '{}': {}", t, e))?;
            outputs.push(b);
        }
    }

    // Write the binary data to stdout
    let mut out = io::stdout().lock();
    out.write_all(&outputs)
        .map_err(|x| format!("Failed to write output: {}", x))?;

    Ok(())
}
