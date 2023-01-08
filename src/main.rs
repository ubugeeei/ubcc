mod ast;
mod codegen;
mod lex;
mod parse;

fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        panic!("Invalid number of arguments");
    }
    let input = argv[1].clone();

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    let ast = parse::parse(input);
    codegen::gen(ast);
    println!("  pop rax");
    println!("  ret");
}
