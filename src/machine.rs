//! VM core implementation for the 16-bit Virtual Machine.
//!
//! This module contains the core components of the virtual machine:
//! registers, operations, instruction parsing, and execution logic.

use std::collections::HashMap;

use crate::memory::{Addressable, LinearMemory};

/// Register set for the 16-bit VM.
///
/// Each register is 16 bits (2 bytes) wide and serves a specific purpose:
/// - A, B, C: General purpose registers
/// - M: Memory operations register
/// - SP: Stack Pointer (grows upward, points to next free location)
/// - PC: Program Counter (points to next instruction)
/// - BP: Base Pointer (for function calls/stack frames)
/// - FLAGS: Status flags register
#[derive(Debug)]
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
    ///
    /// # Parameters
    /// * `v` - Numeric value representing a register
    ///
    /// # Returns
    /// * `Some(Register)` - If the value corresponds to a valid register
    /// * `None` - If the value does not match any register
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
/// Some operations include parameters that provide additional information
/// about how the operation should be performed.
#[derive(Debug)]
#[repr(u8)]
/// Operations supported by the VM.
///
/// Each operation corresponds to a specific instruction opcode.
/// The VM uses a 2-byte instruction format, where the first byte is the opcode
/// and the second byte is an argument (when applicable).
///
/// # Instruction Format
///
/// ```
/// +------------+------------+
/// | Byte 0     | Byte 1     |
/// +------------+------------+
/// | OPCODE     | ARGUMENT   |
/// +------------+------------+
/// ```
///
/// # Opcodes
///
/// - 0x00: NOP - No operation
/// - 0x01: PUSH - Push value onto stack
/// - 0x02: POPREGISTER - Pop value from stack into register
/// - 0x03: ADDSTACK - Add stack values
/// - 0x04: ADDREGISTER - Add register values
/// - 0x05: SIGNAL - Signal the VM
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
    /// # Returns
    /// The 8-bit opcode value
    ///
    /// # Safety
    /// This function uses unsafe code to extract the discriminant value
    /// of the enum variant. The #[repr(u8)] attribute ensures this is valid.
    pub fn value(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    /// Checks if a numeric opcode matches a specific operation.
    ///
    /// # Parameters
    /// * `x` - Numeric opcode to check
    /// * `other` - Operation to compare against
    ///
    /// # Returns
    /// `true` if the opcode matches the operation, `false` otherwise
    pub fn equals(x: u8, other: Self) -> bool {
        x == other.value()
    }
}

/// Parses a 16-bit instruction and extracts the 8-bit argument.
///
/// When instructions are loaded in memory, they are stored as:
/// ```
/// Memory[addr]   = OPCODE (8 bits)
/// Memory[addr+1] = ARGUMENT (8 bits)
/// ```
///
/// When read as a 16-bit value using little-endian format, they become:
/// ```
/// 16-bit value = ARGUMENT (upper 8 bits) | OPCODE (lower 8 bits)
/// ```
///
/// # Parameters
/// * `ins` - The 16-bit instruction to parse
///
/// # Returns
/// * `u8` - The 8-bit argument portion of the instruction
fn parse_instructions_arg(ins: u16) -> u8 {
    ((ins & 0xff00) >> 8) as u8
}

/// Parses a 16-bit instruction into an operation.
///
/// In memory, instructions are stored as two consecutive bytes:
/// ```
/// [Address N]   : OPCODE (8 bits)
/// [Address N+1] : ARGUMENT (8 bits)
/// ```
///
/// When read as a 16-bit value using little-endian format, they become:
/// ```
/// 16-bit value = [ ARGUMENT (8 bits) | OPCODE (8 bits) ]
///                 (upper 8 bits)      (lower 8 bits)
/// ```
///
/// This function extracts the opcode (lower 8 bits) from the instruction
/// and returns the corresponding operation with its argument.
///
/// # Parameters
/// * `ins` - The 16-bit instruction to parse
///
/// # Returns
/// * `Ok(Op)` - The parsed operation if successful
/// * `Err(String)` - Error message if the instruction contains an invalid opcode or argument
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
///
/// Signal handlers are called when the VM executes a SIGNAL instruction.
/// They take a mutable reference to the machine and return a Result.
///
/// # Example
///
/// ```
/// fn halt_signal(vm: &mut Machine) -> Result<(), String> {
///     vm.halt = true;
///     Ok(())
/// }
/// ```
type SignalFunction = fn(&mut Machine) -> Result<(), String>;

/// The main virtual machine structure.
///
/// This struct represents the entire virtual machine, containing
/// registers and memory. It provides methods for executing instructions
/// and manipulating the VM state.
/// The main virtual machine structure.
///
/// This struct represents the entire virtual machine, containing
/// registers, memory, and state information. It provides methods for
/// executing instructions and manipulating the VM state.
///
/// # Fields
///
/// * `registers` - Array of 8 16-bit registers (A, B, C, M, SP, PC, BP, FLAGS)
/// * `halt` - Boolean flag indicating whether the VM is halted
/// * `signal_handlers` - Map of signal codes to handler functions
/// * `memory` - VM memory space (implements the Addressable trait)
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
    ///
    /// Initializes an 8 KB memory space and sets up the registers:
    /// - SP (Stack Pointer) is set to 0x1000
    /// - PC (Program Counter) is set to 0
    /// - All other registers are initialized to 0
    ///
    /// # Returns
    /// A new Machine instance ready for use
    pub fn new() -> Self {
        let memory_size = 8 * 1024; // 8 KB
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
    ///
    /// # Parameters
    /// * `r` - The register to read
    ///
    /// # Returns
    /// The 16-bit value stored in the register
    pub fn get_register(&self, r: Register) -> u16 {
        self.registers[r as usize]
    }

    /// Sets the signal handler for specific signals
    ///
    /// This method:
    /// - Sets the behaviour of different signals
    /// - Gives the user full controll over the signals
    /// Defines a signal handler for a specific signal code.
    ///
    /// Signal handlers are functions that are called when the VM executes
    /// a SIGNAL instruction with the corresponding code.
    ///
    /// # Parameters
    ///
    /// * `index` - The 8-bit signal code to handle
    /// * `f` - The handler function to call when the signal is received
    pub fn define_handler(&mut self, index: u8, f: SignalFunction) {
        self.signal_handlers.insert(index, f);
    }

    /// Pops a 16-bit value from the stack.
    ///
    /// Stack operation: First decrement SP by 2, then read the value.
    /// If the memory read fails, SP is restored to its original value.
    ///
    /// # Returns
    /// * `Ok(u16)` - The popped value
    /// * `Err(String)` - Error message if the pop operation fails
    /// Pops a 16-bit value from the stack.
    ///
    /// Stack operation: First decrement SP by 2, then read the value.
    /// If the memory read fails, SP is restored to its original value.
    ///
    /// # Stack Memory Layout
    ///
    /// ```
    /// Before pop:              After pop (value=0x1234):
    /// +-------------+          +-------------+
    /// | 0x1004 (SP) |          | 0x1002 (SP) |
    /// +-------------+          +-------------+
    /// | 0x1002      | 0x1234   | 0x1002      | 0x1234
    /// +-------------+          +-------------+
    /// | 0x1000      | ...      | 0x1000      | ...
    /// +-------------+          +-------------+
    /// ```
    ///
    /// # Returns
    /// * `Ok(u16)` - The popped value
    /// * `Err(String)` - Error message if the pop operation fails
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
    ///
    /// Stack operation: First write the value at the current SP, then increment SP by 2.
    ///
    /// # Parameters
    /// * `v` - The 16-bit value to push
    ///
    /// # Returns
    /// * `Ok(())` - If the push was successful
    /// * `Err(String)` - Error message if the push operation fails
    /// Pushes a 16-bit value onto the stack.
    ///
    /// Stack operation: First write the value at the current SP, then increment SP by 2.
    ///
    /// # Stack Memory Layout
    ///
    /// ```
    /// Before push:             After push (value=0x5678):
    /// +-------------+          +-------------+
    /// | 0x1002 (SP) |          | 0x1004 (SP) |
    /// +-------------+          +-------------+
    /// | 0x1002      | ???      | 0x1002      | 0x5678
    /// +-------------+          +-------------+
    /// | 0x1000      | 0x1234   | 0x1000      | 0x1234
    /// +-------------+          +-------------+
    /// ```
    ///
    /// # Parameters
    /// * `v` - The 16-bit value to push
    ///
    /// # Returns
    /// * `Ok(())` - If the push was successful
    /// * `Err(String)` - Error message if the push operation fails
    pub fn push(&mut self, v: u16) -> Result<(), String> {
        // For push, first write at current SP, then increment
        let sp = self.registers[Register::SP as usize];
        if !self.memory.write2(sp, v) {
            return Err(format!("memory write fault - 0x{:X}", sp));
        }
        self.registers[Register::SP as usize] += 2;
        Ok(())
    }

    /// Print the current state of registers and pointers of the machine
    /// Prints the current state of the VM to the console.
    ///
    /// This method displays:
    /// - Register values (both in hexadecimal and decimal)
    /// - Stack pointer position
    /// - Program counter position
    ///
    /// This is useful for debugging and for seeing the final state
    /// after program execution.
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
    /// This method:
    /// 1. Reads the next instruction from memory at PC
    /// 2. Increments PC by 2 (each instruction is 2 bytes)
    /// 3. Parses the instruction
    /// 4. Executes the operation
    /// 5. Updates VM state accordingly
    ///
    /// # Returns
    /// * `Ok(())` - If the instruction executed successfully
    /// * `Err(String)` - Error message if execution failed
    /// Executes a single instruction in the VM.
    ///
    /// This method:
    /// 1. Reads the next instruction from memory at PC
    /// 2. Increments PC by 2 (each instruction is 2 bytes)
    /// 3. Parses the instruction into an operation
    /// 4. Executes the operation
    /// 5. Updates VM state accordingly
    ///
    /// # Execution Flow
    ///
    /// ```
    /// Fetch instruction → Increment PC → Parse instruction → Execute operation
    /// ```
    ///
    /// # Returns
    /// * `Ok(())` - If the instruction executed successfully
    /// * `Err(String)` - Error message if execution failed (e.g., invalid instruction)
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
