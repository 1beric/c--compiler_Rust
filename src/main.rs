/*
 * ./src/main.rs
 * Brandon Erickson --- brandonscotterickson@gmail.com
 * This file implements what is called the compiler driver for the C-- language. The language is defined at
 * http://www2.cs.arizona.edu/classes/cs453/fall20/PROJECT/SPEC/cminusminusspec.html#lexical
 * This file contains the logic for parsing the command line args, which are:
 *      <filename>
 *          This is the name of the file to parse.
 *      [--chk_decl]
 *          This dictates whether or not a symbol table will be used to
 *          check the semantics of the file and print errors to stderr.
 *      [--print_coms]
 *          This dictates whether or not to print out the comments to the
 *          command line when parsing the file.
 *      [--print_ast]
 *          This dictates whether or not to print out the abstract syntax
 *          tree that will be created while parsing the file.
 * It is best to run this program using cargo, so an example command-line instruction is:
 *      cargo run ./tests/t1.c --print_ast
 * which will generate the AST structure for the file located at ./tests/t1.c
 */

// these are defined to tell the rust compiler that we do not want to use Rust's community style,
// and that some code will not be used because it can be altered later to be turned on for debugging.
#![allow(nonstandard_style)]
#![allow(dead_code)]

// this is used for static-global variables with thread-safety
use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

// need to manually define modules for each file in the directory
mod ast;
mod error;
mod parser;
mod scanner;
mod symbol_table;

// this defines the command line argument flags that are accessible in other files
lazy_static! {
    pub static ref chk_decl: Mutex<bool> = Mutex::new(false);
    pub static ref print_coms: Mutex<bool> = Mutex::new(false);
    pub static ref print_ast: Mutex<bool> = Mutex::new(false);
}

// this is the main function of the program
fn main() {
    // first we parse the command line args to get a Config struct
    let mut con: Config = Config::new(std::env::args().collect());
    // in order to lock these global Mutexs, we need them to be dropped before
    // we use them later in the program, so we must make the scope stricter
    {
        let mut b = chk_decl.lock().unwrap();
        *b = con.chk_decl;
        b = print_ast.lock().unwrap();
        *b = con.print_ast;
        b = print_coms.lock().unwrap();
        *b = con.print_coms;
    }

    // this inits the scanner with the file and allows it to read in the entire file
    scanner::init(&mut con.file);

    // this is a debug statement that will print the tokens of the file if desired
    // parser::print_tokens();

    // finally, we need to parse the file's contents
    parser::parse();
}

/*
 * Config is a struct that holds the flags from the command line arguments
 */
struct Config {
    file: String,
    chk_decl: bool,
    print_ast: bool,
    print_coms: bool,
}

impl Config {
    /*
     * this is the constructor for the Config struct
     *  args: Vec<String> -- this is the Vec of Strings from the command line
     */
    fn new(args: Vec<String>) -> Config {
        let mut file: String = String::new();
        let mut chk_decl_: bool = false;
        let mut print_ast_: bool = false;
        let mut print_coms_: bool = false;
        let mut i: usize = 1;
        while i < args.len() {
            if args[i] == String::from("--chk_decl") {
                chk_decl_ = true;
            } else if args[i] == String::from("--print_ast") {
                print_ast_ = true;
            } else if args[i] == String::from("--print_coms") {
                print_coms_ = true;
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
        }
    }
}
