#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    CLS,
    RET,
    JP,
    CALL,
    SE,
    SNE,
    LD,
    ADD,
    SUB,
    SUBN,
    OR,
    AND,
    XOR,
    SHR,
    SKP,
    SKNP,
    DRW,
    RND,
    DB,
    NEWLINE,
    LABEL(String),
    IDENT(String),
    NUMBER(u16),
    REGISTER(u8),
    RegI,
    RegDT,
    RegST,
    KeyK,
    FontF,
    BCD,
    IndirectI,
}

fn get_instruct(s: &str) -> &str {
    match s.find(char::is_whitespace) {
        Some(index) => &s[..index],
        None => s,
    }
}

fn lex_operand(s: &str) -> Token {
    match s {
        "I" => Token::RegI,
        "DT" => Token::RegDT,
        "ST" => Token::RegST,
        "K" => Token::KeyK,
        "F" => Token::FontF,
        "B" => Token::BCD,
        "[I]" => Token::IndirectI,
        _ => {
            if s.starts_with("0x") {
                Token::NUMBER(u16::from_str_radix(&s[2..], 16).unwrap())
            } else if s.chars().all(|c| c.is_ascii_digit()) {
                Token::NUMBER(s.parse().unwrap())
            } else if s.starts_with('V') {
                Token::REGISTER(u8::from_str_radix(&s[1..], 16).unwrap())
            } else {
                Token::IDENT(s.to_string())
            }
        }
    }
}

pub fn lex_source(source: &str) -> Vec<Token> {
    let mut lexer: Vec<Token> = Vec::new();

    for mut line in source.lines() {
        line = line.trim();

        if line.is_empty() {
            continue;
        }

        let instruct = get_instruct(line);

        if instruct.starts_with(";") {
            continue;
        } else if instruct.ends_with(":") {
            lexer.push(Token::LABEL(instruct.trim_end_matches(':').to_string()));
        } else {
            match instruct {
                "db" => lexer.push(Token::DB),
                "CLS" => lexer.push(Token::CLS),
                "RET" => lexer.push(Token::RET),
                "JP" => lexer.push(Token::JP),
                "CALL" => lexer.push(Token::CALL),
                "SE" => lexer.push(Token::SE),
                "SNE" => lexer.push(Token::SNE),
                "LD" => lexer.push(Token::LD),
                "ADD" => lexer.push(Token::ADD),
                "SUB" => lexer.push(Token::SUB),
                "SUBN" => lexer.push(Token::SUBN),
                "OR" => lexer.push(Token::OR),
                "AND" => lexer.push(Token::AND),
                "XOR" => lexer.push(Token::XOR),
                "SHR" => lexer.push(Token::SHR),
                "SKP" => lexer.push(Token::SKP),
                "SKNP" => lexer.push(Token::SKNP),
                "DRW" => lexer.push(Token::DRW),
                "RND" => lexer.push(Token::RND),
                _ => panic!("Unsupported instruct {:?}", instruct),
            }

            let rest = line[instruct.len()..].trim();

            let rest = match rest.find(';') {
                Some(i) => rest[..i].trim(),
                None => rest,
            };

            for operand in rest.split(',') {
                let operand = operand.trim();
                if !operand.is_empty() {
                    lexer.push(lex_operand(operand));
                }
            }
        }

        lexer.push(Token::NEWLINE);
    }

    lexer
}
