fn main() -> Result<(), String> {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        panic!("Invalid number of arguments");
    }

    let input = argv[1].clone();
    let lexer = lex::Lexer::new(input);
    let ast = parse::parse(lexer)?;
    codegen::codegen(ast);

    Ok(())
}
