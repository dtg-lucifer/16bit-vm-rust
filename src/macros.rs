//! Macros for helping with code generation.

/// A simple macro for generating register enums with helper methods.
///
/// This macro generates:
/// - An enum with register variants
/// - A from_u8 method to convert from numeric values
/// - A from_str method to convert from string representations
///
/// # Example
///
/// ```
/// define_registers! {
///     #[derive(Debug, PartialEq, Eq, Clone, Copy)]
///     #[repr(u8)]
///     pub enum Register {
///         A = 0x00,
///         B = 0x01
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_registers {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $(
                $(#[$variant_meta])*
                $variant = $value
            ),*
        }

        impl $name {
            /// Convert a numeric value to a register enum.
            $vis fn from_u8(v: u8) -> Option<Self> {
                match v {
                    $(
                        x if x == $value => Some($name::$variant),
                    )*
                    _ => None,
                }
            }

            /// Convert a string representation to a register enum.
            $vis fn from_str(s: &str) -> Result<Self, String> {
                let s_upper = s.to_uppercase();
                match s_upper.as_str() {
                    $(
                        stringify!($variant) => Ok($name::$variant),
                    )*
                    _ => Err(format!("Invalid register name: {}", s)),
                }
            }
        }
    };
}

/// A macro for generating instruction enums with helper methods.
///
/// This macro generates:
/// - An enum with instruction variants of different types
/// - Methods to convert between instructions and their binary representation
/// - Helper methods for working with instructions
///
/// # Dependencies
///
/// This macro assumes a `Register` enum exists in scope, which should be created
/// using the `define_registers!` macro. The `Register` enum must implement conversion
/// to and from `u8`.
///
/// # Example
///
/// ```rust
/// // First define your registers
/// define_registers! {
///     #[derive(Debug, PartialEq, Eq, Clone, Copy)]
///     #[repr(u8)]
///     pub enum Register {
///         A = 0x00,
///         B = 0x01,
///         C = 0x02,
///         D = 0x03
///     }
/// }
///
/// // Then define your instructions
/// define_instructions! {
///     #[derive(Debug, PartialEq, Eq, Clone, Copy)]
///     pub enum Instruction {
///         // No argument instructions
///         Nop = 0x00,
///
///         // Numeric argument instructions
///         Push(u8) = 0x01,
///
///         // Register argument instructions
///         PopRegister(Register) = 0x02,
///
///         // Two register arguments instructions
///         AddRegister(Register, Register) = 0x04
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_instructions {
    // Main entry point that matches different instruction types
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($rest:tt)*
        }
    ) => {
        // First, define the enum with all variants
        $(#[$meta])*
        $vis enum $name {
            $($rest)*
        }

        // Then implement methods for this enum
        impl $name {
            /// Gets the numeric opcode for the instruction
            $vis fn value(&self) -> u8 {
                match self {
                    $name::Nop => 0x00,
                    $name::Push(_) => 0x01,
                    $name::PopRegister(_) => 0x02,
                    $name::PushRegister(_) => 0x03,
                    $name::AddRegister(_, _) => 0x04,
                    $name::AddStack => 0x0F,
                    $name::Signal(_) => 0x09,
                }
            }

            /// Helper function to extract the argument part of an instruction
            $vis fn parse_instruction_arg(ins: u16) -> u8 {
                ((ins >> 8) & 0xFF) as u8
            }

            /// Parse a 16-bit instruction into an Operation
            $vis fn parse_instruction(ins: u16) -> Result<Self, String> {
                let op = (ins & 0xFF) as u8;
                let arg = Self::parse_instruction_arg(ins);

                match op {
                    // No argument instructions
                    0x00 => Ok($name::Nop),

                    // Instructions with numeric argument
                    0x01 => Ok($name::Push(arg)),
                    0x09 => Ok($name::Signal(arg)),

                    // Instructions with register argument
                    0x02 => {
                        Register::from_u8(arg)
                            .ok_or(format!("Unknown register - 0x{:X}", arg))
                            .map(|r| $name::PopRegister(r))
                    },
                    0x03 => {
                        Register::from_u8(arg)
                            .ok_or(format!("Unknown register - 0x{:X}", arg))
                            .map(|r| $name::PushRegister(r))
                    },

                    // Two register argument instructions
                    0x04 => {
                        let reg1 = (arg >> 4) & 0x0F; // Upper 4 bits
                        let reg2 = arg & 0x0F; // Lower 4 bits

                        let r1 = Register::from_u8(reg1)
                            .ok_or(format!("Unknown register - 0x{:X}", reg1))?;
                        let r2 = Register::from_u8(reg2)
                            .ok_or(format!("Unknown register - 0x{:X}", reg2))?;

                        Ok($name::AddRegister(r1, r2))
                    },

                    // AddStack instruction
                    0x0F => Ok($name::AddStack),

                    // Unknown opcode
                    _ => Err(format!("Unknown instruction - 0x{:X}", op)),
                }
            }

            /// Convert the instruction back to its 16-bit binary representation
            $vis fn to_u16(&self) -> u16 {
                match self {
                    // No argument instructions
                    $name::Nop => 0x00,

                    // Numeric argument instructions
                    $name::Push(arg) => {
                        ((arg as u16) << 8) | 0x01
                    },
                    $name::Signal(arg) => {
                        ((arg as u16) << 8) | 0x09
                    },

                    // Register argument instructions
                    $name::PopRegister(reg) => {
                        let reg_val = *reg as u8;
                        ((reg_val as u16) << 8) | 0x02
                    },
                    $name::PushRegister(reg) => {
                        let reg_val = *reg as u8;
                        ((reg_val as u16) << 8) | 0x03
                    },

                    // Two register arguments instructions
                    $name::AddRegister(reg1, reg2) => {
                        let reg1_val = *reg1 as u8;
                        let reg2_val = *reg2 as u8;
                        let combined_arg = ((reg1_val & 0x0F) << 4) | (reg2_val & 0x0F);
                        ((combined_arg as u16) << 8) | 0x04
                    },

                    // AddStack instruction
                    $name::AddStack => 0x0F,
                }
            }

            /// Convert the instruction to a debug string representation
            $vis fn to_string(&self) -> String {
                match self {
                    // No argument instructions
                    $name::Nop => "Nop".to_string(),

                    // Numeric argument instructions
                    $name::Push(arg) => {
                        format!("Push(0x{:02X})", arg)
                    },
                    $name::Signal(arg) => {
                        format!("Signal(0x{:02X})", arg)
                    },

                    // Register argument instructions
                    $name::PopRegister(reg) => {
                        format!("PopRegister({:?})", reg)
                    },
                    $name::PushRegister(reg) => {
                        format!("PushRegister({:?})", reg)
                    },

                    // Two register arguments instructions
                    $name::AddRegister(reg1, reg2) => {
                        format!("AddRegister({:?}, {:?})", reg1, reg2)
                    },

                    // AddStack instruction
                    $name::AddStack => "AddStack".to_string(),
                }
            }
        }
    };
}
