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
    let ast = parse::parse(input);

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("");

    println!("main:");
    println!("  # prologue");
    println!("  # allocate 26 * 8 bytes for local variables");
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");
    println!("");

    codegen::gen(ast);
    println!("");

    println!("  # epilogue");
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}
