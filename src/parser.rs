/*
 * ./src/parser.rs
 * Brandon Erickson --- brandonscotterickson@gmail.com
 * This file implements the scanner for the C-- language. The language is defined at
 * http://www2.cs.arizona.edu/classes/cs453/fall20/PROJECT/SPEC/cminusminusspec.html#lexical
 * This file contains the logic for parsing the Tokens defined in the struct Token for rules in C--
 */

use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

use crate::ast::ASTNode;
use crate::error;
use crate::scanner;
use crate::scanner::Token;
use crate::symbol_table::SymbolTable;

lazy_static! {
    pub static ref symbols: Mutex<SymbolTable> = Mutex::new(SymbolTable::init_global());
}

/*
 * this function parses the input to check for rules defined in C--
 */
pub fn parse() {
    let mut token = scanner::get_token();

    if *super::chk_decl.lock().unwrap() { // must allow println to be called
        symbols
            .lock()
            .unwrap()
            .add_function(&mut String::from("println"), &mut 1);
    }

    prog(&mut token);
}


/*
 * this function prints the tokens from scanner
 */
pub fn print_tokens() {
    scanner::reset_file();
    loop {
        let tok = scanner::get_token();
        if !print_token(&tok) {
            break;
        }
    }
    scanner::reset_file();
}

// this prints a token using its value
// it also returns whether to continue or not
fn print_token(token: &Token) -> bool {
    match token {
        Token::UNDEF => {
            println!("UNDEF TOKEN");
            return false;
        }
        Token::EOF => {
            println!("END OF FILE");
            return false;
        }
        _ => println!("{:?}", token),
    }
    return true;
}

// this funtion matches a token with a desired token
fn match_token(token: &mut Token, to_match: Token) -> Token {
    // print_token(&to_match);
    // print_token(token);
    // println!();
    match token {
        Token::ID(_) => { // in order to ignore the id value, we need to just match the type
            match to_match {
                Token::ID(_) => {
                    // valid
                    let out = token.clone();
                    *token = scanner::get_token();
                    return out;
                }
                _ => {
                    error::print_err_rule(*scanner::line.lock().unwrap(), token, "match_token");
                    return Token::UNDEF;
                }
            }
        }
        Token::INTCONST(_) => {
            match to_match {
                Token::INTCONST(_) => { // in order to ignore the int value, we need to just match the type
                    // valid
                    let out = token.clone();
                    *token = scanner::get_token();
                    return out;
                }
                _ => {
                    error::print_err_rule(*scanner::line.lock().unwrap(), token, "match_token");
                    return Token::UNDEF;
                }
            }
        }
        _ => {
            if *token == to_match { // we can then use PartialEq to check the rest
                // valid
                let out = token.clone();
                *token = scanner::get_token();
                return out;
            }
            error::print_err_rule(*scanner::line.lock().unwrap(), token, "match_token");
            return Token::UNDEF;
        }
    }
}

// this checks the rule for prog
fn prog(token: &mut Token) {
    if *token == Token::EOF {
        return; // EOF is valid here
    }

    match token {
        Token::KW(kw) => {
            if *kw != String::from("int") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "prog");
            }

            mtype(token);

            let mut id: String; // need to grab the string from id
            match match_token(token, Token::ID(String::new())) {
                Token::ID(s) => id = s,
                _ => id = String::new(),
            }

            func_var(token, &mut id);
            prog(token);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "prog"),
    };
    return;
}

// this checks the rule for func_var
fn func_var(token: &mut Token, id: &mut String) {
    match *token {
        Token::SEMI | Token::COMMA => {
            if *super::chk_decl.lock().unwrap() && symbols.lock().unwrap().global_var_def(id) {
                eprintln!("cannot redefine global var: {}", id);
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "func_var");
            }
            { // need a new block to lock symbols in
                symbols.lock().unwrap().add_global(id);
            }
            var_decl(token, true);
        }
        Token::LPAREN => {
            let mut root = func_defn(token, id);
            if *super::print_ast.lock().unwrap() {
                root.print();
            }
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "func_var"),
    };
}

// this checks the rule for var_decl
fn var_decl(token: &mut Token, global: bool) {
    match *token {
        Token::SEMI => {
            match_token(token, Token::SEMI);
            return;
        }
        Token::COMMA => {
            match_token(token, Token::COMMA);
            match match_token(token, Token::ID(String::new())) {
                Token::ID(mut s) => {
                    if global {
                        // global
                        if *super::chk_decl.lock().unwrap()
                            && symbols.lock().unwrap().global_var_def(&mut s)
                        {
                            eprintln!("cannot redefine global var: {}", s);
                            error::print_err_rule(
                                *scanner::line.lock().unwrap(),
                                token,
                                "var_decl",
                            );
                        }
                        { // need a new block to lock symbols in
                            symbols.lock().unwrap().add_global(&mut s);
                        }
                    } else {
                        // body
                        if *super::chk_decl.lock().unwrap()
                            && symbols.lock().unwrap().body_var_param_def(&mut s)
                        {
                            eprintln!("cannot redefine body var: {}", s);
                            error::print_err_rule(
                                *scanner::line.lock().unwrap(),
                                token,
                                "var_decl",
                            );
                        }
                        { // need a new block to lock symbols in
                            symbols.lock().unwrap().add_body_var(&mut s);
                        }
                    }
                }
                _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "var_decl"),
            }
            var_decl(token, global);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "var_decl"),
    }
}

// this checks the rule for mtype
fn mtype(token: &mut Token) {
    match_token(token, Token::KW(String::from("int")));
}

// this checks the rule for func_defn
fn func_defn(token: &mut Token, id: &mut String) -> ASTNode {
    match *token {
        Token::LPAREN => {
            match_token(token, Token::LPAREN);
            let mut params = Vec::new();
            opt_formals(token, &mut params);
            if *super::chk_decl.lock().unwrap()
                && symbols
                    .lock()
                    .unwrap()
                    .function_def(id, &mut (params.len() as u32))
            {
                eprintln!("cannot redefine function: {}", id);
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "func_defn");
            }
            { // need a new block to lock symbols in
                symbols
                    .lock()
                    .unwrap()
                    .add_function(id, &mut (params.len() as u32));
            }
            for mut param in params.clone() {
                symbols.lock().unwrap().add_param(&mut param);
            }
            match_token(token, Token::RPAREN);
            match_token(token, Token::LBRACE);
            opt_var_decls(token);
            let body = opt_stmt_list(token);
            match_token(token, Token::RBRACE);
            return ASTNode::new_FUNC_DEFN(id.clone(), params, body);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "func_defn"),
    };
    return ASTNode::NULL;
}

// this checks the rule for opt_formals
fn opt_formals(token: &mut Token, params: &mut Vec<String>) {
    match token {
        Token::KW(kw) => {
            if *kw != String::from("int") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_formals");
            }
            mtype(token);
            match match_token(token, Token::ID(String::new())) {
                Token::ID(s) => params.push(s),
                _ => params.push(String::new()),
            }
            formals(token, params);
            return;
        }
        Token::RPAREN => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_formals"),
    }
}

// this checks the rule for formals
fn formals(token: &mut Token, params: &mut Vec<String>) {
    match *token {
        Token::COMMA => {
            match_token(token, Token::COMMA);
            mtype(token);
            match match_token(token, Token::ID(String::new())) {
                Token::ID(s) => params.push(s),
                _ => params.push(String::new()),
            }
            formals(token, params);
            return;
        }
        Token::RPAREN => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "formals"),
    }
}

// this checks the rule for opt_var_decls
fn opt_var_decls(token: &mut Token) {
    match token {
        Token::KW(kw) => {
            if *kw == String::from("if")
                || *kw == String::from("return")
                || *kw == String::from("while")
            {
                return;
            }
            if *kw != String::from("int") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_var_decls");
            }
            mtype(token);
            match match_token(token, Token::ID(String::new())) {
                Token::ID(mut s) => {
                    if *super::chk_decl.lock().unwrap()
                        && symbols.lock().unwrap().global_var_def(&mut s)
                    {
                        eprintln!("cannot redefine global var: {}", &mut s);
                        error::print_err_rule(
                            *scanner::line.lock().unwrap(),
                            token,
                            "opt_var_decls",
                        );
                    }
                    { // need a new block to lock symbols in
                        symbols.lock().unwrap().add_global(&mut s);
                    }
                }
                _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_var_decls"),
            }
            var_decl(token, false);
            opt_var_decls(token);
            return;
        }
        Token::ID(_) | Token::LBRACE | Token::SEMI | Token::RBRACE => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_var_decls"),
    }
}

// this checks the rule for opt_stmt_list
fn opt_stmt_list(token: &mut Token) -> ASTNode {
    match token {
        Token::ID(_) | Token::LBRACE | Token::SEMI => {
            let head = stmt(token);
            let next = opt_stmt_list(token);
            if head == ASTNode::NULL {
                return next;
            }
            return ASTNode::new_STMT_LIST(head, next);
        }
        Token::KW(kw) => {
            if *kw == String::from("return")
                || *kw == String::from("while")
                || *kw == String::from("if")
            {
                let head = stmt(token);
                let next = opt_stmt_list(token);
                if head == ASTNode::NULL {
                    return next;
                }
                return ASTNode::new_STMT_LIST(head, next);
            }
            error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_stmt_list");
        }
        Token::RBRACE => return ASTNode::NULL,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_stmt_list"),
    };
    return ASTNode::NULL;
}

// this checks the rule for stmt
fn stmt(token: &mut Token) -> ASTNode {
    match token {
        Token::ID(_) => match match_token(token, Token::ID(String::new())) {
            Token::ID(mut s) => {
                let node = fn_or_assg(token, &mut s);
                match_token(token, Token::SEMI);
                return node;
            }
            _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "stmt"),
        },
        Token::KW(kw) => {
            if *kw == String::from("return") {
                return return_stmt(token);
            } else if *kw == String::from("while") {
                return while_stmt(token);
            } else if *kw == String::from("if") {
                return if_stmt(token);
            } else {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "stmt");
            }
        }
        Token::LBRACE => {
            match_token(token, Token::LBRACE);
            let list = opt_stmt_list(token);
            match_token(token, Token::RBRACE);
            return list;
        }
        Token::SEMI => {
            match_token(token, Token::SEMI);
            return ASTNode::NULL;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "stmt"),
    };
    return ASTNode::NULL;
}

// this checks the rule for fn_or_assg
fn fn_or_assg(token: &mut Token, id: &mut String) -> ASTNode {
    match *token {
        Token::ASSG => {
            {
                let mut sym = symbols.lock().unwrap();
                if *super::chk_decl.lock().unwrap()
                    && !sym.global_var_def(id)
                    && !sym.body_var_param_def(id)
                {
                    eprintln!("cannot assign to a var that has not been defined: {}", id);
                    error::print_err_rule(*scanner::line.lock().unwrap(), token, "fn_or_assg");
                }
            }
            return assg_stmt(token, id.clone());
        }
        Token::LPAREN => {
            return fn_call(token, id);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "fn_or_assg"),
    };
    return ASTNode::NULL;
}

// this checks the rule for if_stmt
fn if_stmt(token: &mut Token) -> ASTNode {
    match token {
        Token::KW(kw) => {
            if *kw != String::from("if") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "if_stmt");
            }
            match_token(token, Token::KW(String::from("if")));
            match_token(token, Token::LPAREN);
            let condition = or_exp(token);
            match_token(token, Token::RPAREN);
            let then_stmt = stmt(token);
            let else_stmt = opt_else(token);
            return ASTNode::new_IF(condition, then_stmt, else_stmt);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "if_stmt"),
    }
    return ASTNode::NULL;
}

// this checks the rule for opt_else
fn opt_else(token: &mut Token) -> ASTNode {
    match token {
        Token::KW(kw) => {
            if *kw == String::from("if")
                || *kw == String::from("return")
                || *kw == String::from("while")
            {
                return ASTNode::NULL;
            }
            if *kw != String::from("else") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_else");
            }
            match_token(token, Token::KW(String::from("else")));
            return stmt(token);
        }
        Token::ID(_) | Token::LBRACE | Token::SEMI | Token::RBRACE => return ASTNode::NULL,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_else"),
    };
    return ASTNode::NULL;
}

// this checks the rule for while_stmt
fn while_stmt(token: &mut Token) -> ASTNode {
    match token {
        Token::KW(kw) => {
            if *kw != String::from("while") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "while_stmt");
            }
            match_token(token, Token::KW(String::from("while")));
            match_token(token, Token::LPAREN);
            let condition = or_exp(token);
            match_token(token, Token::RPAREN);
            let body = stmt(token);
            return ASTNode::new_WHILE(condition, body);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "while_stmt"),
    }
    return ASTNode::NULL;
}

// this checks the rule for return_stmt
fn return_stmt(token: &mut Token) -> ASTNode {
    match token {
        Token::KW(kw) => {
            if *kw != String::from("return") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "return_stmt");
            }
            match_token(token, Token::KW(String::from("return")));
            let expr = opt_arith_exp(token);
            match_token(token, Token::SEMI);
            return ASTNode::new_RETURN(expr);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "return_stmt"),
    }
    return ASTNode::NULL;
}

// this checks the rule for assg_stmt
fn assg_stmt(token: &mut Token, id: String) -> ASTNode {
    match *token {
        Token::ASSG => {
            match_token(token, Token::ASSG);
            return ASTNode::new_ASSG(id, addsub_exp(token));
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "assg_stmt"),
    }
    return ASTNode::NULL;
}

// this checks the rule for opt_fn_call
fn opt_fn_call(token: &mut Token, id: &mut String) -> ASTNode {
    match *token {
        Token::LPAREN => {
            return fn_call(token, id);
        }
        Token::ARITH(_) | Token::BOOL(_) | Token::RPAREN | Token::COMMA | Token::SEMI => {
            return ASTNode::new_ID(id.clone());
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_fn_call"),
    }
    return ASTNode::new_ID(id.clone());
}

// this checks the rule for fn_call
fn fn_call(token: &mut Token, id: &mut String) -> ASTNode {
    match *token {
        Token::LPAREN => {
            match_token(token, Token::LPAREN);
            let mut nargs = 0;
            let args = opt_expr_list(token, &mut nargs);
            if *super::chk_decl.lock().unwrap()
                && !symbols.lock().unwrap().function_def(id, &mut nargs)
            {
                eprintln!(
                    "cannot call a function that has not been defined: {}, {}",
                    id, nargs
                );
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "fn_call");
            }
            match_token(token, Token::RPAREN);
            return ASTNode::new_FUNC_CALL(id.clone(), args);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "fn_call"),
    }
    return ASTNode::NULL;
}

// this checks the rule for opt_expr_list
fn opt_expr_list(token: &mut Token, nargs: &mut u32) -> ASTNode {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            let head = addsub_exp(token);
            *nargs = *nargs + 1;
            let next = expr_list(token, nargs);
            return ASTNode::new_EXPR_LIST(head, next);
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_expr_list");
            }
            let head = addsub_exp(token);
            *nargs = *nargs + 1;
            let next = expr_list(token, nargs);
            return ASTNode::new_EXPR_LIST(head, next);
        }
        Token::RPAREN => return ASTNode::NULL,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_expr_list"),
    }
    return ASTNode::NULL;
}

// this checks the rule for expr_list
fn expr_list(token: &mut Token, nargs: &mut u32) -> ASTNode {
    match *token {
        Token::COMMA => {
            match_token(token, Token::COMMA);
            let head = addsub_exp(token);
            *nargs = *nargs + 1;
            let next = expr_list(token, nargs);
            return ASTNode::new_EXPR_LIST(head, next);
        }
        Token::RPAREN => return ASTNode::NULL,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "expr_list"),
    }
    return ASTNode::NULL;
}

// this checks the rule for or_exp
fn or_exp(token: &mut Token) -> ASTNode {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            let left = and_exp(token);
            return or_no_lr(token, left);
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "or_exp");
            }
            let left = and_exp(token);
            return or_no_lr(token, left);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "or_exp"),
    }
    return ASTNode::NULL;
}

// this checks the rule for or_no_lr
fn or_no_lr(token: &mut Token, left: ASTNode) -> ASTNode {
    match token {
        Token::BOOL(bl) => {
            if *bl != String::from("||") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "or_no_lr");
            }
            match_token(token, Token::BOOL(String::from("||")));
            let and_expr = and_exp(token);
            return or_no_lr(token, ASTNode::new_BOOL(String::from("||"), left, and_expr));
        }
        Token::RPAREN => return left,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "or_no_lr"),
    }
    return left;
}

// this checks the rule for and_exp
fn and_exp(token: &mut Token) -> ASTNode {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            let bool_expr = bool_exp(token);
            return and_no_lr(token, bool_expr);
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "and_exp");
            }
            let bool_expr = bool_exp(token);
            return and_no_lr(token, bool_expr);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "and_exp"),
    }
    return ASTNode::NULL;
}

// this checks the rule for and_no_lr
fn and_no_lr(token: &mut Token, left: ASTNode) -> ASTNode {
    match token {
        Token::BOOL(bl) => {
            if *bl != String::from("&&") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "and_no_lr");
            }
            match_token(token, Token::BOOL(String::from("&&")));
            let bool_expr = bool_exp(token);
            return and_no_lr(
                token,
                ASTNode::new_BOOL(String::from("&&"), left, bool_expr),
            );
        }
        Token::RPAREN => return left,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "and_no_lr"),
    }
    return left;
}

// this checks the rule for bool_exp
fn bool_exp(token: &mut Token) -> ASTNode {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            let op1 = addsub_exp(token);
            let op = relop(token);
            let op2 = addsub_exp(token);
            return ASTNode::new_BOOL(op, op1, op2);
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "bool_exp");
            }
            let op1 = addsub_exp(token);
            let op = relop(token);
            let op2 = addsub_exp(token);
            return ASTNode::new_BOOL(op, op1, op2);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "bool_exp"),
    }
    return ASTNode::NULL;
}

// this checks the rule for opt_arith_exp
fn opt_arith_exp(token: &mut Token) -> ASTNode {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            return addsub_exp(token);
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_arith_exp");
            }
            return addsub_exp(token);
        }
        Token::SEMI => return ASTNode::NULL,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_arith_exp"),
    }
    return ASTNode::NULL;
}

// this checks the rule for addsub_exp
fn addsub_exp(token: &mut Token) -> ASTNode {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            let left = muldiv_exp(token);
            return addsub_no_lr(token, left);
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "addsub_exp");
            }
            let left = muldiv_exp(token);
            return addsub_no_lr(token, left);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "addsub_exp"),
    }
    return ASTNode::NULL;
}

// this checks the rule for addsub_no_lr
fn addsub_no_lr(token: &mut Token, left: ASTNode) -> ASTNode {
    match token.clone() {
        Token::ARITH(ar) => {
            if *ar == String::from("+") {
                match_token(token, Token::ARITH(String::from("+")));
            } else if *ar == String::from("-") {
                match_token(token, Token::ARITH(String::from("-")));
            } else {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "addsub_no_lr");
            }
            let op2 = muldiv_exp(token);
            return addsub_no_lr(token, ASTNode::new_ARITH(ar.clone(), left, op2));
        }
        Token::RPAREN | Token::COMMA | Token::SEMI | Token::BOOL(_) => return left,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "addsub_no_lr"),
    }
    return left;
}

// this checks the rule for muldiv_exp
fn muldiv_exp(token: &mut Token) -> ASTNode {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            let left = arith_exp(token);
            return muldiv_no_lr(token, left);
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "muldiv_exp");
            }
            let left = arith_exp(token);
            return muldiv_no_lr(token, left);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "muldiv_exp"),
    }
    return ASTNode::NULL;
}

// this checks the rule for muldiv_no_lr
fn muldiv_no_lr(token: &mut Token, left: ASTNode) -> ASTNode {
    match token.clone() {
        Token::ARITH(ar) => {
            if *ar == String::from("*") {
                match_token(token, Token::ARITH(String::from("*")));
            } else if *ar == String::from("/") {
                match_token(token, Token::ARITH(String::from("/")));
            } else {
                return left;
            }
            let op2 = arith_exp(token);
            return muldiv_no_lr(token, ASTNode::new_ARITH(ar.clone(), left, op2));
        }
        Token::RPAREN | Token::COMMA | Token::SEMI | Token::BOOL(_) => return left,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "muldiv_no_lr"),
    }
    return left;
}

// this checks the rule for arith_exp
fn arith_exp(token: &mut Token) -> ASTNode {
    match token.clone() {
        Token::ID(_) => match match_token(token, Token::ID(String::new())) {
            Token::ID(mut s) => {
                return opt_fn_call(token, &mut s);
            }
            _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "arith_exp"),
        },
        Token::INTCONST(val) => {
            match_token(token, Token::INTCONST(0));
            return ASTNode::new_INTCONST(val);
        }
        Token::LPAREN => {
            match_token(token, Token::LPAREN);
            let expr = addsub_exp(token);
            match_token(token, Token::RPAREN);
            return expr;
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "arith_exp");
            }
            match_token(token, Token::ARITH(String::from("-")));
            let expr = arith_exp(token);
            return ASTNode::new_ARITH(String::from("UMINUS"), expr, ASTNode::NULL);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "arith_exp"),
    };
    return ASTNode::NULL;
}

// this checks the rule for relop
fn relop(token: &mut Token) -> String {
    match token.clone() {
        Token::BOOL(op) => match op.as_str() {
            ">" => {
                match_token(token, Token::BOOL(String::from(">")));
                return op.clone();
            }
            ">=" => {
                match_token(token, Token::BOOL(String::from(">=")));
                return op.clone();
            }
            "<" => {
                match_token(token, Token::BOOL(String::from("<")));
                return op.clone();
            }
            "<=" => {
                match_token(token, Token::BOOL(String::from("<=")));
                return op.clone();
            }
            "!=" => {
                match_token(token, Token::BOOL(String::from("!=")));
                return op.clone();
            }
            "==" => {
                match_token(token, Token::BOOL(String::from("==")));
                return op.clone();
            }
            _ => {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "relop");
                return String::new();
            }
        },
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "relop"),
    };
    return String::new();
}
