use crate::ir::Instruction;
use crate::lexer::Token;

pub fn parse_tokens(tokens: &[Token]) -> Vec<Instruction> {
    let mut i = 0;
    let mut instructions = Vec::new();

    while i < tokens.len() {
        match &tokens[i] {
            Token::LabelDecl(name) => {
                instructions.push(Instruction::Label(name.clone()));
                i += 1;
            }
            Token::Keyword(k) if k == "NOP" => {
                instructions.push(Instruction::Nop);
                i += 1;
            }
            Token::Keyword(k) if k == "PUSH" => {
                match &tokens[i + 1] {
                    Token::Immediate(n) => {
                        instructions.push(Instruction::PushImmediate(*n));
                    }
                    Token::Hex(n) => {
                        instructions.push(Instruction::PushHex(*n));
                    }
                    Token::Register(r) => {
                        instructions.push(Instruction::PushRegister(r.clone()));
                    }
                    _ => panic!("Invalid operand for PUSH"),
                }
                i += 2;
            }
            Token::Keyword(k) if k == "PUSHR" => {
                if let Token::Register(r) = &tokens[i + 1] {
                    instructions.push(Instruction::PushRegister(r.clone()));
                    i += 2;
                } else {
                    panic!("Invalid operand for PUSHR");
                }
            }
            Token::Keyword(k) if k == "POP" => {
                if let Token::Register(r) = &tokens[i + 1] {
                    instructions.push(Instruction::Pop(r.clone()));
                    i += 2;
                } else {
                    panic!("Invalid operand for POP");
                }
            }
            Token::Keyword(k) if k == "ADDS" => {
                instructions.push(Instruction::AddStack);
                i += 1;
            }
            Token::Keyword(k) if k == "ADDR" => {
                if let (Token::Register(r1), Token::Register(r2)) = (&tokens[i + 1], &tokens[i + 2])
                {
                    instructions.push(Instruction::AddRegister(r1.clone(), r2.clone()));
                    i += 3;
                } else {
                    panic!("Invalid operands for ADDR");
                }
            }
            Token::Keyword(k) if k == "SIG" => {
                if let Token::Hex(n) = &tokens[i + 1] {
                    instructions.push(Instruction::Signal(*n));
                    i += 2;
                } else {
                    panic!("Invalid operand for SIG");
                }
            }
            Token::Keyword(k) if k == "JMP" => {
                if let Token::Keyword(label) = &tokens[i + 1] {
                    instructions.push(Instruction::Jump(label.clone()));
                    i += 2;
                } else {
                    panic!("Invalid jump target");
                }
            }
            _ => panic!("Unexpected token: {:?}", tokens[i]),
        }
    }

    instructions
}
