use std::fmt::Display;

use crate::memory::{Addressable, LinearMemory};

/// Register implementation
#[derive(Debug)]
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

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Push,
    Pop,
    AddStack,
    AddRegister,
}

#[derive(Debug)]
#[repr(u8)]
pub enum Op {
    Nop,
    Push(u8),
    Pop(Register),
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

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(0x{:?})", self)
    }
}

// Instruction = [ 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 ]
//                  OPERATOR       | ARG(s)
//                                 | 8 Bit literal
//                                 | REG1 | REG2
fn parse_instructions(x: u16) -> Result<Instruction, String> {
    let op = (x & 0xff) as u8;

    match op {
        _b if Op::equals(op, Op::Nop) => Ok(Instruction::Nop),
        _ => Err(format!("Error: not a valid instruction - 0x{:X}", op)),
    }
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

        let instruction = self.memory.read2(pc).unwrap();

        // Increment the Program Counter register
        //
        // By 2, because the memory is currently structured as 8 bits
        // while the vm itself is structured in 16bits, so we have to skip another
        // extra 8 bits in the memory to actually cross the whole
        // instruction
        self.registers[Register::PC as usize] = pc + 2;

        let op = parse_instructions(instruction)?;

        println!("{instruction} @ {pc} @ {op:?}");

        match op {
            Instruction::Nop => Ok(()),
            _ => Err(format!("Error: unknown operator 0x{:?}", op)),
        }
    }
}
