use crate::ast::Instruction;
use crate::lexer::Token;

pub fn build_ast(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    let chunks: Vec<&[Token]> = tokens
        .chunk_by(|a, _b| !matches!(a, Token::NEWLINE))
        .collect();

    for chunk in chunks {
        let token = chunk[0].clone();
        let ops = chunk[1..]
            .iter()
            .filter(|c| !matches!(c, Token::NEWLINE))
            .cloned()
            .collect();

        instructions.push(Instruction {
            mnemonic: token,
            operands: ops,
        });
    }

    instructions
}
