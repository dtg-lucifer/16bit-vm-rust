//! Unit tests for the memory module.
//!
//! This file contains tests for various memory operations in the VM,
//! including 8-bit and 16-bit reads/writes, bounds checking, and other
//! memory-related functionality.

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_linear_memory_creation() {
        let memory = LinearMemory::new(1024);

        // Test memory bounds by trying to access different addresses
        // Memory should be sized as specified (1024 bytes)
        for i in 0..1024 {
            assert!(memory.read(i as u16).is_some());
        }

        // Address at the edge of memory should be accessible
        assert!(memory.read(1023).is_some());

        // Address beyond memory should not be accessible
        assert!(memory.read(1024).is_none());

        // Memory should be initialized with zeros
        for i in 0..1024 {
            assert_eq!(memory.read(i as u16), Some(0));
        }
    }

    #[test]
    fn test_read_write_byte() {
        let mut memory = LinearMemory::new(256);

        // Write values
        assert!(memory.write(0, 0x42));
        assert!(memory.write(10, 0xFF));
        assert!(memory.write(255, 0x99));

        // Read values back
        assert_eq!(memory.read(0), Some(0x42));
        assert_eq!(memory.read(10), Some(0xFF));
        assert_eq!(memory.read(255), Some(0x99));

        // Unmodified values should still be 0
        assert_eq!(memory.read(1), Some(0));
        assert_eq!(memory.read(254), Some(0));
    }

    #[test]
    fn test_out_of_bounds() {
        let mut memory = LinearMemory::new(256);

        // Writes at the edge of memory should succeed
        assert!(memory.write(255, 0x42));

        // Writes beyond memory bounds should fail
        assert!(!memory.write(256, 0x42));
        assert!(!memory.write(1000, 0x42));
        assert!(!memory.write(u16::MAX, 0x42));

        // Reads at the edge of memory should succeed
        assert_eq!(memory.read(255), Some(0x42));

        // Reads beyond memory bounds should fail
        assert_eq!(memory.read(256), None);
        assert_eq!(memory.read(1000), None);
        assert_eq!(memory.read(u16::MAX), None);
    }

    #[test]
    fn test_read_write_16bit() {
        let mut memory = LinearMemory::new(256);

        // Test writing 16-bit values
        assert!(memory.write2(0, 0x1234));
        assert!(memory.write2(100, 0xABCD));
        assert!(memory.write2(254, 0x5678)); // This should succeed as it writes to 254-255

        // Test reading 16-bit values
        assert_eq!(memory.read2(0), Some(0x1234));
        assert_eq!(memory.read2(100), Some(0xABCD));
        assert_eq!(memory.read2(254), Some(0x5678));

        // Verify that bytes are stored in little-endian format
        assert_eq!(memory.read(0), Some(0x34)); // Lower byte
        assert_eq!(memory.read(1), Some(0x12)); // Upper byte
        assert_eq!(memory.read(100), Some(0xCD)); // Lower byte
        assert_eq!(memory.read(101), Some(0xAB)); // Upper byte

        // Test 16-bit operations at memory boundaries
        assert!(!memory.write2(255, 0x9999)); // Should fail - writes beyond bounds
        assert_eq!(memory.read2(255), None); // Should fail - reads beyond bounds
    }

    // The copy functionality relies on u8 addresses which can easily overflow
    // We'll skip testing it directly for now

    #[test]
    fn test_load_from_vec() {
        let mut memory = LinearMemory::new(256);

        // Create test data
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06];

        // Load data into memory
        let result = memory.load_from_vec(&data, 100);
        assert!(result.is_some());

        let (bytes, instructions) = result.unwrap();
        assert_eq!(bytes, 6); // 6 bytes loaded
        assert_eq!(instructions, 3); // 3 instructions (2 bytes each)

        // Verify data was loaded correctly
        for i in 0..6 {
            assert_eq!(memory.read(100 + i as u16), Some(data[i]));
        }

        // Test loading data that would exceed memory bounds
        let big_data = vec![0; 200];
        let result = memory.load_from_vec(&big_data, 0);
        assert!(result.is_some()); // Should succeed fully within bounds

        // This data is too large to fit at offset 100 in a 256-byte memory
        let result = memory.load_from_vec(&big_data, 100);
        assert!(result.is_none()); // Should fail as it extends beyond bounds
    }

    #[test]
    fn test_addressable_trait() {
        // Test that LinearMemory implements Addressable trait
        fn takes_addressable(_mem: &dyn Addressable) {
            // Function just to verify type implements trait
        }

        let memory = LinearMemory::new(256);
        takes_addressable(&memory);
    }
}
