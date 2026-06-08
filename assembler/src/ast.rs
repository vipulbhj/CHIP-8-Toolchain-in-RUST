use crate::lexer::Token;

#[derive(Debug)]
pub struct Instruction {
    pub mnemonic: Token,
    pub operands: Vec<Token>,
}
