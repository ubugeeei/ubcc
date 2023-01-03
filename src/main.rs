fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    if argv.len() != 2 {
        eprintln!("Invalid number of arguments");
        std::process::exit(1);
    }

    let input = argv[1].as_str();

    print!(".intel_syntax noprefix\n");
    print!(".global main\n");
    print!("main:\n");

    let mut left_str = input;
    let (num, tail) = strtol(&mut left_str);

    print!("  mov rax, {}\n", num);
    left_str = tail;

    while left_str.len() > 0 {
        let op = match left_str.chars().next().unwrap() {
            '+' => "add",
            '-' => "sub",
            _ => panic!("Invalid operator: {:?}", left_str.chars().next()),
        };
        left_str = &left_str[1..];

        let (num, tail) = strtol(&mut left_str);
        left_str = tail;

        print!(
            "  {} rax, {}\n",
            op,
            num
        );
    }

    print!("  ret\n");
}

/// **Split a string into two parts at the first non-numeric character.** <br/>
/// return -> (head, tail)
/// ```
/// assert_eq!(strtol("123abc123"), ("123", "abc123"));
/// ```
fn strtol(s: &str) -> (&str, &str) {
    let first_non_num = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num)
}
