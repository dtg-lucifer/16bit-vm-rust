//! VM core implementation for the 16-bit Virtual Machine.

use std::collections::HashMap;

use crate::memory::{Addressable, LinearMemory};

/// Register set for the 16-bit VM.
/// Each register is 16 bits (2 bytes) wide.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Register {
    /// General purpose register A (index 0)
    A,
    /// General purpose register B (index 1)
    B,
    /// General purpose register C (index 2)
    C,
    /// Memory operations register (index 3)
    M,
    /// Stack Pointer register - points to next available stack location (index 4)
    SP,
    /// Program Counter register - points to next instruction (index 5)
    PC,
    /// Base Pointer register - for stack frames (index 6)
    BP,
    /// Status flags register (index 7)
    FLAGS,
}

impl Register {
    /// Converts a numeric value to a register enum.
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            x if x == Register::A as u8 => Some(Register::A),
            x if x == Register::B as u8 => Some(Register::B),
            x if x == Register::C as u8 => Some(Register::C),
            x if x == Register::M as u8 => Some(Register::M),
            x if x == Register::SP as u8 => Some(Register::SP),
            x if x == Register::BP as u8 => Some(Register::BP),
            x if x == Register::PC as u8 => Some(Register::PC),
            x if x == Register::FLAGS as u8 => Some(Register::FLAGS),
            _ => None,
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "A" => Ok(Register::A),
            "B" => Ok(Register::B),
            "C" => Ok(Register::C),
            "M" => Ok(Register::M),
            "SP" => Ok(Register::SP),
            "PC" => Ok(Register::PC),
            "BP" => Ok(Register::BP),
            "FLAGS" => Ok(Register::FLAGS),
            _ => Err(format!("Invalid register name: {}", s)),
        }
    }
}

/// Operations supported by the VM.
///
/// Each operation corresponds to a specific instruction opcode.
/// The VM uses a 2-byte instruction format, where the first byte is the opcode
/// and the second byte is an argument (when applicable).
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(u8)]
pub enum Op {
    /// No operation (opcode 0x00)
    Nop,
    /// Push a value onto the stack (opcode 0x01)
    /// Parameter: 8-bit value to push
    Push(u8),
    /// Pop a value from the stack into a register (opcode 0x02)
    /// Parameter: destination register
    PopRegister(Register),
    /// Add top two values on stack, push result (opcode 0x03)
    AddStack,
    /// Add two registers, store result in first register (opcode 0x04)
    /// Parameters: destination register, source register
    AddRegister(Register, Register),
    /// Signal returns the Signal (opcode 0x05)
    /// Parameters: signal integer
    Signal(u8),
}

/// Implementation of operation-related functionality.
impl Op {
    /// Gets the numeric opcode value for this operation.
    ///
    /// # Safety
    /// Uses unsafe code to extract the enum discriminant value
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
fn parse_instructions_arg(ins: u16) -> u8 {
    ((ins & 0xff00) >> 8) as u8
}

/// Parses a 16-bit instruction into an operation.
/// Extracts the opcode (lower 8 bits) and returns the corresponding operation.
fn parse_instructions(ins: u16) -> Result<Op, String> {
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
        x if x == Op::AddStack.value() => Ok(Op::AddStack),
        x if x == Op::Signal(0).value() => Ok(Op::Signal(parse_instructions_arg(ins))),
        // TODO: Implement the ADDREGISTER op code
        _ => Err(format!("unknown op - 0x{:X}", op)),
    }
}

/// Function type for signal handlers in the VM.
/// Called when the VM executes a SIGNAL instruction.
type SignalFunction = fn(&mut Machine) -> Result<(), String>;

/// The main virtual machine structure.
///
/// This struct represents the entire virtual machine, containing
/// registers, memory, and state information.
pub struct Machine {
    /// The VM's register set (8 registers, each 16 bits)
    pub registers: [u16; 8],
    /// Keeps track whether the machine is in halt or not
    pub halt: bool,
    /// Keeps the cache of signal handler methods
    pub signal_handlers: HashMap<u8, SignalFunction>,
    /// The VM's memory (dynamic dispatch allows for different implementations)
    pub memory: Box<dyn Addressable>,
}

impl Machine {
    /// Creates a new virtual machine with initialized state.
    /// SP starts at 0x1000, PC at 0, all other registers at 0
    pub fn new() -> Self {
        let memory_size = 8 * 1024; // -> 8 KB
        let mut machine = Self {
            registers: [0; 8],
            halt: false,
            signal_handlers: HashMap::new(),
            memory: Box::new(LinearMemory::new(memory_size)),
        };
        // Initialize SP to point to the beginning of stack area
        // Starting at address 0x1000 gives plenty of room for both code and stack
        machine.registers[Register::SP as usize] = 0x1000;

        // Initialize PC to 0 (program starts at the beginning of memory)
        machine.registers[Register::PC as usize] = 0;
        machine
    }

    /// Gets the value of a specific register.
    pub fn get_register(&self, r: Register) -> u16 {
        self.registers[r as usize]
    }

    /// Defines a signal handler for a specific signal code.
    /// Called when the VM executes a SIGNAL instruction with the matching code.
    pub fn define_handler(&mut self, index: u8, f: SignalFunction) {
        self.signal_handlers.insert(index, f);
    }

    /// Pops a 16-bit value from the stack.
    /// First decrement SP by 2, then read the value at the new SP location.
    /// Restores SP on error.
    pub fn pop(&mut self) -> Result<u16, String> {
        // For pop, first decrement SP, then read
        self.registers[Register::SP as usize] -= 2;
        let sp = self.registers[Register::SP as usize];
        if let Some(v) = self.memory.read2(sp) {
            Ok(v)
        } else {
            // Restore SP on error
            self.registers[Register::SP as usize] += 2;
            return Err(format!("memory read fault - 0x{:X}", sp));
        }
    }

    /// Pushes a 16-bit value onto the stack.
    /// First write at current SP, then increment SP by 2
    pub fn push(&mut self, v: u16) -> Result<(), String> {
        // For push, first write at current SP, then increment
        let sp = self.registers[Register::SP as usize];
        if !self.memory.write2(sp, v) {
            return Err(format!("memory write fault - 0x{:X}", sp));
        }
        self.registers[Register::SP as usize] += 2;
        Ok(())
    }

    /// Prints the current state of the VM to the console.
    /// Shows register values, stack pointer, and program counter.
    pub fn print_state(&self) {
        println!("-----------------------------------------------");
        println!("----------------Final State--------------------");
        println!("Final output:");
        println!(
            "\tRegister A: 0x{:04X} ({})",
            self.registers[Register::A as usize],
            self.registers[Register::A as usize]
        );
        println!("Registers:");
        for (i, reg) in self.registers.iter().enumerate() {
            let reg_name = match Register::from_u8(i as u8) {
                Some(r) => format!("{:?}", r),
                None => "Unknown".to_string(),
            };
            println!("\tRegister {}: 0x{:04X} ({})", reg_name, reg, reg);
        }
        println!(
            "\tStack Pointer (SP): 0x{:04X}",
            self.registers[Register::SP as usize]
        );
        println!(
            "\tProgram Counter (PC): 0x{:04X}",
            self.registers[Register::PC as usize]
        );
        println!("-----------------------------------------------");
    }

    /// Executes a single instruction in the VM.
    ///
    /// 1. Reads instruction from memory at PC
    /// 2. Increments PC by 2 (each instruction is 2 bytes)
    /// 3. Parses and executes the operation
    pub fn step(&mut self) -> Result<(), String> {
        let pc = self.registers[Register::PC as usize];

        // Read opcode and argument as separate bytes for debugging output
        let opcode = self.memory.read(pc).unwrap_or(0);
        let arg = self.memory.read(pc + 1).unwrap_or(0);

        // Read the full 16-bit instruction (in little-endian format)
        // This gives us a value where:
        // - Lower 8 bits contain the opcode (memory[pc])
        // - Upper 8 bits contain the argument (memory[pc+1])

        let ins = self
            .memory
            .read2(pc)
            .ok_or(format!("memory read fault at PC=0x{:04X}", pc))?;

        // Increment the Program Counter register by 2 to move to the next instruction
        // (each instruction is 2 bytes: 1 for opcode, 1 for argument)
        self.registers[Register::PC as usize] = pc + 2;

        let op = parse_instructions(ins)?;

        // Debug output - consider making this optional or moving to a debug method
        println!(
            "Instruction: opcode=0x{:02X}, arg=0x{:02X} @ PC={} => {op:?}",
            opcode, arg, pc
        );
        println!("SP: 0x{:04X}", self.registers[Register::SP as usize]);

        // Execute the operation
        match op {
            Op::Nop => Ok(()),
            Op::Push(v) => self.push(v.into()),
            Op::PopRegister(r) => {
                let value = self.pop()?;
                self.registers[r as usize] = value;
                Ok(())
            }
            Op::AddStack => {
                let a = self.pop()?;
                let b = self.pop()?;
                let result = a + b;
                println!("AddStack: {} + {} = {}", b, a, result);
                self.push(result)?;
                Ok(())
            }
            Op::AddRegister(r1, r2) => {
                self.registers[r1 as usize] += self.registers[r2 as usize];
                Ok(())
            }
            Op::Signal(s) => {
                let sig_fn = self
                    .signal_handlers
                    .get(&s)
                    .ok_or(format!("unknown signal - 0x{:X}", s))?;
                sig_fn(self)
            }
        }
    }
}
