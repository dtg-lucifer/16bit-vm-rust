//! Memory implementation for the 16-bit VM.
//!
//! Memory Layout:
//! - Program Memory: Starting at address 0x0000
//! - Stack Memory: Starting at address 0x1000 (grows upward)
//! - Memory Size: 8192 bytes (ends at 0x1FFF)

use std::usize;

/// Trait defining memory access operations for the VM.
pub trait Addressable {
    /// Reads a single byte from memory at the specified address.
    fn read(&self, addr: u16) -> Option<u8>;

    /// Writes a single byte to memory at the specified address.
    fn write(&mut self, addr: u16, value: u8) -> bool;

    /// Reads a 16-bit word from memory using little-endian format.
    /// Lower byte at addr, upper byte at addr+1
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

    /// Writes a 16-bit word to memory using little-endian format.
    /// Lower byte at addr, upper byte at addr+1
    fn write2(&mut self, addr: u16, value: u16) -> bool {
        // Extract lower and upper bytes from the 16-bit value
        let lo = (value & 0xff) as u8; // Lower 8 bits
        let hi = ((value >> 8) & 0xff) as u8; // Upper 8 bits

        // Write bytes in little-endian format:
        // Lower byte at addr, upper byte at addr+1
        self.write(addr, lo) && self.write(addr + 1, hi)
    }

    /// Copies a block of memory from one location to another.
    fn copy(&mut self, from: u16, to: u16, n: usize) -> bool {
        for i in 0..n {
            if let Some(x) = self.read(from + (i as u16)) {
                if !self.write(to + (i as u16), x) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    /// Loads data from a vector into memory at the specified address.
    /// Returns the number of bytes and instructions loaded.
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
/// Provides contiguous memory with bounds-checking on all operations.
pub struct LinearMemory {
    /// The actual memory storage as a vector of bytes
    bytes: Vec<u8>,
    /// Total size of the memory in bytes
    size: usize,
}

impl LinearMemory {
    /// Creates a new linear memory instance with the specified size.
    /// All memory locations are initialized to zero.
    pub fn new(n: usize) -> Self {
        Self {
            bytes: vec![0; n],
            size: n,
        }
    }
}

impl Addressable for LinearMemory {
    /// Reads a single byte from memory.
    /// Performs bounds checking to ensure the address is valid.
    fn read(&self, addr: u16) -> Option<u8> {
        if (addr as usize) < self.size {
            Some(self.bytes[addr as usize])
        } else {
            None
        }
    }

    /// Writes a single byte to memory.
    /// Performs bounds checking to ensure the address is valid.
    fn write(&mut self, addr: u16, value: u8) -> bool {
        if (addr as usize) < self.size {
            self.bytes[addr as usize] = value;
            true
        } else {
            false
        }
    }
}
