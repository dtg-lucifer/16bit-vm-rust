//! VM core implementation for the 16-bit Virtual Machine.

use std::collections::HashMap;

use crate::{
    Register, execute_instruction,
    memory::{Addressable, LinearMemory},
    opcodes::parse_instructions,
};

/// Function type for signal handlers in the VM.
/// Called when the VM executes a SIGNAL instruction.
type SignalFunction = fn(&mut Machine) -> Result<(), String>;

/// The main virtual machine structure.
///
/// This struct represents the entire virtual machine, containing
/// registers, memory, and state information.
pub struct Machine {
    /// The VM's register set (13 registers, each 16 bits)
    pub registers: [u16; 13],
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
            registers: [0; 13],
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
            if reg_name == "SP" || reg_name == "PC" || reg_name == "FLAGS" {
                continue;
            }
            println!("\tRegister {}: 0x{:04X} ({})", reg_name, reg, reg);
        }
        println!(
            "\tStack Pointer (SP): 0x{:04X} ({})",
            self.registers[Register::SP as usize],
            self.registers[Register::SP as usize]
        );
        println!(
            "\tProgram Counter (PC): 0x{:04X} ({})",
            self.registers[Register::PC as usize],
            self.registers[Register::PC as usize]
        );
        println!(
            "\tFlags (8 bit): 0b{:08b} ({})",
            self.registers[Register::FLAGS as usize],
            self.registers[Register::FLAGS as usize],
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
            "Instruction: opcode=0x{:02X}, arg=0x{:02X} @ PC={} => {op:?}, SP=0x{:04X}",
            opcode,
            arg,
            pc,
            self.registers[Register::SP as usize]
        );

        execute_instruction(self, op)
    }
}
