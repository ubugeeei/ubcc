fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        panic!("Invalid number of arguments");
    }

    let input = argv[1].clone();
    codegen::codegen(input);
}
