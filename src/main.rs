use crate::lex::Token;
use std::io::{BufWriter, Write};

mod lex;

fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    if argv.len() != 2 {
        panic!("Invalid number of arguments");
    }

    let input = argv[1].as_str();
    let mut lexer = lex::Lexer::new(input);

    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());

    write!(out, ".intel_syntax noprefix\n").unwrap();
    write!(out, ".global main\n").unwrap();
    write!(out, "main:\n").unwrap();

    let mut current_token = lexer.next();
    let Token::Integer(int) = current_token else {
        panic!("Invalid token: {:?}", current_token);
    };
    write!(out, "  mov rax, {}\n", int).unwrap();

    current_token = lexer.next();
    while current_token != Token::Eof {
        let op = match current_token {
            Token::Plus => "add",
            Token::Minus => "sub",
            _ => {
                panic!("Invalid token: {:?}", current_token);
            }
        };
        current_token = lexer.next();
        let Token::Integer(int) = current_token else {
            panic!("Invalid token: {:?}", current_token);
        };
        let _ = write!(out, "  {} rax, {}\n", op, int);
        current_token = lexer.next();
    }

    let _ = write!(out, "  ret\n");
}
