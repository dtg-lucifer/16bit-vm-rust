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
                match s {
                    $(
                        stringify!($variant) => Ok($name::$variant),
                    )*
                    _ => Err(format!("Invalid register name: {}", s)),
                }
            }
        }
    };
}

// #[macro_export]
// macro_rules! define_instructions {
//     (
//         $(#[$meta:meta])*
//         $vis:vis enum $name:ident {
//             $(
//                 $(#[$variant_meta:meta])*
//                 $variant:ident$(($type:ty))? = $value:expr
//             ),* $(,)?
//         }
//     ) => {
//         $vis enum $name {
//             $(
//                 $(#[$variant_meta])*
//                 $variant(($type))? = $value
//             ),*
//         }

//         $vis enum Op {
//             $(
//                 $(#[$variant_meta])*
//                 $variant = $value
//             ),*
//         }

//         /// Gets the numeric opcode for the instruction
//         $vis fn value(&self) -> u8 {
//             unsafe { *<*const _>::from(self).cast::<u8>() }
//         }

//         /// Checks if a numeric opcode matches a specific operation.
//         $vis fn equals(&self, other: Self) -> bool {
//             x == other.value()
//         }

//         $vis fn parse_instructions(ins: u16) -> Result<Self, String>{
//             let op = (ins & 0xff) as u8;

//             match op {
//                 $(
//                     x if x == Op::$variant => {
//                         if let Some(arg) = parse_instructions_arg(ins) {
//                             Ok($name::$variant(arg))
//                         } else {
//                             Err(format!("Missing argument for instruction {}", stringify!($variant)))
//                         }
//                     },
//                 )*
//                 _ => Err(format!("Unknown instruction - 0x{:X}", op)),
//             }
//         }
//     };
// }

// fn parse_instructions(ins: u16) -> Result<Op, String> {
//     let op = (ins & 0xff) as u8;

//     match op {
//         x if x == Op::Nop.value() => Ok(Op::Nop),
//         x if x == Op::Push(0).value() => Ok(Op::Push(parse_instructions_arg(ins))),
//         x if x == Op::PopRegister(Register::A).value() => {
//             let arg = parse_instructions_arg(ins);
//             Register::from_u8(arg)
//                 .ok_or(format!("unknown register - 0x{:X}", arg))
//                 .map(|r| Op::PopRegister(r))
//         }
//         x if x == Op::PushRegister(Register::A).value() => {
//             let arg = parse_instructions_arg(ins);
//             Register::from_u8(arg)
//                 .ok_or(format!("unknown register - 0x{:X}", arg))
//                 .map(|r| Op::PushRegister(r))
//         }
//         x if x == Op::AddRegister(Register::A, Register::A).value() => {
//             let arg = parse_instructions_arg(ins);
//             // The first byte is the opcode
//             // The second byte is divided into two 4 bit parts to store 2 register address
//             let reg1 = (arg >> 4) & 0x0F; // Upper 4 bits
//             let reg2 = arg & 0x0F; // Lower 4 bits
//             let r1 = Register::from_u8(reg1).ok_or(format!("unknown register - 0x{:X}", reg1))?;
//             let r2 = Register::from_u8(reg2).ok_or(format!("unknown register - 0x{:X}", reg2))?;
//             Ok(Op::AddRegister(r1, r2))
//         }
//         x if x == Op::AddStack.value() => Ok(Op::AddStack),
//         x if x == Op::Signal(0).value() => Ok(Op::Signal(parse_instructions_arg(ins))),
//         _ => Err(format!("unknown op - 0x{:X}", op)),
//     }
// }
