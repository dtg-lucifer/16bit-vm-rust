//! Rusty 16-bit VM library - A simple 16-bit virtual machine implementation.
//!
//! This crate provides a stack-based virtual machine with a small instruction set
//! that can be used to execute basic programs. The VM is designed with simplicity
//! and educational purposes in mind.
//!
//! ## Architecture
//!
//! - 8 KB (8192 bytes) of memory
//! - 8 16-bit registers
//! - Stack-based operation model
//! - Simple instruction set
//!
//! ## Usage
//!
//! ```
//! use rustyvm::{Machine, Register};
//!
//! // Create a new VM instance
//! let mut vm = Machine::new();
//!
//! // Program: Add two numbers and store in register A
//! vm.memory.write(0, 0x01);  // PUSH
//! vm.memory.write(1, 10);    // Value 10
//! vm.memory.write(2, 0x01);  // PUSH
//! vm.memory.write(3, 8);     // Value 8
//! vm.memory.write(4, 0x03);  // ADDSTACK
//! vm.memory.write(5, 0);     // Not used
//! vm.memory.write(6, 0x02);  // POPREGISTER
//! vm.memory.write(7, 0);     // Register A
//!
//! // Execute the program
//! vm.step().unwrap();  // PUSH 10
//! vm.step().unwrap();  // PUSH 8
//! vm.step().unwrap();  // ADDSTACK
//! vm.step().unwrap();  // POPREGISTER A
//!
//! // Get the result
//! assert_eq!(vm.get_register(Register::A), 18);
//! ```

/// Machine module provides the core VM implementation.
pub mod machine;

/// Memory module provides the memory system for the VM.
pub mod memory;

// Re-export key components for easier access
pub use crate::machine::*;
pub use crate::memory::*;
