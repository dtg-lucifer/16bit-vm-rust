//! Memory implementation for the 16-bit VM.
//!
//! This module provides the memory interface and a concrete implementation
//! of linear memory for the virtual machine.

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
    /// # Parameters
    /// * `addr` - The memory address to read from (16-bit)
    ///
    /// # Returns
    /// * `Some(u16)` - The 16-bit word at the specified address
    /// * `None` - If the address is invalid or out of bounds
    fn read2(&self, addr: u16) -> Option<u16> {
        if let Some(lo) = self.read(addr) {
            if let Some(hi) = self.read(addr + 1) {
                return Some((lo as u16) | ((hi as u16) << 8));
            }
        }
        None
    }

    /// Writes a 16-bit word to memory at the specified address.
    /// The VM uses little-endian format (lower byte first).
    ///
    /// # Parameters
    /// * `addr` - The memory address to write to (16-bit)
    /// * `value` - The 16-bit value to write
    ///
    /// # Returns
    /// * `true` - If the write was successful
    /// * `false` - If the address is invalid or out of bounds
    fn write2(&mut self, addr: u16, value: u16) -> bool {
        let lo = (value & 0xff) as u8;
        let hi = ((value >> 8) & 0xff) as u8;

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

    /// Writes data read from a vector
    ///
    /// # Parameters
    /// * `from` - The vector storing all of the 8 bit data
    ///
    /// # Returns
    /// * `Option<(usize, usize)>` - The number of bytes written into memory
    /// and the number of operations the CPU has to perform to execute all of those
    ///
    /// # Note
    /// This function can only write the data came in 8 bit format
    fn read_from_vec(&mut self, from: &mut Vec<u8>) -> Option<(usize, usize)> {
        let mut operations: usize = 0;
        for (i, b) in from.iter().enumerate() {
            self.write(i as u16, *b);
            operations += 1;
        }

        Some((operations, operations / 2))
    }
}

/// A flat, linear memory implementation for the VM.
///
/// This struct provides a simple memory space of contiguous bytes
/// with bounds-checking on all operations.
pub struct LinearMemory {
    /// The actual memory storage as a vector of bytes
    bytes: Vec<u8>,
    /// Total size of the memory in bytes
    size: usize,
}

impl LinearMemory {
    /// Creates a new linear memory instance with the specified size.
    ///
    /// # Parameters
    /// * `n` - The size of memory in bytes
    ///
    /// # Returns
    /// A new LinearMemory instance initialized with zeros
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
    /// # Parameters
    /// * `addr` - The memory address to read from
    ///
    /// # Returns
    /// * `Some(u8)` - The byte at the specified address
    /// * `None` - If the address is out of bounds
    fn read(&self, addr: u16) -> Option<u8> {
        if (addr as usize) < self.size {
            Some(self.bytes[addr as usize])
        } else {
            None
        }
    }

    /// Writes a single byte to memory.
    ///
    /// # Parameters
    /// * `addr` - The memory address to write to
    /// * `value` - The byte value to write
    ///
    /// # Returns
    /// * `true` - If the write was successful
    /// * `false` - If the address is out of bounds
    fn write(&mut self, addr: u16, value: u8) -> bool {
        if (addr as usize) < self.size {
            self.bytes[addr as usize] = value;
            true
        } else {
            false
        }
    }
}
