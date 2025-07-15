use crate::memory::{Addressable, LinearMemory};

/// Register implementation
#[derive(Debug)]
#[repr(u8)]
pub enum Register {
    A,
    B,
    C,
    M,
    SP,
    PC,
    BP,
    FLAGS,
}

impl Register {
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
}

#[derive(Debug)]
#[repr(u8)]
pub enum Op {
    Nop,
    Push(u8),
    PopRegister(Register),
    AddStack,
    AddRegister(Register, Register),
}

impl Op {
    pub fn value(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    pub fn equals(x: u8, other: Self) -> bool {
        x == other.value()
    }
}

// Instruction format in memory:
// [Address N]   : OPCODE (8 bits)
// [Address N+1] : ARGUMENT (8 bits)
//
// When processed by parse_instructions, these are combined into a 16-bit value:
// Instruction = [ 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 ]
//                   ARGUMENT      |     OPCODE
//                (upper 8 bits)   |  (lower 8 bits)
fn parse_instructions(ins: u16) -> Result<Op, String> {
    let op = (ins & 0xff) as u8;

    match op {
        x if x == Op::Nop.value() => Ok(Op::Nop),
        x if x == Op::Push(0).value() => {
            let arg = (ins & 0xff00) >> 8;
            Ok(Op::Push(arg as u8))
        }
        x if x == Op::PopRegister(Register::A).value() => {
            let arg = (ins & 0xf00) >> 8;
            if let Some(r) = Register::from_u8(arg.try_into().unwrap()) {
                Ok(Op::PopRegister(r))
            } else {
                Err(format!("unknown register - 0x{:X}", arg))
            }
        }
        x if x == Op::AddStack.value() => Ok(Op::AddStack),
        _ => Err(format!("unknown op - 0x{:X}", op)),
    }
}

pub struct Machine {
    pub registers: [u16; 8],
    pub memory: Box<dyn Addressable>,
}

impl Machine {
    pub fn new() -> Self {
        let memory_size = 8 * 1024; // 8 KB
        let mut machine = Self {
            registers: [0; 8],
            memory: Box::new(LinearMemory::new(memory_size)),
        };
        // Initialize SP to point to the beginning of stack area
        // Starting at address 0x1000 gives plenty of room for both code and stack
        machine.registers[Register::SP as usize] = 0x1000;

        // Initialize PC to 0 (program starts at the beginning of memory)
        machine.registers[Register::PC as usize] = 0;
        machine
    }

    pub fn get_register(&self, r: Register) -> u16 {
        self.registers[r as usize]
    }

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

    pub fn push(&mut self, v: u16) -> Result<(), String> {
        // For push, first write at current SP, then increment
        let sp = self.registers[Register::SP as usize];
        if !self.memory.write2(sp, v) {
            return Err(format!("memory write fault - 0x{:X}", sp));
        }
        self.registers[Register::SP as usize] += 2;
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), String> {
        let pc = self.registers[Register::PC as usize];

        // Read opcode and argument as separate bytes
        let opcode = self.memory.read(pc).unwrap_or(0);
        let arg = self.memory.read(pc + 1).unwrap_or(0);

        // Construct the 16-bit instruction with opcode in lower byte and argument in upper byte
        let instruction = opcode as u16 | ((arg as u16) << 8);

        // Increment the Program Counter register by 2 to move to the next instruction
        // (each instruction is 2 bytes: 1 for opcode, 1 for argument)
        self.registers[Register::PC as usize] = pc + 2;

        let op = parse_instructions(instruction)?;

        println!(
            "Instruction: opcode=0x{:02X}, arg=0x{:02X} @ PC={} => {op:?}",
            opcode, arg, pc
        );
        println!("SP: 0x{:04X}", self.registers[Register::SP as usize]);

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
                self.push(result)?;
                Ok(())
            }
            Op::AddRegister(r1, r2) => {
                self.registers[r1 as usize] += self.registers[r2 as usize];
                Ok(())
            }
        }
    }
}
