//! Binary converter for the Rusty 16-bit VM.
//!
//! Converts hexadecimal text values to binary format.

use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

/// Main entry point for the binary converter.
/// Reads hex values from a file, converts to binary, outputs to stdout.
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

    // Process the file and convert hex values to binary bytes

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
