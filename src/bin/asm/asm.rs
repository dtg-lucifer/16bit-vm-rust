//! Assembler module for Rusty 16-bit VM.
//!
//! This module provides functionality to parse assembly language instructions
//! and convert them to the corresponding bytecode for the VM.
//!
//! # Assembly Language Format
//!
//! The assembler supports the following instructions:
//! - `PUSH #n` - Push decimal value n onto stack
//! - `PUSH $n` - Push hexadecimal value n onto stack
//! - `POP reg` - Pop value from stack into register
//! - `ADDS` - Add top two values on stack and push result
//! - `SIG $n` - Signal the VM with code n
//!
//! # Example
//!
//! ```
//! PUSH #10    ; Push decimal 10
//! PUSH #20    ; Push decimal 20
//! ADDS        ; Add: 10 + 20 = 30
//! POP A       ; Store result in Register A
//! SIG $09     ; Halt signal
//! ```

use rustyvm::{Op, Register};

/// Parses a vector of instruction parts into bytecode.
///
/// This function converts assembly instructions (split into parts) into
/// the corresponding bytecode that can be executed by the VM.
///
/// # Parameters
/// * `parts` - Vector of instruction parts (e.g., ["PUSH", "#10"])
///
/// # Returns
/// * `Ok(Vec<u8>)` - Vector of bytecode if parsing was successful
/// * `Err(String)` - Error message if parsing failed
pub fn parse_parts(parts: Vec<&str>) -> Result<Vec<u8>, String> {
    let mut outputs: Vec<u8> = Vec::new();
    let mut i = 0;
    while i < parts.len() {
        match parts[i] {
            "PUSH" => {
                outputs.push(Op::Push(0).value());
                if i + 1 < parts.len() {
                    let next_part = parts[i + 1];
                    if next_part.starts_with('#') {
                        let value = next_part.trim_start_matches('#');
                        let parsed_value = parse_decimal(value)?;
                        outputs.push(parsed_value);
                        i += 2; // Skip the value we just processed
                        continue;
                    } else if next_part.starts_with('$') {
                        let value = next_part.trim_start_matches('$');
                        let parsed_value = parse_hexadecimal(value)?;
                        outputs.push(parsed_value);
                        i += 2; // Skip the value we just processed
                        continue;
                    }
                }
                // If we get here, we didn't find a valid operand
                return Err(format!("Missing or invalid operand for PUSH instruction"));
            }
            "ADDS" => {
                outputs.push(Op::AddStack.value());
                outputs.push(0); // AddStack doesn't use the second byte, but we need it
                i += 1;
                continue;
            }
            "POP" => {
                outputs.push(Op::PopRegister(Register::A).value());
                if i + 1 < parts.len() {
                    let reg = parts[i + 1];

                    if reg.starts_with('$') {
                        // Handle register values specified in hex
                        let value = reg.trim_start_matches('$');
                        let parsed_value = parse_hexadecimal(value)?;
                        outputs.push(parsed_value);
                        i += 2; // Skip the register value we just processed
                        continue;
                    } else {
                        // Parse register name to its enum value
                        let r = Register::from_str(reg)
                            .map_err(|_| format!("Invalid register name: {}", reg))?;
                        // Push the enum discriminant value (0 for A, 1 for B, etc.)
                        outputs.push(r as u8);
                        i += 2; // Skip the register we just processed
                        continue;
                    }
                } else {
                    return Err(format!("Missing register for POP instruction"));
                }
            }
            "SIG" => {
                outputs.push(Op::Signal(0).value());
                if i + 1 < parts.len() && parts[i + 1].starts_with('$') {
                    let value = parts[i + 1].trim_start_matches('$');
                    let parsed_value = parse_hexadecimal(value)?;
                    outputs.push(parsed_value);
                    i += 2;
                    continue;
                } else {
                    return Err(format!(
                        "Missing or invalid signal value for SIG instruction"
                    ));
                }
            }
            _ => {
                return Err(format!("Unknown instruction: {}", parts[i]));
            }
        }
    }

    Ok(outputs)
}

/// Parses a decimal string into an 8-bit unsigned integer.
///
/// # Parameters
/// * `s` - String representing a decimal number
///
/// # Returns
/// * `Ok(u8)` - The parsed 8-bit value
/// * `Err(String)` - Error message if parsing failed
fn parse_decimal(s: &str) -> Result<u8, String> {
    u8::from_str_radix(s, 10).map_err(|e| format!("Failed to parse '{}' as decimal: {}", s, e))
}

/// Parses a hexadecimal string into an 8-bit unsigned integer.
///
/// # Parameters
/// * `s` - String representing a hexadecimal number
///
/// # Returns
/// * `Ok(u8)` - The parsed 8-bit value
/// * `Err(String)` - Error message if parsing failed
fn parse_hexadecimal(s: &str) -> Result<u8, String> {
    u8::from_str_radix(s, 16).map_err(|e| format!("Failed to parse '{}' as hexadecimal: {}", s, e))
}
