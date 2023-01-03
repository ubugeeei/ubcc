use crate::lex::Token;
use std::io::Write;

mod lex;

fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    if argv.len() != 2 {
        panic!("Invalid number of arguments");
    }

    let input = argv[1].as_str();
    let mut lexer = lex::Lexer::new(input);

    let out = std::io::stdout();
    let mut out = out.lock();

    let _ = write!(out, ".intel_syntax noprefix\n");
    let _ = write!(out, ".global main\n");
    let _ = write!(out, "main:\n");

    let mut current_token = lexer.next();
    let Token::Integer(int) = current_token else {
        panic!("Invalid token: {:?}", current_token);
    };
    let _ = write!(out, "  mov rax, {}\n", int);

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
