use compile::gen;

fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        panic!("Invalid number of arguments");
    }
    let input = argv[1].clone();

    let ast = match parse::parse(input) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    gen(ast);
}
