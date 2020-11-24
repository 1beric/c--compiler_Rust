use crate::scanner;

pub fn print_tokens() {
    loop {
        let tok = scanner::get_token();
        match tok {
            scanner::Token::UNDEF => {
                println!("UNDEF TOKEN");
                break;
            }
            scanner::Token::EOF => {
                println!("END OF FILE");
                break;
            }
            scanner::Token::KW(kw) => println!("KW Token: {}", kw),
            scanner::Token::ARITH(op) => println!("ARITH Token: {}", op),
            scanner::Token::BOOL(op) => println!("BOOL Token: {}", op),
            scanner::Token::ID(name) => println!("ID Token: {}", name),
            scanner::Token::INTCONST(val) => println!("INTCONST Token: {}", val),
            _ => println!("{:?} Token", tok),
        }
    }
    scanner::reset_file();
}
