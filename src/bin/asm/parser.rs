use crate::ir::Instruction;
use crate::lexer::Token;
use std::fmt;

#[derive(Debug)]
pub enum ParseErrorKind {
    UnexpectedToken(Token),
    MissingOperand(&'static str, &'static str),
    InvalidOperand(&'static str, Token),
    InsufficientTokens(usize, usize),
    JumpToInvalidTarget(Token),
}

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub position: usize,
    pub tokens_snapshot: Vec<Token>,
    pub context: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let position_info = format!("Error at token position {}", self.position);

        let error_details = match &self.kind {
            ParseErrorKind::UnexpectedToken(token) => format!("Unexpected token: {:?}", token),
            ParseErrorKind::MissingOperand(instr, expected) => {
                format!("Missing operand for {}. Expected {}", instr, expected)
            }
            ParseErrorKind::InvalidOperand(instr, token) => {
                format!("Invalid operand for {}: {:?}", instr, token)
            }
            ParseErrorKind::InsufficientTokens(expected, actual) => format!(
                "Expected at least {} more tokens, but found only {}",
                expected, actual
            ),
            ParseErrorKind::JumpToInvalidTarget(token) => {
                format!("Invalid jump target: {:?}", token)
            }
        };

        let context = if !self.context.is_empty() {
            format!(" - {}", self.context)
        } else {
            String::new()
        };

        let token_context = self.format_token_context();

        write!(
            f,
            "{}\n{}{}\n\n{}",
            position_info, error_details, context, token_context
        )
    }
}

impl ParseError {
    fn format_token_context(&self) -> String {
        let range_start = self.position.saturating_sub(2);
        let range_end = (self.position + 3).min(self.tokens_snapshot.len());

        let mut result = String::from("Token context:\n");

        for (idx, token) in self.tokens_snapshot[range_start..range_end]
            .iter()
            .enumerate()
        {
            let pos = range_start + idx;
            let marker = if pos == self.position { "→ " } else { "  " };
            result.push_str(&format!("{}{}: {:?}\n", marker, pos, token));
        }

        result
    }

    fn new(kind: ParseErrorKind, position: usize, tokens: &[Token]) -> Self {
        // Create a smaller snapshot of the tokens for context
        let snapshot_start = position.saturating_sub(3);
        let snapshot_end = (position + 4).min(tokens.len());
        let tokens_snapshot = tokens[snapshot_start..snapshot_end].to_vec();

        ParseError {
            kind,
            position,
            tokens_snapshot,
            context: String::new(),
        }
    }

    fn with_context(mut self, context: String) -> Self {
        self.context = context;
        self
    }
}

pub type ParseResult = Result<Vec<Instruction>, ParseError>;

pub fn parse_tokens(tokens: &[Token]) -> ParseResult {
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
                // Check if we have enough tokens
                if i + 1 >= tokens.len() {
                    return Err(ParseError::new(
                        ParseErrorKind::InsufficientTokens(1, 0),
                        i,
                        tokens,
                    )
                    .with_context("PUSH instruction requires an operand".into()));
                }

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
                    invalid => {
                        return Err(ParseError::new(
                            ParseErrorKind::InvalidOperand("PUSH", invalid.clone()),
                            i + 1,
                            tokens,
                        )
                        .with_context(
                            "PUSH expects an immediate value, hex value, or register".into(),
                        ));
                    }
                }
                i += 2;
            }
            Token::Keyword(k) if k == "PUSHR" => {
                // Check if we have enough tokens
                if i + 1 >= tokens.len() {
                    return Err(ParseError::new(
                        ParseErrorKind::InsufficientTokens(1, 0),
                        i,
                        tokens,
                    )
                    .with_context("PUSHR instruction requires a register operand".into()));
                }

                match &tokens[i + 1] {
                    Token::Register(r) => {
                        instructions.push(Instruction::PushRegister(r.clone()));
                        i += 2;
                    }
                    invalid => {
                        return Err(ParseError::new(
                            ParseErrorKind::InvalidOperand("PUSHR", invalid.clone()),
                            i + 1,
                            tokens,
                        )
                        .with_context("PUSHR expects a register name".into()));
                    }
                }
            }
            Token::Keyword(k) if k == "POP" => {
                // Check if we have enough tokens
                if i + 1 >= tokens.len() {
                    return Err(ParseError::new(
                        ParseErrorKind::InsufficientTokens(1, 0),
                        i,
                        tokens,
                    )
                    .with_context("POP instruction requires a register operand".into()));
                }

                match &tokens[i + 1] {
                    Token::Register(r) => {
                        instructions.push(Instruction::Pop(r.clone()));
                        i += 2;
                    }
                    invalid => {
                        return Err(ParseError::new(
                            ParseErrorKind::InvalidOperand("POP", invalid.clone()),
                            i + 1,
                            tokens,
                        )
                        .with_context("POP expects a register name".into()));
                    }
                }
            }
            Token::Keyword(k) if k == "ADDS" => {
                instructions.push(Instruction::AddStack);
                i += 1;
            }
            Token::Keyword(k) if k == "ADDR" => {
                // Check if we have enough tokens
                if i + 2 >= tokens.len() {
                    return Err(ParseError::new(
                        ParseErrorKind::InsufficientTokens(2, tokens.len() - i - 1),
                        i,
                        tokens,
                    )
                    .with_context("ADDR instruction requires two register operands".into()));
                }

                match (&tokens[i + 1], &tokens[i + 2]) {
                    (Token::Register(r1), Token::Register(r2)) => {
                        instructions.push(Instruction::AddRegister(r1.clone(), r2.clone()));
                        i += 3;
                    }
                    (Token::Register(_), invalid) => {
                        return Err(ParseError::new(
                            ParseErrorKind::InvalidOperand(
                                "ADDR (second operand)",
                                invalid.clone(),
                            ),
                            i + 2,
                            tokens,
                        )
                        .with_context("ADDR expects two register names".into()));
                    }
                    (invalid, _) => {
                        return Err(ParseError::new(
                            ParseErrorKind::InvalidOperand("ADDR (first operand)", invalid.clone()),
                            i + 1,
                            tokens,
                        )
                        .with_context("ADDR expects two register names".into()));
                    }
                }
            }
            Token::Keyword(k) if k == "SIG" => {
                // Check if we have enough tokens
                if i + 1 >= tokens.len() {
                    return Err(ParseError::new(
                        ParseErrorKind::InsufficientTokens(1, 0),
                        i,
                        tokens,
                    )
                    .with_context("SIG instruction requires a hex value operand".into()));
                }

                match &tokens[i + 1] {
                    Token::Hex(n) => {
                        instructions.push(Instruction::Signal(*n));
                        i += 2;
                    }
                    invalid => {
                        return Err(ParseError::new(
                            ParseErrorKind::InvalidOperand("SIG", invalid.clone()),
                            i + 1,
                            tokens,
                        )
                        .with_context("SIG expects a hex value".into()));
                    }
                }
            }
            // Token::Keyword(k) if k == "JMP" => {
            //     // Check if we have enough tokens
            //     if i + 1 >= tokens.len() {
            //         return Err(ParseError::new(
            //             ParseErrorKind::InsufficientTokens(1, 0),
            //             i,
            //             tokens,
            //         )
            //         .with_context("JMP instruction requires a label operand".into()));
            //     }

            //     match &tokens[i + 1] {
            //         Token::Keyword(label) => {
            //             instructions.push(Instruction::Jump(label.clone()));
            //             i += 2;
            //         }
            //         invalid => {
            //             return Err(ParseError::new(
            //                 ParseErrorKind::JumpToInvalidTarget(invalid.clone()),
            //                 i + 1,
            //                 tokens,
            //             )
            //             .with_context("JMP expects a label identifier".into()));
            //         }
            //     }
            // }
            Token::Keyword(k) if k == "JMP" || k == "JUMP" => {
                // Just add a TODO for jump instructions
                todo!("Jump instructions not yet implemented: {}", k);
            }
            unexpected => {
                return Err(ParseError::new(
                    ParseErrorKind::UnexpectedToken(unexpected.clone()),
                    i,
                    tokens,
                )
                .with_context(format!(
                    "Unrecognized token in instruction position: {:?}",
                    unexpected
                )));
            }
        }
    }

    Ok(instructions)
}
