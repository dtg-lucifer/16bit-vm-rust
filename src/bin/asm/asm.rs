//! Assembler module for Rusty 16-bit VM.
//!
//! Supports instructions: PUSH #n, PUSH $n, POP reg, ADDS, SIG $n

use rustyvm::{Op, Register};

/// Parses a vector of instruction parts into bytecode.
/// Takes parts like ["PUSH", "#10"] and converts to bytecode.
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
fn parse_decimal(s: &str) -> Result<u8, String> {
    u8::from_str_radix(s, 10).map_err(|e| format!("Failed to parse '{}' as decimal: {}", s, e))
}

/// Parses a hexadecimal string into an 8-bit unsigned integer.
fn parse_hexadecimal(s: &str) -> Result<u8, String> {
    u8::from_str_radix(s, 16).map_err(|e| format!("Failed to parse '{}' as hexadecimal: {}", s, e))
}
