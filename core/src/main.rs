fn main() -> Result<(), String> {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        panic!("Invalid number of arguments");
    }

    let file_path = argv[1].clone();
    let input = match std::fs::read_to_string(file_path) {
        Ok(input) => input,
        Err(e) => panic!("Failed to read file: {}", e),
    };
    let lexer = lex::Lexer::new(input);
    let ast = parse::parse(lexer)?;
    codegen::codegen(ast);

    Ok(())
}
