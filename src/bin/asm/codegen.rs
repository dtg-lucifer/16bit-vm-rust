use crate::ir::Instruction;
use rustyvm::{Op, Register};
use std::collections::HashMap;

pub fn generate_bytecode(instrs: &[Instruction]) -> Result<Vec<u8>, String> {
    let mut bytecode = Vec::new();
    let mut labels = HashMap::new();

    // First pass: map labels to byte offsets
    let mut pc = 0;
    for instr in instrs {
        if let Instruction::Label(name) = instr {
            labels.insert(name.clone(), pc);
        } else {
            pc += 2;
        }
    }

    // Second pass: encode instructions
    for instr in instrs {
        match instr {
            Instruction::Nop => bytecode.extend([Op::Nop.value(), 0]),
            Instruction::PushImmediate(n) => {
                bytecode.extend([Op::Push(0).value(), *n]);
            }
            Instruction::PushHex(n) => {
                bytecode.extend([Op::Push(0).value(), *n]);
            }
            Instruction::PushRegister(r) => {
                let reg = Register::from_str(r).map_err(|_| format!("Invalid register: {}", r))?;
                bytecode.extend([Op::PushRegister(Register::A).value(), reg as u8]);
            }
            Instruction::Pop(r) => {
                let reg = Register::from_str(r).map_err(|_| format!("Invalid register: {}", r))?;
                bytecode.extend([Op::PopRegister(Register::A).value(), reg as u8]);
            }
            Instruction::AddStack => {
                bytecode.extend([Op::AddStack.value(), 0]);
            }
            Instruction::AddRegister(r1, r2) => {
                let reg1 =
                    Register::from_str(r1).map_err(|_| format!("Invalid register: {}", r1))?;
                let reg2 =
                    Register::from_str(r2).map_err(|_| format!("Invalid register: {}", r2))?;
                let m_r = (reg1 as u8) << 4 | (reg2 as u8);
                bytecode.extend([Op::AddRegister(Register::A, Register::B).value(), m_r]);
            }
            Instruction::Signal(n) => {
                bytecode.extend([Op::Signal(0).value(), *n]);
            }
            Instruction::Jump(label) => {
                // let offset = labels
                //     .get(label)
                //     .ok_or_else(|| format!("Undefined label: {}", label))?;
                // bytecode.extend([Op::Jump.value(), *offset as u8]);
                todo!("unimplemented - {label}")
            }
            Instruction::Label(_) => {} // Skip label in final bytecode
        }
    }

    Ok(bytecode)
}
