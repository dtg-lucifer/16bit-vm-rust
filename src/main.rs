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

struct Machine {
    registers: [u16; 8],
    memory: [u8; 5000],
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            registers: [0; 8],
            memory: [0; 5000],
        }
    }

    pub fn get_register(&self, reg: Register) -> u16 {
        match reg {
            Register::A => self.registers[0],
            Register::B => self.registers[1],
            Register::C => self.registers[2],
            Register::M => self.registers[3],
            Register::SP => self.registers[4],
            Register::PC => self.registers[5],
            Register::BP => self.registers[6],
            Register::FLAGS => self.registers[7],
        }
    }

    pub fn set_register(&mut self, reg: Register, value: u16) {
        match reg {
            Register::A => self.registers[0] = value,
            Register::B => self.registers[1] = value,
            Register::C => self.registers[2] = value,
            Register::M => self.registers[3] = value,
            Register::SP => self.registers[4] = value,
            Register::PC => self.registers[5] = value,
            Register::BP => self.registers[6] = value,
            Register::FLAGS => self.registers[7] = value,
        }
    }
}
