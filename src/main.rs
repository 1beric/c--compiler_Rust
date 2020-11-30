#![allow(nonstandard_style)]
#![allow(dead_code)]

use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

mod ast;
mod error;
mod parser;
mod scanner;
mod symbol_table;

lazy_static! {
    pub static ref chk_decl: Mutex<bool> = Mutex::new(false);
    pub static ref print_ast: Mutex<bool> = Mutex::new(false);
    pub static ref print_coms: Mutex<bool> = Mutex::new(false);
    pub static ref gen_code: Mutex<bool> = Mutex::new(false);
}

fn main() {
    let mut con: Config = Config::new(std::env::args().collect());
    {
        let mut b = chk_decl.lock().unwrap();
        *b = con.chk_decl;
        b = print_ast.lock().unwrap();
        *b = con.print_ast;
        b = gen_code.lock().unwrap();
        *b = con.gen_code;
        b = print_coms.lock().unwrap();
        *b = con.print_coms;
    }
    scanner::init(&mut con.file);

    // parser::print_tokens();
    parser::parse();
}

struct Config {
    file: String,
    chk_decl: bool,
    print_ast: bool,
    print_coms: bool,
    gen_code: bool,
}

impl Config {
    fn new(args: Vec<String>) -> Config {
        let mut file: String = String::new();
        let mut chk_decl_: bool = false;
        let mut print_ast_: bool = false;
        let mut print_coms_: bool = false;
        let mut gen_code_: bool = false;
        let mut i: usize = 1;
        while i < args.len() {
            if args[i] == String::from("--chk_decl") {
                chk_decl_ = true;
            } else if args[i] == String::from("--print_ast") {
                print_ast_ = true;
            } else if args[i] == String::from("--print_coms") {
                print_coms_ = true;
            } else if args[i] == String::from("--gen_code") {
                gen_code_ = true;
            } else {
                file = args[i].to_string();
            }
            i += 1;
        }
        Config {
            file,
            chk_decl: chk_decl_,
            print_ast: print_ast_,
            print_coms: print_coms_,
            gen_code: gen_code_,
        }
    }
}
