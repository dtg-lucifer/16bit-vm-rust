#[derive(Debug, Clone)]
pub enum Instruction {
    Nop,
    PushImmediate(u8),
    PushHex(u8),
    PushRegister(String),
    Pop(String),
    AddStack,
    AddRegister(String, String),
    Signal(u8),
    Label(String),
    Jump(String),
}
