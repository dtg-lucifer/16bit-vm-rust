//! Rusty 16-bit VM library - A simple 16-bit virtual machine implementation.
//!
//! This crate provides a stack-based virtual machine with:
//! - 8 KB (8192 bytes) of memory
//! - 8 16-bit registers
//! - Simple instruction set

/// Machine module provides the core VM implementation.
pub mod machine;

/// Memory module provides the memory system for the VM.
pub mod memory;

/// Re-export key components for easier access
pub use crate::machine::*;
pub use crate::memory::*;

// Include test modules
#[cfg(test)]
mod machine_test;
#[cfg(test)]
mod memory_test;
