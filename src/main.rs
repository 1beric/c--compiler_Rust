#![allow(nonstandard_style)]
#![allow(dead_code)]

use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

mod error;
mod parser;
mod scanner;

lazy_static! {
    static ref lexeme: Mutex<String> = Mutex::new(String::new());
    static ref chk_decl: Mutex<bool> = Mutex::new(false);
    static ref print_ast: Mutex<bool> = Mutex::new(false);
    static ref gen_code: Mutex<bool> = Mutex::new(false);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    scanner::init(&args[1]);

    parser::parse();
}

// use lazy_static::lazy_static; // 1.4.0
// use std::sync::Mutex;

// lazy_static! {
//     static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
// }

// fn do_a_call() {
//     ARRAY.lock().unwrap().push(1);
// }

// fn main() {
//     do_a_call();
//     do_a_call();
//     do_a_call();

//     println!("called {:?}", ARRAY.lock().unwrap());
// }
