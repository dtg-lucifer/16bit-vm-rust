use crate::{Machine, Register};

/// Operations supported by the VM.
///
/// Each operation corresponds to a specific instruction opcode.
/// The VM uses a 2-byte instruction format, where the first byte is the opcode
/// and the second byte is an argument (when applicable).
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(u8)]
pub enum Op {
    /// No operation (opcode 0x00)
    Nop = 0x00,
    /// Push a value onto the stack (opcode 0x01)
    /// Parameter: 8-bit value to push
    Push(u8) = 0x01,
    /// Pop a value from the stack into a register (opcode 0x02)
    /// Parameter: destination register
    PopRegister(Register) = 0x02,
    /// Push a register value onto the stack (opcode 0x03)
    /// Parameter: register to push
    PushRegister(Register) = 0x03,
    /// Add top two values on stack, push result (opcode 0x0F)
    AddStack = 0x0F,
    /// Add two registers, store result in first register (opcode 0x04)
    /// Parameters: destination register, source register
    AddRegister(Register, Register) = 0x04,
    /// Signal returns the Signal (opcode 0x09)
    /// Parameters: signal integer
    Signal(u8) = 0x09,
}

/// Implementation of operation-related functionality.
impl Op {
    /// Gets the numeric opcode value for this operation.
    pub fn value(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    /// Checks if a numeric opcode matches a specific operation.
    pub fn equals(x: u8, other: Self) -> bool {
        x == other.value()
    }
}

/// Parses a 16-bit instruction and extracts the 8-bit argument.
/// Uses little-endian format with ARGUMENT in upper 8 bits and OPCODE in lower 8 bits
pub fn parse_instructions_arg(ins: u16) -> u8 {
    ((ins & 0xff00) >> 8) as u8
}

/// Parses a 16-bit instruction into an operation.
/// Extracts the opcode (lower 8 bits) and returns the corresponding operation.
pub fn parse_instructions(ins: u16) -> Result<Op, String> {
    let op = (ins & 0xff) as u8;

    match op {
        x if x == Op::Nop.value() => Ok(Op::Nop),
        x if x == Op::Push(0).value() => Ok(Op::Push(parse_instructions_arg(ins))),
        x if x == Op::PopRegister(Register::A).value() => {
            let arg = parse_instructions_arg(ins);
            Register::from_u8(arg)
                .ok_or(format!("unknown register - 0x{:X}", arg))
                .map(|r| Op::PopRegister(r))
        }
        x if x == Op::PushRegister(Register::A).value() => {
            let arg = parse_instructions_arg(ins);
            Register::from_u8(arg)
                .ok_or(format!("unknown register - 0x{:X}", arg))
                .map(|r| Op::PushRegister(r))
        }
        x if x == Op::AddRegister(Register::A, Register::A).value() => {
            let arg = parse_instructions_arg(ins);
            // The first byte is the opcode
            // The second byte is divided into two 4 bit parts to store 2 register address
            let reg1 = (arg >> 4) & 0x0F; // Upper 4 bits
            let reg2 = arg & 0x0F; // Lower 4 bits
            let r1 = Register::from_u8(reg1).ok_or(format!("unknown register - 0x{:X}", reg1))?;
            let r2 = Register::from_u8(reg2).ok_or(format!("unknown register - 0x{:X}", reg2))?;
            Ok(Op::AddRegister(r1, r2))
        }
        x if x == Op::AddStack.value() => Ok(Op::AddStack),
        x if x == Op::Signal(0).value() => Ok(Op::Signal(parse_instructions_arg(ins))),
        _ => Err(format!("unknown op - 0x{:X}", op)),
    }
}

/// Executes a single instruction in the VM.
pub fn execute_instruction(machine: &mut Machine, op: Op) -> Result<(), String> {
    // Execute the operation
    match op {
        Op::Nop => Ok(()),
        Op::Push(v) => machine.push(v.into()),
        Op::PopRegister(r) => {
            let value = machine.pop()?;
            machine.registers[r as usize] = value;
            Ok(())
        }
        Op::PushRegister(r) => {
            let value = machine.registers[r as usize];
            machine.push(value)?;
            Ok(())
        }
        Op::AddStack => {
            let a = machine.pop()?;
            let b = machine.pop()?;
            let result = a + b;
            machine.push(result)?;
            Ok(())
        }
        Op::AddRegister(r1, r2) => {
            machine.registers[r1 as usize] += machine.registers[r2 as usize];
            Ok(())
        }
        Op::Signal(s) => {
            let sig_fn = machine
                .signal_handlers
                .get(&s)
                .ok_or(format!("unknown signal - 0x{:X}", s))?;
            sig_fn(machine)
        }
    }
}
