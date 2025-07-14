use crate::memory::{Addressable, LinearMemory};

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

#[repr(u8)]
pub enum Op {
    Nop,
}

pub struct Machine {
    pub registers: [u16; 8],
    pub memory: Box<dyn Addressable>,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
            memory: Box::new(LinearMemory::new(8 * 1024)), // 8 KB
        }
    }

    pub fn step(&mut self) -> Result<(), String> {
        let pc = self.registers[Register::PC as usize];

        // Instruction = [ 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 ]
        //                  OPERATOR       | ARG(s)
        //                                 | 8 Bit literal
        //                                 | REG1 | REG2
        let instruction = self.memory.read2(pc).unwrap();

        // Increment the Program Counter register
        //
        // By 2, because the memory is currently structured as 8 bits
        // while the vm itself is structured in 16bits, so we have to skip another
        // extra 8 bits in the memory to actually cross the whole
        // instruction
        self.registers[Register::PC as usize] = pc + 2;

        println!("{instruction} @ {pc}");

        let op = (instruction & 0xff) as u8;
        match op {
            x if x == Op::Nop as u8 => Ok(()),
            _ => Err(format!("Error: unknown operator 0x{:X}", op)),
        }
    }
}
