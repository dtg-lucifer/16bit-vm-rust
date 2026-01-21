//! Intermediate Representation (IR) for assembly instructions.
//!
//! This module defines the `Instruction` enum which represents
//! the parsed assembly instructions before they are converted to bytecode.

/// Represents a single assembly instruction in intermediate form.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// No operation
    Nop,

    /// Push an immediate value onto the stack
    PushImmediate(u8),

    /// Push a hexadecimal value onto the stack
    PushHex(u8),

    /// Push the value from a register onto the stack
    PushRegister(String),

    /// Pop the top of the stack into a register
    Pop(String),

    /// Add the top two values on the stack
    AddStack,

    /// Add two register values
    AddRegister(String, String),

    /// Send a signal with the given code
    Signal(u8),

    /// Jump to a label
    Jump(String),

    /// Label declaration
    Label(String),
}
