/*
 *
 *
 *
 *
 *
 */

use lazy_static::lazy_static; // 1.4.0
use std::fs;
use std::sync::Mutex;

use crate::error;

lazy_static! {
    static ref contents: Mutex<Vec<char>> = Mutex::new(Vec::new());
    static ref offset: Mutex<i32> = Mutex::new(0);
    pub static ref line: Mutex<i32> = Mutex::new(1);
}

pub fn init(file: &String) {
    for c in fs::read_to_string(file)
        .expect("Could not read file.")
        .chars()
    {
        contents.lock().unwrap().push(c);
    }
    println!("file is as follows:\n");
    let chars = contents.lock().unwrap();
    for i in 0..chars.len() {
        print!("{}", chars[i as usize]);
    }
    print!("\n");
}

pub fn reset_file() {
    let mut out = offset.lock().unwrap();
    *out = 0;
}

pub fn get_token() -> Token {
    let mut curr: char;

    curr = next_char(true);
    if curr == '\0' {
        return Token::EOF;
    }

    while is_comment(&mut curr) || is_whitespace(&mut curr) {}

    match curr {
        '{' => return Token::LBRACE,
        '}' => return Token::RBRACE,
        '(' => return Token::LPAREN,
        ')' => return Token::RPAREN,
        ',' => return Token::COMMA,
        ';' => return Token::SEMI,
        '+' => return Token::ARITH(String::from("+")),
        '-' => return Token::ARITH(String::from("-")),
        '*' => return Token::ARITH(String::from("*")),
        '/' => return Token::ARITH(String::from("/")),
        '0'..='9' => return match_intconst(&mut curr),
        'e' | 'i' | 'w' | 'r' => return match_kw(&mut curr),
        '|' => {
            curr = next_char(false);
            if curr != '|' {
                error::print_err_ch(*line.lock().unwrap(), curr);
            }
            return Token::BOOL(String::from("||"));
        }
        '&' => {
            curr = next_char(false);
            if curr != '&' {
                error::print_err_ch(*line.lock().unwrap(), curr);
            }
            return Token::BOOL(String::from("&&"));
        }
        '!' => {
            curr = next_char(false);
            if curr != '=' {
                error::print_err_ch(*line.lock().unwrap(), curr);
            }
            return Token::BOOL(String::from("!="));
        }
        '=' => {
            curr = next_char(false);
            if curr != '=' {
                unget_char();
                return Token::ASSG;
            }
            return Token::BOOL(String::from("=="));
        }
        '<' => {
            curr = next_char(false);
            if curr != '=' {
                unget_char();
                return Token::BOOL(String::from("<"));
            }
            return Token::BOOL(String::from("<="));
        }
        '>' => {
            curr = next_char(false);
            if curr != '=' {
                unget_char();
                return Token::BOOL(String::from(">"));
            }
            return Token::BOOL(String::from(">="));
        }
        _ => {
            let mut s = String::new();
            s.push(curr);
            return match_id(&mut s);
        }
    };
}

fn match_intconst(curr: &mut char) -> Token {
    let mut curr_int = (*curr as i32) - 48;
    loop {
        *curr = next_char(false);
        if *curr > '9' || *curr < '0' {
            break;
        }
        curr_int *= 10;
        curr_int += (*curr as i32) - 48;
    }
    if !is_whitespace(curr) {
        unget_char();
    }
    return Token::INTCONST(curr_int);
}

fn match_kw(curr: &mut char) -> Token {
    let mut sofar = String::new();
    sofar.push(*curr);

    match *curr {
        'i' => {
            *curr = next_char(false);
            match *curr {
                'f' => {
                    sofar.push(*curr);
                    *curr = peek_char();
                    if (*curr).is_alphanumeric() || *curr == '_' {
                        return match_id(&mut sofar);
                    }
                    return Token::KW(sofar);
                }
                'n' => {
                    sofar.push(*curr);
                    *curr = next_char(false);
                    if *curr != 't' {
                        unget_char();
                        return match_id(&mut sofar);
                    }
                    sofar.push(*curr);
                    *curr = peek_char();
                    if (*curr).is_alphanumeric() || *curr == '_' {
                        return match_id(&mut sofar);
                    }
                    return Token::KW(sofar);
                }
                _ => {
                    unget_char();
                    return match_id(&mut sofar);
                }
            }
        }
        'e' => {
            *curr = next_char(false);
            if *curr != 'l' {
                unget_char();
                return match_id(&mut sofar);
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 's' {
                unget_char();
                return match_id(&mut sofar);
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 'e' {
                unget_char();
                return match_id(&mut sofar);
            }
            sofar.push(*curr);
            *curr = peek_char();
            if (*curr).is_alphanumeric() || *curr == '_' {
                return match_id(&mut sofar);
            }
            return Token::KW(sofar);
        }
        'r' => {
            *curr = next_char(false);
            if *curr != 'e' {
                unget_char();
                return match_id(&mut sofar);
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 't' {
                unget_char();
                return match_id(&mut sofar);
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 'u' {
                unget_char();
                return match_id(&mut sofar);
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 'r' {
                unget_char();
                return match_id(&mut sofar);
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 'n' {
                unget_char();
                return match_id(&mut sofar);
            }
            sofar.push(*curr);
            *curr = peek_char();
            if (*curr).is_alphanumeric() || *curr == '_' {
                return match_id(&mut sofar);
            }
            return Token::KW(sofar);
        }
        'w' => {
            *curr = next_char(false);
            if *curr != 'h' {
                unget_char();
                return match_id(&mut sofar);
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 'i' {
                unget_char();
                return match_id(&mut sofar);
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 'l' {
                unget_char();
                return match_id(&mut sofar);
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 'e' {
                unget_char();
                return match_id(&mut sofar);
            }
            sofar.push(*curr);
            *curr = peek_char();
            if (*curr).is_alphanumeric() || *curr == '_' {
                return match_id(&mut sofar);
            }
            return Token::KW(sofar);
        }
        _ => {
            sofar.pop();
            unget_char();
            return match_id(&mut sofar);
        }
    }
}

/*
    char ch;
    while (isalnum(ch = get_next()) || ch == '_')
        strncat(lexeme, &ch, 1);
    if (!isspace(ch))
        unget(ch);
    return ID;
*/
fn match_id(sofar: &mut String) -> Token {
    let mut curr;
    loop {
        curr = next_char(false);
        if !curr.is_alphanumeric() && curr != '_' {
            break;
        }
        sofar.push(curr);
    }
    unget_char();

    return Token::ID(sofar.to_string());
}

fn is_comment(curr: &mut char) -> bool {
    if *curr == '/' {
        *curr = next_char(false);
        if *curr == '*' {
            // println!("block comment");
            loop {
                *curr = next_char(false);
                while *curr == '*' {
                    *curr = next_char(false);
                    if *curr == '/' {
                        return true;
                    }
                }
            }
        } else if *curr == '/' {
            // println!("line comment");
            *curr = next_char(false);
            while *curr != '\n' {
                *curr = next_char(false);
            }
            return true;
        } else {
            unget_char();
            return false;
        }
    }
    false
}

fn is_whitespace(curr: &mut char) -> bool {
    if *curr == ' ' || *curr == '\n' || *curr == '\r' || *curr == '\t' {
        *curr = next_char(false);
        return true;
    }
    false
}

fn unget_char() {
    let out = contents.lock().unwrap()[FA_offset(-1)];
    if out == '\n' {
        let mut l = line.lock().unwrap();
        *l = *l - 1;
    }
}

fn next_char(eof_valid: bool) -> char {
    if contents.lock().unwrap().len() == *(offset.lock().unwrap()) as usize {
        if eof_valid {
            return '\0';
        }
        error::print_eof();
    }
    let out = contents.lock().unwrap()[FA_offset(1)];
    if out == '\n' {
        let mut l = line.lock().unwrap();
        *l = *l + 1;
    }
    out
}

fn peek_char() -> char {
    if contents.lock().unwrap().len() == *(offset.lock().unwrap()) as usize {
        error::print_eof();
    }
    contents.lock().unwrap()[FA_offset(0)]
}

fn FA_offset(off: i32) -> usize {
    let mut out = offset.lock().unwrap();
    *out = *out + off;
    return (*out - off) as usize;
}

#[derive(PartialEq, Debug)]
pub enum Token {
    UNDEF,
    EOF,
    ID(String),
    INTCONST(i32),
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    COMMA,
    SEMI,
    ASSG,
    KW(String),    /* kwINT, kwIF, kwELSE, kwWHILE, kwRETURN */
    ARITH(String), /* opADD, opSUB, opMUL, opDIV */
    BOOL(String),  /* opEQ, opNE, opGT, opGE, opLT, opLE, opAND, opOR, */
}
