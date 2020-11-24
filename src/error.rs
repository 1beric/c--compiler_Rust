pub fn print_err(line: i32, curr: char) {
    eprintln!("ERROR ----- LINE {} ----- CHAR {}", line, curr);
    std::process::exit(1);
}

pub fn print_eof() {
    eprintln!("END OF FILE");
    std::process::exit(1);
}
