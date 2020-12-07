/*
 * ./src/scanner.rs
 * Brandon Erickson --- brandonscotterickson@gmail.com
 * This file implements the scanner for the C-- language. The language is defined at
 * http://www2.cs.arizona.edu/classes/cs453/fall20/PROJECT/SPEC/cminusminusspec.html#lexical
 * This file contains the logic for scanning the input file for Tokens defined in the
 *  struct Token.
 */

use lazy_static::lazy_static; // 1.4.0
use std::fs;
use std::sync::Mutex;

// this allows us to print error messages
use crate::error;

// these are the globals that originate from this file
lazy_static! {
    static ref contents: Mutex<Vec<char>> = Mutex::new(Vec::new()); // the file contents
    static ref offset: Mutex<i32> = Mutex::new(0); // the current offset of the file
    pub static ref line: Mutex<i32> = Mutex::new(1); // the current line number
}

/*
 * this method initializes the contents global with a file name
 *  file: &String -- the file name to load
 */
pub fn init(file: &String) {
    for c in fs::read_to_string(file)
        .expect("Could not read file.")
        .chars()
    {
        contents.lock().unwrap().push(c); // pushes all of the chars into contents
    }
}

/*
 * this method resets the contents global
 */
 pub fn reset_file() {
     let mut out = offset.lock().unwrap();
     *out = 0;
    }
    
/*
 * this method gets the next Token from the contents.
 * returns: Token -- the next Token
 */
pub fn get_token() -> Token {
    let mut curr: char;

    curr = next_char(true); // this allows for the token to be EOF
    if curr == '\0' { 
        return Token::EOF; // eof encountered
    }

    while is_whitespace(&curr) || is_comment(&curr) {
        curr = next_char(false); // must skip past comments and whitespace
    }


    // basic pattern matching
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
        '0'..='9' => return match_intconst(&mut curr), // this is the beginning of an intconst
        'e' | 'i' | 'w' | 'r' => return match_kw(&mut curr), // need to check for the keywords
        '|' => {
            curr = next_char(false);
            if curr != '|' {
                error::print_err_ch(*line.lock().unwrap(), curr);
            }
            return Token::BOOL(String::from("||")); // pattern || matched
        }
        '&' => {
            curr = next_char(false);
            if curr != '&' {
                error::print_err_ch(*line.lock().unwrap(), curr);
            }
            return Token::BOOL(String::from("&&")); // pattern && matched
        }
        '!' => {
            curr = next_char(false);
            if curr != '=' {
                error::print_err_ch(*line.lock().unwrap(), curr);
            }
            return Token::BOOL(String::from("!=")); // pattern != matched
        }
        '=' => {
            curr = next_char(false);
            if curr != '=' {
                unget_char();
                return Token::ASSG;
            }
            return Token::BOOL(String::from("==")); // pattern == matched
        }
        '<' => {
            curr = next_char(false);
            if curr != '=' {
                unget_char();
                return Token::BOOL(String::from("<"));
            }
            return Token::BOOL(String::from("<="));// pattern <= matched
        }
        '>' => {
            curr = next_char(false);
            if curr != '=' {
                unget_char();
                return Token::BOOL(String::from(">"));
            }
            return Token::BOOL(String::from(">=")); // pattern >= matched
        }
        _ => {
            let mut s = String::new();
            s.push(curr);
            return match_id(&mut s); // need to match an id
        }
    };
}

/*
 * this method matches an intconst and ensures it is valid
 */
fn match_intconst(curr: &mut char) -> Token {
    let mut curr_int = (*curr as i32) - 48;
    loop {
        *curr = next_char(false);
        if *curr > '9' || *curr < '0' { // break when curr is no longer a digit
            break;
        }
        curr_int *= 10; // shift the int to the left
        curr_int += (*curr as i32) - 48; // add to curr int
    }
    if curr.is_alphanumeric() || *curr == '_' {
        error::print_err_ch(*line.lock().unwrap(), *curr); // cannot have alphabetical char after intconst
    }
    if !is_whitespace(curr) {
        unget_char(); // if it isnt whitespace then we need to unget it
    }
    return Token::INTCONST(curr_int); // build the intconst
}

/*
 * this function matches the keywords supplied from C--
 */
fn match_kw(curr: &mut char) -> Token {
    let mut sofar = String::new();
    sofar.push(*curr); // the string sofar needs the curr token

    match *curr {
        'i' => {
            *curr = next_char(false);
            match *curr {
                'f' => {
                    sofar.push(*curr);
                    *curr = peek_char();
                    if (*curr).is_alphanumeric() || *curr == '_' { 
                        return match_id(&mut sofar); // need to match an id
                    }
                    return Token::KW(sofar); // this matches if!
                }
                'n' => {
                    sofar.push(*curr);
                    *curr = next_char(false);
                    if *curr != 't' {
                        unget_char();
                        return match_id(&mut sofar); // need to match an id
                    }
                    sofar.push(*curr);
                    *curr = peek_char();
                    if (*curr).is_alphanumeric() || *curr == '_' {
                        return match_id(&mut sofar); // need to match an id
                    }
                    return Token::KW(sofar); // this matches int!
                }
                _ => {
                    unget_char();
                    return match_id(&mut sofar); // need to match an id
                }
            }
        }
        'e' => {
            *curr = next_char(false);
            if *curr != 'l' {
                unget_char();
                return match_id(&mut sofar); // need to match an id
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 's' {
                unget_char();
                return match_id(&mut sofar); // need to match an id
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 'e' {
                unget_char();
                return match_id(&mut sofar); // need to match an id
            }
            sofar.push(*curr);
            *curr = peek_char();
            if (*curr).is_alphanumeric() || *curr == '_' {
                return match_id(&mut sofar); // need to match an id
            }
            return Token::KW(sofar); // this matches else!
        }
        'r' => {
            *curr = next_char(false);
            if *curr != 'e' {
                unget_char();
                return match_id(&mut sofar); // need to match an id
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 't' {
                unget_char();
                return match_id(&mut sofar); // need to match an id
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 'u' {
                unget_char();
                return match_id(&mut sofar); // need to match an id
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 'r' {
                unget_char();
                return match_id(&mut sofar); // need to match an id
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 'n' {
                unget_char();
                return match_id(&mut sofar); // need to match an id
            }
            sofar.push(*curr);
            *curr = peek_char();
            if (*curr).is_alphanumeric() || *curr == '_' {
                return match_id(&mut sofar); // need to match an id
            }
            return Token::KW(sofar); // this matches return!
        }
        'w' => {
            *curr = next_char(false);
            if *curr != 'h' {
                unget_char();
                return match_id(&mut sofar); // need to match an id
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 'i' {
                unget_char();
                return match_id(&mut sofar); // need to match an id
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 'l' {
                unget_char();
                return match_id(&mut sofar); // need to match an id
            }
            sofar.push(*curr);
            *curr = next_char(false);
            if *curr != 'e' {
                unget_char();
                return match_id(&mut sofar); // need to match an id
            }
            sofar.push(*curr);
            *curr = peek_char();
            if (*curr).is_alphanumeric() || *curr == '_' {
                return match_id(&mut sofar); // need to match an id
            }
            return Token::KW(sofar); // this matches while!
        }
        _ => {
            sofar.pop();
            unget_char();
            return match_id(&mut sofar);
        }
    }
}

/*
    // C CODE
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

    return Token::ID(sofar.to_string()); // build the id
}

/*
 * this function checks if there is a block comment or a line comment
 */ 
fn is_comment(wrapper: &char) -> bool {
    let print_coms = *crate::print_coms.lock().unwrap();
    let mut curr = *wrapper;
    if curr == '/' {
        curr = next_char(false);
        if curr == '*' {
            loop {
                curr = next_char(false);
                if print_coms && curr != '*' {
                    print!("{}", curr);
                }
                while curr == '*' {
                    curr = next_char(false);
                    if curr == '/' {
                        return true;
                    }
                    else if print_coms {
                        print!("{}", curr);
                    }
                }
            }
        } else if curr == '/' {
            println!("line comment");
            curr = next_char(false);
            while curr != '\n' {
                if print_coms {
                    print!("{}", curr);
                }
                curr = next_char(false);
            }
            return true;
        } else {
            unget_char();
            return false;
        }
    }
    false
}
/*
 * tests if the char is whitespace
 */
fn is_whitespace(curr: &char) -> bool {
    *curr == ' ' || *curr == '\n' || *curr == '\r' || *curr == '\t'
}

/*
* moves offset back to "unget" a char
*/
fn unget_char() {
    let out = contents.lock().unwrap()[FA_offset(-1) - 1];
    if out == '\n' { // need to remove a line from lines
        let mut l = line.lock().unwrap();
        *l = *l - 1;
    }
}

/*
 * gets the next char in contents
 */
fn next_char(eof_valid: bool) -> char {
    if contents.lock().unwrap().len() == *(offset.lock().unwrap()) as usize {
        if eof_valid {
            return '\0'; // check if eof is valid and if eof has been encountered
        }
        error::print_eof();
    }
    let out = contents.lock().unwrap()[FA_offset(1)];
    if out == '\n' { // need to increment lines
        let mut l = line.lock().unwrap();
        *l = *l + 1;
    }
    out
}

/*
 * peeks the next char!
 */
fn peek_char() -> char {
    if contents.lock().unwrap().len() == *(offset.lock().unwrap()) as usize {
        error::print_eof();
    }
    contents.lock().unwrap()[FA_offset(0)]
}

/*
 * mutex: atomically adds off to offset and returns the previous val
 */
fn FA_offset(off: i32) -> usize {
    let mut out = offset.lock().unwrap();
    *out = *out + off;
    return (*out - off) as usize;
}

// this is the Token enum
#[derive(PartialEq, Debug, Clone)]
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
