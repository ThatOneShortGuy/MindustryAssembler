use std::{collections::HashMap, io::Write};

use regex::Regex;
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Label,
    Instruction,
    Comment,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: String,
    lineno: u32,
    source_lineno: u32,
}

pub fn tokenizer(input: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut lineno = 0;
    let re = Regex::new(r"^(?P<addr>&[^\s]+)?(?:\s+(?P<instr>[^#\n]+)?)?(?P<comment>#.*)?$").unwrap();
    for (source_lineno, line) in input.lines().enumerate() {
        let Some(caps) = re.captures(line) else {
            continue;
        };
        let addr = caps.name("addr");
        let instr = caps.name("instr");
        let comment = caps.name("comment");

        match addr {
            Some(addr) => {
                tokens.push(Token {
                    token_type: TokenType::Label,
                    value: addr.as_str().to_string(),
                    lineno: lineno,
                    source_lineno: source_lineno as u32,
                });
            }
            None => {}
        }

        match instr {
            Some(instr) => {
                tokens.push(Token {
                    token_type: TokenType::Instruction,
                    value: instr.as_str().to_string(),
                    lineno: lineno,
                    source_lineno: source_lineno as u32,
                });
                lineno += 1;
            }
            None => {}
        }

        match comment {
            Some(comment) => {
                tokens.push(Token {
                    token_type: TokenType::Comment,
                    value: comment.as_str().trim().to_string(),
                    lineno: lineno,
                    source_lineno: source_lineno as u32,
                });
            }
            None => {}
        }
    }
    tokens
}

pub fn create_symbol_table(tokens: &Vec<Token>) -> HashMap<String, u32> {
    let mut symbol_table: HashMap<String, u32> = HashMap::new();
    for token in tokens {
        match token.token_type {
            TokenType::Label => {
                symbol_table.insert(token.value.clone(), token.lineno);
            }
            _ => {}
        }
    }
    symbol_table
}

pub fn write_symbol_table(symbol_table: &HashMap<String, u32>, filename: &str) {
    let mut file = std::fs::File::create(filename).unwrap();
    for (key, value) in symbol_table {
        let res = file.write_all(format!("{}: {}\n", key, value).as_bytes());
        match res {
            Ok(_) => {}
            Err(e) => {
                println!("Error writing to file: {}", e);
                std::process::exit(1);
            }
        }
    }
}

pub fn convert_tokens(tokens: &Vec<Token>, symbol_table: &HashMap<String, u32>) -> String {
    let mut res: Vec<(Regex, u32)> = Vec::new();
    for (key, val) in symbol_table {
        res.push((Regex::new(key.as_str()).unwrap(), *val));
    }

    let mut output = String::new();
    let mut currline = tokens[0].source_lineno;

    for token in tokens {
        if token.token_type == TokenType::Label {
            continue;
        }

        while token.source_lineno > currline {
            output.push_str("\n");
            currline += 1;
        }

        // Functional programming go brrrr
        let val = res.iter().fold(token.value.clone(), |acc, (re, val)| {
            re.replace_all(acc.as_str(), val.to_string().as_str())
                .to_string()
        });

        output.push_str(val.as_str());
    }
    output
}
