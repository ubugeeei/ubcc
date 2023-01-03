use crate::lex::Token;

mod lex;

fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    if argv.len() != 2 {
        eprintln!("Invalid number of arguments");
        std::process::exit(1);
    }

    let input = argv[1].as_str();
    let mut lexer = lex::Lexer::new(input);

    print!(".intel_syntax noprefix\n");
    print!(".global main\n");
    print!("main:\n");

    let mut current_token = lexer.next();
    let Token::Integer(int) = current_token else {
        panic!("Invalid token: {:?}", current_token);
    };
    print!("  mov rax, {}\n", int);

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
        print!("  {} rax, {}\n", op, int);
        current_token = lexer.next();
    }

    print!("  ret\n");
}
