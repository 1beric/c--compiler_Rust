#![allow(nonstandard_style)]
#![allow(dead_code)]

use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

mod error;
mod parser;
mod scanner;

lazy_static! {
    pub static ref chk_decl: Mutex<bool> = Mutex::new(false);
    pub static ref print_ast: Mutex<bool> = Mutex::new(false);
    pub static ref gen_code: Mutex<bool> = Mutex::new(false);
}

fn main() {
    let mut con: Config = Config::new(std::env::args().collect());
    let mut b = chk_decl.lock().unwrap();
    *b = con.chk_decl;
    b = print_ast.lock().unwrap();
    *b = con.print_ast;
    b = gen_code.lock().unwrap();
    *b = con.gen_code;
    scanner::init(&mut con.file);

    parser::parse();
}

struct Config {
    file: String,
    chk_decl: bool,
    print_ast: bool,
    gen_code: bool,
}

impl Config {
    fn new(args: Vec<String>) -> Config {
        let mut file: String = String::new();
        let mut chk_decl_: bool = false;
        let mut print_ast_: bool = false;
        let mut gen_code_: bool = false;
        let mut i: usize = 1;
        while i < args.len() {
            if args[i] == String::from("--chk_decl") {
                chk_decl_ = true;
            } else if args[i] == String::from("--print_ast") {
                print_ast_ = true;
            } else if args[i] == String::from("--gen_code") {
                gen_code_ = true;
            } else {
                file = args[i].to_string();
            }
            i+=1;
        }
        Config { file, chk_decl: chk_decl_, print_ast: print_ast_, gen_code: gen_code_}
    }
}