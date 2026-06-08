use crate::{ast::Instruction, lexer::Token};
use std::{collections::HashMap, path::PathBuf};

pub fn compile(instructions: Vec<Instruction>, output_path: PathBuf) {
    let mut labels: HashMap<String, u16> = HashMap::new();
    let mut address: u16 = 0x200;

    for instruction in &instructions {
        match &instruction.mnemonic {
            Token::LABEL(name) => {
                // labels don't advance the address counter
                labels.insert(name.clone(), address);
            }
            Token::DB => address += 1, // db is one byte
            _ => address += 2,         // all opcodes are two bytes
        }
    }

    let mut output: Vec<u8> = Vec::new();

    for instruction in &instructions {
        match (&instruction.mnemonic, instruction.operands.as_slice()) {
            (Token::LABEL(_), _) => {
                // skip, already handled in first pass
            }
            (Token::DB, [Token::NUMBER(n)]) => {
                output.push(*n as u8);
            }
            (Token::CLS, []) => {
                output.push(0x00);
                output.push(0xE0);
            }
            (Token::RET, []) => {
                output.push(0x00);
                output.push(0xEE);
            }
            (Token::JP, [Token::IDENT(label)]) => {
                let addr = labels[label];
                output.push(0x10 | ((addr >> 8) as u8));
                output.push((addr & 0xFF) as u8);
            }
            (Token::JP, [Token::NUMBER(addr)]) => {
                output.push(0x10 | ((addr >> 8) as u8));
                output.push((addr & 0xFF) as u8);
            }
            (Token::LD, [Token::REGISTER(x), Token::NUMBER(kk)]) => {
                output.push(0x60 | *x);
                output.push(*kk as u8);
            }
            (Token::LD, [Token::RegI, Token::IDENT(label)]) => {
                let addr = labels[label];
                output.push(0xA0 | ((addr >> 8) as u8));
                output.push((addr & 0xFF) as u8);
            }
            (Token::LD, [Token::RegI, Token::NUMBER(addr)]) => {
                output.push(0xA0 | ((addr >> 8) as u8));
                output.push((addr & 0xFF) as u8);
            }
            (Token::ADD, [Token::REGISTER(x), Token::REGISTER(y)]) => {
                output.push(0x80 | x);
                output.push((y << 4) | 0x04);
            }
            (Token::DRW, [Token::REGISTER(x), Token::REGISTER(y), Token::NUMBER(n)]) => {
                output.push(0xD0 | x);
                output.push((y << 4) | (*n as u8));
            }
            _ => panic!("unhandled instruction: {:?}", instruction),
        }
    }

    std::fs::write(output_path, &output).unwrap();
}
