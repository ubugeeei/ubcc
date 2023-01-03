fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    if argv.len() != 2 {
        eprintln!("Invalid number of arguments");
        std::process::exit(1);
    }

    print!(".intel_syntax noprefix\n");
    print!(".global main\n");
    print!("main:\n");
    print!("  mov rax, {}\n", argv[1]);
    print!("  ret\n");
}
