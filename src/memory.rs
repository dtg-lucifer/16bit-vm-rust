//! Memory implementation for the 16-bit VM.
//!
//! This module provides the memory interface and a concrete implementation
//! of linear memory for the virtual machine.
//!
//! # Memory Architecture
//!
//! The VM has 8 KB (8192 bytes) of memory laid out as follows:
//!
//! ```
//! +------------------------+ 0x0000
//! |                        |
//! |    Program Memory      |
//! |                        |
//! +------------------------+ ~0x1000
//! |                        |
//! |    Stack Memory        |
//! |  (grows upward)        |
//! |                        |
//! +------------------------+ 0x1FFF
//! ```
//!
//! # Memory Access
//!
//! The VM provides both 8-bit and 16-bit memory access operations.
//! For 16-bit operations, the VM uses little-endian byte order.

use std::usize;

/// Trait defining memory access operations for the VM.
///
/// This trait defines the interface for all memory types that can be used
/// with the VM, allowing for different memory implementations (linear,
/// segmented, etc.) while maintaining a consistent interface.
pub trait Addressable {
    /// Reads a single byte from memory at the specified address.
    ///
    /// # Parameters
    /// * `addr` - The memory address to read from (16-bit)
    ///
    /// # Returns
    /// * `Some(u8)` - The byte at the specified address
    /// * `None` - If the address is invalid or out of bounds
    fn read(&self, addr: u16) -> Option<u8>;

    /// Writes a single byte to memory at the specified address.
    ///
    /// # Parameters
    /// * `addr` - The memory address to write to (16-bit)
    /// * `value` - The byte value to write
    ///
    /// # Returns
    /// * `true` - If the write was successful
    /// * `false` - If the address is invalid or out of bounds
    fn write(&mut self, addr: u16, value: u8) -> bool;

    /// Reads a 16-bit word from memory at the specified address.
    /// The VM uses little-endian format (lower byte first).
    ///
    /// # Memory Layout for 16-bit Values (Little-Endian)
    /// ```
    /// Address N:   Lower byte (least significant byte)
    /// Address N+1: Upper byte (most significant byte)
    /// ```
    ///
    /// # Example
    /// For the 16-bit value 0x1234:
    /// ```
    /// memory[addr]   = 0x34 (lower byte)
    /// memory[addr+1] = 0x12 (upper byte)
    /// ```
    ///
    /// When read with read2(), this returns the 16-bit value 0x1234.
    ///
    /// # Parameters
    /// * `addr` - The memory address to read from (16-bit)
    ///
    /// # Returns
    /// * `Some(u16)` - The 16-bit word at the specified address
    /// * `None` - If the address is invalid or out of bounds
    fn read2(&self, addr: u16) -> Option<u16> {
        if let Some(lo) = self.read(addr) {
            if let Some(hi) = self.read(addr + 1) {
                // Combine bytes in little-endian format:
                // Lower byte from addr, upper byte from addr+1
                return Some((lo as u16) | ((hi as u16) << 8));
            }
        }
        None
    }

    /// Writes a 16-bit word to memory at the specified address.
    /// The VM uses little-endian format (lower byte first).
    ///
    /// # Memory Layout for 16-bit Values (Little-Endian)
    /// ```
    /// Address N:   Lower byte (least significant byte)
    /// Address N+1: Upper byte (most significant byte)
    /// ```
    ///
    /// # Example
    /// For the 16-bit value 0x1234:
    /// ```
    /// memory[addr]   = 0x34 (lower byte)
    /// memory[addr+1] = 0x12 (upper byte)
    /// ```
    ///
    /// # Parameters
    /// * `addr` - The memory address to write to (16-bit)
    /// * `value` - The 16-bit value to write
    ///
    /// # Returns
    /// * `true` - If the write was successful
    /// * `false` - If the address is invalid or out of bounds
    fn write2(&mut self, addr: u16, value: u16) -> bool {
        // Extract lower and upper bytes from the 16-bit value
        let lo = (value & 0xff) as u8; // Lower 8 bits
        let hi = ((value >> 8) & 0xff) as u8; // Upper 8 bits

        // Write bytes in little-endian format:
        // Lower byte at addr, upper byte at addr+1
        self.write(addr, lo) && self.write(addr + 1, hi)
    }

    /// Copies a block of memory from one location to another.
    ///
    /// # Parameters
    /// * `from` - The source starting address (8-bit)
    /// * `to` - The destination starting address (8-bit)
    /// * `n` - The number of bytes to copy
    ///
    /// # Returns
    /// * `true` - If the copy was successful
    /// * `false` - If any address is invalid or out of bounds
    ///
    /// # Note
    /// This function has a limitation - it uses 8-bit addresses which
    /// restricts the addressable range. Consider upgrading to u16.
    ///
    /// # Example
    ///
    /// ```
    /// // Copy 10 bytes from address 0x10 to address 0x20
    /// memory.copy(0x10, 0x20, 10);
    /// ```
    fn copy(&mut self, from: u8, to: u8, n: usize) -> bool {
        for i in 0..n {
            if let Some(x) = self.read((from + (i as u8)).into()) {
                if !self.write((to + (i as u8)).into(), x) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    /// Loads data from a vector into memory at the specified address
    ///
    /// This function is particularly useful for loading programs into the VM's memory.
    ///
    /// # Parameters
    /// * `from: &Vec<u8>` - The vector containing the binary data (8-bit values)
    /// * `addr: u16` - The starting address where data will be written
    ///
    /// # Returns
    /// * `Option<(usize, usize)>` - If successful, returns a tuple containing:
    ///   - The number of bytes written into memory
    ///   - The number of instructions loaded (assuming 2 bytes per instruction)
    /// * `None` - If the operation failed (e.g., memory bounds exceeded)
    ///
    /// # Note
    /// This function can only write data in 8-bit format
    ///
    /// # Example
    ///
    /// ```
    /// let program = vec![0x01, 0x0A, 0x01, 0x08, 0x03, 0x00];  // PUSH 10, PUSH 8, ADDSTACK
    /// if let Some((bytes, instructions)) = vm.memory.load_from_vec(&program, 0) {
    ///     println!("Loaded {} bytes ({} instructions)", bytes, instructions);
    /// }
    /// ```
    fn load_from_vec(&mut self, from: &Vec<u8>, addr: u16) -> Option<(usize, usize)> {
        let mut operations: usize = 0;
        for (i, b) in from.iter().enumerate() {
            if !self.write(addr + (i as u16), *b) {
                return None;
            }
            operations += 1;
        }

        Some((operations, operations / 2))
    }
}

/// A flat, linear memory implementation for the VM.
///
/// This struct provides a simple memory space of contiguous bytes
/// with bounds-checking on all operations.
///
/// # Memory Layout
///
/// The LinearMemory provides a single, contiguous block of memory with
/// addresses ranging from 0x0000 to the size specified at creation.
/// In the default VM configuration, this is 8 KB (8192 bytes).
///
/// # Usage
///
/// ```
/// let memory_size = 8 * 1024;  // 8 KB
/// let memory = LinearMemory::new(memory_size);
/// ```
pub struct LinearMemory {
    /// The actual memory storage as a vector of bytes
    bytes: Vec<u8>,
    /// Total size of the memory in bytes
    size: usize,
}

impl LinearMemory {
    /// Creates a new linear memory instance with the specified size.
    ///
    /// All memory locations are initialized to zero.
    ///
    /// # Parameters
    /// * `n` - The size of memory in bytes (e.g., 8192 for 8 KB)
    ///
    /// # Returns
    /// A new LinearMemory instance initialized with zeros
    ///
    /// # Example
    ///
    /// ```
    /// // Create an 8 KB memory space
    /// let memory = LinearMemory::new(8 * 1024);
    /// ```
    pub fn new(n: usize) -> Self {
        Self {
            bytes: vec![0; n],
            size: n,
        }
    }
}

impl Addressable for LinearMemory {
    /// Reads a single byte from memory.
    ///
    /// Performs bounds checking to ensure the address is valid.
    ///
    /// # Parameters
    /// * `addr` - The memory address to read from (0x0000 to size-1)
    ///
    /// # Returns
    /// * `Some(u8)` - The byte at the specified address if valid
    /// * `None` - If the address is out of bounds
    ///
    /// # Example
    ///
    /// ```
    /// // Read a single byte from address 0x0100
    /// if let Some(value) = memory.read(0x0100) {
    ///     println!("Value at 0x0100: {}", value);
    /// }
    ///
    /// // For instruction bytes at address 0:
    /// // Memory[0] contains the opcode (e.g., 0x01 for PUSH)
    /// // Memory[1] contains the argument (e.g., 0x0A for value 10)
    /// ```
    fn read(&self, addr: u16) -> Option<u8> {
        if (addr as usize) < self.size {
            Some(self.bytes[addr as usize])
        } else {
            None
        }
    }

    /// Writes a single byte to memory.
    ///
    /// Performs bounds checking to ensure the address is valid.
    ///
    /// # Parameters
    /// * `addr` - The memory address to write to (0x0000 to size-1)
    /// * `value` - The byte value to write (0x00 to 0xFF)
    ///
    /// # Returns
    /// * `true` - If the write was successful
    /// * `false` - If the address is out of bounds
    ///
    /// # Example
    ///
    /// ```
    /// // Write value 0x42 to address 0x0100
    /// if memory.write(0x0100, 0x42) {
    ///     println!("Write successful");
    /// }
    /// ```
    fn write(&mut self, addr: u16, value: u8) -> bool {
        if (addr as usize) < self.size {
            self.bytes[addr as usize] = value;
            true
        } else {
            false
        }
    }
}
