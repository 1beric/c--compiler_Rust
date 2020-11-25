pub fn print_err_ch(line: i32, curr: char) {
    eprintln!("ERROR ----- LINE {} ----- CHAR {}", line, curr);
    std::process::exit(1);
}

pub fn print_err_tok(line: i32, curr: &mut crate::scanner::Token) {
    eprintln!("ERROR ----- LINE {}\n      ----- TOKEN {:?}", line, curr);
    std::process::exit(1);
}

pub fn print_err_rule(line: i32, curr: &mut crate::scanner::Token, rule: &str) {
    eprintln!(
        "ERROR ----- LINE {}\n      ----- TOKEN {:?}\n      ----- MSSG {}",
        line, curr, rule
    );
    std::process::exit(1);
}

pub fn print_eof() {
    eprintln!("END OF FILE");
    std::process::exit(1);
}
