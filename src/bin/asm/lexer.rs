#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// e.g. PUSH, POP, etc.
    Keyword(String),
    /// e.g. A, B, C, M, R0, R1 etc.
    Register(String),
    /// e.g. #42
    Immediate(u8),
    /// e.g. $2A
    Hex(u8),
    /// e.g. label: in the form of `label:`
    LabelDecl(String),
}

impl Token {
    pub fn tokenize_line(line: &str) -> Vec<Self> {
        let line = line.trim();
        if line.ends_with(":") {
            return vec![Token::LabelDecl(line.trim_end_matches(":").to_string())];
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut tokens = Vec::new();

        for part in parts {
            if part.starts_with("#") {
                let val = part.trim_start_matches('#').parse::<u8>().unwrap();
                tokens.push(Token::Immediate(val));
            } else if part.starts_with("$") {
                let val = u8::from_str_radix(part.trim_start_matches('$'), 16).unwrap();
                tokens.push(Token::Hex(val));
            } else if ["A", "B", "C", "D"].contains(&part) {
                tokens.push(Token::Register(part.to_string()));
            } else if part.chars().all(char::is_alphanumeric) {
                tokens.push(Token::Keyword(part.to_uppercase()));
            } else {
                panic!("Unknown token: {}", part);
            }
        }
        tokens
    }
}
