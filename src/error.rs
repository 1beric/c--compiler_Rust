/*
 * ./src/error.rs
 * Brandon Erickson --- brandonscotterickson@gmail.com
 * This file implements the error messaging and exit statements for the program.
 */

/*
 * this prints an error with the line and a specific char
 *  line: i32 -- the line number
 *  curr: char -- the char that caused the error.
 */
pub fn print_err_ch(line: i32, curr: char) {
    eprintln!("ERROR ----- LINE {} ----- CHAR {}", line, curr);
    std::process::exit(1);
}

/*
 * this prints an error with the line and a specific char
 *  line: i32 -- the line number
 *  curr: Token -- the Token that caused the error.
 */
pub fn print_err_tok(line: i32, curr: &mut crate::scanner::Token) {
    eprintln!("ERROR ----- LINE {}\n      ----- TOKEN {:?}", line, curr);
    std::process::exit(1);
}

/*
 * this prints an error with the line and a specific char
 *  line: i32 -- the line number
 *  curr: Token -- the Token that caused the error.
 *  rule: &sstr -- the rule that caused the error.
 */
pub fn print_err_rule(line: i32, curr: &mut crate::scanner::Token, rule: &str) {
    eprintln!(
        "ERROR ----- LINE {}\n      ----- TOKEN {:?}\n      ----- MSSG {}",
        line, curr, rule
    );
    std::process::exit(1);
}

/*
 * this prints an error the the file ended too soon
 */
pub fn print_eof() {
    eprintln!("END OF FILE");
    std::process::exit(1);
}

/*
 * this prints an error with a message
 *  msg: &str -- the message
 */
pub fn print_err_msg(msg: &str) {
    eprintln!("MESSAGE: {}", msg);
    std::process::exit(1);
}
