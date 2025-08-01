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
