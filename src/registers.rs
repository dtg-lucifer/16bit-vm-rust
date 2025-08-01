use crate::define_registers;

define_registers! {
    /// Register enum definition with 8 registers.
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[repr(u8)]
    pub enum Register {
        /// General purpose register A (index 0)
        A = 0x00,
        /// General purpose register B (index 1)
        B = 0x01,
        /// General purpose register C (index 2)
        C = 0x02,
        /// Memory operations register (index 3)
        M = 0x03,
        /// Stack Pointer register - points to next available stack location (index 4)
        SP = 0x04,
        /// Program Counter register - points to next instruction (index 5)
        PC = 0x05,
        /// Base Pointer register - for stack frames (index 6)
        BP = 0x06,
        /// Status flags register (index 7)
        FLAGS = 0x07,
        /// Extended register R0 (index 8)
        R0 = 0x08,
        /// Extended register R1 (index 9)
        R1 = 0x09,
        /// Extended register R2 (index 10)
        R2 = 0x0A,
        /// Extended register R3 (index 11)
        R3 = 0x0B,
        /// Extended register R4 (index 12)
        R4 = 0x0C,
    }
}
